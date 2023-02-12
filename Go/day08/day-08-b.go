package main

import (
	"fmt"
	"os"
	"strings"
)

type TreeGrid []string

func newTreeGrid(input string) TreeGrid {
	grid := []string{}

	for _, row := range strings.Split(input, "\n") {
		grid = append(grid, strings.TrimRight(row, "\r"))
	}

	return grid
}

func (t TreeGrid) scenicScore(i,j int) int {
	l := t[i][j]

	scores := [4]int{}

	// Edge
	if (i == 0 || j == 0 || i == len(t) -1 || j == len(t[0])-1) {
		return 0
	}

	// Left trees
	for x:= i -1; x >= 0; x-- {
		if t[x][j] >= l {
			scores[0]++
			break
		}
		scores[0]++
	}

	// Right trees
	for x:= i +1; x < len(t); x++ {
		if t[x][j] >= l {
			scores[1]++
			break
		}
		scores[1]++
	}

	// Top trees
	for y:=j-1; y >= 0; y-- {
		if t[i][y] >= l {
			scores[2]++
			break
		}
		scores[2]++
	}

	// Bottom trees
	for y:=j+1; y < len(t[0]); y++ {
		if t[i][y] >= l {
			scores[3]++
			break
		}
		scores[3]++
	}

	score := 1

	for _, val := range scores {
		score *= val
	}

	return  score
}

func (t TreeGrid) maxScenicScore() {
	maxScore := 0

	for i := 0; i < len(t);i++ {
		for j := 0; j < len(t[0]); j++ {
			maxScore = max(maxScore, t.scenicScore(i,j))
		}
	}

	fmt.Println(maxScore)
}

func max(a,b int) int {
	if a > b {
		return a
	}
	return b
}


func main() {
	bytes, _ := os.ReadFile("./input.txt")
	input := string(bytes)

	grid := newTreeGrid(input)
	grid.maxScenicScore()
}