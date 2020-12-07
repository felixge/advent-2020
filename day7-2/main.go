package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
	"strconv"
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

	sum, err := bagsRequiredInside(input, "shiny gold")
	if err != nil {
		return err
	}
	fmt.Printf("%d\n", sum)

	return nil
}

func readInput(path string) (string, error) {
	data, err := ioutil.ReadFile(path)
	return string(data), err
}

func bagsRequiredInside(input string, bag string) (int, error) {
	rules, err := parseRules(input)
	if err != nil {
		return 0, err
	}

	var requiredInside func(string) int
	requiredInside = func(bag string) int {
		sum := 1
		for _, rule := range rules {
			if rule.bag == bag {
				for containedBag, n := range rule.contains {
					sum += n * requiredInside(containedBag)
				}
				return sum
			}
		}
		panic("unreachable")
	}

	return requiredInside(bag) - 1, nil
}

func parseRules(input string) ([]*rule, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	var rules []*rule
	for _, line := range lines {
		r, err := parseRule(line)
		if err != nil {
			return nil, err
		}
		rules = append(rules, r)
	}
	return rules, nil
}

func parseRule(input string) (*rule, error) {
	parts := strings.Split(input, " bags contain ")
	if len(parts) != 2 {
		return nil, errors.New("bad format: no separator")
	}
	r := &rule{bag: parts[0], contains: map[string]int{}}
	if parts[1] == "no other bags." {
		return r, nil
	}

	matches := ruleRegex.FindAllStringSubmatch(parts[1], -1)
	if len(matches) == 0 {
		return nil, errors.New("bad format: no matches")
	}

	for _, m := range matches {
		n, err := strconv.ParseInt(m[1], 10, 64)
		if err != nil {
			return nil, fmt.Errorf("bad format: %w", err)
		}
		r.contains[m[2]] = int(n)
	}
	return r, nil
}

var ruleRegex = regexp.MustCompile(`([\d]+) ([^,.]+) bags?`)

type rule struct {
	bag      string
	contains map[string]int
}
