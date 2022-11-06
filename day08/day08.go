package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"

	"github.com/leonobilis/adventofcode2021/utils"
)

var uniqueSegNum = []int{2, 3, 4, 7}

type entry struct {
	patterns, output []string
}

func parseInput(input string) (output []entry) {
	for _, s := range strings.Split(input, "\n") {
		parts := strings.Split(s, " | ")
		output = append(output, entry{
			patterns: strings.Split(parts[0], " "),
			output:   strings.Split(parts[1], " "),
		})
	}
	return
}

func isUnique(s string) bool {
	l := len(s)
	for _, i := range uniqueSegNum {
		if l == i {
			return true
		}
	}
	return false
}

func p1(entries []entry) int {
	count := 0
	for _, e := range entries {
		for _, o := range e.output {
			if isUnique(o) {
				count++
			}
		}
	}
	return count
}

func match(s1, s2 string) bool {
	for _, seg := range s2 {
		if !strings.Contains(s1, string(seg)) {
			return false
		}
	}
	return true
}

func determine3(digits235 []string, digit1 string) string {
	for _, digit := range digits235 {
		if match(digit, digit1) {
			return digit
		}
	}
	panic("determine3")
}

func determine6(digits069 []string, digit1 string) string {
	for _, digit := range digits069 {
		if !match(digit, digit1) {
			return digit
		}
	}
	panic("determine6")
}

func determine9(digits069 []string, digit3 string) string {
	for _, digit := range digits069 {
		if match(digit, digit3) {
			return digit
		}
	}
	panic("determine9")
}

func determine0(digits069 []string, digit6, digit9 string) string {
	for _, digit := range digits069 {
		if digit != digit6 && digit != digit9 {
			return digit
		}
	}
	panic("determine0")
}

func determine2(digits235 []string, digit9 string) string {
	for _, digit := range digits235 {
		if !match(digit9, digit) {
			return digit
		}
	}
	panic("determine2")
}

func determine5(digits235 []string, digit2, digit3 string) string {
	for _, digit := range digits235 {
		if digit != digit2 && digit != digit3 {
			return digit
		}
	}
	panic("determine5")
}

func p2(entries []entry) (sum int) {
	for _, e := range entries {
		var digits [10]string
		digits235 := make([]string, 0, 3)
		digits069 := make([]string, 0, 3)
		for _, p := range e.patterns {
			switch len(p) {
			case 2:
				digits[1] = utils.SortString(p)
			case 3:
				digits[7] = utils.SortString(p)
			case 4:
				digits[4] = utils.SortString(p)
			case 7:
				digits[8] = utils.SortString(p)
			case 5:
				digits235 = append(digits235, utils.SortString(p))
			case 6:
				digits069 = append(digits069, utils.SortString(p))
			}
		}
		digits[3] = determine3(digits235, digits[1])
		digits[6] = determine6(digits069, digits[1])
		digits[9] = determine9(digits069, digits[3])
		digits[0] = determine0(digits069, digits[6], digits[9])
		digits[2] = determine2(digits235, digits[9])
		digits[5] = determine5(digits235, digits[2], digits[3])

		output := ""
		for _, o := range e.output {
			sortedO := utils.SortString(o)
			for i, digit := range digits {
				if sortedO == digit {
					output += strconv.Itoa(i)
					break
				}
			}
		}
		sum += utils.Atoi(output)
	}
	return
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	entries := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(entries))
	fmt.Printf("Part 2: %v\n", p2(entries))
}
