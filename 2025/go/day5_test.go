package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay5Example(t *testing.T) {
	t.Parallel()

	tmap := []struct {
		name        string
		input       string
		part1Result string
		part2Result string
	}{{
		name: "example",
		input: `3-5
10-14
16-20
12-18

1
5
8
11
17
32`,
		part1Result: "3",
		part2Result: "14",
	}, {
		name: "edge 1",
		input: `1-2
		2-4
		3-8

0`,
		part1Result: "0",
		part2Result: "8",
	}, {
		name:        "totty input",
		input:       TReadFileStr(t, "./inputs/day5.txt"),
		part1Result: "707",
		part2Result: "361615643045059",
	}}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			t.Parallel()

			day := NewDay5(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
