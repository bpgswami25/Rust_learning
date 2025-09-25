/*
 Declarative Macro - Basic syntex

*/


macro_rules! math{
    ($e1 : expr, $e2: expr, "add")=>{$e1+$e2};
    ($e1 : expr, $e2: expr, "minus")=>{$e1-$e2};
    ($e1 : expr, $e2: expr, "multi")=>{$e1*$e2};
    ($e1 : expr, $e2: expr, "divide")=>{$e1 as f32 / $e2 as f32};
}

fn main()
{
    println!("{}",math!(2,5,"add"));
    println!("{}",math!(2,5,"minus"));
    println!("{}",math!(2,5,"multi"));
    println!("{}",math!(2,5,"divide"));

}