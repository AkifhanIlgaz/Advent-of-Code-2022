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

func (t TreeGrid) isVisible(i,j int) bool {
	l := t[i][j]

	visible := [4]bool{true,true,true,true}

	// Edge
	if (i == 0 || j == 0 || i == len(t) -1 || j == len(t[0])-1) {
		return true
	}

	// Left trees
	for x:= 0; x < i; x++ {
		if t[x][j] >= l {
			visible[0] = false
			break
		}
	}

	// Right trees
	for x:= i +1; x < len(t); x++ {
		if t[x][j] >= l {
			visible[1] = false
			break
		}
	}

	// Top trees
	for y:=0; y < j; y++ {
		if t[i][y] >= l {
			visible[2] = false
			break
		}
	}

	// Bottom trees
	for y:=j+1; y < len(t[0]); y++ {
		if t[i][y] >= l {
			visible[3] = false
			break
		}
	}


	for _, v:= range visible {
		if v == true {
			return true
		}
	}

	return false
}

func (t TreeGrid) calcVisibleTrees() {
	count := 0

	for i := 0; i < len(t);i++ {
		for j := 0; j < len(t[0]); j++ {
			if t.isVisible(i,j) {
				count++
			}
		}
	}

	fmt.Println(count)
}


func main() {
	bytes, _ := os.ReadFile("./input.txt")
	input := string(bytes)

	grid := newTreeGrid(input)
	grid.calcVisibleTrees()
}