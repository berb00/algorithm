// auther: phoenix
// date: 2023-06-24 21:05:54

/*


*/


main()
function main() {
  singleNumberTest()
}

function singleNumberTest(){
  console.log(singleNumber([2,2,1]))      // 1
  console.log(singleNumber([4,1,2,1,2]))  // 4
  console.log(singleNumber([1]))          // 1
  console.log(singleNumber([1,0,1]))      // 0

  
}

function singleNumber(nums: number[]): number {
  let arr:number[] = []
  let length:number = nums.length
  if(nums.length==1) return nums[0]
  for (let i = 0; i < length; i++) {
    const v = nums[i]
    if (arr.includes(v)) {
      let index = arr.findIndex(item=>item === v)
      arr.splice(index,1)
    } else {
      arr.push(v)
    }
  }

  return arr[0]
};



// https://leetcode.cn/problems/single-number/solution/jian-dan-de-jie-fa-by-okisama-qr6b/
function upSingleNumber1(nums: number[]): number {
  return nums.reduce((a, b) => a ^ b)
};
function upSingleNumber2(nums: number[]): number {
  return eval(nums.join('^'))
};

// https://leetcode.cn/problems/single-number/solution/zhao-dao-zhi-chu-xian-yi-ci-de-shu-zi-by-45pc/
function upSingleNumber3(nums: number[]): number {
  /**

异或运算，相异为真，相同为假，所以 a^a = 0 ;0^a = a
因为异或运算 满足交换律 a^b^a = a^a^b = b 所以数组经过异或运算，单独的值就剩下了

      假设 nums = [4,3,3,2,4]
      具体步骤:
          0 ^= 4 -> 4
          4 ^= 3 -> 7
          7 ^= 3 -> 4
          4 ^= 2 -> 6
          6 ^= 4 -> 2
      最后返回 2
   */
  let reduce: number = 0;
  for (let num of nums) {
      reduce = reduce ^ num;
  }
  return reduce;
};
