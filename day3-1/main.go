package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func run() error {
	g, err := readInput("input.txt")
	if err != nil {
		return err
	}

	fmt.Printf("%d\n", g.Solve())
	return nil
}

func readInput(path string) (grid, error) {
	data, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, err
	}
	return newGrid(string(data)), nil
}

func newGrid(s string) grid {
	s = strings.TrimSpace(s)
	var g grid
	var row []rune
	for _, c := range s {
		if c == '\n' {
			g = append(g, row)
			row = nil
		} else {
			row = append(row, c)
		}
	}
	if len(row) > 0 {
		g = append(g, row)
	}
	return g
}

type grid [][]rune

func (g grid) Lookup(x, y int) rune {
	if y >= len(g) {
		return 0
	}
	return g[y][x%len(g[y])]
}

func (g grid) Solve() int {
	x, y := 0, 0
	count := 0
	for {
		x += 3
		y += 1
		v := g.Lookup(x, y)
		if v == 0 {
			return count
		} else if v == '#' {
			count++
		}
	}
	panic("unreachable")
}
