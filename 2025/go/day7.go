package main

import (
	"fmt"
	"strings"
)

type day7 struct {
	input string

	result1 string
	result2 string
}

func NewDay7(input string) Day {
	return &day7{input: input}
}

const (
	DAY7_START    = "S"
	DAY7_SPLITTER = "^"
)

func (d *day7) process() error {
	if d.result1 != "" {
		return nil
	}

	var result1 int
	var result2 int

	lines := strings.Split(strings.TrimSpace(d.input), "\n")

	width := len(strings.TrimSpace(lines[0]))
	beams := make([]int, width)
	newBeams := make([]int, width)
	for _, line := range lines {
		line = strings.TrimSpace(line)

		sidx := strings.Index(line, DAY7_START)
		if sidx != -1 {
			beams[sidx] = 1
			continue
		}

		splitters := FindAllIdx(line, DAY7_SPLITTER)
		if len(splitters) == 0 {
			continue
		}

		// fmt.Println(splitters)
		copy(newBeams, beams)

		for _, splitterIdx := range splitters {
			if beams[splitterIdx] == 0 {
				continue
			}

			result1++
			newBeams[splitterIdx] -= beams[splitterIdx]
			if splitterIdx > 0 {
				newBeams[splitterIdx-1] += beams[splitterIdx]
			}
			if splitterIdx < width {
				newBeams[splitterIdx+1] += beams[splitterIdx]
			}
		}

		// fmt.Println(line)
		// fmt.Println(d.beamsToStr(newBeams))

		beams, newBeams = newBeams, beams
	}

	for _, beam := range beams {
		result2 += beam
	}

	d.result1 = fmt.Sprintf("%d", result1)
	d.result2 = fmt.Sprintf("%d", result2)

	return nil
}

func (d *day7) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day7) Part2() (string, error) {
	d.process()

	return d.result2, nil
}

func (d *day7) beamsToStr(beams []int) string {
	b := strings.Builder{}
	b.Grow(len(beams))
	for _, beam := range beams {
		if beam > 10 {
			b.WriteString("+")
		} else if beam > 0 {
			b.WriteString(fmt.Sprintf("%d", beam))
		} else {
			b.WriteString(".")
		}
	}

	return b.String()
}
