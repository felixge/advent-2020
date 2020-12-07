package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSumGroupCounts(t *testing.T) {
	tests := []struct {
		input string
		sum   int
	}{
		{
			input: `
abcx
abcy
abcz
			`,
			sum: 3,
		},
		{
			input: `
abc

a
b
c

ab
ac

a
a
a
a

b
`,
			sum: 6,
		},
	}
	for _, test := range tests {
		got := SumGroupCounts(test.input)
		assert.Equal(t, got, test.sum)
	}
}
