// auther: phoenix
// date: 2023-06-19 07:59:16

/*



*/

fn main() {
  is_alindrome_test();
}

fn is_alindrome_test() {
  println!("{}", is_palindrome(String::from("A man, a plan, a canal: Panama"))); // true
  println!("{}", is_palindrome(String::from("race a car"))); // true
  println!("{}", is_palindrome(String::from(" "))); // true
}

fn is_palindrome (s:String)->bool {
  let c:String = s.chars()
  .filter(|c| c.is_alphanumeric()|| c.is_ascii_digit()).collect::<String>();
  let s:String = String::from(c).to_lowercase();

  s == s.chars().rev().collect::<String>()
}


// https://leetcode.cn/problems/valid-palindrome/solution/rustliang-xing-dai-ma-by-star_tears-kz2t/
// 转换小写，过滤字符，逆序比较
pub fn up_is_palindrome(s: String) -> bool {
  let s1 = s
      .to_lowercase()
      .chars()
      .filter(|c| c.is_alphabetic() || c.is_alphanumeric())
      .collect::<String>();
  s1 == s1.chars().rev().collect::<String>()
}