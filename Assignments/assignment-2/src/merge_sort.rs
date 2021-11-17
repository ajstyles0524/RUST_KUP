use log::*;
pub fn mergesort(arr: &[i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    mergesort(&arr[..mid]);
    mergesort(&arr[mid..]);

    // to_vec() is used to create vector from array
    // vector is module in Rust that provides the container space to store value.
    let mut thing = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut thing[..]);
    debug!("sorted: {:?}", thing);
}
/// Merge helper.
//arr1 for left side
//arr2 for right side
fn merge(arr1: &[i32], arr2: &[i32], ret: &mut [i32]) {
    let mut left = 0; // Head of left pile.
    let mut right = 0; // Head of right pile.
    let mut index = 0;
    // Compare element and insert back to result array.
    while left < arr1.len() && right < arr2.len() {
        if arr1[left] <= arr2[right] {
            ret[index] = arr1[left];
            index += 1;
            left += 1;
        } else {
            ret[index] = arr2[right];
            index += 1;
            right += 1;
        }
    }

    // Copy the reset elements to returned array.
    if left < arr1.len() {
        // if there is anything left in the left half append it after sorted member.
        ret[index..].copy_from_slice(&arr1[left..]);
    }
    if right < arr2.len() {
        // if there is anything left in the right half append it after sorted member.
        ret[index..].copy_from_slice(&arr2[right..]);
    }
}
pub fn mergesortmain(arr: &[i32]) {
    mergesort(arr);
    info!("Merge sort is performed");
}
