use darling::{FromDeriveInput, FromVariant};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span, TokenTree};
use quote::quote;

pub fn derive_fix_value(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let darling_context = DataFieldWithVariants::from_derive_input(&ast).unwrap();
    let identifier = darling_context.ident;
    let matching_cases = darling_context
        .data
        .clone()
        .map_enum_variants(|enum_variant| {
            let enum_discriminant = enum_variant.variant.as_str();
            let enum_discriminant_len = enum_variant.variant.as_bytes().len();
            let enum_variant = enum_variant.ident;
            quote! {
                Self::#enum_variant => {
                    buffer.extend_from_slice(#enum_discriminant.as_bytes());
                    #enum_discriminant_len
                },
            }
        })
        .take_enum()
        .expect("Invalid enum");
    let deserialize_matching_cases = darling_context
        .data
        .map_enum_variants(|enum_variant| {
            let enum_discriminant = enum_variant.variant.as_str();
            let enum_variant = enum_variant.ident;
            let bstring: proc_macro2::TokenStream =
                TokenTree::from(Literal::byte_string(enum_discriminant.as_bytes())).into();
            quote! {
                #bstring => Ok(Self::#enum_variant)
            }
        })
        .take_enum()
        .expect("Invalid enum");
    let minifix_crate_info = proc_macro_crate::crate_name("minifix").expect("Cargo.toml minifix issues");
    let minifix_crate_name = match minifix_crate_info {
        proc_macro_crate::FoundCrate::Itself => Ident::new("crate", Span::call_site()),
        proc_macro_crate::FoundCrate::Name(s) => Ident::new(s.as_str(), Span::call_site()),
    };
    let generated = quote! {
        impl<'a> FieldType<'a> for #identifier {
            type Error = ();
            type SerializeSettings = ();

            fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
            where
                B: #minifix_crate_name::Buffer,
            {
                match self {
                    #(#matching_cases)*
                }
            }

            fn deserialize(data: &'a [u8]) -> ::std::result::Result<Self, <Self as FieldType<'a>>::Error> {
                match data {
                    #(#deserialize_matching_cases),*,
                    _ => ::std::result::Result::Err(())
                }
            }
        }
    };
    generated.into()
}

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(minifix))]
struct EnumVariantInfo {
    ident: syn::Ident,
    variant: String,
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(minifix))]
struct DataFieldWithVariants {
    ident: syn::Ident,
    data: darling::ast::Data<EnumVariantInfo, darling::util::Ignored>,
}
