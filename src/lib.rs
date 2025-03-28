
pub mod extract;
pub mod generators;

pub struct Context {
    pub files: Vec<FileUnit>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
        }
    }
    
}

pub struct FileUnit {
    pub structs: Vec<StructUnit>,
    pub doc: Option<String>,
}

impl FileUnit {
    pub fn new() -> Self {
        Self {
            structs: Vec::new(),
            doc: None,
        }
    }
}

pub struct FieldUnit {
    pub name: String,
    pub ty: String,
    pub doc: Option<String>,
}

impl FieldUnit {
    pub fn new(name: String, ty: String, doc: Option<String>) -> Self {
        Self { name, ty, doc }
    }
}

pub struct StructUnit {
    pub name: String,
    pub fields: Vec<FieldUnit>,
    pub derive: Vec<String>,
    pub doc: Option<String>,
}

impl StructUnit {
    pub fn new(name: String) -> Self {
        Self { name, fields:Default::default(), derive:Default::default(), doc: None }
    }
}
