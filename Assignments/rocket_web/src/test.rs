#[cfg(test)]
mod test {
    use crate::launch::launch;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    #[test]
    fn hello_world_success() {
        let client = Client::tracked(launch()).expect("valid rocket instance");
        let response = client.get("/api/world").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
    #[test]
    fn hello_world_failure() {
        let client = Client::tracked(launch()).expect("valid rocket instance");
        let response = client.get("/api/world").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_ne!(response.into_string().unwrap(), "Hello, Rust!");
    }
    #[test]
    fn hello_world_status_failure() {
        let client = Client::tracked(launch()).expect("valid rocket instance");
        let response = client.get("/world").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
