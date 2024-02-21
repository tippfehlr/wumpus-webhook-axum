use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use chrono::{Duration, Utc};

async fn index() -> &'static str {
    "Put\n\nhttps://wumpus-webhook.shuttleapp.rs/?webhook=<your-discord-webhook>\n\ninto the shuttle `webhook` field.\nThe password doesn't matter, input anything for the form to be happy.\n\n\nSource code: https://github.com/tippfehlr/wumpus-webhook"
}

async fn handle_webhook(
    Query(query): Query<HashMap<String, String>>,
    Json(data): Json<serde_json::Value>,
) -> StatusCode {
    println!("request received");

    let client = reqwest::Client::new();
    let body = r#"
        {
            "content": "",
            "tts": false,
            "embeds": [
                {
                "id": 10674342,
                "title": "🔥 New vote! 🔥",
                "description": "<@{{userId}}> just voted for <@{{botId}}>!\nYou can vote again <t:{{timeStamp}}:R>!",
                "color": 2326507,
                "fields": []
                }
            ],
            "components": [],
            "actions": {},
            "username": "Wumpus.store",
            "avatar_url": "https://cdn.discordapp.com/avatars/1207368481977147443/980c30c5c2e7896201f655a13af1037f.webp?size=80"
        }"#
        .replace("{{userId}}", &data.get("userId").unwrap().to_string())
        .replace("{{botId}}", &data.get("botId").unwrap().to_string())
        .replace("{{timeStamp}}", &(Utc::now() + Duration::hours(12)).timestamp().to_string());

    match client
        .post(&query.get("webhook").unwrap().to_string())
        .body(body)
        .header("content-type", "application/json")
        .send()
        .await
    {
        Ok(res) => {
            if res.status().is_success() {
                println!("Successful request to Discord");
                StatusCode::OK
            } else {
                eprintln!("Not successful: {:#?}", res);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        Err(_) => {
            eprintln!("Failed to send request to Discord");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index).post(handle_webhook));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
