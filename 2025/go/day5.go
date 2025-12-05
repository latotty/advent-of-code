package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type day5 struct {
	input string

	result1 string
	result2 string
}

type day5FreshRange struct {
	From int
	To   int
}

func NewDay5(input string) Day {
	return &day5{input: input}
}

func (d *day5) process() error {
	if d.result1 != "" {
		return nil
	}

	var result1 int
	var result2 int

	freshRanges := make([]day5FreshRange, 0)
	lines := strings.Split(d.input, "\n")
	for i, line := range lines {
		line = strings.TrimSpace(line)

		if line == "" {
			lines = lines[i+1:]
			break
		}

		splitted := strings.Split(line, "-")
		if len(splitted) != 2 {
			return fmt.Errorf("invalid range: %s", line)
		}

		from, err := strconv.Atoi(splitted[0])
		if err != nil {
			return fmt.Errorf("invalid num(%s): %w", splitted[0], err)
		}

		to, err := strconv.Atoi(splitted[1])
		if err != nil {
			return fmt.Errorf("invalid num(%s): %w", splitted[0], err)
		}

		freshRanges = append(freshRanges, day5FreshRange{from, to})
	}

	slices.SortFunc(freshRanges, func(a, b day5FreshRange) int {
		cmp := a.From - b.From

		if cmp == 0 {
			return a.To - b.To
		}

		return cmp
	})

	// fmt.Println(freshRanges)

	for _, line := range lines {
		line = strings.TrimSpace(line)

		itemID, err := strconv.Atoi(line)
		if err != nil {
			return fmt.Errorf("invalid num(%s): %w", line, err)
		}

		for _, freshRange := range freshRanges {
			// fmt.Printf("%d ~> [%d, %d] = %t;%t\n", itemID, freshRange.From, freshRange.To, freshRange.From > itemID, freshRange.To < itemID)

			if freshRange.From <= itemID && freshRange.To >= itemID {
				result1++
				break
			}

			if freshRange.From > itemID {
				break
			}
		}
	}

	for i := 0; i < len(freshRanges); i++ {
		from := freshRanges[i].From
		to := freshRanges[i].To
		for _, fr2 := range freshRanges[i+1:] {
			if fr2.From > to {
				break
			}

			to = MaxCmp(fr2.To, to)
			i++
		}

		result2 += to - from + 1
	}

	d.result1 = fmt.Sprintf("%d", result1)
	d.result2 = fmt.Sprintf("%d", result2)

	return nil
}

func (d *day5) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day5) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
