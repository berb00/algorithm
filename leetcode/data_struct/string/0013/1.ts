// auther: phoenix
// date: 2023-06-10 10:28:27

/*


*/

demo()
function demo() {
  console.log(romanToInt1('III'))     // 3
  console.log(romanToInt1('IV'))      // 4
  console.log(romanToInt1('IX'))      // 9
  console.log(romanToInt1('LVIII'))   // 58
  console.log(romanToInt1('MCMXCIV')) // 1994
}

function romanToInt1(str: string):number{
  let param = {
    'I': 1,
    'V': 5,
    'X': 10,
    'L': 50,
    'C': 100,
    'D': 500,
    'M': 1000,
  }
  let length = str.length
  if(length <= 0) {
    return 0
  }

  let number = 0
  for(let i = 0; i < length; i++){
    if(i < length - 1 && param[str[i]] < param[str[i + 1]]) {
      number -= param[str[i]]
    } else {
      number += param[str[i]]
    }
  }

  return number
}