const fs = require("fs");

const input = fs.readFileSync("./input.txt", "utf8");

let grid = [];

for (line of input.split("\n")) {
  grid.push(line.split("").slice(0, line.length - 1));
}

function scenicScore(i, j) {
  let l = grid[i][j];

  let scores = new Array(4).fill(0);

  if (i == 0 || j == 0 || i == grid.length - 1 || j == grid[0].length - 1) {
    return 0;
  }

  // Left trees
  for (x = i - 1; x >= 0; x--) {
    if (grid[x][j] >= l) {
      scores[0]++;
      break;
    }
    scores[0]++;
  }

  // Right trees
  for (x = i + 1; x < grid.length; x++) {
    if (grid[x][j] >= l) {
      scores[1]++;
      break;
    }
    scores[1]++;
  }

  // Top trees
  for (y = j - 1; y >= 0; y--) {
    if (grid[i][y] >= l) {
      scores[2]++;
      break;
    }
    scores[2]++;
  }

  // Bottom trees
  for (y = j + 1; y < grid[0].length; y++) {
    if (grid[i][y] >= l) {
      scores[3]++;
      break;
    }
    scores[3]++;
  }

  return scores.reduce((acc, x) => acc * x, 1);
}

let maxScenicScore = 0;

for (let i = 0; i < grid.length; i++) {
  for (let j = 0; j < grid[0].length; j++) {
    maxScenicScore = Math.max(maxScenicScore, scenicScore(i, j));
  }
}

console.log(maxScenicScore);
