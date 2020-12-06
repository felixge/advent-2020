package main

import (
	"testing"
)

func TestLookup(t *testing.T) {
	tests := []struct {
		grid string
		x, y int
		want rune
	}{
		// x repeat
		{"#.#..", 0, 0, '#'},
		{"#.#..", 1, 0, '.'},
		{"#.#..", 2, 0, '#'},
		{"#.#..", 3, 0, '.'},
		{"#.#..", 4, 0, '.'},

		{"#.#..", 5, 0, '#'},
		{"#.#..", 6, 0, '.'},
		{"#.#..", 7, 0, '#'},
		{"#.#..", 8, 0, '.'},
		{"#.#..", 9, 0, '.'},

		// out of y bound
		{"#.#..", 0, 1, 0},

		// x repeat on y > 0
		{"#.#..\n..#..", 0, 1, '.'},
		{"#.#..\n..#..", 1, 1, '.'},
		{"#.#..\n..#..", 2, 1, '#'},
		{"#.#..\n..#..", 3, 1, '.'},
		{"#.#..\n..#..", 4, 1, '.'},
		{"#.#..\n..#..", 5, 1, '.'},
	}
	for _, test := range tests {
		g := newGrid(test.grid)
		got := g.Lookup(test.x, test.y)
		if got != test.want {
			t.Fatalf(
				"got=%c want=%c grid=%q x=%d y=%d\n",
				got,
				test.want,
				test.grid,
				test.x,
				test.y,
			)
		}
	}
}

func TestSolve(t *testing.T) {
	g := newGrid(`
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
`)
	if got, want := g.Solve(), 7; got != want {
		t.Fatalf("got=%d want=%d", got, want)
	}
}
