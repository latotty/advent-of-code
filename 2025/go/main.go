package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

var days = map[string]func(io.Reader) Day{
	"day1": NewDay1,
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

	// Look up day constructor
	constructor, ok := days[dayName]
	if !ok {
		fmt.Fprintf(os.Stderr, "Unknown day: %s\n", dayName)
		os.Exit(1)
	}

	// Create day instance
	day := constructor(inputReader)

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
