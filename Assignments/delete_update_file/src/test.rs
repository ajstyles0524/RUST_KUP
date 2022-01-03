#[cfg(test)]
pub mod tests {
    pub use crate::handler::handlers::{create_file, delete_file, rename_file};
    pub use serde_json::json;

    #[actix_web::test]
    pub async fn create_success() {
        assert_eq!(
            create_file("src/abc.html".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": true})).unwrap()
        );
    }

    #[actix_web::test]
    pub async fn create_failure() {
        assert_eq!(
            create_file("back/abc.html".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
    }

    #[actix_web::test]
    pub async fn rename_success() {
        assert_eq!(
            rename_file("src/abc.html".to_string(), "src/cba.html".to_string())
                .await
                .unwrap(),
            serde_json::to_string_pretty(&json!({"status": true})).unwrap()
        );
    }

    #[actix_web::test]
    pub async fn rename_failure() {
        assert_eq!(
            rename_file("bac/abc.html".to_string(), "abc/cba.html".to_string())
                .await
                .unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
    }

    #[actix_web::test]
    pub async fn delete_success() {
        assert_eq!(
            delete_file("src/cba.html".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": true})).unwrap()
        );
    }

    #[actix_web::test]
    pub async fn delete_failure() {
        assert_eq!(
            delete_file("src/baf.html".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
    }
}
