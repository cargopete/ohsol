use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IdlError {
    pub code: u32,
    pub name: String,
    #[serde(default)]
    pub msg: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IdlMetadata {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub spec: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Idl {
    #[serde(default)]
    pub address: String,

    #[serde(default)]
    pub metadata: Option<IdlMetadata>,

    // Legacy format fields
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub version: String,

    #[serde(default)]
    pub errors: Vec<IdlError>,
}

impl Idl {
    pub fn is_modern_format(&self) -> bool {
        !self.address.is_empty() && self.metadata.is_some()
    }

    pub fn get_name(&self) -> String {
        if let Some(metadata) = &self.metadata {
            metadata.name.clone()
        } else {
            self.name.clone()
        }
    }

    pub fn get_version(&self) -> String {
        if let Some(metadata) = &self.metadata {
            metadata.version.clone()
        } else {
            self.version.clone()
        }
    }
}

pub fn parse_idl(json: &str) -> anyhow::Result<Idl> {
    serde_json::from_str(json).map_err(|e| anyhow::anyhow!("Failed to parse IDL: {}", e))
}
