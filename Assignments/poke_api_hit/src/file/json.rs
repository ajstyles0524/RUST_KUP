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
pub fn hit_point() -> Result<(), reqwest::Error> {
    env_logger::init();
    let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")?
        .json::<HashMap<String, serde_json::Value>>()?;
    let resp_1 = &content["flavor_text_entries"][0]["flavor_text"];
    let resp_2 = &content["color"]["name"];
    let resp_3 = &content["flavor_text_entries"][3]["version"]["name"];
    let resp_4 = &content["capture_rate"];
    info!("{:#?}", resp_1);
    info!("{:#?}", resp_2);
    info!("{:#?}", resp_3);
    info!("{:#?}", resp_4);
    Ok(())
}
