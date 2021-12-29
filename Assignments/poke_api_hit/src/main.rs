pub mod test;
pub mod file {
    pub mod json;
}
use crate::file::json::hit_point;
fn main() {
    let key = "base_happiness".to_string();
    hit_point(key).ok();
}
