# Crypto News
Crypto News is a simple and sleek web app that lets users track real-time cryptocurrency market data and stay updated with the latest crypto-related news. Just type a coin name (e.g., "bitcoin") and hit Search to view its market value and relevant news articles.
# Features
 Search for cryptocurrency market data

 Get the latest news related to cryptocurrencies

 Graceful error handling for missing data or API issues

 Clean and modern responsive UI

 Suggests supported cryptocurrencies

# How It Works
User enters a cryptocurrency name (e.g., bitcoin)

App fetches:

Market data like price and market cap

Latest news articles

Displays everything beautifully on the same page
## Usage

To run this project locally, ensure that you have Rust installed. You can follow the instructions to install Rust [here](https://www.rust-lang.org/learn/get-started).

### Steps to run:

1. Clone the repository:
    ```bash
    git clone (https://github.com/yakhiyayeva/cryptonewsinrust)
    cd (https://github.com/yakhiyayeva/cryptonewsinrust)
    ```

2. Install dependencies:
    ```bash
    cargo build
    ```

3. Run the server:
    ```bash
    cargo run


## Demo Screenshot
![image](https://github.com/user-attachments/assets/439511c5-cf8f-4db4-bc0c-5adeecdd6d0d)



- **Get Latest Crypto News**
    ```
    GET http://127.0.0.1:8080/crypto-news
    ```

    Response:
    ```html
    <html>
      <body>
        <h3>Title of Article</h3>
        <p><strong>Source:</strong> Some Source<br>
        <strong>Published At:</strong> 2022-01-01<br>
        <strong>Description:</strong> This is an article about crypto news<br>
        <a href="article-url" target="_blank">Read more</a></p>
      </body>
    </html>
    ```

- **Get Crypto Market Data**
    ```
    GET http://127.0.0.1:8080/crypto-data
    ```

    Response:
    ```html
    <html>
      <body>
        <h3>Bitcoin (BTC)</h3>
        <p><strong>Price:</strong> $45,000<br>
        <strong>Market Cap:</strong> $850 Billion<br>
        <strong>Volume:</strong> $35 Billion</p>
      </body>
    </html>
    ```

- **Get Top 10 Cryptos**
    ```
    GET http://127.0.0.1:8080/top-cryptos/10
    ```

    Response:
    ```json
    [
      {"name": "Bitcoin", "symbol": "BTC", "market_cap": 850000000000},
      {"name": "Ethereum", "symbol": "ETH", "market_cap": 450000000000}
    ]
    ```

## Contributions

Feel free to fork this project and contribute! If you have suggestions or improvements, feel free to open an issue or submit a pull request.


