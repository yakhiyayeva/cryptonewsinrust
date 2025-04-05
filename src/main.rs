use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CoinGeckoResponse {
    id: String,
    symbol: String,
    name: String,
    current_price: f64,
    market_cap: f64,
    total_volume: f64,
}

const GNEWS_API_KEY: &str = "4c36c1998801bb2cc1c4ee0b7dcb5259";
const GNEWS_API_URL: &str = "https://gnews.io/api/v4/search?q=crypto&lang=en";
const COINGECKO_API_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd";

async fn get_crypto_news() -> impl Responder {
    let client = Client::new();
    let url = format!("{}&token={}", GNEWS_API_URL, GNEWS_API_KEY);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.unwrap_or_else(|_| "{}".to_string());
                println!("GNews Raw Response: {}", body);

                let json_result: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();

                if let Some(articles) = json_result["articles"].as_array() {
                    let articles_html = articles.iter().map(|article| {
                        let title = article["title"].as_str().unwrap_or("No Title");
                        let source = article["source"]["name"].as_str().unwrap_or("Unknown Source");
                        let published_at = article["publishedAt"].as_str().unwrap_or("Unknown Date");
                        let description = article["description"].as_str().unwrap_or("No Description");
                        let url = article["url"].as_str().unwrap_or("#");

                        format!(
                            "<h3>{}</h3><p><strong>Source:</strong> {}<br><strong>Published At:</strong> {}<br><strong>Description:</strong> {}<br><a href=\"{}\" target=\"_blank\">Read more</a></p>",
                            title, source, published_at, description, url
                        )
                    }).collect::<Vec<String>>().join("<hr>");

                    HttpResponse::Ok().body(format!("<html><body>{}</body></html>", articles_html))
                } else {
                    HttpResponse::InternalServerError().body("Error parsing GNews articles")
                }
            } else {
                HttpResponse::InternalServerError().body("GNews API returned error")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch from GNews API"),
    }
}

async fn get_crypto_data() -> impl Responder {
    let client = Client::new();

    match client.get(COINGECKO_API_URL).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let crypto_data: Vec<CoinGeckoResponse> = response.json().await.unwrap_or_default();

                let data = crypto_data.into_iter().map(|crypto| {
                    format!(
                        "<h3>{} ({})</h3><p><strong>Price:</strong> ${}<br><strong>Market Cap:</strong> ${}<br><strong>Volume:</strong> ${}</p>",
                        crypto.name, crypto.symbol, crypto.current_price, crypto.market_cap, crypto.total_volume
                    )
                }).collect::<Vec<String>>().join("<hr>");

                HttpResponse::Ok().body(format!("<html><body>{}</body></html>", data))
            } else {
                HttpResponse::InternalServerError().body("CoinGecko API returned error")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch from CoinGecko API"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on: http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/crypto-news", web::get().to(get_crypto_news))
            .route("/crypto-data", web::get().to(get_crypto_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
