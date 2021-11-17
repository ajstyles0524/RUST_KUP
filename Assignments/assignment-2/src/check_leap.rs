use log::*;
pub fn count_leap() {
    let arr: [i32; 5] = [2016, 2020, 2000, 2100, 2200];
    let mut count = 0;
    for item in arr{
        if item % 4 == 0 && item % 100 != 0 || item % 400 == 0 {
            count += 1;
        }
    }
    debug!("{}", count);
}
