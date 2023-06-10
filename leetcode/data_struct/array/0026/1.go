// auther: phoenix
// date: 2023-06-09 18:52:16
package main

import "fmt"

/*


 */

func main() {
	fmt.Println(removeDuplicates1([]int{1, 1, 2}))
	fmt.Println(removeDuplicates1([]int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}))
}

func removeDuplicates1(nums []int) int {
	length := len(nums)
	slow := 1
	for i := 1; i < length; i++ {
		if nums[i] != nums[i-1] {
			nums[slow] = nums[i]
			slow += 1
		}
	}
	return slow
}
