/* 
### **Question 4: HashMap and Looping**
**Write a program that takes a list of words and counts the frequency of each word using a `HashMap`. Print the results in a loop.**

**Concepts Covered:**
- HashMap
- Looping
- Mutable References

---



use std::collections::HashMap;

fn hash_map(v: &Vec<&str>) {
    let mut h = HashMap::new();

    for &word in v {
        let count = h.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in &h {
        println!("{}: {}", word, count);
    }
}

fn main() {
    let v = vec!["abc", "ced", "efg", "abc", "ced"];
    hash_map(&v);
}



*/

/*

use std::collections::HashMap;

fn hash_map(v: &Vec<&str>)
{
    let mut h = HashMap::new();
    for a in v {
        h.insert(a,a.len());
    }

    println!(" Map {:?}",h);

}


fn main()
{
    let v =vec!["abc", "ced","efg"];
    hash_map(&v);

}*/




//--------------------------------------------------------------------------

/* 
### **Question 5: Structs, Enums, and Control Flow**
**Design a simple game character system using a struct `Character` and an enum `Role` with variants like `Warrior`, `Mage`, and `Archer`. Based on the role, print a different message using a match statement.**

**Concepts Covered:**
- Structs
- Enums
- Conditionals (`match`)
- Ownership (if passing structs around)

*/

// Define the Role enum
enum Role {
    Warrior,
    Mage,
    Archer,
}

// Define the Character struct
struct Character {
    name: String,
    role: Role,
}

// Function to print a message based on the role
fn describe_character(character: &Character) {
    match character.role {
        Role::Warrior => println!("{} is a brave Warrior, skilled in melee combat!", character.name),
        Role::Mage => println!("{} is a wise Mage, master of arcane arts!", character.name),
        Role::Archer => println!("{} is a swift Archer, deadly with a bow!", character.name),
    }
}

// Example usage
fn main() {
    let c1 = Character {
        name: String::from("Aragon"),
        role: Role::Warrior,
    };

    let c2 = Character {
        name: String::from("Elena"),
        role: Role::Mage,
    };

    let c3 = Character {
        name: String::from("Robin"),
        role: Role::Archer,
    };

    describe_character(&c1);
    describe_character(&c2);
    describe_character(&c3);
}
