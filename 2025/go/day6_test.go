package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay6Example(t *testing.T) {
	t.Parallel()

	tmap := []struct {
		name        string
		input       string
		part1Result string
		part2Result string
	}{{
		name: "example",
		input: `123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  `,
		part1Result: "4277556",
		part2Result: "3263827",
	}}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			t.Parallel()

			day := NewDay6(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
