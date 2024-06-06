import assert from "assert";
import { square } from "../build/debug.js";
assert.strictEqual(square(2), 4);
console.log("ok");
