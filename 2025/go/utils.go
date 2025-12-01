package main

func AbsInt64(num int64) int64 {
	if num < 0 {
		return num * -1
	}

	return num
}
