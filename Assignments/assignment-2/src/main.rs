use log::*;
mod check_leap;
mod linear_binary;
mod merge_sort;

fn main() {
    env_logger::init();
    let arr1: [i32; 5] = [10, 20, 30, 40, 50];
    let arr2: [i32; 5] = [10, 30, 60, 80, 50];
    let arr3 = [10, 30, 60, 80, 100];
    linear_binary::binary_search(&arr1, 0, 4, 30);
    info!("Binary search is performed");
    linear_binary::lsearch(&arr2, 30, 0);
    info!("linear search is performed");
    merge_sort::mergesortmain(&arr3);
    info!("Merge sort is performed");
    check_leap::count_leap();
    info!("Checking leap year is performed");
}
