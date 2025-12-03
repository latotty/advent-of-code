package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay1Example(t *testing.T) {
	t.Parallel()

	tmap := []struct {
		name        string
		input       string
		part1Result string
		part2Result string
	}{{
		name: "example",
		input: `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`,
		part1Result: "3",
		part2Result: "6",
	}, {
		name: "biggus",
		input: `R250
				L400`,
		part1Result: "2",
		part2Result: "7",
	}}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			t.Parallel()

			day := NewDay1(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
