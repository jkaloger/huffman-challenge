import { test, expect } from "bun:test";

import { encode, decode } from "./index";

const raw = "this text is totally rad, dude";
const encoded =
  "111 01011 1000 1011 110 111 0111 0001 111 110 1000 1011 110 111 10100 111 0110 1001 1001 0100 110 10101 0110 001 01010 110 001 0000 001";

test("encodes correctly", () => {
  expect(encode(raw)).toBe(encoded);
});

test("decodes correctly", () => {
  expect(decode(encoded)).toBe(raw);
});

test("decode/encodes correctly", () => {
  expect(decode(encode(raw))).toBe(raw);
});
