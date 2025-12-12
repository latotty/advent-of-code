package main

import (
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"

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
	best := math.MaxInt

	counter := 0

	var search func(spotIdx int, current []int, totalPresses int)
	search = func(spotIdx int, current []int, totalPresses int) {
		counter++
		if counter%1000000 == 0 {
			fmt.Printf("current: %v target: %v spotIdx: %d totalPresses: %d best: %d counter: %d\n", current, m.joltageTarget, spotIdx, totalPresses, best, counter)
		}

		if totalPresses >= best {
			return
		}

		if vecn.Equals(m.joltageTarget, current) {
			best = totalPresses
			return
		}

		if spotIdx >= len(m.joltageTarget) {
			return
		}

		spotTarget := m.joltageTarget[spotIdx]
		if current[spotIdx] == spotTarget {
			search(spotIdx+1, current, totalPresses)
			return
		}

		var relevantSwitches [][]int
	switchLoop:
		for _, s := range m.switchVecs {
			for i := 0; i < spotIdx; i++ {
				if s[i] > 0 {
					continue switchLoop
				}
			}
			if s[spotIdx] == 0 {
				continue
			}

			div := vecn.Div(vecn.Sub(m.joltageTarget, current), s)
			if div > 0 {
				relevantSwitches = append(relevantSwitches, s)
			}
		}

		if len(relevantSwitches) == 0 {
			return
		}

		pruneMap := make(map[string]bool)

		var groupSearch func(next []int, idx, totalPresses int)
		groupSearch = func(next []int, idx, totalPresses int) {
			if idx >= len(relevantSwitches) {
				return
			}

			s := relevantSwitches[idx]
			div := vecn.Div(vecn.Sub(m.joltageTarget, next), s)

			if div == 0 {
				groupSearch(next, idx+1, totalPresses)
				return
			}

			for n := div; n >= 0; n-- {
				nextnext := vecn.Add(next, vecn.Mul(s, n))

				if nextnext[spotIdx] == spotTarget {
					if _, ok := pruneMap[fmt.Sprintf("%v", nextnext)]; !ok {
						pruneMap[fmt.Sprintf("%v", nextnext)] = true
						search(spotIdx+1, nextnext, totalPresses+n)
					}
				} else {
					groupSearch(nextnext, idx+1, totalPresses+n)
				}
			}
		}

		groupSearch(current, 0, totalPresses)
	}

	search(0, make([]int, len(m.joltageTarget)), 0)
	return best
}
