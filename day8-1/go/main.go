package main

import (
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
	input, err := readInput("input.txt")
	if err != nil {
		return err
	}

	sum, err := accBeforeLoop(input)
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

func accBeforeLoop(input string) (int, error) {
	v := newVm(input)
	pcs := map[int]bool{}
	for {
		if err := v.Next(); err != nil {
			return 0, err
		}
		pc := v.PC()
		if pcs[pc] {
			return v.Acc(), nil
		}
		pcs[pc] = true
	}
}

func newVm(input string) *vm {
	return &vm{
		ops: strings.Split(strings.TrimSpace(input), "\n"),
	}
}

type vm struct {
	ops []string
	pc  int
	acc int
}

func (v *vm) Next() error {
	op := v.ops[v.pc]
	parts := strings.Split(op, " ")
	if len(parts) != 2 {
		return fmt.Errorf("bad op: %s", op)
	}
	n, err := strconv.ParseInt(parts[1], 10, 64)
	if err != nil {
		return fmt.Errorf("bad op: %s", op)
	}
	switch parts[0] {
	case "nop":
		v.pc++
	case "acc":
		v.acc += int(n)
		v.pc++
	case "jmp":
		v.pc += int(n)
	}
	return nil
}

func (v *vm) PC() int {
	return v.pc
}

func (v *vm) Acc() int {
	return v.acc
}
