use serde::Serialize;

#[derive(Serialize)]
pub struct PokemonProduct {
    pub url: String,
    pub image: String,
    pub name: String,
    pub price: String,
}
