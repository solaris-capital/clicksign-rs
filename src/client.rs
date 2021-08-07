use crate::models::signers::{Signer, SignerToDocument};
use error_chain::bail;
use reqwest::Response;
use reqwest::StatusCode;
use serde_json::Value;
use std::collections::HashMap;

/// This struct defines a clicksign Client
#[derive(Debug)]
pub struct Client {
    /// * host (String): The base URL for clicksign API
    pub host: String,
    /// * access_token (String): Access token provided by clicksign. Follow [this instructions](https://developers.clicksign.com/docs/pimeiros-passos#2-gere-um-access-token) to generate your token.
    pub access_token: String,
    /// * client: A reqwest http client.
    pub client: reqwest::Client,
}

/// Implementation for client struct
impl Client {
    /// Given an access_token and an optional host, creates a Client instance.
    ///
    /// # Arguments
    /// * access_token (&str): Access token provided by clicksign.
    /// * host (&str, optional): The base URL for clicksign API.
    ///
    /// # Example
    /// ```
    /// use clicksign::client::Client;
    ///
    /// let client = Client::new(
    ///    "c9d91ece-9b3b-4def-abac-25b645cb083c",
    ///    Some("https://api.example.com"),
    /// );
    /// assert_eq!("https://api.example.com", client.host);
    /// assert_eq!("c9d91ece-9b3b-4def-abac-25b645cb083c", client.access_token);
    /// ```
    pub fn new(access_token: &str, host: Option<&str>) -> Self {
        Self {
            host: host.unwrap_or("https://app.clicksign.com/").to_string(),
            access_token: access_token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Given a path (endpoint), generates a full url based on host.
    ///
    /// # Example
    /// ```
    /// use clicksign::client::Client;
    ///
    /// let client = Client::new(
    ///    "some_access_token",
    ///    Some("https://api.example.com/"),
    /// );
    /// let new_url = client.build_url("foo");
    /// assert_eq!("https://api.example.com/foo?access_token=some_access_token", new_url);
    /// ```
    pub fn build_url(&self, endpoint: &str) -> String {
        format!(
            "{}{}?access_token={}",
            self.host, endpoint, self.access_token
        )
    }

    /// Given a Response object, return the body content or the appropriate message error
    async fn handler(&self, response: Response) -> Result<String, Box<dyn std::error::Error>> {
        match response.status() {
            StatusCode::CREATED | StatusCode::OK | StatusCode::ACCEPTED => {
                Ok(response.text().await.unwrap())
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("500 Internal Server Error")
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("503 Service Unavailable")
            }
            StatusCode::UNAUTHORIZED => {
                bail!("401 Unauthorized")
            }
            StatusCode::FORBIDDEN => {
                bail!("403 Forbidden")
            }
            StatusCode::BAD_REQUEST => {
                bail!("400 Bad Request: {}", response.text().await.unwrap())
            }
            resp => {
                bail!(format!("Received response: {:?}", resp))
            }
        }
    }

    /// Create a new document, based on model
    /// Reference: <https://developers.clicksign.com/docs/criar-documento-via-modelos>
    ///
    /// # Arguments
    /// * template_id (&str): The ID of the template for which you want to create the document.
    /// * template_body (&str): A json-like string to fill in the document template
    ///
    /// # Example
    /// ```no_run
    /// async {
    ///   use clicksign::client::Client;
    ///
    ///   let client = Client::new(
    ///      "some_access_token",
    ///      Some("https://api.example.com/"),
    ///   );
    ///   let template_body = r#"
    ///      {
    ///        "document": {
    ///          "path": "/Modelos/Teste-123.docx",
    ///          "template": {
    ///            "data": {
    ///              "Company Name": "Clicksign Gestão de Documentos S.A.",
    ///              "Address": "R. Teodoro Sampaio 2767, 10° andar",
    ///              "Phone": "(11) 3145-2570",
    ///              "Website": "https://www.clicksign.com"
    ///            }
    ///          }
    ///        }
    ///      }
    ///  "#;
    ///
    ///  let document = client.create_document_by_model("template_id", template_body)
    ///      .await
    ///       .unwrap();
    ///  };
    /// ```
    pub async fn create_document_by_model(
        &self,
        template_id: &str,
        template_body: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let value: Value = serde_json::from_str(template_body)?;
        let url = self.build_url(&format!("templates/{}/documents", template_id));
        let resp = self
            .client
            .post(url)
            .json(&value)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let result: Value = serde_json::from_str(&self.handler(resp).await.unwrap())?;

        Ok(result)
    }

    /// Create a new signer
    /// Reference: <https://developers.clicksign.com/docs/criar-signatario>
    ///
    /// # Arguments
    /// * request_body (&str): A json-like string with the data of new signer
    ///
    /// # Example
    /// ```
    /// async {
    ///   use clicksign::client::Client;
    ///
    ///   let client = Client::new(
    ///      "some_access_token",
    ///      Some("https://api.example.com/"),
    ///   );
    ///   let signer_body = r#"
    ///      {
    ///          "signer": {
    ///              "email": "fulano@example.com",
    ///              "phone_number": "11999999999",
    ///              "auths": [
    ///                "email"
    ///              ],
    ///              "name": "Marcos Zumba",
    ///              "documentation": "123.321.123-40",
    ///              "birthday": "1983-03-31",
    ///              "has_documentation": true,
    ///              "selfie_enabled": true,
    ///              "handwritten_enabled": true,
    ///              "official_document_enabled": true,
    ///              "liveness_enabled": true
    ///          }
    ///      }
    ///   "#;
    ///   let signer = client.create_signer(signer_body).await.unwrap();
    /// };
    /// ```
    pub async fn create_signer(
        &self,
        request_body: &str,
    ) -> Result<HashMap<String, Signer>, Box<dyn std::error::Error>> {
        let value: HashMap<String, Signer> = serde_json::from_str(request_body)?;
        let url = self.build_url("signers");
        let resp = self
            .client
            .post(url)
            .json(&value)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let result: HashMap<String, Signer> =
            serde_json::from_str(&self.handler(resp).await.unwrap())?;

        Ok(result)
    }

    /// Adding a signer to document
    /// Reference: <https://developers.clicksign.com/docs/adicionar-signatario-a-documento>
    ///
    /// # Arguments
    /// * request_body (&str): A json-like string with the data of the signer and the document key
    ///
    /// # Example
    /// ```
    /// async {
    ///   use clicksign::client::Client;
    ///
    ///   let client = Client::new(
    ///      "some_access_token",
    ///      Some("https://api.example.com/"),
    ///   );
    ///   let request_body = r#"
    ///      {
    ///        "list": {
    ///          "document_key": "27b02527-a576-46ee-b01c-bb4e694036c4",
    ///          "signer_key": "79301388-9567-4320-90ce-9e6f60e70d28",
    ///          "sign_as": "sign",
    ///          "group": 1,
    ///          "message": "Por favor, assine o documento para completar o seu cadastro."
    ///        }
    ///      }
    ///   "#;
    ///   let result = client.add_signer_to_document(request_body).await.unwrap();
    /// };
    /// ```
    pub async fn add_signer_to_document(
        &self,
        request_body: &str,
    ) -> Result<HashMap<String, SignerToDocument>, Box<dyn std::error::Error>> {
        let value: HashMap<String, SignerToDocument> = serde_json::from_str(request_body)?;
        let url = self.build_url("lists");
        let resp = self
            .client
            .post(url)
            .json(&value)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let result: HashMap<String, SignerToDocument> =
            serde_json::from_str(&self.handler(resp).await.unwrap())?;

        Ok(result)
    }

    /// Sending email notification to signer
    /// Reference: <https://developers.clicksign.com/docs/adicionar-signatario-a-documento>
    ///
    /// # Arguments
    /// * request_body (&str): A json-like string with the request body
    ///
    /// # Example
    /// ```
    /// async {
    ///   use clicksign::client::Client;
    ///
    ///   let client = Client::new(
    ///      "some_access_token",
    ///      Some("https://api.example.com/"),
    ///   );
    ///   let request_body = r#"
    ///      {
    ///        "request_signature_key": "0d5a9615-2bb8-3a23-6584-33ff436bb990",
    ///        "message": "Prezado, seu documento já está disponível para assinatura",
    ///        "url": "https://www.example.com/abc"
    ///      }
    ///   "#;
    ///   let result = client.request_signing_by_email(request_body).await.unwrap();
    /// };
    /// ```
    pub async fn request_signing_by_email(
        &self,
        request_body: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let value: HashMap<String, String> = serde_json::from_str(request_body)?;
        let url = self.build_url("notifications");
        let _ = self
            .client
            .post(url)
            .json(&value)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        Ok(())
    }
}
