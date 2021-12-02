package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	utils "github.com/lev7/adventofcode2021"
)

type submarine struct {
	hpos, depth, aim int
}

type instruction struct {
	cmd string
	val int
}

func parseInput(input string) (output []instruction) {
	for _, s := range strings.Split(input, "\n") {
		cmdval := strings.Split(s, " ")
		output = append(output, instruction{cmdval[0], utils.Atoi(cmdval[1])})
	}
	return
}

func p1(instructions []instruction) int {
	sub := submarine{}
	for _, inst := range instructions {
		switch inst.cmd {
		case "forward":
			sub.hpos += inst.val
		case "down":
			sub.depth += inst.val
		case "up":
			sub.depth -= inst.val
		}
	}
	return sub.depth * sub.hpos
}

func p2(instructions []instruction) int {
	sub := submarine{}
	for _, inst := range instructions {
		switch inst.cmd {
		case "forward":
			sub.hpos += inst.val
			sub.depth += sub.aim * inst.val
		case "down":
			sub.aim += inst.val
		case "up":
			sub.aim -= inst.val
		}
	}
	return sub.depth * sub.hpos
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	instructions := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(instructions))
	fmt.Printf("Part 2: %v\n", p2(instructions))
}
