import { formatBytes } from "./format.js";
import { test, expect } from "vitest";

test("formats bytes less than 1024", () => {
  expect(formatBytes(512)).toBe("512 B");
});

test("formats KiB correctly", () => {
  expect(formatBytes(2048)).toBe("2 KiB");
  expect(formatBytes(1536)).toBe("1.5 KiB");
  expect(formatBytes(1024)).toBe("1 KiB");
});

test("formats MiB correctly", () => {
  expect(formatBytes(1048576)).toBe("1 MiB");
  expect(formatBytes(1572864)).toBe("1.5 MiB");
  expect(formatBytes(2097152)).toBe("2 MiB");
});

test("formats GiB correctly", () => {
  expect(formatBytes(1073741824)).toBe("1 GiB");
  expect(formatBytes(1610612736)).toBe("1.5 GiB");
  expect(formatBytes(2147483648)).toBe("2 GiB");
});

test("formats large values with no decimal if intValue >= 1000", () => {
  expect(formatBytes(1024 * 1024 * 1000)).toBe("1000 MiB");
});
