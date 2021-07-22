use clicksign::client::Client;

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