// auther: phoenix
// date: 2023-06-19 07:59:23

/*


*/


main()
function main() {
  isPalindromeTest()
}

function isPalindromeTest(){
  console.log(isPalindrome('A man, a plan, a canal: Panama')) // true
  console.log(isPalindrome('race a car')) // false
  console.log(isPalindrome(' ')) // true
}

function isPalindrome (s:string) {
  let str = s.toLocaleLowerCase().replace(/[^a-zA-Z0-9]/g, "")
  return str === str.split('').reverse().join('')
}