package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay1Example(t *testing.T) {
	tmap := []struct {
		name        string
		input       string
		part1Result string
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
	}, {
		name: "biggus",
		input: `R250
				L400`,
		part1Result: "2",
	}}

	for _, input := range tmap {
		t.Run(input.name, func(t *testing.T) {
			// t.Parallel()

			day := NewDay1(input.input)
			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)
		})
	}
}
