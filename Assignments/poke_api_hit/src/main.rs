pub mod test;
pub mod file {
    pub mod json;
}
use crate::file::json::hit_point;
fn main() {
    hit_point().ok();
}
