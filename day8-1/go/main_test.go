package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_accumBeforeLoop(t *testing.T) {
	input := `
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
`
	got, err := accBeforeLoop(input)
	require.NoError(t, err)
	require.Equal(t, 5, got)
}
