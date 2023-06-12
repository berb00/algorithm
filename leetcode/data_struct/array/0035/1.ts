// auther: phoenix
// date: 2023-06-12 09:40:17

/*


*/

demo()
function demo() {
  searchInsert1Test()
}


function searchInsert1Test(){
  console.log(searchInsert1([1,3,5,6], 5)) // 2
  console.log(searchInsert1([1,3,5,6], 2)) // 1
  console.log(searchInsert1([1,3,5,6], 7)) // 4
  console.log(searchInsert1([1,3,5,6], 0)) // 0
}
function searchInsert1(nums: number[], target: number):number{
  let index:number = 0
  const length:number = nums.length
  const contain = nums.includes(target)
  for (let i = 0; i < length; i++) {
    const el = nums[i]
    if (contain && target === el) {
      return i
    }
    if (!contain && target >= el) {
      index =  i + 1
    }
  }
  return index
}