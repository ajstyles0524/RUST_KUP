use log::*;
mod fibo;
fn main() {
    env_logger::init();
    debug!("Hello, world!");
    for num in 1..11 {
        debug!("fibonacci ({}) = {}", num, fibo::fib(num));
    }
}
