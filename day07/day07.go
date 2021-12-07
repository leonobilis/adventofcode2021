package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"

	utils "github.com/lev7/adventofcode2021"
)

func parseInput(input string) (output []int) {
	for _, s := range strings.Split(input, ",") {
		output = append(output, utils.Atoi(s))
	}
	sort.Ints(output)
	return
}

func p1(positions []int) int {
	median := positions[len(positions)/2]

	var fuel int
	for _, p := range positions {
		fuel += utils.Abs(p - median)
	}
	return fuel
}

func calcFuel(pos int, positions []int) (sum int) {
	for _, p := range positions {
		n := utils.Abs(p - pos)
		sum += n * (n + 1) / 2
	}
	return
}

func p2(positions []int) int {
	minPos, maxPos := positions[0], positions[len(positions)-1]
	minFuel := int(^uint(0) >> 1)
	for i := minPos; i <= maxPos; i++ {
		fuel := calcFuel(i, positions)
		if fuel < minFuel {
			minFuel = fuel
		}
	}
	return minFuel
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	entries := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(entries))
	fmt.Printf("Part 2: %v\n", p2(entries))
}
