use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    color: Color,
    egg_groups: Vec<Color>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    name: String,
    url: String,
}
/// main: main function is used to get the URL
///
/// #Arguments
///
/// No Arguments
///
/// Return
///
/// Return reqwest::Result<()>
#[tokio::main]
pub async fn main() -> reqwest::Result<()> {
    let result = reqwest::get("https://pokeapi.co/api/v2/pokemon-species/ditto")
        .await?
        .json::<Welcome>()
        .await?;
    info!("{:?}", result.egg_groups[0]);
    Ok(())
}
