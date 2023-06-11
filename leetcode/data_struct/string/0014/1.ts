// auther: phoenix
// date: 2023-06-10 14:13:55

/*


*/

demo()
function demo() {
  longestCommonPrefix1Test()
}


function longestCommonPrefix1Test(){
  console.log(longestCommonPrefix1(["flower","flow","flight"]))
  console.log(longestCommonPrefix1(["dog","racecar","car"]))
  console.log(longestCommonPrefix1(["cir","car"]))

  
}

function longestCommonPrefix1(strs: string[]): string{
  const length:number = strs.length
  if (length <= 0) {
    return ''
  }

  let str:string = ''
  const first = strs[0]
  const firstLength:number = first.length
  for (let i = 0; i < firstLength; i++) {
    let firstChar = first[i];
    for (let j = 0; j < length; j++) {
      if (strs[j].length <= i) {
				return str
			}
      if (firstChar !== strs[j][i]) {
        return str
      }
    }
    str += firstChar
  }

  return str
}