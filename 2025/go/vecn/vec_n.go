package vecn

import (
	"math"
	"slices"
)

func Equals(v1, v2 []int) bool {
	return slices.Equal(v1, v2)
}

func Add(v1, v2 []int) []int {
	if len(v1) != len(v2) {
		panic("difflen")
	}

	res := make([]int, len(v1))
	copy(res, v1)
	for i, n := range v2 {
		res[i] += n
	}

	return res
}

func Sub(v1, v2 []int) []int {
	if len(v1) != len(v2) {
		panic("difflen")
	}

	res := make([]int, len(v1))
	copy(res, v1)
	for i, n := range v2 {
		res[i] -= n
	}

	return res
}

func Mul(v1 []int, num int) []int {
	res := make([]int, len(v1))
	copy(res, v1)

	for i := 0; i < len(v1); i++ {
		res[i] *= num
	}

	return res
}

func Div(v1, v2 []int) int {
	if len(v1) != len(v2) {
		panic("difflen")
	}

	resDiv := math.MaxInt

	for i := 0; i < len(v1); i++ {
		if v2[i] == 0 {
			continue
		}
		div := v1[i] / v2[i]

		if resDiv > div {
			resDiv = div
		}
	}
	if resDiv == math.MaxInt {
		return 0
	}

	return resDiv
}

func AllLte(v1, v2 []int) bool {
	if len(v1) != len(v2) {
		panic("difflen")
	}

	for i := 0; i < len(v1); i++ {
		if v1[i] > v2[i] {
			return false
		}
	}

	return true
}
