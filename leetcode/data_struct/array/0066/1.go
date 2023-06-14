// auther: phoenix
// date: 2023-06-14 09:37:05
package main

import "fmt"

/*


 */

func main() {
	plusOne1Test()
}

func plusOne1Test() {
	fmt.Println(plusOne1([]int{1, 2, 3}))    // [1,2,4]
	fmt.Println(plusOne1([]int{4, 3, 2, 1})) // [4,3,2,2]
	fmt.Println(plusOne1([]int{0}))          // [1]
}
func plusOne1(digits []int) []int {
	length := len(digits)
	digits[length-1] = digits[length-1] + 1

	return digits
}
