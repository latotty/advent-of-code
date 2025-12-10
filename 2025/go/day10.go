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

	return fmt.Sprintf("%d", result), nil
}

type day10Machine struct {
	lightsTarget string
	switches     [][]int
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
