package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("./input.txt")
	input := string(bytes)

	findFirstStartOfPacketMarker(input)
}


func findFirstStartOfPacketMarker(dataStream string) {
	chunks := windows(dataStream, 4)

	for i, chunk := range chunks {
		if isAllUnique(chunk) {
			fmt.Println(i +4)
			break
		}
	}
}

func windows(input string, windowSize int ) []string {
	chunks := []string{}

	for i := 0 ; i<= len(input) - windowSize; i++ {
		chunks = append(chunks, input[i: i+4])
	}

	return chunks
}

func isAllUnique(input string) bool {
	uniqueChars := ""

	for _, ch := range input {
		if !strings.ContainsRune(uniqueChars, ch) {
			uniqueChars += string(ch)
		}
	}

	return len(uniqueChars) == 4
}