package main

import (
	"fmt"
	"math"
)

func getUnused(max int, used, bl []int) int {
	if max == len(used) {
		return 0
	}

takoe:
	for i := 1; i <= max; i++ {
		for _, s := range used {
			if s == i {
				continue takoe
			}
		}
		for _, s := range bl {
			if s == i {
				continue takoe
			}
		}
		return i
	}

	return 0
}

func verify(shit []int) bool {
	if len(shit) == 1 {
		return true
	}

	prev := 0
	for i, s := range shit {
		if i > 0 {
			sq := math.Sqrt(float64(prev + s))
			if fmt.Sprintf("%.1f", sq) != fmt.Sprintf("%d.0", int(sq)) {
				return false
			}
		}

		prev = s
	}

	return true
}

func SquareSumsRow(n int) []int {
	var used, bl []int
	iters := 0
	for iters < 1000 {
		for i := 1; i <= n; i++ {
			can := getUnused(n, used, bl)
			if !verify(used) {
				bl = append(bl, can)
				continue
			}
			used = append(used, can)
		}
		if verify(used) {
			fmt.Println("Iters", iters)
			return used
		}

		used = nil
		bl = nil
		iters++
	}
	return nil
}

func main() {
	fmt.Println(15, SquareSumsRow(15))
}
