// auther: phoenix
// date: 2023-06-10 09:54:41

/*



*/

fn main() {
  println!("{:?}", remove_element1(&mut vec![3,2,2,3], 3));
  println!("{:?}", remove_element1(&mut vec![0,1,2,2,3,0,4,2], 2));
}

pub fn remove_element1(nums: &mut Vec<i32>, val: i32) -> usize {
  let length:usize = nums.len();
  let mut removed: usize = 0;

  for i in 0..length {
    if val == nums[i - removed] {
      nums.remove(i - removed);
      removed += 1;
    } else {
      continue;
    }

  }

  nums.len()
}