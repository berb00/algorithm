// auther: phoenix
// date: 2023-06-16 09:33:00

/*


*/


main()
function main() {
  mergeTest()
}

function mergeTest(){
  let num1:number[] = [1,2,3,0,0,0]
  let num2:number[] = [1]
  let num3:number[] = [0]

  merge(num1, 3, [2,5,6], 3)
  console.log("num1: ", num1) // [1,2,2,3,5,6]

  merge(num2, 1, [], 0)
  console.log("num2: ", num2) // [1]

  merge(num3, 0, [1], 1)
  console.log("num3: ", num3) // [1]
}

function merge (num1: number[], m: number, num2:number[], n: number) {
  num1.splice(m, n, ...num2)
  num1 = num1.sort((a:number, b:number) => a - b)
}