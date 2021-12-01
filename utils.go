package utils

import (
	"strconv"
	"strings"
)

func Atoi(a string) int {
	i, err := strconv.Atoi(a)
	if err != nil {
		panic(err)
	}
	return i
}

func GetIntArray(input string) (output []int) {
	for _, s := range strings.Split(input, "\n") {
		output = append(output, Atoi(s))
	}
	return
}
