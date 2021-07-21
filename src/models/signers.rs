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

#[derive(Debug, Serialize, Deserialize)]
pub struct SignerToDocument {
    key: Option<String>,
    request_signature_key: Option<String>,
    document_key: String,
    signer_key: String,
    sign_as: String,
    created_at: Option<String>,
    updated_at: Option<String>,
    url: Option<String>,
    group: i32,
    message: String
}