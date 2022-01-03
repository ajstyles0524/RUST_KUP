use serde_json::json;
use std::fs;
use std::fs::File;
/// create_file function is used to create a file at the given location.
///
/// #Arguments
///
/// file_path: file_path is of type String
///
/// #Return
///
/// Returns serde_json::Result<String>
pub async fn create_file(file_path: String) -> serde_json::Result<String> {
    log::info!("Creating a file");
    let file = File::create(file_path).is_ok();
    let json_response = json!({ "status": file });
    serde_json::to_string_pretty(&json_response)
}
/// rename_file function is used to rename a file present at the given location.
///
/// #Arguments
///
/// from: from is of type String,current name of the file
/// to: to is of type String,the new name of the file
///
/// #Return
///
/// Returns serde_json::Result<String>
pub async fn rename_file(from: String, to: String) -> serde_json::Result<String> {
    log::info!("Re-naming the name of file");
    let file = fs::rename(from, to).is_ok();
    let json_response = json!({ "status": file });
    serde_json::to_string_pretty(&json_response)
}
/// delete_file function is used to delete a file present at the given location.
///
/// #Arguments
///
/// file_path: file_path is of type String
///
/// #Return
///
/// Returns serde_json::Result<String>
pub async fn delete_file(file_path: String) -> serde_json::Result<String> {
    log::info!("Deleting a file present at the given path. ");
    let file = fs::remove_file(file_path).is_ok();
    let json_response = json!({ "status": file });
    serde_json::to_string_pretty(&json_response)
}
