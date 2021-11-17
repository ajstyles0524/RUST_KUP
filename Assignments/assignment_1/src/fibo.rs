pub fn fib(num: i32) -> i32 {
    if num <= 0 {
         0
    } else if num == 1 {
         1
    } else {
         fib(num - 1) + fib(num - 2)
    }
}
