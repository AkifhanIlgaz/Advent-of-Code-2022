package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("./input.txt");
	text := strings.TrimSpace(string(bytes))
	
	pointMap := map[byte]int{
		'A': 1,
		'X': 1,
		'B': 2,
		'Y': 2,
		'C': 3,
		'Z': 3,
	}

	point := 0

	for _, round := range strings.Split(text, "\n") {
		opponent, me := pointMap[round[0]], pointMap[round[2]]

		point += calculatePoint(opponent, me)
	}

	fmt.Println(point)
}


func calculatePoint(opponent, me int ) int {
	point := 0
	diff := abs(opponent,me)

	if opponent == me {
		point += 3
	} else if (diff ==1 && me > opponent) || (me == 1 && opponent == 3) {
		point += 6
	}
	point += me
	return point
}


func abs(a,b int) int {
	diff := a-b

	if diff < 0 {
		diff *= -1
	}

	return diff
}