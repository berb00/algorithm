// auther: phoenix
// date: 2023-06-10 10:28:20
package main

import (
	"fmt"
)

/*


 */

func main() {
	fmt.Println(romanToInt1("III"))     // 3
	fmt.Println(romanToInt1("IV"))      // 4
	fmt.Println(romanToInt1("IX"))      // 9
	fmt.Println(romanToInt1("LVIII"))   // 58
	fmt.Println(romanToInt1("MCMXCIV")) // 1994
}

func romanToInt1(str string) int {
	length := len(str)
	if length == 0 {
		return 0
	}

	count := 0
	param := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}
	for i, v := range str {
		val := param[byte(v)]
		// fmt.Println(i, v, val)
		if i < length-1 && val < param[str[i+1]] {
			count -= val
		} else {
			count += val
		}
	}

	return count
}
