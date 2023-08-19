use std::collections::HashMap;

use mockito::Server;

#[tokio::test]
async fn create_user_api() {
    let mut server = Server::new_async().await;

    server
        .mock("POST", "/users")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::JsonString(
            r#"{
                "user_name": "integrationTest001",
                "email": "integrationTest001@email.com",
                "password": "PasswordintegrationTest001",
                "role": "0"
            }"#
            .to_string(),
        ))
        .create_async()
        .await;

    let mut map = HashMap::new();
    map.insert("user_name", "integrationTest001");
    map.insert("email", "integrationTest001@email.com");
    map.insert("password", "PasswordintegrationTest001");
    map.insert("role", "0");

    let res = reqwest::Client::new()
        .post(format!("{}/users", server.url()))
        .version(reqwest::Version::HTTP_11)
        .json(&map)
        .send()
        .await
        .unwrap();
    
    assert_eq!(200, res.status());
}
