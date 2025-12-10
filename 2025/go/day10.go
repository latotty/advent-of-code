package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
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
	nextJoltages := make([][]int, 0)
	stepMap := make(map[string]int, 0)

	defaultJoltage := make([]int, len(m.lightsTarget))
	stepMap[fmt.Sprintf("%v", defaultJoltage)] = 0
	nextJoltages = append(nextJoltages, defaultJoltage)

	for stepIdx := 1; true; stepIdx++ {
		newJoltages := make([][]int, 0)
		for _, joltage := range nextJoltages {
			matchingSwitches := Filter(m.switches, func(switches []int) bool {
				var hasGood bool
				for _, s := range switches {
					if joltage[s] < m.joltageTarget[s] {
						hasGood = true
					} else {
						return false
					}
				}

				return hasGood
			})

			// fmt.Printf("---\n%v <- %v\n", joltage, matchingSwitches)

			for _, ms := range matchingSwitches {
				tempJoltage := make([]int, len(joltage))
				copy(tempJoltage, joltage)

				for _, s := range ms {
					tempJoltage[s]++
				}

				// fmt.Printf("%v <- %v\n", tempJoltage, ms)

				if slices.Equal(m.joltageTarget, tempJoltage) {
					return stepIdx
				}
				key := fmt.Sprintf("%v", tempJoltage)
				if _, ok := stepMap[key]; ok {
					continue
				}

				stepMap[key] = stepIdx
				newJoltages = append(newJoltages, tempJoltage)
			}
		}
		nextJoltages = newJoltages
		if len(nextJoltages) == 0 {
			panic("damn")
		}
	}

	return 0
}
