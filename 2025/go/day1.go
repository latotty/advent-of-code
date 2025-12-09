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
	parsedInput []int

	result1 string
	result2 string
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

		num, err := strconv.Atoi(numText)
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

func (d *day1) process() error {
	if d.result1 != "" {
		return nil
	}

	if err := d.parseInput(); err != nil {
		return err
	}

	dialAt := 50

	atZero := 0
	passedZero := 0
	for _, step := range d.parsedInput {
		startAtZero := dialAt == 0
		passedZero += AbsInt(step) / 100
		step %= 100

		dialAt += step
		if (!startAtZero && dialAt < 0) || dialAt > 100 {
			passedZero++
		}

		dialAt %= 100

		if dialAt < 0 {
			dialAt = 100 + dialAt
		}

		if dialAt == 0 {
			if !startAtZero {
				passedZero++
			}
			atZero++
		}

		// fmt.Printf("%d -> %d -> %d | %d\n", step, dialAt, atZero, passedZero)
	}

	d.result1 = fmt.Sprintf("%d", atZero)
	d.result2 = fmt.Sprintf("%d", passedZero)

	return nil
}

func (d *day1) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day1) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
