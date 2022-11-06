package main

import (
	"fmt"
	"io/ioutil"

	"github.com/leonobilis/adventofcode2021/utils"
)

func p1(entries []int) int {
	count := 0
	for i, e := range entries {
		if i < len(entries)-1 && e < entries[i+1] {
			count++
		}
	}
	return count
}

func p2(entries []int) int {
	count := 0
	prev := 0
	for i := 1; i < len(entries)-2; i++ {
		val := entries[i] + entries[i+1] + entries[i+2]
		if val > prev {
			count++
		}
		prev = val
	}
	return count
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	entries := utils.GetIntArray(string(input))
	fmt.Printf("Part 1: %v\n", p1(entries))
	fmt.Printf("Part 2: %v\n", p2(entries))
}
