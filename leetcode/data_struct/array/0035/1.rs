// auther: phoenix
// date: 2023-06-12 09:40:14

/*



*/

fn main() {
  search_insert1_test();
}

fn search_insert1_test() {
  println!("{}", search_insert1(vec![1,3,5,6], 5)); // 2
  println!("{}", search_insert1(vec![1,3,5,6], 2)); // 1
  println!("{}", search_insert1(vec![1,3,5,6], 7)); // 4
  println!("{}", search_insert1(vec![1,3,5,6], 0)); // 0
}

fn search_insert1(nums: Vec<isize>, target: isize) -> usize {
  let mut index:usize = 0;
  let length:usize = nums.len();
  let contain: bool = nums.contains(&target);
  for i in 0..length {
    if contain && nums.get(i).unwrap() == &target {
      return i;
    }
    if !contain && nums.get(i).unwrap() <= &target {
      index = i + 1;
    }
  }
  return index;
}