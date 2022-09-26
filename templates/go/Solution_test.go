package Solution

import (
	"testing" 
	"fmt"
	"github.com/stretchr/testify/assert"
)

func TestAdd(t *testing.T) {
	test_data := []struct {
		x        int
		y        int
		expected int
	}{
		{x: 1, y: 2, expected: 3},
		{x: 2, y: -2, expected: 0}}

	for index, test := range test_data {
		t.Run(fmt.Sprintf("%v) %v + %v = %v", index, test.x, test.y, test.y), func(t *testing.T) {
			assert.Equal(t, add(test.x, test.y), test.expected)
		})
	}
}
