const fs = require("fs");

const text = fs.readFileSync("./input.txt", "utf-8");

let cals = [0];

for (let cal of text.split("\n\n")) {
  const sum = cal
    .split("\n")
    .map((c) => parseInt(c, 10))
    .reduce((acc, el) => acc + el);

  for (let i = 0; i < cals.length; i++) {
    if (sum > cals[i]) {
      cals.splice(i, 0, sum);
      break;
    }
  }
}

console.log(cals[0] + cals[1] + cals[2]);
