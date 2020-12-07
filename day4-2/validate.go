package main

import (
	"regexp"
	"strconv"
	"strings"
)

func countValidPassports(s string) int {
	count := 0
	for _, passport := range splitPassports(s) {
		if passportValid(passport) {
			count++
		}
	}
	return count
}

func splitPassports(s string) []string {
	return strings.Split(strings.TrimSpace(s), "\n\n")
}

func passportValid(p string) bool {
	type state int
	const (
		stateKey state = iota
		stateValue
	)

	remaining := requiredFields()

	var (
		key   string
		value string
	)
	var s state
	for i, c := range p {
		eof := i+1 == len(p)
		switch s {
		case stateKey:
			if c == ':' {
				delete(remaining, key)
				s = stateValue
			} else {
				key += string(c)
			}
		case stateValue:
			whiteSpace := c == ' ' || c == '\n'
			if !whiteSpace {
				value += string(c)
			}

			if whiteSpace || eof {
				validator, ok := fieldValidators[key]
				if ok && !validator(value) {
					return false
				}
				key = ""
				value = ""
				s = stateKey
			}

		}
	}
	return len(remaining) == 0
}

func requiredFields() map[string]struct{} {
	required := map[string]struct{}{}
	for field := range fieldValidators {
		required[field] = struct{}{}
	}
	return required
}

var fieldValidators = map[string]func(string) bool{
	"byr": func(s string) bool { return intBetween(s, 1920, 2002) },
	"iyr": func(s string) bool { return intBetween(s, 2010, 2020) },
	"eyr": func(s string) bool { return intBetween(s, 2020, 2030) },
	"hgt": func(s string) bool {
		matches := hgtRegexp.FindStringSubmatch(s)
		if len(matches) != 3 {
			return false
		}
		switch matches[2] {
		case "cm":
			return intBetween(matches[1], 150, 193)
		case "in":
			return intBetween(matches[1], 59, 76)
		default:
			return false
		}
	},
	"hcl": func(s string) bool { return hclRegexp.MatchString(s) },
	"ecl": func(s string) bool { return eclValid[s] },
	"pid": func(s string) bool { return pidRegexp.MatchString(s) },
}

var (
	hgtRegexp = regexp.MustCompile(`^([\d]+)(cm|in)$`)
	hclRegexp = regexp.MustCompile(`^#[0-9a-f]{6}$`)
	eclValid  = map[string]bool{
		"amb": true,
		"blu": true,
		"brn": true,
		"gry": true,
		"grn": true,
		"hzl": true,
		"oth": true,
	}
	pidRegexp = regexp.MustCompile(`^[0-9]{9}$`)
)

func intBetween(s string, min, max int) bool {
	n, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		return false
	}
	return int(n) >= min && int(n) <= max
}
