package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/leonobilis/adventofcode2021/utils"
	"github.com/yourbasic/graph"
)

func parseInput(input string) (output [][]int) {
	for _, line := range strings.Split(input, "\n") {
		var inner []int
		for _, c := range strings.Split(line, "") {
			inner = append(inner, utils.Atoi(c))
		}
		output = append(output, inner)
	}
	return
}

func getGraph(grid [][]int) *graph.Mutable {
	rows, cols := len(grid), len(grid[0])
	g := graph.New(rows * cols)
	for y, line := range grid {
		for x, val := range line {
			if x > 0 {
				g.AddCost((x-1)*cols+y, x*cols+y, int64(val))
			}
			if x < rows-1 {
				g.AddCost((x+1)*cols+y, x*cols+y, int64(val))
			}
			if y > 0 {
				g.AddCost(x*cols+y-1, x*cols+y, int64(val))
			}
			if y < cols-1 {
				g.AddCost(x*cols+y+1, x*cols+y, int64(val))
			}
		}
	}
	return g
}

func p1(grid [][]int) int64 {
	g := getGraph(grid)
	_, dist := graph.ShortestPath(g, 0, len(grid)*len(grid[0])-1)
	return dist
}

func p2(grid [][]int) int64 {
	var grid5 [][]int
	rows, cols := len(grid), len(grid[0])
	for x := 0; x < rows*5; x++ {
		var inner []int
		for y := 0; y < cols*5; y++ {
			val := grid[y%rows][x%cols] + x/cols + y/cols
			if val > 9 {
				val %= 9
			}
			inner = append(inner, val)
		}
		grid5 = append(grid5, inner)
	}
	g := getGraph(grid5)
	_, dist := graph.ShortestPath(g, 0, rows*5*cols*5-1)
	return dist
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	grid := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(grid))
	fmt.Printf("Part 2: %v\n", p2(grid))
}
