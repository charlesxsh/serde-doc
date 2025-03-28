

use std::io::Write;

use crate::Context;

use super::{Generator, GeneratorConfig};
use anyhow::{Context as _, Result};


pub struct JsonSchemaGenerator {}

impl JsonSchemaGenerator {
    pub fn new() -> Self {
        JsonSchemaGenerator {}
    }
}

impl Generator for JsonSchemaGenerator {
    fn generate(&self, ctx: &Context, config:&GeneratorConfig) -> Result<()> {
        let mut content = String::new();

        match &config.output {
            Some(output) => {
              
                let mut file = std::fs::File::create(&output)?;

                file.write(content.as_bytes())
                    .with_context(|| format!("failed to write to file: {:?}", output))?;
                
            }
            None => {
                println!("{}", content);
            }
        }
        Ok(())
    }
}
