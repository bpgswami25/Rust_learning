

fn main()
{
    let s = String::from("Rust");  // s is ownwer of value
    println!("{}",s); //brorow 
    println!("{}",s); //brorow 
    let s1 = s; //Moved the value
    //let s2 = s;  //Error because s is not valide after s1 = s

    
    let s = String::from("Rust");  // s is ownwer of value
    let s1 = &s; // borrowing value as immutable refrence.
    let s2 = &s; // borrowing value as immutable refrence.

    let s3 = s.clone(); //copy of value

    let s4 = s;  // moved or transfer the owership

    // println!("{}",s); //Error




    
}