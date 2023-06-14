// auther: phoenix
// date: 2023-06-14 09:37:12

/*


*/

demo()
function demo() {
  plusOne1Test()
}


function plusOne1Test(){
  console.log(plusOne1([1,2,3]))    // [1,2,4]
  console.log(plusOne1([4,3,2,1]))  // [4,3,2,2]
  console.log(plusOne1([0]))        // [1]
}

function plusOne1(digits: number[]):number[]{
  const length = digits.length
  for (let i = length - 1; i >= 0; i++) {
    if (digits[i] !== 9) {
      ++digits[i]
      for (let j = i + 1; j < length; j++) {
        digits[j] = 0
      }
      return digits
    }
  }

  const nums = new Array(length + 1).fill(0)
  nums[0] = 1
  return nums
}