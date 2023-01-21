const fs = require("fs");

const text = fs.readFileSync("./input.txt", "utf8").trimEnd();

const pointMap = new Map([
  ["A", 1],
  ["X", 1],
  ["B", 2],
  ["Y", 2],
  ["C", 3],
  ["Z", 3],
]);

let point = 0;

text.split("\n").forEach((round) => {
  let [opponent, me] = [pointMap.get(round[0]), pointMap.get(round[2])];
  point += calculatePoint(opponent, me);
});

function calculatePoint(opponent, me) {
  let point = 0;
  let diff = Math.abs(opponent - me);

  if (opponent == me) {
    point += 3;
  } else if ((diff == 1 && me > opponent) || (me == 1 && opponent == 3)) {
    point += 6;
  }
  point += me;

  return point;
}

console.log(point);
