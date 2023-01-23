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

	totalPriority := 0

	for _, pack := range strings.Split(text, "\n") {
		middle := len(pack) / 2
		first, second := pack[:middle], pack[middle:]
		common := findCommon(first, second)
		totalPriority += prioritize(common)
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

func findCommon(first, second string) rune {
	common := 'a'

	for _, item := range first {
		if strings.ContainsRune(second, item) {
			common = item
		}
	}

	return common
}