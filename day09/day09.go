package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"

	"github.com/leonobilis/adventofcode2021/utils"
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

func isLowPoint(x, y int, heightmap [][]int) bool {
	if y > 0 && heightmap[y][x] >= heightmap[y-1][x] {
		return false
	}
	if y < len(heightmap)-1 && heightmap[y][x] >= heightmap[y+1][x] {
		return false
	}
	if x > 0 && heightmap[y][x] >= heightmap[y][x-1] {
		return false
	}
	if x < len(heightmap[y])-1 && heightmap[y][x] >= heightmap[y][x+1] {
		return false
	}
	return true
}

func p1(heightmap [][]int) int {
	risk := 0
	for y := 0; y < len(heightmap); y++ {
		for x := 0; x < len(heightmap[y]); x++ {
			if isLowPoint(x, y, heightmap) {
				risk += heightmap[y][x] + 1
			}
		}
	}
	return risk
}

type coord struct {
	x, y int
}

func basinSize(x, y int, heightmap [][]int, seen map[coord]struct{}) int {
	_, s := seen[coord{x, y}]
	if s {
		return 0
	}
	seen[coord{x, y}] = struct{}{}
	if y < 0 || y >= len(heightmap) || x < 0 || x >= len(heightmap[y]) || heightmap[y][x] == 9 {
		return 0
	}

	return 1 + basinSize(x-1, y, heightmap, seen) + basinSize(x+1, y, heightmap, seen) +
		basinSize(x, y-1, heightmap, seen) + basinSize(x, y+1, heightmap, seen)
}

func p2(heightmap [][]int) int {
	basins := make([]int, 0)
	seen := make(map[coord]struct{})
	for y := 0; y < len(heightmap); y++ {
		for x := 0; x < len(heightmap[y]); x++ {
			bs := basinSize(x, y, heightmap, seen)
			if bs > 0 {
				basins = append(basins, bs)
			}
		}
	}
	sort.Ints(basins)
	lb := len(basins)
	return basins[lb-1] * basins[lb-2] * basins[lb-3]
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	heightmap := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(heightmap))
	fmt.Printf("Part 2: %v\n", p2(heightmap))
}
