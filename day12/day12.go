package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type path struct {
	a, b string
}

func parseInput(input string) (output []path) {
	for _, s := range strings.Split(input, "\n") {
		points := strings.Split(s, "-")
		output = append(output, path{
			a: points[0],
			b: points[1],
		})
		output = append(output, path{
			a: points[1],
			b: points[0],
		})
	}
	return
}

func copyVisited(visited map[string]int) map[string]int {
	visitedCopy := make(map[string]int)
	for k, v := range visited {
		visitedCopy[k] = v
	}
	return visitedCopy
}

func traverse(paths []path, p path, visited map[string]int, allowedSmallVisit int) int {
	if p.b == "end" {
		return 1
	}
	v, s := visited[p.b]
	if s {
		if v >= allowedSmallVisit {
			return 0
		}
		allowedSmallVisit--
	}
	if p.b[0] > "Z"[0] {
		visited = copyVisited(visited)
		visited[p.b] = v + 1
	}
	count := 0
	for _, p2 := range paths {
		if p2.a == p.b {
			count += traverse(paths, p2, visited, allowedSmallVisit)
		}
	}
	return count
}

func p1(paths []path) (count int) {
	for _, p := range paths {
		if p.a == "start" {
			visited := make(map[string]int)
			visited["start"] = 1
			count += traverse(paths, p, visited, 1)
		}
	}
	return
}

func p2(paths []path) (count int) {
	for _, p := range paths {
		if p.a == "start" {
			visited := make(map[string]int)
			visited["start"] = 2
			count += traverse(paths, p, visited, 2)
		}
	}
	return
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	paths := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(paths))
	fmt.Printf("Part 2: %v\n", p2(paths))
}
