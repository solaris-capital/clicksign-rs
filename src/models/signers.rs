use serde::{Deserialize, Serialize};

/// This struct defines a signer with the attributes described in the API documentation
///
/// detailed instructions at <https://developers.clicksign.com/docs/criar-signatario#atributos-para-a-cria%C3%A7%C3%A3o-de-signat%C3%A1rios>
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

/// This struct defines a request and response body for POST /api/v1/lists endpoint
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
    message: String,
}
