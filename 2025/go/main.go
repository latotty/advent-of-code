package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

var days = map[string]func(string) Day{
	"day1": NewDay1,
	"day2": NewDay2,
	"day3": NewDay3,
	"day4": NewDay4,
	"day5": NewDay5,
	"day6": NewDay6,
	"day7": NewDay7,
	"day8": NewDay8,
	"day9": NewDay9,
}

func main() {
	if len(os.Args) < 3 {
		fmt.Fprintln(os.Stderr, "Usage: go run . <day[/part]> <input-file|->\n  Examples:\n    go run . day1 input.txt\n    go run . day1/1 input.txt\n    cat input.txt | go run . day1 -")
		os.Exit(1)
	}

	dayArg := os.Args[1]
	inputSource := os.Args[2]

	// Parse day and optional part
	dayName := dayArg
	var part string
	if idx := strings.Index(dayArg, "/"); idx != -1 {
		dayName = dayArg[:idx]
		part = dayArg[idx+1:]
	}

	switch part {
	case "", "1", "2":
	default:
		fmt.Fprintf(os.Stderr, "Invalid part: %s (must be 1 or 2)\n", part)
		os.Exit(1)
	}

	// Get input reader
	var inputReader io.Reader
	if inputSource == "-" {
		inputReader = os.Stdin
	} else {
		file, err := os.Open(inputSource)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error opening file: %v\n", err)
			os.Exit(1)
		}
		defer file.Close()
		inputReader = file
	}

	var input string
	if b, err := io.ReadAll(inputReader); err != nil {
		fmt.Fprintf(os.Stderr, "Failed to read input: %v\n", err)
		os.Exit(1)
	} else {
		input = string(b)
	}

	// Look up day constructor
	constructor, ok := days[dayName]
	if !ok {
		fmt.Fprintf(os.Stderr, "Unknown day: %s\n", dayName)
		os.Exit(1)
	}

	// Create day instance
	day := constructor(input)

	// Run requested parts
	hasError := false

	if part == "" || part == "1" {
		result, err := day.Part1()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			hasError = true
		} else {
			fmt.Println(result)
		}
	}

	if part == "" || part == "2" {
		result, err := day.Part2()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			hasError = true
		} else {
			fmt.Println(result)
		}
	}

	if hasError {
		os.Exit(1)
	}
}
