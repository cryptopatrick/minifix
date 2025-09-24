use heck::{ToPascalCase, ToShoutySnakeCase};
use minifix_dictionary::{Dictionary, TagU32};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

const MINIFIX_VERSION: &str = "0.1.0";

/// Final optimized code generator targeting <3,000 lines with maximum compression
pub struct FinalOptimizedCodegen {
    dictionary: Dictionary,
}

impl FinalOptimizedCodegen {
    pub fn new(dictionary: Dictionary) -> Self {
        Self { dictionary }
    }

    /// Generate the most compact code possible while maintaining API compatibility
    pub fn generate(&self) -> String {
        let notice = self.generated_code_notice();
        let imports = self.generate_imports();
        let field_definitions_data = self.generate_field_definitions_data();
        let field_constants_macro = self.generate_field_constants_macro();
        let compact_enum_definitions =
            self.generate_compact_enum_definitions();
        let constants_invocation = self.generate_constants_invocation();

        format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
            notice,
            imports,
            field_definitions_data,
            field_constants_macro,
            constants_invocation,
            compact_enum_definitions
        )
    }

    fn generated_code_notice(&self) -> String {
        format!(
            "// Generated automatically by miniFIX {} on {}.\n// ULTRA-COMPACT VERSION - Reduced from 13,900 to <3,000 lines.\n//",
            MINIFIX_VERSION,
            chrono::Utc::now().to_rfc2822(),
        )
    }

    fn generate_imports(&self) -> String {
        "use minifix::dict::{FieldLocation, FixDatatype};\nuse minifix::definitions::HardCodedFixFieldDefinition;\nuse minifix::FieldType;".to_string()
    }

    /// Generate ultra-compact field definitions
    fn generate_field_definitions_data(&self) -> String {
        let mut field_tuples = Vec::new();

        for field in self.dictionary.fields().iter() {
            let location = self.determine_field_location(field.tag());
            let data_type = format!("{:?}", field.data_type().basetype())
                .replace("dict::", "");

            field_tuples.push(format!(
                "(\"{}\",{},FixDatatype::{},FieldLocation::{})",
                field.name(),
                field.tag().get(),
                data_type,
                location
            ));
        }

        format!(
            "const FIELDS: &[(&str, u32, FixDatatype, FieldLocation)] = &[{}];",
            field_tuples.join(",")
        )
    }

    /// Generate macro for field constants
    fn generate_field_constants_macro(&self) -> String {
        "macro_rules! field_constants {\n    ($($name:ident = $idx:expr),*) => {\n        $(pub const $name: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {\n            name: FIELDS[$idx].0,\n            tag: FIELDS[$idx].1,\n            data_type: FIELDS[$idx].2,\n            location: FIELDS[$idx].3,\n        };)*\n    };\n}".to_string()
    }

    /// Generate the constants invocation
    fn generate_constants_invocation(&self) -> String {
        let mut constant_mappings = Vec::new();

        for (index, field) in self.dictionary.fields().iter().enumerate() {
            let const_name = field.name().to_shouty_snake_case();
            constant_mappings.push(format!("{} = {}", const_name, index));
        }

        // Split into chunks to avoid exceeding macro limits
        let chunk_size = 100;
        let chunks: Vec<_> = constant_mappings.chunks(chunk_size).collect();

        let mut invocations = Vec::new();
        for chunk in chunks {
            invocations
                .push(format!("field_constants! {{ {} }}", chunk.join(", ")));
        }

        invocations.join("\n")
    }

    /// Generate ultra-compact enum definitions
    fn generate_compact_enum_definitions(&self) -> String {
        let mut enum_definitions = Vec::new();

        for field in self.dictionary.fields().iter() {
            if let Some(enums) = field.enums() {
                let enum_name = field.name().to_pascal_case();
                let mut variants = Vec::new();

                for enum_val in enums {
                    let variant_name = self.sanitize_variant_name(
                        enum_val.description().to_pascal_case(),
                    );
                    variants.push(format!(
                        "#[minifix(variant=\"{}\")] {}",
                        enum_val.value(),
                        variant_name
                    ));
                }

                enum_definitions.push(format!(
                    "#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,FieldType)] pub enum {} {{{}}}",
                    enum_name,
                    variants.join(",")
                ));
            }
        }

        enum_definitions.join("\n")
    }

    fn determine_field_location(&self, tag: TagU32) -> &'static str {
        match tag.get() {
            1..=50 => "Header",
            89..=93 => "Trailer",
            _ => "Body",
        }
    }

    fn sanitize_variant_name(&self, name: String) -> String {
        let mut sanitized = name;
        if !sanitized.chars().next().unwrap_or('_').is_ascii_alphabetic() {
            sanitized = format!("_{}", sanitized);
        }
        sanitized
    }
}

fn main() -> io::Result<()> {
    let path = project_root().join("src").join("generated_fix44.rs");
    let mut file = File::create(path)?;
    let fix_dictionary = Dictionary::fix44();
    let rust_code = {
        let codegen = FinalOptimizedCodegen::new(fix_dictionary);
        codegen.generate()
    };
    file.write_all(rust_code.as_bytes())?;
    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
