// auther: phoenix
// date: 2023-06-13 07:39:19
package main

import "strings"

/*


 */

func main() {
	lengthOfLastWord1Test()
}

func lengthOfLastWord1Test() {
	println(lengthOfLastWord1("Hello World"))
	println(lengthOfLastWord1("   fly me   to   the moon  "))
	println(lengthOfLastWord1("luffy is still joyboy"))
}

func lengthOfLastWord1(s string) int {
	str := strings.Trim(s, " ")
	strs := strings.Split(str, " ")
	lastWord := strs[len(strs)-1]

	return len(lastWord)
}
