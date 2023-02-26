const fs = require("fs");

const input = fs.readFileSync("./input.txt", "utf8");

class Point {
  constructor(x = 0, y = 0) {
    this.x = x;
    this.y = y;
  }

  move(direction) {
    switch (direction) {
      case "R":
        this.x++;
        break;
      case "L":
        this.x--;
        break;
      case "D":
        this.y--;
        break;
      case "U":
        this.y++;
        break;
    }
  }
}

class Grid {
  constructor() {
    this.head = new Point();
    this.tail = new Point();
    this.visitedPositions = new Set();
    this.visitedPositions.add(new Point());
  }

  moveHead(direction) {
    this.head.move(direction);
  }

  moveTail() {
    if (this.head.x == this.tail.x) {
      if (this.head.y > this.tail.y) {
        while (!this.isTouching()) {
          this.tail.move("U");
        }
      } else {
        while (!this.isTouching()) {
          this.tail.move("D");
        }
      }
      if (!this.visitedPositions.has(this.tail)) {
        this.visitedPositions.add({ ...this.tail });
      }
    }

    if (this.head.y == this.tail.y) {
      if (this.head.x > this.tail.x) {
        while (!this.isTouching()) {
          this.tail.move("R");
        }
      } else {
        while (!this.isTouching()) {
          this.tail.move("L");
        }
      }
      if (!this.visitedPositions.has({ ...this.tail })) {
        this.visitedPositions.add(this.tail);
      }
    }

    while (!this.isTouching()) {
      if (this.head.x > this.tail.x && this.head.y > this.tail.y) {
        this.tail.move("R");
        this.tail.move("U");
      } else if (this.head.x > this.tail.x && this.head.y < this.tail.y) {
        this.tail.move("D");
        this.tail.move("R");
      } else if (this.head.x < this.tail.x && this.head.y < this.tail.y) {
        this.tail.move("D");
        this.tail.move("L");
      } else if (this.head.x < this.tail.x && this.head.y > this.tail.y) {
        this.tail.move("U");
        this.tail.move("L");
      }
      if (!this.visitedPositions.has(this.tail)) {
        this.visitedPositions.add({ ...this.tail });
      }
    }
  }

  isTouching() {
    let temp = [-1, 0, 1];

    return (
      temp.includes(this.head.x - this.tail.x) &&
      temp.includes(this.head.y - this.tail.y)
    );
  }

  executeMotion(motion) {
    const m = motion.split(" ");
    const [direction, steps] = [m[0], parseInt(m[1])];

    for (let i = 0; i < steps; i++) {
      this.moveHead(direction);
      if (!this.isTouching()) {
        this.moveTail();
      }
    }
  }
}

let grid = new Grid();

for (let motion of input.split("\n")) {
  grid.executeMotion(motion);
}

console.log(grid.visitedPositions.size);

// let map = new Map();

// let p1 = new Point();

// map.set([0, 1], 1);

// map.set([0, 1], 2);
// console.log(map);
