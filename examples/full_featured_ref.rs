use memoize::memoize;
use std::time::{Instant, Duration};
use std::thread;

#[derive(Debug, Clone)]
struct ComplexStruct {
    s: String,
    b: bool,
    i: Instant,
}

#[memoize(TimeToLive: Duration::from_secs(2), Capacity: 123)]
fn hello(key: String) -> &'static ComplexStruct {
    println!("hello: {}", key);
    Box::leak(Box::new(ComplexStruct {
        s: key,
        b: false,
        i: Instant::now(),
    }))
}

fn main() {
    println!("result: {:?}", hello("ABC".to_string()));
    println!("result: {:?}", hello("DEF".to_string()));
    println!("result: {:?}", hello("ABC".to_string()));  //Same as first
    thread::sleep(core::time::Duration::from_millis(2100));
    println!("result: {:?}", hello("EFG".to_string()));
    println!("result: {:?}", hello("ABC".to_string()));  //Refreshed
    println!("result: {:?}", hello("EFG".to_string()));  //Same as first
    println!("result: {:?}", hello("ABC".to_string()));  //Same as refreshed
    println!("result: {:?}", memoized_original_hello("ABC".to_string()));
}
