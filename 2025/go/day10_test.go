package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay10Example(t *testing.T) {
	t.Parallel()

	tmap := []struct {
		name        string
		input       string
		part1Result string
		part2Result string
	}{
		{
			name: "example",
			input: `
				[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
				[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
				[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
				`,
			part1Result: "7",
			part2Result: "33",
		},
		{
			name:        "small1",
			input:       `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}`,
			part1Result: "2",
			part2Result: "10",
		},
		{
			name:        "small2",
			input:       `[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}`,
			part1Result: "3",
			part2Result: "12",
		},
		{
			name:        "small3",
			input:       `[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`,
			part1Result: "2",
			part2Result: "11",
		},
	}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			t.Parallel()

			day := NewDay10(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
