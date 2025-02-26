import { applySecondDict } from "../../src/compression/applySecondDict";
// Mock the secondDict used in applySecondDict
const secondDict: Record<string, string> = {
  ".....": "!",
  "....": "*",
  "...": "#"
};

describe("applySecondDict", () => {
  test("should correctly apply secondDict transformations", () => {
    expect(applySecondDict("..... .... ...")).toBe("!*#");
    expect(applySecondDict("..... ..... .... ...")).toBe("!!*#");
    expect(applySecondDict(".... ... .....")).toBe("*#!");
  });

  test("should return the same string if no transformation is possible", () => {
    expect(applySecondDict("abcdef")).toBe("abcdef");
  });

  test("should handle mixed cases with partial matches", () => {
    expect(applySecondDict("..... abc ....")).toBe("! abc *");
  });

  test("should correctly apply secondDict transformations", () => {
    const result = applySecondDict("..... .... ...");
    console.log("Received output:", result); // Debugging log
    expect(result).toBe("!*#");
  });
  
});
