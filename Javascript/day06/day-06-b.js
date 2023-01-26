const fs = require("fs");

const input = fs.readFileSync("./input.txt", "utf8");

FirstStartOfMessageMarker(input, 14);

function FirstStartOfMessageMarker(dataStream, numOfDistinctChars) {
  let chunks = windows(dataStream, numOfDistinctChars);

  for (let i = 0; i < chunks.length; i++) {
    const chunk = chunks[i];
    if (isAllUnique(chunk, numOfDistinctChars)) {
      console.log(i + numOfDistinctChars);
      break;
    }
  }
}

/**
 *
 * @param {string} arr
 * @param {number} windowSize
 * @returns {string[]}
 */
function windows(input, windowSize) {
  let result = [];

  for (let i = 0; i <= input.length - windowSize; i++) {
    result.push(input.slice(i, i + windowSize));
  }

  return result;
}

/**
 *
 * @param {string} chunk
 * @returns {boolean}
 */
function isAllUnique(chunk, numOfDistinctChars) {
  let uniqueChars = [];

  for (let ch of chunk) {
    if (!uniqueChars.includes(ch)) {
      uniqueChars.push(ch);
    }
  }

  return uniqueChars.length == numOfDistinctChars;
}
