use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Signer {
    key: Option<String>,
    email: String,
    phone_number: String,
    auths: Vec<String>,
    name: String,
    documentation: String,
    birthday: String,
    has_documentation: bool,
    delivery: String,
    selfie_enabled: bool,
    handwritten_enabled: bool,
    official_document_enabled: bool,
    liveness_enabled: bool,
    created_at: Option<String>,
    updated_at: Option<String>,
}
