
// main.rs

extern "C" {
    fn say_hello();
    fn add_num(a:i32,b:i32)->i32;
}

fn main() {
    unsafe {
        say_hello();
        println!("Sum of 2 and 3 is :{}",add_num(2,3));
    }
}
