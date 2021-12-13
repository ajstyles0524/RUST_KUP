use log::info;
#[get("/world")]
/// hello is a handler which is used to handling /world route
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return &'static str
fn hello() -> &'static str {
    info!("Return Response Successfully");
    "Hello, world!"
}
#[launch]
/// rocket is a function which creates a hello route,mounts the route at the /api,and launches the application
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return Rocket<Build>
pub fn launch() -> _ {
    rocket::build().mount("/api", routes![hello])
}