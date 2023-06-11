// auther: phoenix
// date: 2023-06-11 22:26:34

/*


*/

demo()
function demo() {
  isValid1Test()
}

function isValid1Test(){
  console.log(isValid1('[]'))
  console.log(isValid1('[)'))
  console.log(isValid1('[()]'))
  console.log(isValid1('[()()]'))
  console.log(isValid1('[()[]]'))
  console.log(isValid1('[{()[]}]'))
}
function isValid1(s:string):boolean{
  const length:number = s.length
  if(length === 0 || length % 2 === 1) {
    return false
  }

  const pairs = new Map([
    [')', '('],
    [']', '['],
    ['}', '{'],
  ])
  const stack:string[] = []
  for (const c of s) {
    if (pairs.has(c)) { // 右侧符号
      if (!stack.length || stack[stack.length - 1] !== pairs.get(c)) { // 栈内无符号 || 栈顶符号与右侧匹配符号不同
        return false
      }
      stack.pop()   // 右侧符号出栈
    } else {

      stack.push(c) // 左侧符号入栈
    }
  }


  return !stack.length
}