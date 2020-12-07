package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_MaxSeatID(t *testing.T) {
	got, err := MaxSeatID(`
FBFBBFFRLR
BFFFBBFRRR
BBFFBBFRLL
FFFBBBFRRR
`)
	require.NoError(t, err)
	require.Equal(t, got, 820)
}

func Test_parseSeat(t *testing.T) {
	tests := []struct {
		input string
		seat  *seat
		id    int
	}{
		{"FBFBBFFRLR", &seat{44, 5}, 357},
		{"BFFFBBFRRR", &seat{70, 7}, 567},
		{"FFFBBBFRRR", &seat{14, 7}, 119},
		{"BBFFBBFRLL", &seat{102, 4}, 820},
	}
	for _, test := range tests {
		s, err := parseSeat(test.input)
		require.NoError(t, err)
		require.Equal(t, s, test.seat)
		require.Equal(t, s.ID(), test.id)
	}
}
