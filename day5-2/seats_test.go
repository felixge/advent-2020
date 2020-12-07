package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_MissingSeatID(t *testing.T) {
	seats := []*seat{
		{2, 1},
		{1, 3},
		{1, 7},
		{1, 2},
		{1, 4},
		{1, 6},
		{2, 0},
	}
	id, err := missingSeatID(seats)
	require.NoError(t, err)
	require.Equal(t, id, 13)

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
