package vecn

import (
	"math"
)

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

func SubMut(v1, v2 []int) {
	if len(v1) != len(v2) {
		panic("difflen")
	}

	for i, n := range v2 {
		v1[i] -= n
	}
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

func Sum(v1 []int) int {
	sum := 0
	for _, n := range v1 {
		sum += n
	}
	return sum
}

func AnyNegative(v1 []int) bool {
	for _, n := range v1 {
		if n < 0 {
			return true
		}
	}

	return false
}
