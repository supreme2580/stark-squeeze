import { createInterface } from 'readline';

/**
 * Class to handle the progress bar in the terminal
 */
export class ProgressBar {
    private total: number;
    private current: number;
    private barLength: number;
    private startTime: number;
    private rl: any;

    constructor(total: number, barLength: number = 40) {
        this.total = total;
        this.current = 0;
        this.barLength = barLength;
        this.startTime = Date.now();
        this.rl = createInterface({
            input: process.stdin,
            output: process.stdout
        });
    }

    /**
     * Updates the current progress
     * @param current - The current progress value
     */
    update(current: number) {
        this.current = current;
        this.draw();
    }

    /**
     * Draws the progress bar in the terminal
     */
    private draw() {
        const progress = this.current / this.total;
        const filledLength = Math.floor(progress * this.barLength);
        const emptyLength = this.barLength - filledLength;
        
        const filledBar = '█'.repeat(filledLength);
        const emptyBar = '░'.repeat(emptyLength);
        
        const percentage = Math.round(progress * 100);
        const elapsedTime = Math.max((Date.now() - this.startTime) / 1000, 0.001); // Evitar división por cero
        const speed = this.current / elapsedTime;
        const eta = isFinite(speed) && speed > 0 ? (this.total - this.current) / speed : 0;

        const progressBar = `\r[${filledBar}${emptyBar}] ${percentage}% | ${this.current}/${this.total} | ${speed.toFixed(2)} it/s | ETA: ${eta.toFixed(1)}s`;
        
        this.rl.write(progressBar);
    }

    /**
     * Finishes the progress bar
     */
    finish() {
        this.rl.write('\n');
        this.rl.close();
    }
} 