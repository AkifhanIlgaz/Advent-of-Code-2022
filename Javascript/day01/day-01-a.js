const fs = require("fs");

const text = fs.readFileSync("./input.txt", "utf-8");

let cals = text.split("\n\n").map((cals) =>
  cals
    .split("\n")
    .map((c) => parseInt(c, 10))
    .reduce((acc, el) => acc + el)
);

console.log(Math.max(...cals));
