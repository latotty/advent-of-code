package main

import (
	"cmp"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"testing"
)

func AbsInt(num int) int {
	if num < 0 {
		return num * -1
	}

	return num
}

func TReadFileStr(t *testing.T, filename string) string {
	t.Helper()
	content, err := os.ReadFile(filename)
	if err != nil {
		t.Error(err)
	}

	return string(content)
}

func MaxCmp[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}

	return b
}

func MinCmp[T cmp.Ordered](a, b T) T {
	if a < b {
		return a
	}

	return b
}

func FindAllIdx(s, substr string) []int {
	res := make([]int, 0)
	for i := 0; i < len(s); {
		idx := strings.Index(s[i:], substr)
		if idx == -1 {
			return res
		}

		res = append(res, i+idx)
		i += idx + 1
	}

	return res
}

func PowInt(x, y int) int {
	return int(math.Pow(float64(x), float64(y)))
}

type Pos3 struct {
	X, Y, Z int
}

func NewPos3FromStrSlice(strCoords []string) (*Pos3, error) {
	if len(strCoords) != 3 {
		return nil, fmt.Errorf("invalid coord length: %d", len(strCoords))
	}

	intCoords := make([]int, 0, 3)
	for _, str := range strCoords {
		num, err := strconv.Atoi(str)
		if err != nil {
			return nil, fmt.Errorf("invalid number(%s): %w", str, err)
		}

		intCoords = append(intCoords, num)
	}

	return &Pos3{intCoords[0], intCoords[1], intCoords[2]}, nil
}

func StringPos3Slice(s []*Pos3) string {
	b := strings.Builder{}
	b.WriteString("[")
	for i, p := range s {
		b.WriteString(p.GoString())

		if i < len(s)-1 {
			b.WriteString(",")
		}
	}
	b.WriteString("]")
	return b.String()
}

func (p1 *Pos3) SqrDist(p2 *Pos3) int {
	return PowInt(p2.X-p1.X, 2) + PowInt(p2.Y-p1.Y, 2) + PowInt(p2.Z-p1.Z, 2)
}

func (p *Pos3) GoString() string {
	return fmt.Sprintf("Pos3{%d,%d,%d}", p.X, p.Y, p.Z)
}

type Pos2 struct {
	X, Y int
}

func NewPos2FromStrSlice(strCoords []string) (*Pos2, error) {
	if len(strCoords) != 2 {
		return nil, fmt.Errorf("invalid coord length: %d", len(strCoords))
	}

	intCoords := make([]int, 0, 2)
	for _, str := range strCoords {
		num, err := strconv.Atoi(str)
		if err != nil {
			return nil, fmt.Errorf("invalid number(%s): %w", str, err)
		}

		intCoords = append(intCoords, num)
	}

	return &Pos2{intCoords[0], intCoords[1]}, nil
}

func (p1 *Pos2) Area(p2 *Pos2) int {
	return (AbsInt(p2.X-p1.X) + 1) * (AbsInt(p2.Y-p1.Y) + 1)
}

func (p *Pos2) GoString() string {
	return fmt.Sprintf("Pos{%d,%d}", p.X, p.Y)
}
