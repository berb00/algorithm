// auther: phoenix
// date: 2023-06-08 15:08:29

/*



*/

fn main() {
  println!("{:?}", two_sum1([2, 7, 11, 15].to_vec(), 9));
  println!("{:?}", two_sum1([3, 2, 4].to_vec(), 6));
  println!("{:?}", two_sum1([3, 3].to_vec(), 6));
}

fn two_sum1(nums:Vec<usize>, target:usize) -> [usize; 2] {
  let length:usize = nums.len();
  for i in 0..length {
    for j in i+1..length {
      if nums.get(i).unwrap() + nums.get(j).unwrap() == target {
        return [i, j];
      }
    }
  }

  return [0,0];
}
