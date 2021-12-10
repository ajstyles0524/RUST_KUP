use log::debug;
#[get("/")]
/// world is a handler which is used to process the request and return a response
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return &'static str
fn hello() -> &'static str {
    debug!("{}", "some log message");
    "Hello, world!"
}
#[launch]
/// rocket is a function which is used to startq the server
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return Rocket<Build>
pub fn launch() -> _ {
    rocket::build().mount("/", routes![hello])
}
