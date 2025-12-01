package main

import (
	"io"
)

type Day interface {
	Part1() (string, error)
	Part2() (string, error)
}

type day1 struct {
	input io.Reader
}

func NewDay1(input io.Reader) Day {
	return &day1{input: input}
}

func (d *day1) Part1() (string, error) {
	return "0", nil
}

func (d *day1) Part2() (string, error) {
	return "0", nil
}
