use tokio::io::AsyncBufReadExt;

pub(crate) struct WebHook {
    webhook_urls: Vec<String>,
    client: reqwest::Client,
    token: Option<String>,
}

impl WebHook {
    pub(crate) fn new() -> Self {
        let token = std::env::var("WEBHOOK_TOKEN").ok();

        if token.is_none() {
            eprintln!("WARN: WebHook disabled (no token)")
        }

        Self {
            webhook_urls: get_webhook_urls(),
            client: reqwest::Client::new(),
            token,
        }
    }

    pub(crate) async fn success(&self, name: &str, org: &str) {
        if self.token.is_none() {
            return;
        }

        let token = self.token.as_ref().unwrap();

        for url in &self.webhook_urls {
            if let Err(err) = self
                .client
                // this is not very good, but i do not care
                .get(format!("{}?name={}&org={}&token={}", url, name, org, token))
                .send()
                .await
            {
                crate::discord_log::log(&format!("Error sending to {}: {}", url, err)).await;
            }
        }
    }
}

fn get_webhook_urls() -> Vec<String> {
    let urls = std::env::var("WEBHOOK_URLS");

    if let Ok(urls) = urls {
        urls.split(",").map(String::from).collect()
    } else {
        vec![]
    }
}
