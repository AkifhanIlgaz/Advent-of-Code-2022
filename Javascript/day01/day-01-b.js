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

/*
let cals = text.split("\n\n").map((cals) =>
  cals
    .split("\n")
    .map((c) => parseInt(c, 10))
    .reduce((acc, el) => acc + el)
);

cals.sort((a, b) => b - a);
*/

console.log(cals.slice(0, 3).reduce((acc, c) => acc + c));
