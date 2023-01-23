const fs = require("fs");

const text = fs.readFileSync("./input.txt", "utf8").trimEnd();

let totalPriority = 0;

let elves = text.split("\n");

for (let i = 0; i < elves.length; i += 3) {
  let group = elves.slice(i, i + 3);
  let badge = findBadge(group);
  totalPriority += prioritize(badge);
}

console.log(totalPriority);

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
 * @param {string[]} group
 * @returns {string}
 */
function findBadge(group) {
  return group[0]
    .split("")
    .find((item) => group[1].includes(item) && group[2].includes(item));
}
