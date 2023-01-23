const fs = require("fs");

const text = fs.readFileSync("./input.txt", "utf8").trimEnd();

let totalPriority = 0;

for (let pack of text.split("\n")) {
  let [first, second] = splitCompartments(pack);
  let common = findCommon(first, second);
  totalPriority += prioritize(common);
}

console.log(totalPriority);

/**
 *
 * @param {string} pack
 * @returns {string[]}
 */
function splitCompartments(pack) {
  let middle = pack.length / 2; // We are assured that each compartment has same number of items. So we don't need Math.floor()
  return [pack.substring(0, middle), pack.substring(middle)];
}

/**
 *
 * @param {string} item
 * @returns {number}
 */
function prioritize(item) {
  if (item >= "a") {
    return item.charCodeAt(0) - "a".charCodeAt(0) + 1;
  } else {
    return item.charCodeAt(0) - "A".charCodeAt(0) + 27;
  }
}

/**
 *
 * @param {string} first
 * @param {string} second
 * @returns {string}
 */
function findCommon(first, second) {
  return first.split("").find((item) => second.includes(item));
}
