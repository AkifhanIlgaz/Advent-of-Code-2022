package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
	bytes, _ := os.ReadFile("./input.txt")
	text := string(bytes)

	elves := strings.Split(text, "\n")
	totalPriority := 0

	for i := 0; i < len(elves); i+=3 {
		group := elves[i: i+3]
		badge := findBadge(group)
		totalPriority += prioritize(badge)
	}

	fmt.Println(totalPriority)
}

func prioritize(item rune) int {
	if unicode.IsLower(item) {
		return int(item) -'a' + 1
	} else {
		return int(item) - 'A' +27
	}
}

func findBadge(group []string) rune {
	common := 'a'

	for _, item := range group[0] {
		if strings.ContainsRune(group[1], item) && strings.ContainsRune(group[2], item) {
			common = item
		}
	}

	return common
}