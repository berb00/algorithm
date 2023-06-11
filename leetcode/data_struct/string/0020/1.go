// auther: phoenix
// date: 2023-06-11 22:26:16
package main

import (
	"fmt"
)

/*


 */

func main() {
	isValid1Test()

}

func isValid1Test() {
	fmt.Println(isValid1("[]"))
	fmt.Println(isValid1("[)"))
	fmt.Println(isValid1("[()()]"))
	fmt.Println(isValid1("[()[]]"))
	fmt.Println(isValid1("[{()[]}]"))
}

func isValid1(s string) bool {
	length := len(s)
	if length == 0 || length%2 == 1 {
		return false
	}

	stack := []byte{}
	pairs := map[byte]byte{
		')': '(',
		'}': '{',
		']': '[',
	}

	for i := 0; i < length; i++ {
		if pairs[s[i]] > 0 {
			if len(stack) == 0 || stack[len(stack)-1] != pairs[s[i]] {
				return false
			}
			stack = stack[:len(stack)-1]
		} else {
			stack = append(stack, s[i])
		}
	}

	return len(stack) == 0
}
