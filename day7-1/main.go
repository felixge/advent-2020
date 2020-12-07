package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
	"sort"
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

	sum, err := countBagsThatCanContain(input, "shiny gold")
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

func countBagsThatCanContain(input string, bag string) (int, error) {
	bags, err := bagsThatCanContain(input, bag)
	return len(bags), err
}

func bagsThatCanContain(input string, bag string) ([]string, error) {
	rules, err := parseRules(input)
	if err != nil {
		return nil, err
	}

	heads := []string{bag}
	results := map[string]bool{}

	for len(heads) > 0 {
		var newHeads []string
		for _, rule := range rules {
			for _, head := range heads {
				if _, ok := rule.contains[head]; ok {
					if !results[rule.bag] {
						results[rule.bag] = true
						newHeads = append(newHeads, rule.bag)
					}
				}
			}
		}
		heads = newHeads
	}

	var resultsS []string
	for result := range results {
		resultsS = append(resultsS, result)
	}
	sort.Strings(resultsS)

	return resultsS, nil
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
