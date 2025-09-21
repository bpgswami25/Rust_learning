// main.rs

unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 5;
    let b = 7;
    let result = unsafe { add(a, b) };
    println!("The sum of {} and {} is {}", a, b, result);
}
