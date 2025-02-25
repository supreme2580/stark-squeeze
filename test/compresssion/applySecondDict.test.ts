import { applySecondDict, secondDict } from '../../src/compression/applySecondDict';,\
// import { } from "../../src/constants/dictionaries";

describe("applySecondDict", () => {
  test("should replace encoded patterns with corresponding values", () => {
    for (const [pattern, replacement] of Object.entries(secondDict)) {
      expect(applySecondDict(pattern)).toBe(replacement);
    }
  });

  test("should replace multiple occurrences correctly", () => {
    const testStr = Object.keys(secondDict).join("");
    const expectedStr = Object.values(secondDict).join("");
    expect(applySecondDict(testStr)).toBe(expectedStr);
  });

  test("should prioritize longer patterns when they overlap", () => {
    const overlappingDict = {
      "abc": "X",
      "ab": "Y",
      "c": "Z"
    };
    Object.assign(secondDict, overlappingDict);
    expect(applySecondDict("abc")).toBe("X");
    expect(applySecondDict("ab" + "c")).toBe("XZ");
  });

  test("should handle input with no matching patterns", () => {
    expect(applySecondDict("xyz"))
      .toBe("xyz");
  });

  test("should handle mixed input with matches and non-matches", () => {
    const somePattern = Object.keys(secondDict)[0];
    const someReplacement = secondDict[somePattern];
    expect(applySecondDict(`test${somePattern}end`))
      .toBe(`test${someReplacement}end`);
  });

  test("should return an empty string for an empty input", () => {
    expect(applySecondDict("")).toBe("");
  });
});
