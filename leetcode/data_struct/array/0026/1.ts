// auther: phoenix
// date: 2023-06-09 18:37:24

/*


*/

test()
function test () {
  console.log(removeDuplicates1([1,1,2]))
  console.log(removeDuplicates1([0,0,1,1,1,2,2,3,3,4]))
}

function removeDuplicates1 (nums:number[]):number {
  return [...new Set(nums)].length
}

function removeDuplicates2 (nums:number[]):number {

  
  return 0
}

function removeDuplicates(nums) {
  const n = nums.length;
  if (n === 0) {
      return 0;
  }
  let fast = 1, slow = 1;
  while (fast < n) {
      if (nums[fast] !== nums[fast - 1]) {
          nums[slow] = nums[fast];
          ++slow;
      }
      ++fast;
  }
  return slow;
};
