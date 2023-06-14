// auther: phoenix
// date: 2023-06-14 09:37:09

/*



*/

fn main() {
  plus_one1_test();
}

fn plus_one1_test(){
  println!("{:?}", plus_one1(vec![1,2,3]));     // [1,2,4]
  println!("{:?}", plus_one1(vec![4,3,2,1]));   // [4,3,2,2]
  println!("{:?}", plus_one1(vec![0]));         // [1]
  println!("{:?}", plus_one1(vec![9,9]));         // [1]
}

fn plus_one1(digits: Vec<i32>) -> Vec<i32> {
  let length:usize = digits.len();
  let mut arr:Vec<i32> = digits;
  for i in (0..length).rev(){
    if arr[i] != 9 {
      arr[i] += 1;
      for j in (i+1)..length {
        arr[j] = 0;
      }
      return arr;
    }
  }
  let mut nums: Vec<i32> = vec![];
  for _ in 0..length+1 {
    nums.push(0);
  }
  nums[0] = 1;
  return nums;
}

