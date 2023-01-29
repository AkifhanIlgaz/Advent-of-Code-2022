package main

import (
	"bufio"
	"log"
	"os"
	"strings"
)

type Node struct {
	name                    string
	fileSize, directorySize int
	isDirectory             bool
	parent                  *Node
	childs                  []*Node
}

func (node *Node) calculateDirectorySize() {
	total := 0
	for _, child := range node.childs {
		if child.isDirectory {
			if child.directorySize == -1 {
				child.calculateDirectorySize()
			}
			total += child.directorySize
		} else {
			total += child.fileSize
		}
	}

	node.directorySize = total
}

func (node *Node) calculateTotalSizeOfDirectoriesWithSizeLessThan(maxSize int) int {
	totalSize := 0

	if node.isDirectory && node.directorySize <= maxSize {
		totalSize += node.directorySize
	}

	for _, child := range node.childs {
		if child.isDirectory {
			totalSize += child.calculateTotalSizeOfDirectoriesWithSizeLessThan(maxSize)
		}
	}

	return totalSize
}

func startsWith(s, prefix string) bool {
	return len(s) >= len(prefix) && s[:len(prefix)] == prefix
}

func main() {
	input, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)

	root := &Node{name: "/", fileSize: -1, directorySize: -1, isDirectory: true, parent: nil, childs: make([]*Node, 0)}

	currentNode := root

	for scanner.Scan() {
		if startsWith(scanner.Text(), "$") {
			command := scanner.Text()[2:]
			if startsWith(command, "cd") {
				tokens := strings.Split(command, "")
				if tokens
			}
		}
	}

}