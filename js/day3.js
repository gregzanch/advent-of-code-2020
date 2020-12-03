const map = require("fs").readFileSync("res/3/input", "utf8").split("\n");

function getFeature(pos, pattern) {
  const index = pos % pattern.length;
  return pattern[index];
}

getFeature(36, map[0]); //?

/*
Right 1, down 1.
Right 3, down 1. (This is the slope you already checked.)
Right 5, down 1.
Right 7, down 1.
Right 1, down 2.
*/

function getTrees(slope) {
  let current = 0;
  let trees = 0;
  for (let i = 0; i < map.length; i += slope[1]) {
    if (getFeature(current, map[i]) === "#") {
      trees += 1;
    }
    current += slope[0];
  }

  return trees;
}

let slopes = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
];

getTrees(slopes[1]); //?

let trees = slopes.map(getTrees).reduce((a, b) => a * b); //?
