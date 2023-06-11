// auther: phoenix
// date: 2023-06-10 14:13:46
package main

import (
	"fmt"
)

/*


 */

func main() {
	longestCommonPrefix1Test()
}

func longestCommonPrefix1Test() {
	// fmt.Println(longestCommonPrefix1([]string{"flower", "flow", "flight"}))
	// fmt.Println(longestCommonPrefix1([]string{"dog", "racecar", "car"}))
	// fmt.Println(longestCommonPrefix1([]string{"cir", "car"}))
	fmt.Println(longestCommonPrefix1([]string{"ab", "a"}))
}

func longestCommonPrefix1(strs []string) string {
	length := len(strs)
	if length <= 0 {
		return ""
	}

	first := strs[0]
	firstLength := len(strs[0])
	str := ""

	for i := 0; i < firstLength; i++ {
		firstChar := first[i]
		for j := 0; j < length; j++ {
			if len(strs[j]) <= i {
				return str
			}
			if firstChar != strs[j][i] {
				return str
			}
		}
		str += string(firstChar)
	}

	return str
}
