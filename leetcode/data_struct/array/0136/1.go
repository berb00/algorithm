// auther: phoenix
// date: 2023-06-24 21:50:47
package main

import (
	"fmt"
)

/*


 */

func main() {
	singleNumberTest()
}

func singleNumberTest() {
	fmt.Println(singleNumber([]int{2, 2, 1}))       // 1
	fmt.Println(singleNumber([]int{4, 1, 2, 1, 2})) // 4
	fmt.Println(singleNumber([]int{1}))             // 1
	fmt.Println(singleNumber([]int{1, 0, 1}))       // 0

}

func singleNumber(nums []int) int {
	length := len(nums)
	mp := make(map[int]int)

	if length == 1 {
		return nums[0]
	}

	for i := 0; i < length; i++ {
		v := nums[i]
		if _, ok := mp[v]; ok {
			mp[v] = 0
		} else {
			mp[v] = 1
		}
	}

	for k, v := range mp {
		if v == 1 {
			return k
		}
	}
	return -1
}

// https://leetcode.cn/problems/single-number/solution/zhi-chu-xian-yi-ci-de-yuan-su-by-disco-2-q634/
// 异或，两个相同的数异或得到0，将数组所有元素进行异或操作，得到的结果即为只出现一次的元素。
func upSingleNumber1(nums []int) (ans int) {
	// 异或 ^
	for _, v := range nums {
		ans = ans ^ v
	}
	return
}
