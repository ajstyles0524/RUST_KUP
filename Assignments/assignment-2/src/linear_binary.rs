use log::*;
use std::cmp::Ordering;
// linear search using recursion amd immutable variables
pub fn lsearch(arr: &[i32], search_element: i32, index: i32) {
    if arr.len() == index as usize {
        warn!("Number is not present");
        debug!("Number is not present in the given array");
        return;
    }
    if arr[index as usize] == search_element {
        debug!("\n Item {} found at index {}\n", search_element, index);
        return;
    }
    lsearch(arr, search_element, index + 1);
}

// binary search using recursion and immutable variables

pub fn binary_search(arr: &[i32], starting_index: i32, ending_index: i32, search_element: i32) {
    if ending_index > starting_index {
        //for find element
        let mid_index= starting_index + (ending_index - 1) / 2;
        match  arr[mid_index as usize] .cmp(&search_element ) {
            Ordering::Greater =>  binary_search(arr, starting_index, mid_index - 1, search_element),
            Ordering::Less =>  binary_search(arr, mid_index + 1, ending_index, search_element),
            Ordering::Equal =>  debug!("\n Item {} found at index {}\n", search_element, mid_index)
        }
    }
}
