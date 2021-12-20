use log::*;
use rocket::launch;
use rocket::routes;
use rocket::Rocket;
use rocket::{get, Build};
#[get("/")]
/// index is a handler which is used to to hit Google end point
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// No Return
async fn index() {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://www.googleapis.com/auth/calendar")
        .send()
        .await
        .unwrap();
    if resp.status().is_success() {
        info!("success!");
        debug!("Status: {}", resp.status());
    } else if resp.status().is_server_error() {
        error!("server error!");
        debug!("Something else happened.\n Status: {}", resp.status());
    } else {
        debug!("Something else happened. \n Status: {:?}", resp.status());
    }
}
/// rocket is a function which creates a index route,mounts the route at the /,and launches the application
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return Rocket<Build>
#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
