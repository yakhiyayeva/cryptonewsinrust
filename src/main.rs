use actix_files::{NamedFile, Files};
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use lru_cache::LruCache;
use std::sync::Mutex;
use chrono::{DateTime, Utc};

// Cache for API responses (LRU with 100 items capacity)
type ApiCache = Mutex<LruCache<String, String>>;

#[derive(Debug, Serialize, Deserialize)]
struct NewsArticle {
    title: String,
    description: Option<String>,
    url: String,
    published_at: String,
    source_name: String,
    sentiment: Option<f32>,  // For bonus feature
}

#[derive(Debug, Serialize, Deserialize)]
struct CryptoData {
    id: String,
    symbol: String,
    name: String,
    current_price: f64,
    market_cap: f64,
    image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GNewsArticle {
    title: String,
    description: Option<String>,
    url: String,
    publishedAt: String,
    source: Option<Source>,
}

#[derive(Debug, Deserialize)]
struct Source {
    name: String,
}

#[derive(Debug, Deserialize)]
struct GNewsResponse {
    articles: Vec<GNewsArticle>,
}

#[derive(Debug, Deserialize)]
struct CoinGeckoCoin {
    id: String,
    symbol: String,
    name: String,
    current_price: f64,
    market_cap: f64,
    image: Option<String>,
}

// Initialize cache
lazy_static::lazy_static! {
    static ref CACHE: ApiCache = Mutex::new(LruCache::new(100));
}

#[get("/")]
async fn index() -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[get("/news/{crypto}")]
async fn get_news(
    crypto: web::Path<String>,
    client: web::Data<Client>,
) -> impl Responder {
    let cache_key = format!("news_{}", crypto);
    
    // Check cache first
    if let Some(cached) = CACHE.lock().unwrap().get_mut(&cache_key) {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(cached.clone());
    }

    let api_key = env::var("GNEWS_API_KEY").expect("GNEWS_API_KEY not set");
    let url = format!(
        "https://gnews.io/api/v4/search?q={}&lang=en&token={}",
        crypto, api_key
    );

    match client.get(&url).send().await {
        Ok(response) => {
            if let Ok(news) = response.json::<GNewsResponse>().await {
                let articles: Vec<NewsArticle> = news.articles.into_iter().map(|a| {
                    let sentiment = analyze_sentiment(&a.title); // Bonus feature
                    
                    // Parse and format date
                    let parsed_date = DateTime::parse_from_rfc3339(&a.publishedAt)
                        .unwrap_or_else(|_| Utc::now().into());
                    let formatted_date = parsed_date.format("%Y-%m-%d %H:%M:%S").to_string();
                    
                    NewsArticle {
                        title: a.title,
                        description: a.description,
                        url: a.url,
                        published_at: formatted_date,
                        source_name: a.source.map(|s| s.name).unwrap_or_else(|| "Unknown".into()),
                        sentiment,
                    }
                }).collect();
                
                let json = serde_json::to_string(&articles).unwrap();
                
                // Cache the response
                CACHE.lock().unwrap().insert(cache_key, json.clone());
                
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json)
            } else {
                HttpResponse::InternalServerError().json("Failed to parse news")
            }
        }
        Err(e) => {
            eprintln!("GNews API error: {}", e);
            HttpResponse::InternalServerError().json("Failed to fetch news")
        }
    }
}

#[get("/crypto/{crypto}")]
async fn get_crypto(
    crypto: web::Path<String>,
    client: web::Data<Client>,
) -> impl Responder {
    let cache_key = format!("crypto_{}", crypto);
    
    if let Some(cached) = CACHE.lock().unwrap().get_mut(&cache_key) {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(cached.clone());
    }

    let url = format!(
        "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids={}",
        crypto
    );

    match client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Accept", "application/json")
        .send()
        .await
    {
        Ok(response) => {
            // Сохраняем статус перед использованием response
            let status = response.status();
            
            if status.is_success() {
                match response.json::<Vec<CoinGeckoCoin>>().await {
                    Ok(coins) => {
                        if let Some(coin) = coins.first() {
                            let data = CryptoData {
                                id: coin.id.clone(),
                                symbol: coin.symbol.clone(),
                                name: coin.name.clone(),
                                current_price: coin.current_price,
                                market_cap: coin.market_cap,
                                image_url: coin.image.clone(),
                            };
                            
                            let json = serde_json::to_string(&data).unwrap();
                            CACHE.lock().unwrap().insert(cache_key, json.clone());
                            
                            return HttpResponse::Ok()
                                .content_type("application/json")
                                .body(json);
                        }
                        HttpResponse::NotFound().json("Cryptocurrency not found")
                    }
                    Err(e) => {
                        eprintln!("Failed to parse CoinGecko response: {}", e);
                        HttpResponse::InternalServerError().json("Failed to parse crypto data")
                    }
                }
            } else if status == 404 {
                HttpResponse::NotFound().json("Cryptocurrency not found")
            } else {
                match response.text().await {
                    Ok(body) => {
                        eprintln!("CoinGecko API error ({}): {}", status, body);
                        HttpResponse::InternalServerError().json("Failed to fetch crypto data")
                    }
                    Err(e) => {
                        eprintln!("Failed to read error body: {}", e);
                        HttpResponse::InternalServerError().json("Failed to fetch crypto data")
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("CoinGecko API request failed: {}", e);
            HttpResponse::InternalServerError().json("Failed to connect to CoinGecko API")
        }
    }
}
// Bonus: Simple sentiment analysis
fn analyze_sentiment(text: &str) -> Option<f32> {
    // This is a very basic implementation
    let positive_words = ["up", "rise", "bull", "growth", "positive"];
    let negative_words = ["down", "fall", "bear", "drop", "negative"];
    
    let mut score: f32 = 0.0;
    let words = text.to_lowercase();
    
    for word in positive_words {
        if words.contains(word) {
            score += 0.2;
        }
    }
    
    for word in negative_words {
        if words.contains(word) {
            score -= 0.2;
        }
    }
    
    Some(score.clamp(-1.0, 1.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(index)
            .service(get_news)
            .service(get_crypto)
            .service(Files::new("/static", "./static"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}