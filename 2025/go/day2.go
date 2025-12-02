package main

import (
	"bytes"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type day2 struct {
	input string

	result1 string
	result2 string
}

func NewDay2(input string) Day {
	return &day2{input: input}
}

func (d *day2) process() error {
	if d.result1 != "" {
		return nil
	}

	var task1Sum, task2Sum uint64

	for _, nums := range strings.Split(d.input, ",") {
		nums = strings.TrimSpace(nums)

		if nums == "" {
			continue
		}

		splitNums := strings.Split(nums, "-")
		if splitNums == nil || len(splitNums) != 2 {
			return fmt.Errorf("invalid range: %s", nums)
		}

		fromNum, err := strconv.ParseUint(splitNums[0], 10, 64)
		if err != nil {
			return fmt.Errorf("invalid number: %s", splitNums[0])
		}

		toNum, err := strconv.ParseUint(splitNums[1], 10, 64)
		if err != nil {
			return fmt.Errorf("invalid number: %s", splitNums[1])
		}

		var b bytes.Buffer

	idLoop:
		for id := fromNum; id <= toNum; id++ {
			b.Reset()
			fmt.Fprintf(&b, "%d", id)

			leng := b.Len()
			bytes := b.Bytes()

		repNumLoop:
			for repNum := 2; repNum <= leng; repNum++ {
				if leng%repNum != 0 {
					continue
				}

				partLength := leng / repNum

				part0 := bytes[0:partLength]

				for partNum := 1; partNum < repNum; partNum++ {
					partN := bytes[partLength*partNum : partLength*(partNum+1)]

					// fmt.Printf("... %d - %d/%d - %s=%s\n", id, repNum, partNum, string(part0), string(partN))

					if !slices.Equal(part0, partN) {
						continue repNumLoop
					}
				}

				if repNum <= 2 {
					// fmt.Printf("part1Found: %d\n", id)
					task1Sum += id
				}

				task2Sum += id
				continue idLoop
			}
		}
	}

	d.result1 = fmt.Sprintf("%d", task1Sum)
	d.result2 = fmt.Sprintf("%d", task2Sum)

	return nil
}

func (d *day2) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day2) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
