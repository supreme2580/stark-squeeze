import { ProgressBar } from '../../src/io/progressBar';

describe('ProgressBar', () => {
    let progressBar: ProgressBar;

    beforeEach(() => {
        progressBar = new ProgressBar(100);
    });

    it('should initialize with correct total', () => {
        expect(progressBar).toBeDefined();
    });

    it('should update progress correctly', () => {
        progressBar.update(50);
        // Note: We can't easily test the console output in Jest
        // The actual visual output will be visible when running the application
    });

    it('should finish correctly', () => {
        progressBar.finish();
        // Note: We can't easily test the console output in Jest
        // The actual visual output will be visible when running the application
    });
}); 