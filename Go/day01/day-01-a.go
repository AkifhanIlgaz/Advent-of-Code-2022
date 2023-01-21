package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	bytes, err := os.ReadFile("./input.txt")
	if err != nil {
		panic(1)
	}
	text := string(bytes)

	cals := calsToSum(strings.Split(text, "\n\n"))

	fmt.Println(cals[0])
}

func calsToSum(calories []string) []int {
	cals := make([]int, len(calories))

	for i, cal := range calories {
		sum := 0
		for _, c:= range strings.Split(cal, "\n") {
			conv, _ := strconv.Atoi(string(c))
			sum += conv
		}
		cals[i] = sum
	}

	sort.Slice(cals, func(i, j int) bool { return cals[i] > cals[j]})

	return cals
}
