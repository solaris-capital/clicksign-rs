use error_chain::bail;
use reqwest::Response;
use reqwest::StatusCode;
use serde_json::Value;

pub struct Client {
    host: String,
    access_token: String,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            host: String::from("https://sandbox.clicksign.com/api/v1/"),
            access_token: String::from("2bdbdc19-c3bf-45db-b8a7-fddb43896c93"),
            client: reqwest::Client::new(),
        }
    }
}

impl Client {
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
}
