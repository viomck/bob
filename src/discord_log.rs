// bob - Docker image build agent
// Copyright (C) 2022 Violet McKinney <opensource@viomck.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use gethostname::gethostname;
use lazy_static::lazy_static;
use serde_json::json;
use serenity::http;
use serenity::http::Http;
use std::env;

lazy_static!(
    // (un/poorly-)documented behavior:  an empty/garbage token is fine for
    // anonymous calls (like websockets) in serenity.
    static ref CLIENT: Http = Http::new("");
);

async fn log_result(text: &str) -> serenity::Result<()> {
    eprintln!("{}", &text);

    let text = if text.len() > 2000 {
        text.get(0..2000).unwrap()
    } else {
        text
    };

    let content = format!(
        "[`{}`] {}",
        gethostname().to_str().unwrap_or("unknown"),
        &text
    );

    let value = json!({ "content": content });
    let map = value.as_object().unwrap();
    CLIENT
        .execute_webhook(get_webhook_id(), &get_webhook_token(), true, &map)
        .await
        .map(|_| ())
}

pub(crate) async fn log(text: &str) {
    if let Err(err) = log_result(text).await {
        eprintln!("ERROR Could not log to discord: {}", err)
    }
}

pub(crate) async fn hello_world() {
    log_result("INFO I am awake!").await.unwrap();
}

fn get_webhook_id() -> u64 {
    env::var("DISCORD_WEBHOOK_ID")
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

fn get_webhook_token() -> String {
    env::var("DISCORD_WEBHOOK_TOKEN").unwrap()
}
