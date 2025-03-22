import { secondDict } from "../../constants/dictionaries";

export function applySecondDict(encodedStr: string): string {
  // Sort keys by descending length to prioritize longer patterns
  const patterns = Object.keys(secondDict).sort((a, b) => b.length - a.length);
  let result = "";
  let i = 0;
  
  while (i < encodedStr.length) {
    let matched = false;
    // Try each pattern from longest to shortest
    for (const pattern of patterns) {
      if (encodedStr.substring(i, i + pattern.length) === pattern) {
        result += secondDict[pattern];
        i += pattern.length;
        matched = true;
        break;
      }
    }
    // If no pattern matched, append the current character (should rarely happen given '.' is a key)
    if (!matched) {
      result += encodedStr[i];
      i++;
    }
  }
  
  return result;
}
