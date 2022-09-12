package main

func checkInclusion(s1 string, s2 string) bool {

	oc := make(map[byte]int)
	nc := make(map[byte]int)
	n := len(s1)
	n2 := len(s2)
	for i := 0; i < n; i++ {
		oc[s1[i]]++
	}

	matches := 0
	i := 0
	for ; i < n; i++ {
		nc[s2[i]]++
	}

	if !diff(oc, nc) {
		return true
	}
	for k, v := range oc {
		if v == nc[k] {
			matches += 1
		}
	}
	j := 0
	for ; i < n2; i++ {
		ocj := nc[s2[j]]
		oci := nc[s2[i]]
		if ocj == oc[s2[j]] {
			matches--
		}
		if oci == oc[s2[i]] {
			matches--
		}

		if ocj-1 == oc[s2[j]] {
			matches++
		}

		if oci+1 == oc[s2[i]] {
			matches++
		}
		
		nc[s2[j]]--
		nc[s2[i]]++

		j++
	}
	return false
}

func diff(m1 map[byte]int, m2 map[byte]int) bool {
	for k, v := range m1 {
		if m2[k] != v {
			return true
		}
	}
	return false
}

func main() {

}
