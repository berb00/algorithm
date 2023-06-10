// auther: phoenix
// date: 2023-06-10 10:28:24

use std::collections::HashMap;


/*



*/

fn main() {
  // up1_roman_to_int_test()
  roman_to_int1_test()
}

#[allow(unused)]
fn roman_to_int1_test(){
  // println!("{}", roman_to_int1(String::from("III")));     // 3
	println!("{}", roman_to_int1(String::from("IV")));      // 4
	// println!("{}", roman_to_int1(String::from("IX")));      // 9
	// println!("{}", roman_to_int1(String::from("LVIII")));   // 58
	// println!("{}", roman_to_int1(String::from("MCMXCIV"))); // 1994
}

fn roman_to_int1(s: String) -> usize {
  let mut hash = HashMap::new();
  hash.insert("I", 1);
  hash.insert("V", 5);
  hash.insert("X", 10);
  hash.insert("L", 50);
  hash.insert("C", 100);
  hash.insert("D", 500);
  hash.insert("M", 1000);
  let mut count:usize = 0;

  count
}

// https://leetcode.cn/problems/roman-to-integer/solution/shi-xing-jie-fa-foldhao-xiang-man-liao-b-earq/
// 逆向循环 + fold + 模式匹配。
#[allow(unused)]
fn up1_roman_to_int_test(){
  println!("{}", up1_roman_to_int(String::from("III")));     // 3
	println!("{}", up1_roman_to_int(String::from("IV")));      // 4
	println!("{}", up1_roman_to_int(String::from("IX")));      // 9
	println!("{}", up1_roman_to_int(String::from("LVIII")));   // 58
	println!("{}", up1_roman_to_int(String::from("MCMXCIV"))); // 1994
}
pub fn up1_roman_to_int(s: String) -> i32 {
  s.bytes().rev().fold((0, 0), |res, cur| {
      let n = match cur {
          b'I' => 1,   b'V' => 5,   b'X' => 10,   b'L' => 50,
          b'C' => 100, b'D' => 500, b'M' => 1000, _ => -9999
      };
      (if n < res.1 { res.0 - n } else { res.0 + n }, n)
  }).0
}


// https://leetcode.cn/problems/roman-to-integer/solution/rust-luo-ma-shu-zi-zhuan-zheng-shu-by-xian-za-zhi-/
#[allow(unused)]
fn up2_roman_to_int_test(){
  println!("{}", up2_roman_to_int(String::from("III")));     // 3
	println!("{}", up2_roman_to_int(String::from("IV")));      // 4
	println!("{}", up2_roman_to_int(String::from("IX")));      // 9
	println!("{}", up2_roman_to_int(String::from("LVIII")));   // 58
	println!("{}", up2_roman_to_int(String::from("MCMXCIV"))); // 1994
}
pub fn up2_roman_to_int(s: String) -> i32 {
  let tr = vec![('I',1),('V',5),('X',10),('L',50),('C',100),('D',500),('M',1000)];
  let hash:HashMap<char,i32>=tr.iter().cloned().collect();
  
  let chars:Vec<char> = s.chars().collect();
  let mut ans = 0;
  let n = chars.len();
  for i in 0..n-1{
      let cc=hash[&chars[i]];
      let nc=hash[&chars[i+1]];
      if cc<nc {
          ans -= cc; 
      }else{
          ans += cc;
      }
  } 
  ans+hash[&chars[n-1]]
}


