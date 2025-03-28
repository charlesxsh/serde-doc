pub mod markdown;
pub mod jsonschema;
use anyhow::{bail, Result};

use crate::Context;

pub struct GeneratorConfig {
    pub output: String,
}
pub trait Generator {
    fn generate(&self, ctx: &Context, config:&GeneratorConfig) -> Result<()>;
}

pub fn get_generator(name: &str) -> Result<Box<dyn Generator>> {
    match name {
        "markdown" => Ok(Box::new(markdown::MarkdownGenerator::new())),
        "jsonschema" => Ok(Box::new(jsonschema::JsonSchemaGenerator::new())),
        _ => bail!("Unknown generator: {}", name),
    }
}