package main

import (
	"fmt"
	"strconv"
	"strings"
)

type day6 struct {
	input string

	result1 string
	result2 string
}

func NewDay6(input string) Day {
	return &day6{input: input}
}

const (
	DAY6_ADD = "+"
	DAY6_MUL = "*"
)

func (d *day6) process() error {
	if d.result1 != "" {
		return nil
	}

	var result1 int
	var result2 int

	lines := strings.Split(d.input, "\n")
	operatorLine := strings.TrimSpace(lines[len(lines)-1])
	operators := make([]string, 0)
	for _, part := range strings.Split(operatorLine, " ") {
		part = strings.TrimSpace(part)
		switch part {
		case DAY6_ADD, DAY6_MUL:
			operators = append(operators, part)
		case "":
			continue
		default:
			return fmt.Errorf("invalid operator: %s", part)
		}
	}

	part1ColValues := make([]int, len(operators))
	for _, line := range lines[0 : len(lines)-1] {
		partIdx := 0
		for _, part := range strings.Split(line, " ") {
			part = strings.TrimSpace(part)
			if part == "" {
				continue
			}

			value, err := strconv.Atoi(part)
			if err != nil {
				return fmt.Errorf("invalid num(%s): %w", part, err)
			}

			switch operators[partIdx] {
			case DAY6_ADD:
				part1ColValues[partIdx] += value
			case DAY6_MUL:
				part1ColValues[partIdx] = MaxCmp(part1ColValues[partIdx], 1) * value
			}

			partIdx++
		}
	}

	for _, val := range part1ColValues {
		result1 += val
	}

	numLines := lines[0 : len(lines)-1]
	part2Values := make([]int, len(operators))
	blockIdx := 0
	var b strings.Builder
	for colIdx := 0; colIdx < len(numLines[0]); colIdx++ {
		b.Reset()
		for _, line := range numLines {
			b.WriteByte(line[colIdx])
		}

		s := strings.TrimSpace(b.String())

		if s == "" {
			blockIdx++
			continue
		}

		value, err := strconv.Atoi(s)
		if err != nil {
			return fmt.Errorf("invalid num(%s): %w", s, err)
		}

		switch operators[blockIdx] {
		case DAY6_ADD:
			part2Values[blockIdx] += value
		case DAY6_MUL:
			part2Values[blockIdx] = MaxCmp(part2Values[blockIdx], 1) * value
		}
	}

	for _, val := range part2Values {
		result2 += val
	}

	d.result1 = fmt.Sprintf("%d", result1)
	d.result2 = fmt.Sprintf("%d", result2)

	return nil
}

func (d *day6) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day6) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
