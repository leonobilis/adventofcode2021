package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/leonobilis/adventofcode2021/utils"
)

type coord struct {
	x, y int
}

type line struct {
	a, b coord
}

func parseInput(input string) (output []line) {
	for _, s := range strings.Split(input, "\n") {
		points := strings.Split(s, " -> ")
		pointA := strings.Split(points[0], ",")
		pointB := strings.Split(points[1], ",")
		output = append(output, line{
			a: coord{x: utils.Atoi(pointA[0]), y: utils.Atoi(pointA[1])},
			b: coord{x: utils.Atoi(pointB[0]), y: utils.Atoi(pointB[1])},
		})
	}
	return
}

func p1(lines []line) int {
	diagram := make(map[coord]int)
	for _, line := range lines {
		if line.a.x == line.b.x {
			y1, y2 := line.a.y, line.b.y
			if line.b.y < line.a.y {
				y1, y2 = y2, y1
			}
			for ; y1 <= y2; y1++ {
				diagram[coord{line.a.x, y1}]++
			}
		} else if line.a.y == line.b.y {
			x1, x2 := line.a.x, line.b.x
			if line.b.x < line.a.x {
				x1, x2 = x2, x1
			}
			for ; x1 <= x2; x1++ {
				diagram[coord{x1, line.a.y}]++
			}
		}
	}
	count := 0
	for _, d := range diagram {
		if d >= 2 {
			count++
		}
	}
	return count
}

func increment(n int) int {
	return n + 1
}

func decrement(n int) int {
	return n - 1
}

func lessOrEqual(n1, n2 int) bool {
	return n1 <= n2
}

func greaterOrEqual(n1, n2 int) bool {
	return n1 >= n2
}

func p2(lines []line) int {
	diagram := make(map[coord]int)
	for _, line := range lines {
		if line.a.x == line.b.x {
			y1, y2 := line.a.y, line.b.y
			if line.b.y < line.a.y {
				y1, y2 = y2, y1
			}
			for ; y1 <= y2; y1++ {
				diagram[coord{line.a.x, y1}]++
			}
		} else if line.a.y == line.b.y {
			x1, x2 := line.a.x, line.b.x
			if line.b.x < line.a.x {
				x1, x2 = x2, x1
			}
			for ; x1 <= x2; x1++ {
				diagram[coord{x1, line.a.y}]++
			}
		} else {
			var xCond, yCond func(n1, n2 int) bool
			var xOper, yOper func(n int) int
			if line.a.x < line.b.x {
				xCond = lessOrEqual
				xOper = increment
			} else {
				xCond = greaterOrEqual
				xOper = decrement
			}
			if line.a.y < line.b.y {
				yCond = lessOrEqual
				yOper = increment
			} else {
				yCond = greaterOrEqual
				yOper = decrement
			}
			x1, y1, x2, y2 := line.a.x, line.a.y, line.b.x, line.b.y
			for ; xCond(x1, x2) && yCond(y1, y2); x1, y1 = xOper(x1), yOper(y1) {
				diagram[coord{x1, y1}]++
			}
		}
	}
	count := 0
	for _, d := range diagram {
		if d >= 2 {
			count++
		}
	}
	return count
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	lines := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(lines))
	fmt.Printf("Part 2: %v\n", p2(lines))
}
