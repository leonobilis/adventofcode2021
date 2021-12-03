package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

const NumLen = 12

func parseInput(input string) (output []int) {
	for _, s := range strings.Split(input, "\n") {
		i, _ := strconv.ParseInt(s, 2, 64)
		output = append(output, int(i))
	}
	return
}

func bitAt(i, pos int) int {
	return i & (1 << pos) >> pos
}

func p1(report []int) int {
	var gamma, epsilon int
	for i := 0; i < NumLen; i++ {
		var count0, count1 int
		for _, r := range report {
			if bitAt(r, i) == 1 {
				count1++
			} else {
				count0++
			}
		}
		if count1 > count0 {
			gamma |= (1 << i)
		} else {
			epsilon |= (1 << i)
		}
	}
	return gamma * epsilon
}

func getRating(report []int, bitCriteria func(int, int) int) int {
	for i := NumLen - 1; i >= 0 && len(report) > 1; i-- {
		var count0, count1 int
		newReport := make([]int, 0)
		for _, r := range report {
			if bitAt(r, i) == 1 {
				count1++
			} else {
				count0++
			}
		}
		bit := bitCriteria(count0, count1)
		for _, r := range report {
			if bitAt(r, i) == bit {
				newReport = append(newReport, r)
			}
		}
		report = newReport
	}
	return report[0]
}

func p2(report []int) int {
	oxygen := getRating(report, func(count0, count1 int) int {
		if count1 >= count0 {
			return 1
		} else {
			return 0
		}
	})
	co2 := getRating(report, func(count0, count1 int) int {
		if count0 <= count1 {
			return 0
		} else {
			return 1
		}
	})
	return oxygen * co2
}

func main() {
	inputRead, _ := ioutil.ReadFile("input.txt")
	input := parseInput(string(inputRead))
	fmt.Printf("Part 1: %v\n", p1(input))
	fmt.Printf("Part 2: %v\n", p2(input))
}
