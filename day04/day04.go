package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/leonobilis/adventofcode2021/utils"
)

type Board struct {
	b                      [][]int
	marked                 map[int]struct{}
	markedRows, markedCols [5]int
	win                    bool
}

func NewBoard(b [][]int) Board {
	var board Board
	board.b = b
	board.marked = make(map[int]struct{})
	return board
}

func (self *Board) Mark(number int) {
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {
			if self.b[i][j] == number {
				self.marked[number] = struct{}{}
				self.markedRows[i]++
				self.markedCols[j]++
				if self.markedRows[i] == 5 || self.markedCols[j] == 5 {
					self.win = true
				}
				return
			}
		}
	}
}

func (self Board) SumUnmarked() (sum int) {
	for _, row := range self.b {
		for _, n := range row {
			if _, contains := self.marked[n]; !contains {
				sum += n
			}
		}
	}
	return
}

func (self Board) Win() bool {
	return self.win
}

func parseInput(input string) (numbers []int, Boards []Board) {
	for i, s := range strings.Split(input, "\n\n") {
		if i == 0 {
			for _, n := range strings.Split(s, ",") {
				numbers = append(numbers, utils.Atoi(n))
			}
		} else {
			var b [][]int
			for _, row := range strings.Split(s, "\n") {
				var rowNums []int
				for _, r := range strings.Fields(row) {
					rowNums = append(rowNums, utils.Atoi(r))
				}

				b = append(b, rowNums)
			}
			Boards = append(Boards, NewBoard(b))
		}
	}
	return
}

func p1(numbers []int, boards []Board) int {
	for _, n := range numbers {
		for i := 0; i < len(boards); i++ {
			boards[i].Mark(n)
			if boards[i].Win() {
				return boards[i].SumUnmarked() * n
			}
		}
	}
	return 0
}

func p2(numbers []int, boards []Board) int {
	for _, n := range numbers {
		var newBoards []Board
		for i := 0; i < len(boards); i++ {
			boards[i].Mark(n)
			if !boards[i].Win() {
				newBoards = append(newBoards, boards[i])
			} else if len(boards) == 1 {
				return boards[i].SumUnmarked() * n
			}
		}
		boards = newBoards

	}
	return 0
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	numbers, boards := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(numbers, boards))
	numbers, boards = parseInput(string(input))
	fmt.Printf("Part 2: %v\n", p2(numbers, boards))
}
