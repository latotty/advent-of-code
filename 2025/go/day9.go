package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

type day9 struct {
	input       string
	rawVertices []*Pos2

	vertices                             []*Pos2
	width, height                        int
	xDecompressionMap, yDecompressionMap map[int]int

	edges []*day9Edge

	markedGrid [][]byte
	prefixGrid [][]int

	result1 string
	result2 string
}

func NewDay9(input string) Day {
	return &day9{input: input}
}

func (d *day9) parseInput() error {
	lines := strings.Split(strings.TrimSpace(d.input), "\n")
	d.rawVertices = make([]*Pos2, len(lines))
	for i, line := range lines {
		line := strings.TrimSpace(line)
		strCoords := strings.Split(line, ",")
		pos, err := NewPos2FromStrSlice(strCoords)
		if err != nil {
			return fmt.Errorf("line parse err: %w", err)
		}
		d.rawVertices[i] = pos
	}

	return nil
}

func (d *day9) ensureClockwise() {
	signedArea := 0
	for i, p1 := range d.rawVertices {
		p2 := OverflowIndex(d.rawVertices, i+1)
		signedArea += p1.X * p2.Y
		signedArea -= p2.X * p1.Y
	}
	if signedArea < 0 {
		slices.Reverse(d.rawVertices)
	}
}

func (d *day9) compressVertices() {
	xCoordsMap := make(map[int]int)
	yCoordsMap := make(map[int]int)

	d.xDecompressionMap = make(map[int]int)
	d.yDecompressionMap = make(map[int]int)
	for _, s := range d.rawVertices {
		xCoordsMap[s.X] = -1
		yCoordsMap[s.Y] = -1
	}
	mappedCoord := 0
	sortedXCoords := slices.Sorted(maps.Keys(xCoordsMap))
	for i, x := range sortedXCoords {
		if i > 0 && x-sortedXCoords[i-1] > 1 {
			mappedCoord++
		}
		xCoordsMap[x] = mappedCoord
		d.xDecompressionMap[mappedCoord] = x
		mappedCoord++
	}
	d.width = mappedCoord

	mappedCoord = 0
	sortedYCoords := slices.Sorted(maps.Keys(yCoordsMap))
	for i, y := range sortedYCoords {
		if i > 0 && y-sortedYCoords[i-1] > 1 {
			mappedCoord++
		}
		yCoordsMap[y] = mappedCoord
		d.yDecompressionMap[mappedCoord] = y
		mappedCoord++
	}
	d.height = mappedCoord

	d.vertices = make([]*Pos2, len(d.rawVertices))
	for i, s := range d.rawVertices {
		d.vertices[i] = &Pos2{xCoordsMap[s.X], yCoordsMap[s.Y]}
	}
}

func (d *day9) generateEdges() {
	d.edges = make([]*day9Edge, len(d.vertices))
	for i, p1 := range d.vertices {
		d.edges[i] = newDay9Edge(p1, OverflowIndex(d.vertices, i+1))
	}
}

type day9Edge struct {
	rect2

	horizontal, vertical, lowXInner, lowYInner bool
}

func newDay9Edge(p1, p2 *Pos2) *day9Edge {
	e := &day9Edge{
		rect2: *NewRect2(p1, p2),
	}

	if p1.X == p2.X {
		e.vertical = true
		e.lowXInner = p1.Y < p2.Y
	} else {
		e.horizontal = true
		e.lowYInner = p1.X > p2.X
	}

	return e
}

const (
	DAY9_MARK_WALL  = byte(1)
	DAY9_MARK_INNER = byte(2)
)

