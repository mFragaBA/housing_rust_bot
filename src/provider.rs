use serde::{Serialize, Deserialize};
use super::property::Property;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProviderData {
    pub base_url: String,
    pub sources: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Provider {
    Zonaprop(ProviderData),
    // Argenprop(ProviderData),
    // Mercadolibre(ProviderData),
    // Properati(ProviderData),
    // Inmobusqueda(ProviderData),
}
