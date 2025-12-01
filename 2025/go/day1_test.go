package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestDay1Example(t *testing.T) {
	const example = `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`

	day := NewDay1(strings.NewReader(example))
	result, err := day.Part1()

	require.NoError(t, err)
	require.Equal(t, "3", result)
}
