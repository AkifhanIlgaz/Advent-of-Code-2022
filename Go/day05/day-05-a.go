package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type instruction struct{
	numOfCrates   int
	src, dest     int
}

func newInstruction(input string) instruction {
	temp := []int{}

	ins := strings.Fields(input)

	for _, val := range ins {
		num, err := strconv.Atoi(val)
		if err == nil {
			temp = append(temp, num)
		}
	}
	
	return instruction {
		numOfCrates: temp[0],
		src: temp[1],
		dest: temp[2],
	}
}

type stacks [][]byte

func initStacks(structure []string) stacks  {
	stacks := make([][]byte, 10)
	
	for i := len(structure) - 1; i >= 0; i-- {
		line := structure[i]
		index := 1
		for j := 1; j < len(line); j += 4 {
			crane := line[j]
			if crane != ' ' {
				stacks[index] = append(stacks[index], crane)
			}
			index++
		}
		
	}

	return stacks
}

func (s *stacks) rearrange(instructions []string) {
	for _, ins := range instructions {
		instruction := newInstruction(ins)
		s.moveCranes(instruction)
	}
}

func (s *stacks) moveCranes(instruction instruction) {
		for i := 0; i< instruction.numOfCrates; i++ {
			crane := (*s)[instruction.src][len((*s)[instruction.src]) - 1] // Get last element
			(*s)[instruction.src] = pop((*s)[instruction.src])
			(*s)[instruction.dest] = append((*s)[instruction.dest], crane)
		}
}

func (s stacks) printTopOfEachStack() {
	for i:=1 ; i< len(s); i++ {
		fmt.Print(string(s[i][len(s[i]) -1]))
	}
}

func pop(arr []byte) []byte {
	return arr[:len(arr) -1]
}

func main() {
	bytes, _ := os.ReadFile("./input.txt")
	text := strings.Split(string(bytes), "\n")

	stackStructure := text[:8]
	instructions := text[10:]

	stacks := initStacks(stackStructure)
	stacks.rearrange(instructions)

	stacks.printTopOfEachStack()	
}

