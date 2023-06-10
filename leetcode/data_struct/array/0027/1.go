// auther: phoenix
// date: 2023-06-10 09:54:36
package main

import (
	"fmt"
)

/*


 */

func main() {
	fmt.Println(removeElement1([]int{3, 2, 2, 3}, 3))
	fmt.Println(removeElement1([]int{0, 1, 2, 2, 3, 0, 4, 2}, 2))
}

func removeElement1(nums []int, val int) int {
	length := len(nums)
	removed := 0

	for i := 0; i < length; i++ {
		index := i - removed
		if val == nums[index] {
			nums = append(nums[:index], nums[index+1:]...)
			removed++
		}
	}

	return len(nums)
}
