const fs = require("fs");

const text = fs.readFileSync("./input.txt", "utf8").trimEnd();

let point = 0;

for (let round of text.split("\n")) {
  let opponent = shapeToPoint(round[0]);
  let strategy = round[2];

  point += calculatePoint(opponent, strategy);
}

/**
 *
 * @param {string} shape
 * @returns number
 */
function shapeToPoint(shape) {
  // "A" => 65
  return shape.charCodeAt(0) - 65 + 1;
}

/**
 *
 * @param {number} opponent
 * @param {string} strategy
 * @returns {number}
 */
function calculatePoint(opponent, strategy) {
  let myMove = 0;

  if (strategy == "X") {
    myMove = opponent - 1;
    if (myMove == 0) {
      myMove = 3;
    }
  } else if (strategy == "Y") {
    myMove = opponent + 3;
  } else {
    myMove = (opponent + 1) % 3;
    if (myMove == 0) {
      myMove = 3;
    }
    myMove += 6;
  }

  return myMove;
}

console.log(point);
