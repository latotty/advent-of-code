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
	}{{
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
		part2Result: "0",
	}}

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
