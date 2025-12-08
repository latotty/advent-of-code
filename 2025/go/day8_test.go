package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay8Example(t *testing.T) {
	t.Parallel()

	tmap := []struct {
		name           string
		skip           bool
		input          string
		part1Result    string
		part2Result    string
		maxConnections int
	}{
		{
			name: "example",
			skip: false,
			input: `162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689`,
			part1Result:    "40",
			part2Result:    "25272",
			maxConnections: 10,
		},
	}

	for _, input := range tmap {
		if input.skip {
			continue
		}
		t.Run(input.name, func(t *testing.T) {
			t.Parallel()

			day := NewDay8(input.input)
			if d, ok := day.(*day8); ok {
				d.MaxConnections = input.maxConnections
			}

			result, err := day.Part1()

			require.NoError(t, err)
			require.Equal(t, input.part1Result, result)

			result, err = day.Part2()

			require.NoError(t, err)
			require.Equal(t, input.part2Result, result)
		})
	}
}
