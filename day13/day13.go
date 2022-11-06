package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/leonobilis/adventofcode2021/utils"
)

type Grid struct {
	m map[coord]struct{}
}

func NewGrid() Grid {
	var grid Grid
	grid.m = make(map[coord]struct{})
	return grid
}

func (self *Grid) AddDot(dot coord) {
	self.m[dot] = struct{}{}
}

func (self *Grid) FoldY(y int) {
	new := NewGrid()
	for dot, _ := range self.m {
		if dot.y > y {
			new.AddDot(coord{dot.x, 2*y - dot.y})
		} else {
			new.AddDot(dot)
		}
	}
	*self = new
}

func (self *Grid) FoldX(x int) {
	new := NewGrid()

	for dot, _ := range self.m {
		if dot.x > x {
			new.AddDot(coord{2*x - dot.x, dot.y})
		} else {
			new.AddDot(dot)
		}
	}
	*self = new
}

func (self Grid) CountDots() int {
	return len(self.m)
}

func (self Grid) Max() (max coord) {
	for c, _ := range self.m {
		if c.x > max.x {
			max = coord{c.x, max.y}
		}
		if c.y > max.y {
			max = coord{max.x, c.y}
		}
	}
	return
}

type coord struct {
	x, y int
}

func parseInput(input string) (grid Grid, folds []coord) {
	grid = NewGrid()
	dotsFolds := strings.Split(input, "\n\n")
	for _, dot := range strings.Split(dotsFolds[0], "\n") {
		coordStr := strings.Split(dot, ",")
		grid.AddDot(coord{utils.Atoi(coordStr[0]), utils.Atoi(coordStr[1])})
	}
	for _, fold := range strings.Split(dotsFolds[1], "\n") {
		foldAlong := strings.Split(fold, "=")
		if foldAlong[0] == "fold along x" {
			folds = append(folds, coord{utils.Atoi(foldAlong[1]), 0})
		} else if foldAlong[0] == "fold along y" {
			folds = append(folds, coord{0, utils.Atoi(foldAlong[1])})
		}
	}
	return
}

func p1(grid Grid, folds []coord) int {
	if folds[0].x == 0 {
		grid.FoldY(folds[0].y)
	} else {
		grid.FoldX(folds[0].x)
	}

	return grid.CountDots()
}

func p2(grid Grid, folds []coord) {
	for _, fold := range folds {
		if fold.x == 0 {
			grid.FoldY(fold.y)
		} else {
			grid.FoldX(fold.x)
		}
	}
	max := grid.Max()
	for y := 0; y <= max.y; y++ {
		for x := 0; x <= max.x; x++ {
			if _, ok := grid.m[coord{x, y}]; ok {
				fmt.Print("#")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	grid, folds := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(grid, folds))
	grid, folds = parseInput(string(input))
	fmt.Printf("Part 2:\n")
	p2(grid, folds)
}
