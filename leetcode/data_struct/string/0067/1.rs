// auther: phoenix
// date: 2023-06-15 14:49:46

use std::cmp;
/*


    let s2 = format!("{1}是个体重{0:>0width$}KG，身高{height:?}cm的矮胖子",
                     115,
                     "肥仔",
                     width = 5,
                     height = 163);

肥仔是个体重00115KG，身高163cm的矮胖子

*/

fn main() {
  add_binary1_test();
}

fn add_binary1_test () {
  println!("{}", add_binary1(String::from("11"), String::from("1")));        // 100
  println!("{}", add_binary1(String::from("1010"), String::from("1011")));   // 10101
}

fn add_binary1(a: String, b:String) -> String {
  let mut sum:String = String::from("");
  let mut add:usize = 0;  // 进位
  let max_length:usize = cmp::max(a.len(), b.len());

  let mut s1:String = format!("{0:>0width$}", a, width = max_length);
  let mut s2:String = format!("{0:>0width$}", b, width = max_length);

  for i in (0..max_length).rev() {
    println!("-{} ", s1.get(i..i+1));
    // let sum1:usize = s1[i].to_



  }

  return sum
}


// https://leetcode.cn/problems/add-binary/solution/rust-yuan-li-wo-du-dong-yong-apiyi-xing-dai-ma-ke-/
// 方法二、进制转换
pub fn add_binary2(a: String, b: String) -> String {
  format!("{:b}", i128::from_str_radix(a.as_str(),2).unwrap() + i128::from_str_radix(b.as_str(),2).unwrap())
}
