use std::path::Path;

use anyhow::{ Context as _, Result};

use crate::{Context, FieldUnit, FileUnit, StructUnit};

pub fn process_path<S: AsRef<Path>>(context: &mut Context, p: S) -> Result<()> {
    let path = p.as_ref();
    if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {

        let code = std::fs::read_to_string(path)?;
        let unit = process_code(code).with_context(|| format!("failed to parse file: {:?}", path))?;
        context.files.push(unit);
    } else if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() {
                process_path(context, entry.path())?;
            } else if entry_path.is_dir() {
                process_path(context, entry.path())?;
            }
        }
    }
    Ok(())
}

fn process_code<S: AsRef<str>>(code: S) -> Result<FileUnit> {
    let tree: syn::File = syn::parse_str(code.as_ref()).context("failed to parse code")?;
    let mut unit = FileUnit::new();
    for item in &tree.items {
        if let syn::Item::Struct(item_struct) = item {
            unit.structs.push(process_struct(item_struct).context("failed to parse struct")?);
        }
    }
    Ok(unit)
}

fn process_struct(item_struct: &syn::ItemStruct) -> Result<StructUnit> {
    let mut struct_unit = StructUnit::new(
        item_struct.ident.to_string(),
    );

    if let Some(comment) = item_struct.attrs.iter().find(|attr| attr.path().is_ident("doc")) {
        if let Ok(name) = comment.meta.require_name_value() {
            if let syn::Expr::Lit(expr_list) = &name.value {
                if let syn::Lit::Str(lit) = &expr_list.lit {
                    struct_unit.doc = Some(lit.value());
                }
            }
        }
    }

    if let syn::Fields::Named(fields) = &item_struct.fields {
        for field in &fields.named {
            let name = field.ident.as_ref().unwrap().to_string();
            let ty = match &field.ty {
                syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
                _ => "Unknown".to_string(),
            };
            let doc:Option<String> = field.attrs.iter()
            .find(|attr| attr.path().is_ident("doc"))
            .and_then(|attr|{
                if let Ok(name) = attr.meta.require_name_value() {
                    if let syn::Expr::Lit(expr_list) = &name.value {
                        if let syn::Lit::Str(lit) = &expr_list.lit {
                            return Some(lit.value());
                        }
                    }
                    
                }
               None
            });
            
            
            struct_unit.fields.push(FieldUnit::new (
                name,
                ty,
                doc
            ));
        }
    }

    for attr in &item_struct.attrs {
        if attr.path().is_ident("derive") {
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    struct_unit.derive.push(ident.to_string());
                } 

                Ok(())
            })?;            
        }
    }

    Ok(struct_unit)
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_struct() -> Result<()> {
        let code = r#"
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
"#;
        let unit = process_code(code)?;

        assert_eq!(unit.structs.len(), 1);
        let struct_unit = &unit.structs[0];
        assert_eq!(struct_unit.name, "Point");
        assert_eq!(struct_unit.fields.len(), 2);
        assert_eq!(struct_unit.fields[0].name, "x");
        assert_eq!(struct_unit.fields[0].ty, "i32");
        assert_eq!(struct_unit.fields[1].name, "y");
        assert_eq!(struct_unit.fields[1].ty, "i32");
        assert_eq!(struct_unit.derive.len(), 3);
        assert_eq!(struct_unit.derive[0], "Serialize");
        Ok(())
    }

    #[test]
    fn test_parse_struct_with_comment() -> Result<()> {
        let code = r#"
#[derive(Serialize, Deserialize, Debug)]
/// This is a point struct
struct Point {
    /// The x coordinate
    x: i32,
    /// The y coordinate
    y: i32, 
}
"#;
        let unit = process_code(code)?;

        assert_eq!(unit.structs.len(), 1);
        let struct_unit = &unit.structs[0];
        assert_eq!(struct_unit.name, "Point");
        assert_eq!(struct_unit.fields.len(), 2);
        assert_eq!(struct_unit.fields[0].name, "x");
        assert_eq!(struct_unit.fields[0].ty, "i32");
        assert_eq!(struct_unit.fields[1].name, "y");
        assert_eq!(struct_unit.fields[1].ty, "i32");
        assert_eq!(struct_unit.derive.len(), 3);
        assert_eq!(struct_unit.derive[0], "Serialize");
        assert_eq!(struct_unit.doc, Some(" This is a point struct".to_string()));
        assert_eq!(struct_unit.fields[0].doc, Some(" The x coordinate".to_string()));
        assert_eq!(struct_unit.fields[1].doc, Some(" The y coordinate".to_string()));

        Ok(())
    }
}