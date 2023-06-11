// auther: phoenix
// date: 2023-06-11 22:26:29

use std::collections::HashMap;
/*



*/

fn main() {
  is_valid1_test();
}

fn is_valid1_test() {
  println!("{}", is_valid1(String::from("[]")));
  println!("{}", is_valid1(String::from("[)")));
  println!("{}", is_valid1(String::from("[()]")));
  println!("{}", is_valid1(String::from("[()()]")));
  println!("{}", is_valid1(String::from("[()[]]")));

}

fn is_valid1(s:String)-> bool {
  let length:usize = s.len();
  if length == 0 || length % 2 == 1 {
    return false;
  }

  let mut stack:Vec<char> = [].to_vec();
  let pairs_vec = vec![(')','('),('}','{'),(']','[')];
  let pairs:HashMap<char,char>=pairs_vec.iter().cloned().collect();
  
  for c in s.chars() {
    if pairs.contains_key(&c) {
      if stack.len() == 0 || stack[stack.len() - 1] != *pairs.get(&c).unwrap() {
        return false;
      }
      stack.pop();
    } else {
      stack.push(c);
    }
  }
  stack.len() == 0
}