package main

import (
	"cmp"
	"os"
	"testing"
)

func AbsInt64(num int64) int64 {
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
