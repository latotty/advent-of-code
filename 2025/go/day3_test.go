package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay3Example(t *testing.T) {
	t.Parallel()

	tmap := []struct {
		name        string
		input       string
		part1Result string
		part2Result string
	}{{
		name: "example",
		input: `987654321111111
811111111111119
234234234234278
818181911112111`,
		part1Result: "357",
		part2Result: "3121910778619",
	}}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			t.Parallel()

			day := NewDay3(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
