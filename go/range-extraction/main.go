package main

import (
	"fmt"
	"strconv"
	"strings"
)

func Solution(list []int) string {
	var (
		ret []string
		l   int
	)
	if len(list) == 1 {
		return strconv.Itoa(list[0])
	}

	for i, v := range list {
		if i == 0 {
			continue
		}
		if v != list[i-1]+1 || i == len(list)-1 {
			switch {
			case i == len(list)-1:
				if v != list[i-1]+1 && i-l != 1 {
					if i-l == 2 {
						ret = append(ret, strconv.Itoa(list[l]), strconv.Itoa(list[i-1]), strconv.Itoa(v))
					} else {
						ret = append(ret, strconv.Itoa(list[l])+"-"+strconv.Itoa(list[i-1]), strconv.Itoa(v))
					}
				} else {
					if i-l == 1 {
						ret = append(ret, strconv.Itoa(list[i-1]), strconv.Itoa(v))
					} else {
						ret = append(ret, strconv.Itoa(list[l])+"-"+strconv.Itoa(v))
					}
				}
			case i-l == 1:
				ret = append(ret, strconv.Itoa(list[l]))
				break
			case i-l == 2:
				ret = append(ret, strconv.Itoa(list[i-2]), strconv.Itoa(list[i-1]))
				break
			default:
				ret = append(ret, strconv.Itoa(list[l])+"-"+strconv.Itoa(list[i-1]))
			}
			l = i
		}
	}
	return strings.Join(ret, ",")
}

func main() {
	fmt.Println(Solution([]int{138, 144, 145, 148}))
	fmt.Println(Solution([]int{-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20}))
	fmt.Println(Solution([]int{40, 44, 48, 51, 52, 54, 55, 58, 67, 73}))
	fmt.Println(Solution([]int{17, 18, 19, 20, 22, 23}))
	fmt.Println(Solution([]int{-7, -6, -5, -4, -2}))
	fmt.Println(Solution([]int{-7, -6, -5}))
	fmt.Println(Solution([]int{-7, -6}))
	fmt.Println(Solution([]int{-7}))
}
