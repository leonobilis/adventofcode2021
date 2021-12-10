package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

var closeChar = map[uint8]uint8{
	"("[0]: ")"[0],
	"["[0]: "]"[0],
	"{"[0]: "}"[0],
	"<"[0]: ">"[0],
}

func validateChain(line string, index int) (int, uint8, []uint8) {
	if index == len(line) {
		return index, 0, []uint8{}
	}
	character := line[index]
	switch character {
	case "("[0], "["[0], "{"[0], "<"[0]:
		i, end, error := validateChain(line, index+1)
		if i == len(line) {
			error = append(error, closeChar[character])
			return i, 0, error
		}
		if len(error) > 0 {
			return 0, 0, error
		}
		if end == closeChar[character] {
			return validateChain(line, i+1)
		}
		return 0, 0, []uint8{end}
	}
	return index, character, []uint8{}
}

func errorScore(line string) int {
	if _, _, error := validateChain(line, 0); len(error) == 1 {
		switch error[0] {
		case ")"[0]:
			return 3
		case "]"[0]:
			return 57
		case "}"[0]:
			return 1197
		case ">"[0]:
			return 25137
		}

	}
	return 0
}

func p1(subsystem []string) int {
	score := 0
	for _, line := range subsystem {
		score += errorScore(line)
	}
	return score
}

var pointMap = map[uint8]int{
	")"[0]: 1,
	"]"[0]: 2,
	"}"[0]: 3,
	">"[0]: 4,
}

func calcScore(missing []uint8) (score int) {
	for _, m := range missing {
		score *= 5
		score += pointMap[m]
	}
	return
}

func p2(subsystem []string) int {
	scores := make([]int, 0)
	for _, line := range subsystem {
		if i, _, error := validateChain(line, 0); len(error) != 0 && i == len(line) {
			scores = append(scores, calcScore(error))
		}
	}
	sort.Ints(scores)
	return scores[len(scores)/2]
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	lines := strings.Split(string(input), "\n")
	fmt.Printf("Part 1: %v\n", p1(lines))
	fmt.Printf("Part 2: %v\n", p2(lines))
}
