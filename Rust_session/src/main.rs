use std::mem;

fn addValue(x:i32,y:i32) -> i32 {
    x+y
}


fn print_str(s: &mut String) {
    s.push_str(("print"));
    println!("Got: {}", s);
}



fn main()
{
    let s = String::from("Rust");  // s is ownwer of value
    let s0 = "hello"; // it is immutable and don't have push or modify function.
    println!("{}",s); //brorow 
    println!("{}",s); //brorow 
    let s1 = s; //Moved the value
    //let s2 = s;  //Error because s is not valide after s1 = s

    
    let s = String::from("Rust");  // s is ownwer of value
    let s1 = &s; // borrowing value as immutable refrence.
    let s2 = &s; // borrowing value as immutable refrence.

    let s3 = s.clone(); //copy of value

    let mut s4 = s;  // moved or transfer the owership

    // println!("{}",s); //Error

    // when we pass the value to function as pass by value, ownership will transfer 
    // unless the type implements the Copy trait.

    let a =3;
    let b =5;

    let result = addValue(a,b);

    println!("value {}",a);

    s4.push_str("foo");

    print_str(&mut s4); // passing value as mut string

    println!("value {:?}",s4.bytes());
   // println!("size_of::<String>() = {}", mem::size_of::<String>());

    //borrowing rules
    /*
        You can have either:
        One mutable reference (&mut T)
        Or any number of immutable references (&T)
     */

    let mut x = 5; // 
    
    let r1 = &x; // immutable ref
    let r2 = &x; // immutable ref
    
    println!("{} {}", r1, r2); // 
    let mut r3 = &mut x; // mutable ref
    

    
}