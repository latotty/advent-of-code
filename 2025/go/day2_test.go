package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay2Example(t *testing.T) {
	tmap := []struct {
		name        string
		input       string
		part1Result string
		part2Result string
	}{{
		name:        "example",
		input:       "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
		part1Result: "1227775554",
		part2Result: "4174379265",
	}}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			// t.Parallel()

			day := NewDay2(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
