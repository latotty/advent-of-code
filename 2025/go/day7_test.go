package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay7Example(t *testing.T) {
	t.Parallel()

	tmap := []struct {
		name        string
		input       string
		part1Result string
		part2Result string
	}{
		{
			name: "example",
			input: `.......S.......
		...............
		.......^.......
		...............
		......^.^......
		...............
		.....^.^.^.....
		...............
		....^.^...^....
		...............
		...^.^...^.^...
		...............
		..^...^.....^..
		...............
		.^.^.^.^.^...^.
		...............`,
			part1Result: "21",
			part2Result: "40",
		},
		{
			name:        "totty input",
			input:       TReadFileStr(t, "./inputs/day7.txt"),
			part1Result: "1570",
			part2Result: "15118009521693",
		},
	}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			t.Parallel()

			day := NewDay7(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
