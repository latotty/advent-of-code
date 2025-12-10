package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay9Example(t *testing.T) {
	t.Parallel()

	tmap := []struct {
		name        string
		input       string
		part1Result string
		part2Result string
	}{
		{
			name: "example",
			input: `7,1
					11,1
					11,7
					9,7
					9,5
					2,5
					2,3
					7,3`,
			part1Result: "50",
			part2Result: "24",
		},
		{
			name: "small1",
			input: `0,0
					9,0
					9,9
					0,9`,
			part1Result: "100",
			part2Result: "100",
		},
		// https://www.reddit.com/r/adventofcode/comments/1pi5rqn/2025_day_9_part_2_check_your_solution_with_this/
		{
			name: "reddittest1",
			input: `1,0
					3,0
					3,6
					16,6
					16,0
					18,0
					18,9
					13,9
					13,7
					6,7
					6,9
					1,9`,
			part1Result: "180",
			part2Result: "30",
		},
		{
			name: "reddittest2",
			input: `1,1
					8,1
					8,3
					3,3
					3,4
					8,4
					8,9
					18,9
					18,11
					5,11
					5,9
					4,9
					4,11
					1,11
					1,7
					6,7
					6,6
					1,6`,
			part1Result: "198",
			part2Result: "88",
		},
		{
			name: "reddittest3",
			input: `1,5
					3,5
					3,8
					7,8
					7,5
					9,5
					9,10
					11,10
					11,3
					6,3
					6,7
					4,7
					4,1
					13,1
					13,12
					1,12`,
			part1Result: "156",
			part2Result: "72",
		},
		{
			name:        "example",
			input:       TReadFileStr(t, "./inputs/day9.txt"),
			part1Result: "4733727792",
			part2Result: "1566346198",
		},
	}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			t.Parallel()

			day := NewDay9(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
