# Crypto Pulse

Crypto Pulse is a web application that allows users to track real-time cryptocurrency market data and read the latest news related to the crypto world. Users can search for popular coins like Bitcoin, Ethereum, Cardano, and more by entering their names or symbols.
# Features
 Search for cryptocurrency market data

 Get the latest news related to cryptocurrencies

 Graceful error handling for missing data or API issues

 Clean and modern responsive UI

 Suggests supported cryptocurrencies

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
    ```

4. Access the API locally at:
    - [http://127.0.0.1:8080/crypto-news](http://127.0.0.1:8080/crypto-news) – Fetches the latest cryptocurrency news.
    - [http://127.0.0.1:8080/crypto-data](http://127.0.0.1:8080/crypto-data) – Fetches real-time cryptocurrency market data (price, market cap, and volume).
    - [http://127.0.0.1:8080/health](http://127.0.0.1:8080/health) – Check the server health status.
    - [http://127.0.0.1:8080/top-cryptos/{count}](http://127.0.0.1:8080/top-cryptos/10) – Fetches the top N cryptocurrencies by market cap.

## Demo Screenshot
![image](https://github.com/user-attachments/assets/422cc2c1-844d-4a52-b78f-6e566a30eb4b)


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


