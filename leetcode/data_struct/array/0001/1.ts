// auther: phoenix
// date: 2023-06-08 15:08:43



function twoSum1 (nums:number[], target:number): number[] {
  const len:number = nums.length
  for (let i = 0; i < len; i++) {
    for (let j = i+1; j < (len); j++) {
      if(nums[i] + nums[j] === target) {
        return [i, j]
      }
    }
  }
  return []
}

test()
function test(){
  console.log(twoSum1([2, 5, 9, 7], 9))
  console.log(twoSum1([3, 2, 4], 6))
  console.log(twoSum1([3, 3], 6))
}