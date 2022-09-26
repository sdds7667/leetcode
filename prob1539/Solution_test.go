package Solution

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAdd(t *testing.T) {
	test_data := []struct {
		arr      [5]int
		k        int
		expected int
	}{
		{arr: [5]int{2, 3, 4, 7, 11}, k: 5, expected: 9},
		{arr: [5]int{2, 3, 4, 7, 11}, k: 1, expected: 1},
		{arr: [5]int{2, 3, 4, 7, 11}, k: 2, expected: 5},
		{arr: [5]int{2, 3, 4, 7, 11}, k: 3, expected: 6},
		{arr: [5]int{2, 3, 4, 7, 11}, k: 4, expected: 8},
		{arr: [5]int{2, 3, 4, 7, 11}, k: 10, expected: 15},
	}

	for index, test := range test_data {
		t.Run(fmt.Sprintf("%v) %v + %v = %v", index, test.arr, test.k, test.expected), func(t *testing.T) {
			assert.Equal(t, findKthPositive(test.arr, test.k), test.expected)
		})
	}
}
