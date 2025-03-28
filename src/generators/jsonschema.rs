

use std::io::Write;

use crate::Context;

use super::{Generator, GeneratorConfig};
use anyhow::Result;


pub struct JsonSchemaGenerator {}

impl JsonSchemaGenerator {
    pub fn new() -> Self {
        JsonSchemaGenerator {}
    }
}

impl Generator for JsonSchemaGenerator {
    fn generate(&self, ctx: &Context, config:&GeneratorConfig) -> Result<()> {
        let mut content = String::new();

        let mut file = std::fs::File::create(&config.output)?;
        file.write(content.as_bytes())?;
        Ok(())
    }
}
