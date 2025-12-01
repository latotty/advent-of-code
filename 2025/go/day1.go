package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Day interface {
	Part1() (string, error)
	Part2() (string, error)
}

type day1 struct {
	input       string
	parsedInput []int64
}

func NewDay1(input string) Day {
	return &day1{input: input}
}

func (d *day1) parseInput() error {
	if d.parsedInput != nil {
		return nil
	}

	for _, line := range strings.Split(d.input, "\n") {
		line = strings.TrimSpace(line)

		if line == "" {
			continue
		}

		numText := line[1:]

		num, err := strconv.ParseInt(numText, 10, 32)
		if err != nil {
			return fmt.Errorf("invalid number %s: %w", numText, err)
		}

		switch line[0:1] {
		case "L":
			{
				d.parsedInput = append(d.parsedInput, num*-1)
			}
		case "R":
			{
				d.parsedInput = append(d.parsedInput, num)
			}
		}
	}

	return nil
}

const DAY1_DIALSIZE = 100

func (d *day1) Part1() (string, error) {
	if err := d.parseInput(); err != nil {
		return "", err
	}

	dialAt := int64(50)

	atZero := 0
	for _, step := range d.parsedInput {
		dialAt += step
		dialAt %= 100

		if dialAt < 0 {
			dialAt = 100 + (dialAt)
		}

		if dialAt == 0 {
			atZero++
		}

		// fmt.Printf("%d -> %d -> %d\n", step, dialAt, atZero)

	}

	return fmt.Sprintf("%d", atZero), nil
}

func (d *day1) Part2() (string, error) {
	return "0", nil
}
