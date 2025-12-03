package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type day3 struct {
	input string

	result1 string
	result2 string
}

func NewDay3(input string) Day {
	return &day3{input: input}
}

func (d *day3) bigNumFinder2000(bank string, numLen int) (int64, error) {
	var startIdx int
	var resRunes []rune

	for i := 0; i < numLen; i++ {
		maxPossIdx := len(bank) - (numLen - len(resRunes) - 1)
		pool := []rune(bank[startIdx:maxPossIdx])
		part := slices.Max(pool)
		idx := slices.Index(pool, part)

		startIdx = startIdx + idx + 1
		resRunes = append(resRunes, part)
	}

	resStr := string(resRunes)
	// fmt.Printf("%s -> %s\n", bank, resStr)

	res, err := strconv.ParseInt(resStr, 10, 64)
	if err != nil {
		return 0, fmt.Errorf("invalid num: %s", resStr)
	}

	return res, nil
}

func (d *day3) process() error {
	if d.result1 != "" {
		return nil
	}

	var result1 int64
	var result2 int64

	for _, bank := range strings.Split(d.input, "\n") {
		bank = strings.TrimSpace(bank)

		res, err := d.bigNumFinder2000(bank, 2)
		if err != nil {
			return fmt.Errorf("bigNumFinder2000 err: %w", err)
		}

		result1 += res

		res, err = d.bigNumFinder2000(bank, 12)
		if err != nil {
			return fmt.Errorf("bigNumFinder2000 err: %w", err)
		}

		result2 += res
	}

	d.result1 = fmt.Sprintf("%d", result1)
	d.result2 = fmt.Sprintf("%d", result2)

	return nil
}

func (d *day3) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day3) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
