import readline from "readline";

export function splitBy5(binaryString: string): string[] {
    if (typeof binaryString !== "string" || !/^[01]*$/.test(binaryString)) {
        throw new Error("Input must be a valid binary string.");
    }

    const totalSize = binaryString.length;
    if (totalSize === 0) {
        console.log("⚠️ Input is an empty string. Nothing to process.");
        return [];
    }

    console.log(`🔢 Total binary size: ${totalSize} bits`);

    const chunks: string[] = [];
    const chunkSize = 5;

    const progressBarLength = 30;
    const updateInterval = Math.ceil(totalSize / progressBarLength);

    for (let i = 0; i < totalSize; i += chunkSize) {
        const chunk = binaryString.slice(i, i + chunkSize).padEnd(chunkSize, "0");
        chunks.push(chunk);

        if (i % updateInterval === 0 || i + chunkSize >= totalSize) {
            const progress = Math.min((i + chunkSize) / totalSize, 1);
            const completed = Math.floor(progress * progressBarLength);
            const remaining = progressBarLength - completed;

            readline.cursorTo(process.stdout, 0);
            process.stdout.write(
                `🚀 Progress: [${"=".repeat(completed)}${" ".repeat(remaining)}] ${Math.round(
                    progress * 100
                )}%`
            );
        }
    }

    console.log("\n✅ Splitting complete!");
    return chunks;
}

//Implementation

// const binaryString = "110101110101010101010101010101010101010101010101";
// const chunks = splitBy5(binaryString);
// console.log("Chunks:", chunks);