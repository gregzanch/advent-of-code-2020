const input = require("fs").readFileSync("res/4/input", "utf8");

const makeObject = (fields) =>
  fields.reduce((a, b) => Object.assign(a, { [b[0]]: b[1] }), {});

const rules = {
  byr: (value) =>
    String(value).match(/^\d{4}$/gim) &&
    Number(value) >= 1920 &&
    Number(value) <= 2002,
  iyr: (value) =>
    String(value).match(/^\d{4}$/gim) &&
    Number(value) >= 2010 &&
    Number(value) <= 2020,
  eyr: (value) =>
    String(value).match(/^\d{4}$/gim) &&
    Number(value) >= 2020 &&
    Number(value) <= 2030,
  hgt: (value) => {
    const val = String(value).trim(); //?
    const num = Number(val.replace(/in|cm/gim, "").trim()); //?
    const cm = val.match(/^\d{3}cm$/gim) && num >= 150 && num <= 193; //?
    const inch = val.match(/^\d{2}in$/gim) && num >= 59 && num <= 76; //?
    return cm || inch;
  },
  hcl: (value) => {
    let val = String(value);
    return !!val.match(/^\#[0-9a-f]{6}$/gim);
  },
  ecl: (value) => {
    return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(value);
  },
  pid: (value) => {
    return !!String(value).match(/^[0-9]{9}$/gim);
  },
  cid: (value) => {
    return true;
  },
};

const validateField = (field) => {
  const id = field[0].trim();
  const value = field[1];
  const valid = rules[id](value);
  return valid;
};

const entries = input;
// .filter((x) => x.length == 7)

const getValid = (str: string) => {
  return str.split("\n\n").map((x) => {
    const fields = x.split(/\s+/gim).map((y) => y.split(":"));
    const validFields = fields.filter((x) => validateField(x));
    return validFields;
  });
};

const keys = ["ecl", "eyr", "byr", "hcl", "pid", "iyr", "hgt"].sort();
const keystr = keys.join("");

const hasAllFields = (fields) =>
  fields
    .map((x) => x[0])
    .filter((x) => x !== "cid")
    .sort()
    .join("") === keystr;

const solution = getValid(input).filter(hasAllFields).length; //?
