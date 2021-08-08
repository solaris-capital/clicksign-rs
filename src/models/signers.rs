use serde::{Deserialize, Serialize};

/// This struct defines a signer with the attributes described in the API documentation
/// The optional fields only make sense in the Responses body.
/// Check [clicksign docs](https://developers.clicksign.com/docs/criar-signatario) for detailed info.
#[derive(Debug, Serialize, Deserialize)]
pub struct Signer {
    /// Unique identifier for the signer into clicksign environment
    pub key: Option<String>,
    /// Email of the signer who must sign the document
    pub email: String,
    /// Phone number for sending Whatsapp or SMS. (Must be sent 11 numbers)
    pub phone_number: String,
    /// Authentication type for signing (email, sms, whatsapp, API and/or pix).
    pub auths: Vec<String>,
    /// Signer's full name.
    pub name: String,
    /// Signer's CPF
    pub documentation: String,
    /// Signer's date of birth
    pub birthday: String,
    /// The default value is true. If false, isn't possible to send the fields "documentation" and "birthday".
    pub has_documentation: bool,
    /// Informs how the signer receives the signature confirmation and finalized document notifications.
    pub delivery: String,
    /// Check [clicksign documentation](https://developers.clicksign.com/docs/criar-signatario#atributos-para-a-cria%C3%A7%C3%A3o-de-signat%C3%A1rios) for more info about this field.
    pub selfie_enabled: bool,
    /// Check [clicksign documentation](https://developers.clicksign.com/docs/criar-signatario#atributos-para-a-cria%C3%A7%C3%A3o-de-signat%C3%A1rios) for more info about this field.
    pub handwritten_enabled: bool,
    /// Check [clicksign documentation](https://developers.clicksign.com/docs/criar-signatario#atributos-para-a-cria%C3%A7%C3%A3o-de-signat%C3%A1rios) for more info about this field.
    pub official_document_enabled: bool,
    /// Check [clicksign documentation](https://developers.clicksign.com/docs/criar-signatario#atributos-para-a-cria%C3%A7%C3%A3o-de-signat%C3%A1rios) for more info about this field.
    pub liveness_enabled: bool,
    /// Signer creation datetime
    pub created_at: Option<String>,
    /// Signer update datetime
    pub updated_at: Option<String>,
}

/// This struct defines a request and response body for POST /api/v1/lists endpoint
/// The optional fields only make sense in the Responses body.
/// Check [clicksign docs](https://developers.clicksign.com/docs/adicionar-signatario-a-documento) for detailed info.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignerToDocument {
    /// A key for clicksign internal stuffs (Response-only field)
    pub key: Option<String>,
    /// A key for clicksign internal stuffs (Response-only field)
    pub request_signature_key: Option<String>,
    /// Unique key of the document within Clicksign
    pub document_key: String,
    /// Signer's unique key within Clicksign
    pub signer_key: String,
    /// Under what title the signature will be carried out
    pub sign_as: String,
    /// Event creation datetime (Response-only field)
    pub created_at: Option<String>,
    /// Event update datetime (Response-only field)
    pub updated_at: Option<String>,
    /// URL to signing document (Response-only field)
    pub url: Option<String>,
    /// The message will be sent in the body of the signature request email to the signers.
    pub message: String,
}
