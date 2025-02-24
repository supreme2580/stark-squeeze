export function reverseSecondDict(encodedStr: string): string {
    const secondDict: Record<string, string> = {
      "!": ".....",
      "*": "....",
      "#": "..."
    };
  
    const inverseSecondDict: Record<string, string> = Object.fromEntries(
      Object.entries(secondDict).map(([key, value]) => [value, key])
    );
  
    const sortedKeys = Object.keys(inverseSecondDict).sort((a, b) => b.length - a.length);
  
    let decodedStr = encodedStr;
  
    for (const key of sortedKeys) {
      decodedStr = decodedStr.split(key).join(inverseSecondDict[key]);
    }
  
    return decodedStr;
  }
  
