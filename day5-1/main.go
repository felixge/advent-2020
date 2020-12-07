package main

import (
	"fmt"
	"io/ioutil"
	"os"
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
	max, err := MaxSeatID(input)
	if err != nil {
		return err
	}
	fmt.Printf("%d\n", max)

	return nil
}

func readInput(path string) (string, error) {
	data, err := ioutil.ReadFile(path)
	return string(data), err
}
