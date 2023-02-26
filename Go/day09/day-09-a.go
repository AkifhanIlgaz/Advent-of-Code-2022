package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type point struct {
	x, y int
}

func (p *point) move(direction rune) {
	switch direction {
	case 'R':
		p.x++
	case 'L':
		p.x--
	case 'D':
		p.y--
	case 'U':
		p.y++
	}
}

func main() {

	//Read input file
	input, _ := os.Open("input.txt")
	defer input.Close()
	sc := bufio.NewScanner(input)

	visitedPositions := make(map[point]bool)
	head, tail := point{0, 0}, point{0, 0}
	visitedPositions[tail] = true

	for sc.Scan() {
		direction := rune(sc.Text()[0])
		steps, _ := strconv.Atoi(sc.Text()[2:])

		for steps > 0 {
			head.move(direction)
			steps--
			tail = adjustTail(tail, head)
			visitedPositions[tail] = true
		}
	}

	fmt.Println(len(visitedPositions))
}

func adjustTail(tail point, head point) point {
	newTail := tail
	switch (point{head.x - tail.x, head.y - tail.y}) {
	case point{-2, 1}, point{-1, 2}, point{0, 2}, point{1, 2}, point{2, 1}:
		newTail.y++
	}
	switch (point{head.x - tail.x, head.y - tail.y}) {
	case point{1, 2}, point{2, 1}, point{2, 0}, point{2, -1}, point{1, -2}:
		newTail.x++
	}
	switch (point{head.x - tail.x, head.y - tail.y}) {
	case point{2, -1}, point{1, -2}, point{0, -2}, point{-1, -2}, point{-2, -1}:
		newTail.y--
	}
	switch (point{head.x - tail.x, head.y - tail.y}) {
	case point{-1, -2}, point{-2, -1}, point{-2, -0}, point{-2, 1}, point{-1, 2}:
		newTail.x--
	}
	return newTail
}
