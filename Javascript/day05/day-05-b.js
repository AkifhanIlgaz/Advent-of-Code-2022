class Cargoship {
  constructor(structure) {
    this.stacks = new Array(10);
    for (let i = structure.length - 1; i >= 0; i--) {
      const line = structure[i];
      let index = 1;
      for (let j = 1; j < line.length; j += 4) {
        const crane = line[j];
        if (crane !== " ") {
          let temp = this.stacks[index] || [];
          temp.push(crane);
          this.stacks[index] = temp;
          //  Why doesn't work ?
          //   this.stacks[index].push(crane)
        }
        index++;
      }
    }
  }

  /**
   *
   * @param {string[]} instructions
   */
  rearrange(instructions) {
    for (let ins of instructions) {
      let instruction = new Instruction(ins);
      this.moveCranes(instruction);
    }
  }

  /**
   *
   * @param {Instruction} instruction
   */
  moveCranes(instruction) {
    let range =
      this.stacks[instruction.src].length - instruction.numberOfCrates;

    for (let crate of this.stacks[instruction.src].splice(
      range,
      instruction.numberOfCrates
    )) {
      this.stacks[instruction.dest].push(crate);
    }
  }

  printTopOfEachStack() {
    let result = "";

    for (let i = 1; i < this.stacks.length; i++) {
      result += this.stacks[i].at(-1);
    }
    console.log(result);
  }
}

class Instruction {
  /**
   *
   * @param {string} input
   */
  constructor(input) {
    let temp = [];

    const ins = input.split(" ");

    for (let val of ins) {
      let num = parseInt(val, 10);

      if (num) temp.push(num);
    }

    this.numberOfCrates = temp[0];
    this.src = temp[1];
    this.dest = temp[2];
  }
}

const fs = require("fs");

const text = fs.readFileSync("./input.txt", "utf8").split("\n");

const stackStructure = text.slice(0, 8);
const instructions = text.slice(10);

let cargo = new Cargoship(stackStructure);
cargo.rearrange(instructions);

cargo.printTopOfEachStack();
