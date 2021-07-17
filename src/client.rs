use crate::models::signers::Signer;
use error_chain::bail;
use reqwest::Response;
use reqwest::StatusCode;
use serde_json::Value;
use std::collections::HashMap;

pub struct Client {
    host: String,
    access_token: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(access_token: &str, host: Option<&str>) -> Self {
        Self {
            host: host.unwrap_or("https://app.clicksign.com/").to_string(),
            access_token: access_token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!(
            "{}{}?access_token={}",
            self.host, endpoint, self.access_token
        )
    }

    async fn handler(&self, response: Response) -> Result<String, Box<dyn std::error::Error>> {
        match response.status() {
            StatusCode::CREATED | StatusCode::OK => Ok(response.text().await.unwrap()),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client_with_default_host() {
        let client = Client::new("c9d91ece-9b3b-4def-abac-25b645cb083c", None);
        assert_eq!("https://app.clicksign.com/", client.host);
    }

    #[test]
    fn test_new_client_with_no_default_host() {
        let client = Client::new(
            "c9d91ece-9b3b-4def-abac-25b645cb083c",
            Some("https://api.example.com"),
        );
        assert_eq!("https://api.example.com", client.host);
    }

    #[test]
    fn test_build_url() {
        let client = Client::new(
            "c9d91ece-9b3b-4def-abac-25b645cb083c",
            Some("https://api.example.com/"),
        );
        let url = client.build_url("my-path");
        assert_eq!(
            "https://api.example.com/my-path?access_token=c9d91ece-9b3b-4def-abac-25b645cb083c",
            url
        );
    }
}
