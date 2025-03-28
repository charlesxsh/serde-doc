

use std::io::Write;

use crate::{Context, StructUnit};

use super::{Generator, GeneratorConfig};
use anyhow::{Context as _, Result};
use serde_json::{json, Value};


pub struct JsonSchemaGenerator {}

impl JsonSchemaGenerator {
    pub fn new() -> Self {
        JsonSchemaGenerator {}
    }
}

impl Generator for JsonSchemaGenerator {
    fn generate(&self, ctx: &Context, config:&GeneratorConfig) -> Result<()> {
        let mut content = String::new();
        let structs = ctx.files.iter()
        .flat_map(|file| file.structs.iter())
        .filter(|s| {
            if let Some(ref structs) = config.structs {
                structs.contains(&s.name)
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
        
        let json_schema = generate_json_schema(&structs);
        content.push_str(&json_schema.to_string());
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



pub fn generate_json_schema(structs: &Vec<&StructUnit>) -> Value {
    let mut definitions = serde_json::Map::new();

    for s in structs {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        for field in &s.fields {
            let field_schema = match field.ty.as_str() {
                "String" => json!({ "type": "string" }),
                "i32" | "i64" => json!({ "type": "integer" }),
                "f32" | "f64" => json!({ "type": "number" }),
                "bool" => json!({ "type": "boolean" }),
                _ => json!({ "$ref": format!("#/definitions/{}", field.ty) }),
            };

            let mut field_entry = field_schema;

            if let Some(doc) = &field.doc {
                field_entry["description"] = json!(doc);
            }

            properties.insert(field.name.clone(), field_entry);
            required.push(field.name.clone());
        }

        let mut struct_schema = json!({
            "type": "object",
            "properties": properties,
            "required": required,
        });

        if let Some(doc) = &s.doc {
            struct_schema["description"] = json!(doc);
        }

        definitions.insert(s.name.clone(), struct_schema);
    }

    json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": definitions
    })
}
