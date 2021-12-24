use log::info;
use std::collections::HashMap;
/// main is a function that can use to get the URL path of the request
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return reqwest::Result<()> type
pub fn hit_point() -> reqwest::Result<()> {
    env_logger::init();
    let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")?
        .json::<HashMap<String, serde_json::Value>>()?;
    info!("{:#?}", content["flavor_text_entries"][22].get("flavor_text"));
    Ok(())
}
