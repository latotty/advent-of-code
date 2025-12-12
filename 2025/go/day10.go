package main

import (
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
	"time"

	"github.com/latotty/advent-of-code/2025/go/vecn"
)

type day10 struct {
	input string
}

func NewDay10(input string) Day {
	return &day10{input: input}
}

func (d *day10) Part1() (string, error) {
	var result int

	for _, line := range strings.Split(strings.TrimSpace(d.input), "\n") {
		machine := &day10Machine{}
		machine.parseLine(line)
		result += machine.switchOn()
	}

	return fmt.Sprintf("%d", result), nil
}

func (d *day10) Part2() (string, error) {
	var result int

	for _, line := range strings.Split(strings.TrimSpace(d.input), "\n") {
		machine := &day10Machine{}
		machine.parseLine(line)
		result += machine.joltageUp()
	}

	return fmt.Sprintf("%d", result), nil
}

type day10Machine struct {
	lightsTarget  string
	joltageTarget []int
	switches      [][]int
	switchVecs    [][]int
}

func (m *day10Machine) parseLine(line string) {
	// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
	line = strings.TrimSpace(line)

	m.switches = make([][]int, 0)

	parts := strings.Split(line, " ")
	for _, part := range parts {
		part = strings.TrimSpace(part)

		switch part[0] {
		case '[': // lights
			part = part[1 : len(part)-1]
			m.lightsTarget = part
		case '(':
			part = part[1 : len(part)-1]
			idxs := strings.Split(part, ",")

			switchIdxs := make([]int, len(idxs))
			for i, idxStr := range idxs {
				num, err := strconv.Atoi(idxStr)
				if err != nil {
					panic(err)
				}
				switchIdxs[i] = num
			}
			m.switches = append(m.switches, switchIdxs)

			switchVec := make([]int, len(m.lightsTarget))
			for _, s := range switchIdxs {
				switchVec[s] = 1
			}
			m.switchVecs = append(m.switchVecs, switchVec)

		case '{':
			part = part[1 : len(part)-1]
			idxs := strings.Split(part, ",")
			m.joltageTarget = make([]int, len(idxs))
			for i, idxStr := range idxs {
				num, err := strconv.Atoi(idxStr)
				if err != nil {
					panic(err)
				}
				m.joltageTarget[i] = num
			}

		}
	}
}

func (m *day10Machine) switchOn() int {
	nextLights := make([]string, 0)
	stepMap := make(map[string]int, 0)

	defaultLights := strings.Repeat(".", len(m.lightsTarget))
	stepMap[defaultLights] = 0
	nextLights = append(nextLights, defaultLights)

	for stepIdx := 1; true; stepIdx++ {
		newLights := make([]string, 0)
		for _, lights := range nextLights {
			wrongSpots := make([]int, 0)
			for i, c := range []byte(lights) {
				if m.lightsTarget[i] != c {
					wrongSpots = append(wrongSpots, i)
				}
			}
			if len(wrongSpots) == 0 {
				panic("shouldntbehere")
			}

			matchingSwitches := Filter(m.switches, func(switches []int) bool {
				for _, ws := range wrongSpots {
					if slices.Contains(switches, ws) {
						return true
					}
				}

				return false
			})

			for _, ms := range matchingSwitches {
				tempLightsArr := []byte(strings.Clone(lights))
				for _, s := range ms {
					if tempLightsArr[s] == '.' {
						tempLightsArr[s] = '#'
					} else {
						tempLightsArr[s] = '.'
					}
				}
				tempLights := string(tempLightsArr)

				if m.lightsTarget == tempLights {
					return stepIdx
				}

				if _, ok := stepMap[tempLights]; ok {
					continue
				}

				stepMap[tempLights] = stepIdx
				newLights = append(newLights, tempLights)
			}
		}
		nextLights = newLights
	}

	return 0
}

func (m *day10Machine) joltageUp() int {
	zero := make([]int, len(m.joltageTarget))

	counter := 0
	start := time.Now()

	var search func(remaining []int, switchVecs [][]int) int
	search = func(remaining []int, switchVecs [][]int) int {
		counter++
		if counter%100000 == 0 {
			elapsed := time.Since(start).Seconds()
			fmt.Printf("\rStates: %dM | Time: %.1fs | Rate: %.0fk/s",
				counter/1000000, elapsed, float64(counter)/elapsed/1000)
		}

		if slices.Equal(remaining, zero) {
			return 0
		}

		minAffectingCount := math.MaxInt
		targetPosition := -1
		for i := 0; i < len(remaining); i++ {
			ac := 0
			for _, s := range switchVecs {
				if s[i] > 0 {
					ac++
				}
			}
			if ac > 0 && ac < minAffectingCount || (ac == minAffectingCount && remaining[targetPosition] < remaining[i]) {
				targetPosition = i
				minAffectingCount = ac
			}
		}

		availableSwitches := make([][]int, 0, len(switchVecs))
		nextAvailableSwitches := make([][]int, 0, len(switchVecs))
		for _, s := range switchVecs {
			if s[targetPosition] > 0 {
				availableSwitches = append(availableSwitches, s)
			} else {
				nextAvailableSwitches = append(nextAvailableSwitches, s)
			}
		}

		if len(availableSwitches) == 0 {
			return math.MaxInt
		}

		counts := make([]int, len(availableSwitches))
		counts[len(counts)-1] = remaining[targetPosition]

		nextRemaining := make([]int, len(remaining))

		result := math.MaxInt

		for counts := range CombinationsFromRight(counts) {
			copy(nextRemaining, remaining)
			for i, c := range counts {
				vecn.SubMut(nextRemaining, vecn.Mul(availableSwitches[i], c))
			}

			if vecn.AnyNegative(nextRemaining) {
				continue
			}

			res := search(nextRemaining, nextAvailableSwitches)
			if res != math.MaxInt {
				result = MinCmp(result, res+remaining[targetPosition])
			}
		}

		return result
	}

	result := search(m.joltageTarget, m.switchVecs)
	fmt.Printf("\n✅ Final: %d (explored %dM states in %.1fs)\n", result, counter/1000000, time.Since(start).Seconds())
	return result
}
