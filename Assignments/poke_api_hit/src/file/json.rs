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
/// Return reqwest::Result<i64> type
pub fn hit_point(key:String) -> reqwest::Result<i64> {
    let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")?
        .json::<HashMap<String, serde_json::Value>>()?;
    let resp_2 = content[&key].as_i64();
    let value = match resp_2
    {
        Some(value) => value,
        _=> 0,
    };
    info!("{:?}", (resp_2));
    Ok(value)
}
