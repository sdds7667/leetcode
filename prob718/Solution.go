package main

func max(x int, y int) int {
	if x > y {
		return x
	}
	return y
}

func findLength(nums1 []int, nums2 []int) int {

	r := len(nums1)
	c := len(nums2)

	dp := make([][]int, r+1)
	for j := range dp {
		dp[j] = make([]int, c+1)
	}

	m := 0

	for i := r - 1; i >= 0; i-- {
		for j := c - 1; j >= 0; j-- {
			if nums1[i] == nums2[j] {
				dp[i][j] = 1 + dp[i+1][j+1]
				m = max(m, dp[i][j])
			}
		}
	}
	return m
}
