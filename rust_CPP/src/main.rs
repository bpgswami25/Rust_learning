unsafe extern "C"{
   unsafe fn callFromRustToCPP();
}

#[unsafe(no_mangle)]
pub extern "C" fn callFromCPPToRust()
{
    println!("Called rust function from CPP file");

}


fn main() {
    println!("Hello, world!");
    unsafe {callFromRustToCPP()};
}
