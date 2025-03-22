import { binaryToDots } from "../../src/compression/v1/binaryToDots";
import { firstDict } from "../../src/constants/dictionaries";

describe("binaryToDots", () => {
  it("should convert a binary string to the correct dot representation", () => {
    expect(binaryToDots("00000")).toBe(firstDict["00000"]);
    expect(binaryToDots("00001")).toBe(firstDict["00001"]);
    expect(binaryToDots("00010")).toBe(firstDict["00010"]);
    expect(binaryToDots("1101011101")).toBe(".. . ... .");
  });

  it("should pad binary strings that are not multiples of 5", () => {
    expect(binaryToDots("1101")).toBe(firstDict["01101"]);
    expect(binaryToDots("110101")).toBe(firstDict["11010"] + " " + firstDict["10000"]);
  });

  it("should throw an error for invalid input", () => {
    expect(() => binaryToDots("12345")).toThrow("Input must be a binary string.");
    expect(() => binaryToDots("abcde")).toThrow("Input must be a binary string.");
    expect(() => binaryToDots("10102")).toThrow("Input must be a binary string.");
  });

  it("should handle empty strings correctly", () => {
    expect(binaryToDots("")).toBe("");
  });
});