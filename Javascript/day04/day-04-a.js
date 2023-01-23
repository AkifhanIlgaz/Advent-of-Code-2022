const fs = require("fs");

const text = fs.readFileSync("./input.txt", "utf8").trimEnd();

let total = 0;

for (let pair of text.split("\n")) {
  let [first, second] = pair.split(",");
  let [x1, x2] = first.split("-");
  let [y1, y2] = second.split("-");

  if (isSubset(+x1, +x2, +y1, +y2)) {
    total++;
  }
}

console.log(total);

function isSubset(x1, x2, y1, y2) {
  return (x1 >= y1 && x2 <= y2) || (y1 >= x1 && y2 <= x2);
}
