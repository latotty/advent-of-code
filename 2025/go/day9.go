package main

import (
	"fmt"
	"strings"
)

type day9 struct {
	input string

	result1 string
	result2 string
}

func NewDay9(input string) Day {
	return &day9{input: input}
}

func (d *day9) process() error {
	if d.result1 != "" {
		return nil
	}

	var result1 int
	var result2 int

	lines := strings.Split(strings.TrimSpace(d.input), "\n")
	squares := make([]*Pos2, len(lines))
	for i, line := range lines {
		line := strings.TrimSpace(line)
		strCoords := strings.Split(line, ",")
		pos, err := NewPos2FromStrSlice(strCoords)
		if err != nil {
			return fmt.Errorf("line parse err: %w", err)
		}
		squares[i] = pos
	}

	for i, s1 := range squares {
		for _, s2 := range squares[i+1:] {
			result1 = MaxCmp(result1, s1.Area(s2))
		}
	}

	d.result1 = fmt.Sprintf("%d", result1)
	d.result2 = fmt.Sprintf("%d", result2)

	return nil
}

func (d *day9) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day9) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
