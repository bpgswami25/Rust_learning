use std::collections::VecDeque;

pub fn reverse(input: &str) -> String {
 
      let mut deque = VecDeque::new();

    for a in input.chars(){
        deque.push_front(a);
    }
    let ret:String = deque.into_iter().collect();
   // println!("{:?}",ret);
   ret 
}
