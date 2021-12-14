package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func parseInput(input string) (string, map[string][2]string) {
	templateRules := strings.Split(input, "\n\n")
	rules := make(map[string][2]string)
	for _, dot := range strings.Split(templateRules[1], "\n") {
		pairElement := strings.Split(dot, " -> ")
		rules[pairElement[0]] = [2]string{
			string([]byte{pairElement[0][0], pairElement[1][0]}),
			string([]byte{pairElement[1][0], pairElement[0][1]}),
		}
	}
	return templateRules[0], rules
}

func p1(template string, rules map[string][2]string) uint {
	for i := 0; i < 10; i++ {
		var builder strings.Builder
		for t := 0; t < len(template)-1; t++ {
			next := rules[template[t:t+2]]
			builder.WriteString(next[0])
		}
		builder.WriteByte(template[len(template)-1])
		template = builder.String()
	}
	elements := make(map[rune]uint)
	for _, element := range template {
		elements[element]++
	}
	max, min := uint(0), ^uint(0)>>1
	for _, v := range elements {
		if v > max {
			max = v
		}
		if v < min {
			min = v
		}
	}
	return max - min
}

func countElements(pair string, rules map[string][2]string, elements map[byte]uint, i, stop int) {
	if i == stop {
		return
	}
	next := rules[pair]
	elements[next[0][1]]++
	countElements(next[0], rules, elements, i+1, stop)
	countElements(next[1], rules, elements, i+1, stop)
}

type ElCount struct {
	el    byte
	count uint
}

func p2(template string, rules map[string][2]string) uint {
	initialBuild := 20
	for i := 0; i < initialBuild; i++ {
		var builder strings.Builder
		for t := 0; t < len(template)-1; t++ {
			next := rules[template[t:t+2]]
			builder.WriteString(next[0])
		}
		builder.WriteByte(template[len(template)-1])
		template = builder.String()
	}
	stop := 40 - initialBuild
	aggregator := make(chan ElCount)
	finish := make(chan bool)
	elementsSum := make(map[byte]uint)
	pairs := make(map[string]uint)
	for t := 0; t < len(template)-1; t++ {
		elementsSum[template[t]]++
		pairs[template[t:t+2]]++
	}
	for pair, factor := range pairs {
		go func(p string, f uint) {
			elements := make(map[byte]uint)
			countElements(p, rules, elements, 0, stop)
			for el, count := range elements {
				aggregator <- ElCount{el, count * f}
			}
			finish <- true
		}(pair, factor)
	}
	elementsSum[template[len(template)-1]]++
	for f := 0; f < len(pairs); {
		select {
		case <-finish:
			f++
		case a := <-aggregator:
			elementsSum[a.el] += a.count
		}
	}
	max, min := uint(0), ^uint(0)>>1
	for _, v := range elementsSum {
		if v > max {
			max = v
		}
		if v < min {
			min = v
		}
	}
	return max - min
}

func main() {
	input, _ := ioutil.ReadFile("input.txt")
	template, rules := parseInput(string(input))
	fmt.Printf("Part 1: %v\n", p1(template, rules))
	fmt.Printf("Part 2: %v\n", p2(template, rules))
}
