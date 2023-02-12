const fs = require("fs");

const input = fs.readFileSync("./input.txt", "utf8");

let grid = [];

for (line of input.split("\n")) {
  grid.push(line.split("").slice(0, line.length - 1));
}

function isVisible(i, j) {
  let l = grid[i][j];

  let visible = new Array(4).fill(true);

  if (i == 0 || j == 0 || i == grid.length - 1 || j == grid[0].length - 1) {
    return true;
  }

  // Left trees
  for (x = 0; x < i; x++) {
    if (grid[x][j] >= l) {
      visible[0] = false;
      break;
    }
  }

  // Right trees
  for (x = i + 1; x < grid.length; x++) {
    if (grid[x][j] >= l) {
      visible[1] = false;
      break;
    }
  }

  // Top trees
  for (y = 0; y < j; y++) {
    if (grid[i][y] >= l) {
      visible[2] = false;
      break;
    }
  }

  // Bottom trees
  for (y = j + 1; y < grid[0].length; y++) {
    if (grid[i][y] >= l) {
      visible[3] = false;
      break;
    }
  }

  return visible.includes(true);
}

let count = 0;

for (let i = 0; i < grid.length; i++) {
  for (let j = 0; j < grid[0].length; j++) {
    if (isVisible(i, j)) {
      count++;
    }
  }
}
console.log(grid);
console.log(count);
