// auther: phoenix
// date: 2023-06-13 07:39:24

/*



*/

fn main() {
  length_of_last_word1_test();
}

fn length_of_last_word1_test () {
  println!("{}", length_of_last_word1(String::from("Hello World")));
  println!("{}", length_of_last_word1(String::from("   fly me   to   the moon  ")));
  println!("{}", length_of_last_word1(String::from("luffy is still joyboy")));
}
fn length_of_last_word1(s: String) -> usize {
  let s1:&str = s.trim();
  let strs:Vec<&str> = s1.split(' ').collect();
  let last_word:&str = strs[strs.len() - 1];
  
  last_word.len()
}