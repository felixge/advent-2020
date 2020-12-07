package main

import (
	"errors"
	"fmt"
	"sort"
	"strings"
)

func MissingSeatID(s string) (int, error) {
	lines := strings.Split(strings.TrimSpace(s), "\n")
	var seats []*seat
	for _, line := range lines {
		s, err := parseSeat(line)
		if err != nil {
			return 0, fmt.Errorf("bad seat: %q: %w", line, err)
		}
		seats = append(seats, s)
	}
	return missingSeatID(seats)
}

func missingSeatID(seats []*seat) (int, error) {
	sort.Slice(seats, func(i, j int) bool {
		return seats[i].ID() < seats[j].ID()
	})
	prevID := -1
	for _, s := range seats {
		id := s.ID()
		if prevID != -1 && prevID+2 == id {
			return id - 1, nil
		}
		prevID = id
	}
	return 0, errors.New("seat not found")
}

func parseSeat(input string) (*seat, error) {
	if len(input) != 10 {
		return nil, errors.New("bad seat length")
	}
	s := &seat{}

	var err error
	s.Row, err = binarySpacePartition(input[0:7], 'F', 'B')
	if err != nil {
		return nil, err
	}
	s.Column, err = binarySpacePartition(input[7:10], 'L', 'R')
	if err != nil {
		return nil, err
	}

	return s, nil
}

func binarySpacePartition(input string, lower rune, upper rune) (int, error) {
	min := 0
	max := 1<<(len(input)) - 1
	for _, m := range input {
		switch m {
		case lower:
			max = min + (max-min)/2
		case upper:
			min = max - (max-min)/2
		default:
			return 0, fmt.Errorf("bad char: %q", string(m))
		}
	}
	if min != max {
		return 0, fmt.Errorf("unclear partition: min=%d max=%d", min, max)
	}
	return min, nil
}

type seat struct {
	Row    int
	Column int
}

func (s seat) ID() int {
	return s.Row*8 + s.Column
}
