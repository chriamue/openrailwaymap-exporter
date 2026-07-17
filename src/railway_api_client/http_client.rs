//! Shared HTTP client construction for railway API clients.
use reqwest::Client;

/// Creates a `reqwest::Client` for railway API requests.
///
/// Overpass rejects requests without a User-Agent with 406 Not Acceptable.
/// Browsers set their own User-Agent, so it is only set on native targets.
pub fn http_client() -> Client {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .unwrap_or_default()
    }
    #[cfg(target_arch = "wasm32")]
    {
        Client::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    #[cfg_attr(target_arch = "wasm32", ignore)]
    async fn test_sends_user_agent() {
        let expected_user_agent =
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        let mock = mock("GET", "/")
            .match_header("user-agent", expected_user_agent.as_str())
            .with_status(200)
            .create();

        let client = http_client();
        let result = client.get(server_url()).send().await;

        mock.assert();
        assert!(result.is_ok());
    }
}
