// auther: phoenix
// date: 2023-06-10 09:54:46

/*


*/

demo()
function demo() {
  console.log(removeElement1([3,2,2,3], 3))
  console.log(removeElement1([0,1,2,2,3,0,4,2], 2))
}

function removeElement1(nums:number[], val: number):number{
  let len:number = nums.length
  let removed:number = 0
  for (let i = 0; i < len; i++) {
    if(val === nums[i - removed]) {
      nums.splice(i - removed, 1)
      removed++
    }
  }
  return nums.length
}