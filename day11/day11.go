package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	utils "github.com/lev7/adventofcode2021"
)

func parseInput(input string) (output [][]int) {
	for _, y := range strings.Split(input, "\n") {
		row := make([]int, 0)
		for _, x := range strings.Split(y, "") {
			row = append(row, utils.Atoi(x))
		}
		output = append(output, row)
	}
	return
}

func increaseAdjacent(x, y int, grid [][]int) {
	for yy := y - 1; yy <= y+1; yy++ {
		for xx := x - 1; xx <= x+1; xx++ {
			if (xx != x || yy != y) && yy >= 0 && yy < len(grid) && xx >= 0 && xx < len(grid[yy]) {
				grid[yy][xx]++
				if grid[yy][xx] == 10 {
					increaseAdjacent(xx, yy, grid)
				}
			}
		}
	}
}

func processStep(grid [][]int) (flashes int) {
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[y]); x++ {
			grid[y][x]++
			if grid[y][x] == 10 {
				increaseAdjacent(x, y, grid)
			}
		}
	}
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[y]); x++ {
			if grid[y][x] > 9 {
				flashes++
				grid[y][x] = 0
			}
		}
	}
	return
}

func p1(grid [][]int) int {
	flashes := 0
	for step := 0; step < 100; step++ {
		flashes += processStep(grid)
	}
	return flashes
}

func p2(grid [][]int) int {
	for step := 100; ; step++ {
		if processStep(grid) == len(grid)*len(grid) {
			return step + 1
		}
	}
	return 0
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	grid := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(grid))
	fmt.Printf("Part 2: %v\n", p2(grid))
}
