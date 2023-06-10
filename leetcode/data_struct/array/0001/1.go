// auther: phoenix
// date: 2023-06-08 15:08:52
package main

import "fmt"

/*


 */

func main() {
	fmt.Println(twoSum1([]int{2, 7, 11, 15}, 9))
	fmt.Println(twoSum1([]int{3, 2, 4}, 6))
	fmt.Println(twoSum1([]int{3, 3}, 6))
}

func twoSum1(nums []int, target int) []int {
	length := len(nums)
	for i, v1 := range nums {
		for j := i + 1; j < length; j++ {
			if v1+nums[j] == target {
				return []int{i, j}
			}
		}
	}
	return []int{}
}
