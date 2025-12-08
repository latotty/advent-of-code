package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

type day8 struct {
	input string

	result1 string
	result2 string

	MaxConnections int
}

func NewDay8(input string) Day {
	return &day8{input: input, MaxConnections: 1000}
}

func (d *day8) parseInput() ([]*Pos3, error) {
	lines := strings.Split(strings.TrimSpace(d.input), "\n")
	boxCoords := make([]*Pos3, len(lines))
	for i, line := range lines {
		line = strings.TrimSpace(line)

		coords := strings.Split(line, ",")
		pos, err := NewPos3FromStrSlice(coords)
		if err != nil {
			return nil, fmt.Errorf("invalid line %d: %w", i, err)
		}

		boxCoords[i] = pos
	}

	return boxCoords, nil
}

func (d *day8) calculateDistanceMap(boxCoords []*Pos3) map[int][][]*Pos3 {
	distMap := make(map[int][][]*Pos3, 0)
	for i1, p1 := range boxCoords {
		for _, p2 := range boxCoords[i1+1:] {
			sqrDist := p1.SqrDist(p2)

			arr, ok := distMap[sqrDist]
			if !ok {
				arr = make([][]*Pos3, 0)
			}

			arr = append(arr, []*Pos3{p1, p2})
			distMap[sqrDist] = arr
		}
	}

	return distMap
}

func (d *day8) process() error {
	if d.result1 != "" {
		return nil
	}

	boxCoords, err := d.parseInput()
	if err != nil {
		return fmt.Errorf("parse error: %w", err)
	}

	distMap := d.calculateDistanceMap(boxCoords)

	// fmt.Println(distMap)

	circuits := make([]map[*Pos3]bool, 0)

	dists := slices.Sorted(maps.Keys(distMap))
	for i, connects := 0, 0; i < len(dists); i++ {
		pairs := distMap[dists[i]]

		for _, pair := range pairs {
			matchedCircuits := make([]int, 0, 2)

			connects++

			// fmt.Println("---")
			// fmt.Println(StringPos3Slice(pair))

			for _, pos := range pair {
				for i, circuit := range circuits {
					if circuit[pos] {
						matchedCircuits = append(matchedCircuits, i)
						break
					}
				}
			}

			switch len(matchedCircuits) {
			case 0: // new circuit
				// fmt.Println("created new circuit: ", len(circuits))
				circuit := make(map[*Pos3]bool, 2)
				circuit[pair[0]] = true
				circuit[pair[1]] = true
				circuits = append(circuits, circuit)
			case 1: // join one to the other
				// fmt.Println("joined box to circuit: ", matchedCircuits[0])
				circuits[matchedCircuits[0]][pair[0]] = true
				circuits[matchedCircuits[0]][pair[1]] = true
			case 2: // merge two circuits
				if matchedCircuits[0] != matchedCircuits[1] {
					smallI := MinCmp(matchedCircuits[0], matchedCircuits[1])
					bigI := MaxCmp(matchedCircuits[0], matchedCircuits[1])
					// fmt.Println("merge two circuits: ", smallI, bigI)

					circuits[smallI][pair[0]] = true
					circuits[smallI][pair[1]] = true
					maps.Copy(circuits[smallI], circuits[bigI])

					circuits = slices.Delete(circuits, bigI, bigI+1)
				}
			}

			fullyConnected := len(circuits) == 1 && len(circuits[0]) == len(boxCoords)

			if connects == d.MaxConnections || (d.result1 == "" && fullyConnected) {
				slices.SortFunc(circuits, func(g1, g2 map[*Pos3]bool) int {
					return len(g2) - len(g1)
				})

				result1 := 1

				// fmt.Println(circuits)
				for i := 0; i < 3 && i < len(circuits); i++ {
					// fmt.Println(i, ":", len(circuits[i]))
					result1 *= len(circuits[i])
				}

				d.result1 = fmt.Sprintf("%d", result1)
			}

			if fullyConnected {
				result2 := pair[0].X * pair[1].X
				d.result2 = fmt.Sprintf("%d", result2)

				return nil
			}

		}
	}

	return nil
}

func (d *day8) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day8) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
