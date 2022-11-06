package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/leonobilis/adventofcode2021/utils"
)

const maxTimer = 9

func parseInput(input string) (output []int) {
	for _, s := range strings.Split(input, ",") {
		output = append(output, utils.Atoi(s))
	}
	return
}

func simulation(lanternfishes []int, days int) int {
	lCount := make([]int, maxTimer)
	for _, l := range lanternfishes {
		lCount[l]++
	}
	for i := 0; i < days; i++ {
		newLanternfishes := lCount[0]
		for j := 1; j < maxTimer; j++ {
			lCount[j-1] = lCount[j]
		}
		lCount[8] = newLanternfishes
		lCount[6] += newLanternfishes
	}
	sum := 0
	for _, l := range lCount {
		sum += l
	}
	return sum
}

func p1(lanternfishes []int) int {
	return simulation(lanternfishes, 80)
}

func p2(lanternfishes []int) int {
	return simulation(lanternfishes, 256)
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	entries := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(entries))
	fmt.Printf("Part 2: %v\n", p2(entries))
}
