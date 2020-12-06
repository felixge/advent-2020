package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
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
	inputs, err := readInput("input.txt")
	if err != nil {
		return err
	}
	for i, a := range inputs {
		for j, b := range inputs {
			if i == j {
				continue
			} else if a+b != 2020 {
				continue
			}
			fmt.Printf("%d\n", a*b)
			return nil
		}
	}
	return errors.New("not found")
}

func readInput(path string) ([]int, error) {
	data, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, err
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	inputs := make([]int, 0, len(lines))
	for _, line := range lines {
		num, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			return nil, err
		}
		inputs = append(inputs, int(num))
	}
	return inputs, nil
}
