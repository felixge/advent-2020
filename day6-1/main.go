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
	input, err := readInput("input.txt")
	if err != nil {
		return err
	}
	sum := SumGroupCounts(input)
	fmt.Printf("%d\n", sum)

	return nil
}

func readInput(path string) (string, error) {
	data, err := ioutil.ReadFile(path)
	return string(data), err
}

func SumGroupCounts(input string) int {
	groups := strings.Split(strings.TrimSpace(input), "\n\n")
	var sum int
	for _, group := range groups {
		sum += groupCount(group)
	}
	return sum
}

func groupCount(group string) int {
	answers := map[rune]struct{}{}
	for _, person := range strings.Split(group, "\n") {
		for _, answer := range person {
			answers[answer] = struct{}{}
		}
	}
	return len(answers)
}
