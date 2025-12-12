package main

import (
	"cmp"
	"fmt"
	"iter"
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

func MinMaxCmp[T cmp.Ordered](a, b T) (T, T) {
	if a < b {
		return a, b
	}

	return b, a
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

func OverflowIndex[T any](a []T, i int) T {
	if i >= len(a) {
		i -= len(a)
	}
	return a[i]
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

func (p *Pos2) String() string {
	return fmt.Sprintf("Pos2{%d,%d}", p.X, p.Y)
}

func (p1 *Pos2) Equals(p2 *Pos2) bool {
	return p1.X == p2.X && p1.Y == p2.Y
}

func GetSquareEdges(p1, p2 *Pos2) [][]*Pos2 {
	minX, maxX := MinMaxCmp(p1.X, p2.X)
	minY, maxY := MinMaxCmp(p1.Y, p2.Y)

	corners := []*Pos2{{minX, minY}, {maxX, minY}, {maxX, maxY}, {minX, maxY}}

	return [][]*Pos2{
		{corners[0], corners[1]},
		{corners[1], corners[2]},
		{corners[2], corners[3]},
		{corners[3], corners[0]},
	}
}

func Filter[T any](ss []T, test func(T) bool) (ret []T) {
	for _, s := range ss {
		if test(s) {
			ret = append(ret, s)
		}
	}
	return
}

func PrintGrid[T byte | int](grid [][]T, padding int) {
	format := fmt.Sprintf("%%%dd", padding)
	fmt.Println("---")
	for _, line := range grid {
		for _, c := range line {
			fmt.Printf(format, c)
		}
		fmt.Printf("\n")
	}
	fmt.Println("---")
}

func Combinations[T any](arr []T) iter.Seq[[]T] {
	return func(yield func([]T) bool) {
		n := len(arr)

		for i := 1; i < (1 << n); i++ {
			combo := make([]T, 0, n)
			for j := 0; j < n; j++ {
				if i&(1<<j) != 0 {
					combo = append(combo, arr[j])
				}
			}

			if !yield(combo) {
				return
			}
		}
	}
}
