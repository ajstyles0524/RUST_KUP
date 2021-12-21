use log::*;
use rocket::launch;
use rocket::routes;
use rocket::Rocket;
use rocket::{get, Build};
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    #[serde(rename = "_id")]
    id: i64,
    name: Name,
    contribs: Vec<String>,
    awards: Vec<Award>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Award {
    award: String,
    year: i64,
    by: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    first: String,
    last: String,
}
#[get("/")]
async fn index(){
    let result = reqwest::get("https://mocki.io/v1/3fee675e-373e-44c7-bb2e-5fc6512fb055")
        .await.unwrap()
        .json::<Welcome>()
        .await.unwrap();
    info!("{:?}", result.awards[0]);
}
#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
