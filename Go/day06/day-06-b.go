package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	bytes, _ := os.ReadFile("./input.txt")
	input := string(bytes)

	FirstStartOfMessageMarker(input, 14)
}


func FirstStartOfMessageMarker(dataStream string, numOfDistinctChars int) {
	chunks := windows(dataStream, numOfDistinctChars)

	for i, chunk := range chunks {
		if isAllUnique(chunk, numOfDistinctChars) {
			fmt.Println(i + numOfDistinctChars)
			break
		}
	}
}

func windows(input string, windowSize int ) []string {
	chunks := []string{}

	for i := 0 ; i<= len(input) - windowSize; i++ {
		chunks = append(chunks, input[i: i+windowSize])
	}

	return chunks
}

func isAllUnique(input string, numOfDistinctChars int) bool {
	uniqueChars := ""

	for _, ch := range input {
		if !strings.ContainsRune(uniqueChars, ch) {
			uniqueChars += string(ch)
		}
	}

	return len(uniqueChars) == numOfDistinctChars
}