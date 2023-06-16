// auther: phoenix
// date: 2023-06-15 14:49:49

/*

11
01

1010
1011



*/

// 方法一、模拟手工计算
addBinary1test()
function addBinary1test() {
  console.log(addBinary1('11', '1'))      // 100
  console.log(addBinary1('1010', '1011')) // 10101
}

function addBinary1 (a: string, b: string) : string {
  let sum:string = ''
  let add = 0 // 进位
  let maxLength = Math.max(a.length, b.length)
  a = a.padStart(maxLength, '0')
  b = b.padStart(maxLength, '0')

  for (let i = maxLength - 1; i >= 0 ; i--) {
    let sum1 = Number(a[i]) + Number(b[i]) + add
    switch (sum1) {
      case 0:
        sum += '0'
        add = 0
        break;
      case 1:
        sum += '1'
        add = 0
        break;
      case 2:
        sum += '0'
        add = 1
        break;
      case 3:
        sum += '1'
        add = 1
        break;
      default:
        break;
    }
  }
  sum = sum.split('').reverse().join('')
  if (add === 1) {
    sum = '1' + sum
  }

  return sum
}

// 方法二、进制转换
// addBinary2test()
function addBinary2test() {
  console.log(addBinary2('11', '1'))      // 100
  console.log(addBinary2('1010', '1011')) // 10101
}
function addBinary2 (a: string, b: string) : string {
  let a10 = parseInt(a, 2)
  let b10 = parseInt(b, 2)

  return (a10 + b10).toString(2) + ''
}