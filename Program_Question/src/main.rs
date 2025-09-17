/* 
### **Question 4: HashMap and Looping**
**Write a program that takes a list of words and counts the frequency of each word using a `HashMap`. Print the results in a loop.**

**Concepts Covered:**
- HashMap
- Looping
- Mutable References

---

*/

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