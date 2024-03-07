use axum::{routing::get, Json, Router};
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(scrape_me));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Define a custom data structure to store the scraped data
#[derive(Serialize)]
struct PokemonProduct {
    url: String,
    image: String,
    name: String,
    price: String,
}

// Function to scrape data
async fn scrape_me() -> Json<Vec<PokemonProduct>> {
    // Initialize the vector that will store the scraped data
    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();
    // Download the target HTML document
    let response = reqwest::get("https://scrapeme.live/shop/").await;
    // Get the HTML content from the request response
    let html_content = response.unwrap().text().await.unwrap();
    // Parse the HTML document
    let document = Html::parse_document(&html_content);

    // Define the CSS selector to get all products on the page
    let html_product_selector = Selector::parse("li.product").unwrap();
    // Apply the CSS selector to get all products
    let html_products = document.select(&html_product_selector);

    // Iterate over each HTML product to extract data from it
    for html_product in html_products {
        // Scraping logic to retrieve the info of interest
        let url = html_product
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned)
            .unwrap();
        let image = html_product
            .select(&Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned)
            .unwrap();
        let name = html_product
            .select(&Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>())
            .unwrap();
        let price = html_product
            .select(&Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>())
            .unwrap();

        // Instantiate a new Pokemon product with the scraped data and add it to the list
        let pokemon_product = PokemonProduct {
            url,
            image,
            name,
            price,
        };

        pokemon_products.push(pokemon_product);
    }

    Json(pokemon_products)
}
