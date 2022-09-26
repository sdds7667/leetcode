package Solution

func findKthPositive(arr []int, k int) int {
	l := 0
	r := len(arr) - 1

	for l <= r {
		m := (l + r) / 2
		vm := arr[m] - m - 1
		if vm >= k {
			r = m - 1
		} else {
			l = m + 1
		}
	}
    l--
    if (l == -1) {
        return k
    }
    
    return arr[l] + k - (arr[l] - l - 1)
}

