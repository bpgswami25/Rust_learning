fn main() {
    let name  = "Alice";

    println!("My name is {}",name);

    let mut name_m = String::from("Alice M");

    println!("mutable name is {}", name_m);

    name_m.push('a');
    println!("mutable name is {}", name_m);

    name_m.pop();
    println!("mutable name is {}", name_m);

    name_m.push_str(" London");
    println!("mutable name is {}", name_m);

    println!("String has value {} contain size {} char{}",name_m.is_empty() ,name_m.len() ,name_m.capacity());

    let mut num_vec = vec![1,2,3,4,5];
    num_vec.push(6);
    println!("vector has value {:?} contain size {} char{}",num_vec ,num_vec.len() ,num_vec.capacity());

    let num_arr: [i32; 5] = [1,2,3,4,5];

    println!("array has value {:?} contain size {} char{}",num_arr ,num_arr.len() ,std::mem::size_of_val(&num_arr) );

    let tup_m = (1, "hello", 4.5, 'c'); 
    println!("tuple has value {:?} contain size {}",tup_m ,std::mem::size_of_val(&tup_m) );

    let (a, b, c, d) = arthm(10, 5);
    println!("sum {}, sub {}, mul {}, div {}",a, b, c, d);
}



fn arthm(a: i32, b: i32)->(i32, i32, i32, i32){
    (a+b, a-b, a*b, a/b)
}
