use axum::Json;
use scraper::{Html, Selector};

use crate::models::PokemonProduct;

pub async fn scrape_me() -> Json<Vec<PokemonProduct>> {
    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    let response = reqwest::get("https://scrapeme.live/shop/").await;

    if response.is_ok() {
        let html_content = response.unwrap().text().await.unwrap();

        let document = Html::parse_document(&html_content);

        let html_product_selector = Selector::parse("li.product").unwrap();

        let html_products = document.select(&html_product_selector);

        for html_product in html_products {
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

            let pokemon_product = PokemonProduct {
                url,
                image,
                name,
                price,
            };

            pokemon_products.push(pokemon_product);
        }
    }

    Json(pokemon_products)
}

pub async fn scrape_me_too() -> Json<Vec<PokemonProduct>> {
    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();
    let first_page = "https://scrapeme.live/shop/page/1/";

    let mut pages_to_scrape: Vec<String> = vec![first_page.to_owned()];
    let mut pages_discovered: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut i = 1;
    let max_iterations = 5;

    while !pages_to_scrape.is_empty() && i <= max_iterations {
        let page_to_scrape = pages_to_scrape.remove(0);
        let response = reqwest::get(page_to_scrape).await;

        if let Ok(response) = response {
            let html_content = response.text().await.unwrap();
            let document = scraper::Html::parse_document(&html_content);
            let html_product_selector = scraper::Selector::parse("li.product").unwrap();
            let html_products = document.select(&html_product_selector);

            for html_product in html_products {
                let url = html_product
                    .select(&scraper::Selector::parse("a").unwrap())
                    .next()
                    .and_then(|a| a.value().attr("href"))
                    .map(str::to_owned)
                    .unwrap();
                let image = html_product
                    .select(&scraper::Selector::parse("img").unwrap())
                    .next()
                    .and_then(|img| img.value().attr("src"))
                    .map(str::to_owned)
                    .unwrap();
                let name = html_product
                    .select(&scraper::Selector::parse("h2").unwrap())
                    .next()
                    .map(|h2| h2.text().collect::<String>())
                    .unwrap();
                let price = html_product
                    .select(&scraper::Selector::parse(".price").unwrap())
                    .next()
                    .map(|price| price.text().collect::<String>())
                    .unwrap();

                let pokemon_product = PokemonProduct {
                    url,
                    image,
                    name,
                    price,
                };

                pokemon_products.push(pokemon_product);
            }

            let html_pagination_link_selector = scraper::Selector::parse("a.page-numbers").unwrap();
            let html_pagination_links = document.select(&html_pagination_link_selector);

            let mut pagination_futures = Vec::new();
            for html_pagination_link in html_pagination_links {
                let pagination_url = html_pagination_link
                    .value()
                    .attr("href")
                    .unwrap()
                    .to_owned();
                if !pages_discovered.contains(&pagination_url) {
                    pages_discovered.insert(pagination_url.clone());
                    if !pages_to_scrape.contains(&pagination_url) {
                        pages_to_scrape.push(pagination_url.clone());
                    }
                    pagination_futures.push(reqwest::get(pagination_url).await);
                }
            }
        }

        i += 1;
    }

    Json(pokemon_products)
}

pub async fn crawl_scrape_me() -> Json<Vec<PokemonProduct>> {
    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    let browser = headless_chrome::Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://scrapeme.live/shop/").unwrap();

    let html_products = tab.wait_for_elements("li.product").unwrap();

    for html_product in html_products {
        let url = html_product
            .wait_for_element("a")
            .unwrap()
            .get_attributes()
            .unwrap()
            .unwrap()
            .get(1)
            .unwrap()
            .to_owned();
        let image = html_product
            .wait_for_element("img")
            .unwrap()
            .get_attributes()
            .unwrap()
            .unwrap()
            .get(5)
            .unwrap()
            .to_owned();
        let name = html_product
            .wait_for_element("h2")
            .unwrap()
            .get_inner_text()
            .unwrap();
        let price = html_product
            .wait_for_element(".price")
            .unwrap()
            .get_inner_text()
            .unwrap();
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
