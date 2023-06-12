// auther: phoenix
// date: 2023-06-12 09:40:11
package main

import "fmt"

/*


 */

func main() {
	searchInsert1Test()
}

func searchInsert1Test() {
	fmt.Println(searchInsert1([]int{1, 3, 5, 6}, 5))
	fmt.Println(searchInsert1([]int{1, 3, 5, 6}, 2))
	fmt.Println(searchInsert1([]int{1, 3, 5, 6}, 7))
	fmt.Println(searchInsert1([]int{1, 3, 5, 6}, 0))
}

func IsContain(items []int, item int) bool {
	for _, eachItem := range items {
		if eachItem == item {
			return true
		}
	}
	return false
}

func searchInsert1(nums []int, target int) int {
	index := 0
	length := len(nums)
	contains := IsContain(nums, target)

	for i := 0; i < length; i++ {
		if contains && nums[i] == target {
			return i
		}
		if !contains && nums[i] <= target {
			index = i + 1
		}
	}

	return index
}
