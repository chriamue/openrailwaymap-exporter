//! Shared HTTP client construction for railway API clients.
use reqwest::Client;

/// Creates a `reqwest::Client` for railway API requests.
///
/// Overpass rejects requests without a User-Agent with 406 Not Acceptable.
/// Browsers set their own User-Agent, so it is only set on native targets.
pub fn http_client() -> Client {
    Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    #[cfg_attr(target_arch = "wasm32", ignore)]
    async fn test_sends_user_agent() {
        let expected_user_agent =
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_header("user-agent", expected_user_agent.as_str())
            .with_status(200)
            .create_async()
            .await;

        let client = http_client();
        let result = client.get(server.url()).send().await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }
}
