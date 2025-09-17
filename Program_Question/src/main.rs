/*

###  **4. Match Statement**
**Question:**  
Use `match` to print the day of the week based on a number (1 to 7).

**Topics:** Pattern matching, Enums  
**Input:** `3`  
**Output:** `"Wednesday"`

---
//todo!() for close program
*/

fn print_day(x:i8){

    match x{
        1=>println!("MON"),
        2=>println!("TUE"),
        3=>println!("WED"),
        4=>println!("THU"),
        5=>println!("FRI"),
        6=>println!("SAT"),
        7=>println!("SUN"),
        i8::MIN..=0_i8 | 8_i8..=i8::MAX =>println!("invalid value")
    }
}

fn main(){
    print_day(3);
    print_day(10);
    print_day(2);


}