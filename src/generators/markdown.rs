use std::io::Write;

use crate::Context;

use super::{Generator, GeneratorConfig};
use anyhow::Result;


pub struct MarkdownGenerator {}

impl MarkdownGenerator {
    pub fn new() -> Self {
        MarkdownGenerator {}
    }
}


impl Generator for MarkdownGenerator {
    fn generate(&self, ctx: &Context, config: &GeneratorConfig) -> Result<()> {
        let mut content = String::new();
        // content.push_str(&format!("# {} Documentation\n", ctx.name));
        content.push_str("## Structs\n");
        for file_unit in &ctx.files {
            for struct_unit in &file_unit.structs { 

                content.push_str(&format!("### {}\n", struct_unit.name));
                if !struct_unit.derive.is_empty() {
                content.push_str(&format!("Derives: {}\n", struct_unit.derive.join(", ")));
                }
                if let Some(comment) = &struct_unit.doc {
                content.push_str(&format!("{}\n", comment));
                }
                content.push_str("Fields:\n");
                for field in &struct_unit.fields {
                    content.push_str(&format!("- {}: {}\n", field.name, field.ty));
                    if let Some(comment) = &field.doc {
                        content.push_str(&format!("  - {}\n", comment));
                    }
                }
            }
        }
        let mut file = std::fs::File::create(&config.output)?;

        file.write(content.as_bytes())?;
        Ok(())
    }
}