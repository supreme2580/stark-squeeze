import { dotsToBinary } from "../../src/decompression/dotsToBinary";

describe("dotsToBinary", () => {
    it("should convert dot representation strings back to binary strings", () => {
      expect(dotsToBinary(".. . . . .")).toBe("0110001000010000100001");
      expect(dotsToBinary(". . . ."))
        .toBe("01000010000100001");
      expect(dotsToBinary("... .."))
        .toBe("01110011");
    });
  
    it("should handle empty strings correctly", () => {
      expect(dotsToBinary("")).toBe("");
    });
  
    it("should correctly convert dot representations that include padding", () => {
      expect(dotsToBinary(". . . . ."))
        .toBe("0100001000010000100001");
    });
  });  