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
	entries, err := readInput("input.txt")
	if err != nil {
		return err
	}

	var valid int
	for _, entry := range entries {
		if entry.Valid() {
			valid++
		}
	}
	fmt.Printf("%d\n", valid)
	return nil
}

func readInput(path string) ([]entry, error) {
	data, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, err
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	entries := make([]entry, 0, len(lines))
	for _, line := range lines {
		var e entry
		_, err := fmt.Sscanf(line, "%d-%d %c: %s", &e.min, &e.max, &e.letter, &e.password)
		if err != nil {
			return nil, err
		}
		entries = append(entries, e)
	}
	return entries, nil
}

type entry struct {
	policy
	password string
}

func (e entry) Valid() bool {
	var count int
	for _, c := range e.password {
		if c == e.letter {
			count++
		}
	}
	return count >= e.min && count <= e.max
}

type policy struct {
	min    int
	max    int
	letter rune
}
