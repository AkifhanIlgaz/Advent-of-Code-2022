package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("./input.txt")
	text := strings.TrimSpace(string(bytes)) 

	point := 0

	for _, round := range strings.Split(text, "\n") {
		opponent , strategy := shapeToPoint(round[0]), round[2]
		point += calculatePoint(opponent, strategy)
	}

	fmt.Println(point)
}

func calculatePoint(opponent int , strategy byte) int {
	var myMove int

	if strategy == 'X' {
		myMove = opponent - 1 
		if myMove == 0 {
			myMove = 3
		}
	} else if strategy == 'Y' {
		myMove = opponent + 3
	} else {
		myMove = (opponent +1) % 3
		if myMove == 0 {
			myMove = 3
		}
		myMove += 6
	}

	return myMove
}


func shapeToPoint(shape byte) int {
	return int(shape - 'A') + 1
}