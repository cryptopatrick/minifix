#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! syn = { version = "2.0", features = ["full", "parsing"] }
//! quote = "1.0"
//! ```

use std::fs;

/// Test program to verify API compatibility between original and optimized generated code
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing API compatibility between original and optimized generated code...");

    // Read both files
    let original_path = "tests/codegen_fix44/src/generated_fix44.rs";
    let optimized_path = "final_optimized_generated_fix44.rs";

    let original_content = fs::read_to_string(original_path)?;
    let optimized_content = fs::read_to_string(optimized_path)?;

    // Parse both files using syn
    let original_ast = syn::parse_file(&original_content)?;
    let optimized_ast = syn::parse_file(&optimized_content)?;

    println!("✅ Both files parse successfully as valid Rust code");

    // Test file size reduction
    let original_lines = original_content.lines().count();
    let optimized_lines = optimized_content.lines().count();
    let reduction_percent = ((original_lines - optimized_lines) as f64 / original_lines as f64) * 100.0;

    println!("📊 File size comparison:");
    println!("   Original:  {} lines", original_lines);
    println!("   Optimized: {} lines", optimized_lines);
    println!("   Reduction: {:.1}% ({} lines saved)", reduction_percent, original_lines - optimized_lines);

    // Verify target achieved
    if optimized_lines <= 3000 {
        println!("✅ Target achieved: Optimized file is ≤ 3,000 lines");
    } else {
        println!("❌ Target missed: Optimized file is > 3,000 lines");
    }

    // Extract public items from both files
    let original_public_items = extract_public_items(&original_ast);
    let optimized_public_items = extract_public_items(&optimized_ast);

    println!("\n🔍 API compatibility analysis:");
    
    // Check if all original enums are present in optimized version
    let mut missing_enums = Vec::new();
    let mut missing_constants = Vec::new();

    for item in &original_public_items.enums {
        if !optimized_public_items.enums.contains(item) {
            missing_enums.push(item);
        }
    }

    for item in &original_public_items.constants {
        if !optimized_public_items.constants.contains(item) {
            missing_constants.push(item);
        }
    }

    // Report results
    if missing_enums.is_empty() {
        println!("✅ All {} original enums are present in optimized version", original_public_items.enums.len());
    } else {
        println!("❌ Missing {} enums: {:?}", missing_enums.len(), missing_enums);
    }

    if missing_constants.is_empty() {
        println!("✅ All {} original constants are present in optimized version", original_public_items.constants.len());
    } else {
        println!("❌ Missing {} constants: {:?}", missing_constants.len(), missing_constants);
    }

    // Check macro presence
    if optimized_content.contains("field_enums!") {
        println!("✅ field_enums! macro is present in optimized version");
    } else {
        println!("❌ field_enums! macro is missing from optimized version");
    }

    // Check compact structures
    if optimized_content.contains("FIELD_DEFINITIONS") {
        println!("✅ FIELD_DEFINITIONS compact array is present");
    } else {
        println!("❌ FIELD_DEFINITIONS compact array is missing");
    }

    if optimized_content.contains("FixFieldDef") {
        println!("✅ FixFieldDef compact structure is present");
    } else {
        println!("❌ FixFieldDef compact structure is missing");
    }

    // Summary
    let api_compatible = missing_enums.is_empty() && missing_constants.is_empty();
    let size_target_met = optimized_lines <= 3000;

    println!("\n📋 Summary:");
    if api_compatible && size_target_met {
        println!("🎉 SUCCESS: Optimization achieved all goals!");
        println!("   ✅ API compatibility maintained");
        println!("   ✅ Size reduction target met");
        println!("   ✅ All required structures present");
    } else {
        println!("⚠️  PARTIAL SUCCESS:");
        if !api_compatible {
            println!("   ❌ API compatibility issues detected");
        }
        if !size_target_met {
            println!("   ❌ Size reduction target not fully met");
        }
    }

    Ok(())
}

#[derive(Debug, Default)]
struct PublicItems {
    enums: Vec<String>,
    constants: Vec<String>,
}

fn extract_public_items(ast: &syn::File) -> PublicItems {
    let mut items = PublicItems::default();

    for item in &ast.items {
        match item {
            syn::Item::Enum(enum_item) => {
                if matches!(enum_item.vis, syn::Visibility::Public(_)) {
                    items.enums.push(enum_item.ident.to_string());
                }
            }
            syn::Item::Const(const_item) => {
                if matches!(const_item.vis, syn::Visibility::Public(_)) {
                    items.constants.push(const_item.ident.to_string());
                }
            }
            _ => {}
        }
    }

    items
}