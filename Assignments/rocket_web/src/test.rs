#[cfg(test)]
mod test {
    use crate::launch::launch;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    #[test]
    fn hello_world() {
        let client = Client::tracked(launch()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}