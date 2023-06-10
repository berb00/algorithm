// auther: phoenix
// date: 2023-06-10 09:05:09

/*



*/

fn main() {
  println!("{}", remove_duplicates1(&mut vec![1, 1, 2]));
  println!("{}", remove_duplicates1(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));
}

fn remove_duplicates1(nums: &mut Vec<usize>) -> usize {
  let length:usize = nums.len();
  if length == 0 {
    return 0;
  }

  let mut slow:usize = 1;
  for i in 1..length {
    if nums.get(i) != nums.get(i - 1) {
      nums[slow] = nums[i];
      slow += 1;
    }
  }

  slow
}