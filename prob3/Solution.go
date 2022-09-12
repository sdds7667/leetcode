package main

import "fmt"

func lengthOfLongestSubstring(s string) int {
	mp := make(map[rune]int)
	ls := 0
	b := 0
	for p, c := range s {
		// fmt.Println(mp)
		if val, ok := mp[c]; ok && ls <= val {
			ls = val + 1
		}
		mp[c] = p
		if p-ls+1 > b {
			b = p - ls + 1
		}
	}
	return b
}

func main() {
	fmt.Println(lengthOfLongestSubstring("abba"))
}
