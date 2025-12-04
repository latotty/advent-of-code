package main

import (
	"fmt"
	"strings"
)

type day4 struct {
	input string

	result1 string
	result2 string
}

func NewDay4(input string) Day {
	return &day4{input: input}
}

const (
	DAY4_SPACE = byte('.')
	DAY4_PAPER = byte('@')
)

func (d *day4) iterate(width, height int, table, table2 []byte) int {
	var result int

	for y := 0; y < height; y++ {
		row := table[y*width:]

		for x := 0; x < height; x++ {
			current := row[x]

			if current != DAY4_PAPER {
				continue
			}

			emptyAmount := 0
			for y2 := y - 1; y2 <= y+1 && emptyAmount < 5; y2++ {
				if y2 < 0 || y2 >= height {
					emptyAmount += 3
					continue
				}

				row2 := table[y2*width:]

				for x2 := x - 1; x2 <= x+1 && emptyAmount < 5; x2++ {
					if x2 < 0 || x2 >= width || row2[x2] == DAY4_SPACE {
						emptyAmount += 1
					}
				}
			}

			if emptyAmount >= 5 {
				table2[y*width+x] = DAY4_SPACE
				result++
			}
		}
	}

	return result
}

func (d *day4) process() error {
	if d.result1 != "" {
		return nil
	}

	splitted := strings.Split(d.input, "\n")
	height := len(splitted)
	width := len(splitted[0])
	table := make([]byte, width*height)
	table2 := make([]byte, width*height)

	for y, row := range splitted {
		row = strings.TrimSpace(row)
		copy(table2[y*width:], row)
	}

	result1 := -1
	var result2 int
	for {
		copy(table, table2)
		result := d.iterate(width, height, table, table2)

		if result1 == -1 {
			result1 = result
		}

		if result == 0 {
			break
		}

		result2 += result
	}

	d.result1 = fmt.Sprintf("%d", result1)
	d.result2 = fmt.Sprintf("%d", result2)

	return nil
}

func (d *day4) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day4) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
