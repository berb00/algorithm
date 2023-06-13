// auther: phoenix
// date: 2023-06-13 07:39:31

/*


*/

demo()
function demo() {
  lengthOfLastWord1Test()
}

function lengthOfLastWord1Test(){
  console.log(lengthOfLastWord1('Hello World'))
  console.log(lengthOfLastWord1('   fly me   to   the moon  '))
  console.log(lengthOfLastWord1('luffy is still joyboy'))
}

function lengthOfLastWord1(s:string): number{
  let str = s.trim()
  let strs = str.split(' ')
  let lastWord = strs[strs.length - 1]

  return lastWord.length
}