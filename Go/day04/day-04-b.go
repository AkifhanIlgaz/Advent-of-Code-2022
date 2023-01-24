package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("./input.txt")
	text := string(bytes)

	sum := 0

	for _, pair := range strings.Split(text, "\n") {
		ranges := strings.Split(pair, ",")

		first := strings.SplitN(ranges[0], "-", 2)
		x1 := toInt(first[0])
		x2 := toInt(first[1])

		second := strings.SplitN(ranges[1], "-", 2)
		y1 := toInt(second[0])
		y2 := toInt(second[1])

		if isOverlap(x1, x2,y1,y2) {
			sum++
		}
	}
	
	fmt.Println(sum)
}


func toInt(s string) int {
	v, err := strconv.Atoi(s)
	if err != nil {
		os.Exit(1)
	}
	return v
}

func isOverlap(x1,x2,y1,y2 int) bool {
	return x1 <= y2 && y1 <= x2
}