func (d *day9) buildMarkedGrid() {
	d.markedGrid = make([][]byte, d.height)
	for y := 0; y < d.height; y++ {
		d.markedGrid[y] = make([]byte, d.width)
	}

	for _, edge := range d.edges {
		for y := edge.Min.Y; y <= edge.Max.Y; y++ {
			for x := edge.Min.X; x <= edge.Max.X; x++ {
				d.markedGrid[y][x] = DAY9_MARK_WALL
			}
		}
	}

	for _, edge := range d.edges {
		if edge.vertical {
			x := edge.Max.X
			if edge.lowXInner {
				x -= 1
			} else {
				x += 1
			}
			for y := edge.Min.Y; y <= edge.Max.Y; y++ {
				d.floodFillMarkedGrid(x, y)
			}
		} else {
			y := edge.Max.Y
			if edge.lowYInner {
				y -= 1
			} else {
				y += 1
			}
			for x := edge.Min.X; x <= edge.Max.X; x++ {
				d.floodFillMarkedGrid(x, y)
			}
		}
	}
}

func (d *day9) floodFillMarkedGrid(x, y int) {
	if y < 0 || y >= d.height || x < 0 || x >= d.width || d.markedGrid[y][x] > 0 {
		return
	}

	d.markedGrid[y][x] = DAY9_MARK_INNER

	for y1 := y - 1; y1 <= y+1; y1++ {
		for x1 := x - 1; x1 <= x+1; x1++ {
			if y1 == y && x1 == x {
				continue
			}

			d.floodFillMarkedGrid(x1, y1)
		}
	}
}

func (d *day9) buildPrefixGrid() {
	d.prefixGrid = make([][]int, d.height)
	for y := 0; y < d.height; y++ {
		d.prefixGrid[y] = make([]int, d.width)
		for x := 0; x < d.width; x++ {
			num := 0
			if d.markedGrid[y][x] > 0 {
				num++
			}
			if x > 0 {
				num += d.prefixGrid[y][x-1] // add left
			}
			if y > 0 {
				num += d.prefixGrid[y-1][x] // add top
			}
			if x > 0 && y > 0 {
				num -= d.prefixGrid[y-1][x-1] // subtract overlap
			}

			d.prefixGrid[y][x] = num
		}
	}
}

func (d *day9) process() error {
	if d.result1 != "" {
		return nil
	}

	if err := d.parseInput(); err != nil {
		return err
	}

	d.ensureClockwise()

	d.compressVertices()

	d.generateEdges()

	d.buildMarkedGrid()

	// PrintGrid(d.markedGrid, 0)

	d.buildPrefixGrid()

	// PrintGrid(d.prefixGrid, 4)

	var result1 int
	var result2 int

	for i, v1 := range d.vertices {
		for _, v2 := range d.vertices[i+1:] {
			rect := NewRect2(v1, v2)

			rawArea := NewRect2(&Pos2{d.xDecompressionMap[v1.X], d.yDecompressionMap[v1.Y]}, &Pos2{d.xDecompressionMap[v2.X], d.yDecompressionMap[v2.Y]}).Area()
			result1 = MaxCmp(result1, rawArea)

			if rawArea < result2 {
				continue
			}

			area := rect.Area()

			sum := d.prefixGrid[rect.Max.Y][rect.Max.X]

			if rect.Min.X > 0 {
				sum -= d.prefixGrid[rect.Max.Y][rect.Min.X-1] // remove left part
			}
			if rect.Min.Y > 0 {
				sum -= d.prefixGrid[rect.Min.Y-1][rect.Max.X] // remove top part
			}
			if rect.Min.X > 0 && rect.Min.Y > 0 {
				sum += d.prefixGrid[rect.Min.Y-1][rect.Min.X-1] // add back corner (removed twice)
			}

			if area != sum {
				continue
			}

			result2 = MaxCmp(result2, rawArea)
		}
	}

	d.result1 = fmt.Sprintf("%d", result1)
	d.result2 = fmt.Sprintf("%d", result2)

	return nil
}

func (d *day9) Part1() (string, error) {
	d.process()

	return d.result1, nil
}

func (d *day9) Part2() (string, error) {
	d.process()

	return d.result2, nil
}
