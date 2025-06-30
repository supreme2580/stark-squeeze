import { useState } from 'react';
import { Upload, FileText, CheckCircle, Zap } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';

const Index = () => {
  const [dragActive, setDragActive] = useState(false);
  const [uploadedFile, setUploadedFile] = useState<File | null>(null);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [isUploading, setIsUploading] = useState(false);
  const [uploadComplete, setUploadComplete] = useState(false);
  const [txHash] = useState('0x1234...abcd'); // Placeholder for now
  const [status, setStatus] = useState('Ready');

  const handleDrag = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true);
    } else if (e.type === "dragleave") {
      setDragActive(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);

    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      const file = e.dataTransfer.files[0];
      if (file.type === 'application/pdf' || file.name.endsWith('.doc') || file.name.endsWith('.docx')) {
        setUploadedFile(file);
        simulateUpload();
      }
    }
  };

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const file = e.target.files[0];
      if (file.type === 'application/pdf' || file.name.endsWith('.doc') || file.name.endsWith('.docx')) {
        setUploadedFile(file);
        simulateUpload();
      }
    }
  };

  const simulateUpload = () => {
    setIsUploading(true);
    setUploadComplete(false);
    setStatus('Initializing compression...');
    setUploadProgress(0);

    const statuses = [
      'Analyzing file structure...',
      'Applying Stark compression...',
      'Optimizing data blocks...',
      'Finalizing compression...',
      'Verification complete!'
    ];

    const interval = setInterval(() => {
      setUploadProgress((prev) => {
        const newProgress = prev + 20;
        if (newProgress >= 100) {
          clearInterval(interval);
          setIsUploading(false);
          setUploadComplete(true);
          setStatus('Compression successful!');
          return 100;
        }
        const statusIndex = Math.floor(newProgress / 20) - 1;
        setStatus(statuses[statusIndex] || `Processing: ${newProgress}%`);
        return newProgress;
      });
    }, 800);
  };

  const resetUpload = () => {
    setUploadedFile(null);
    setUploadProgress(0);
    setIsUploading(false);
    setUploadComplete(false);
    setStatus('Ready');
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 flex flex-col items-center justify-center p-4">
      <div className="w-full max-w-4xl mx-auto">
        {/* Logo and Title Section */}
        <div className="text-center mb-8">
          <div className="flex items-center justify-center gap-3 mb-4">
            <div className="p-3 bg-blue-600 rounded-xl">
              <Zap className="w-8 h-8 text-white" />
            </div>
            <h1 className="text-5xl font-bold text-white">Stark Squeeze</h1>
          </div>
          
          {/* Introduction Section */}
          <div className="max-w-2xl mx-auto mb-8">
            <h2 className="text-2xl font-semibold text-blue-300 mb-4">Advanced File Compression System</h2>
            <p className="text-slate-300 text-lg leading-relaxed mb-4">
              Experience next-generation file compression with Stark Squeeze. Our advanced compression algorithms 
              reduce file sizes by up to 90% while maintaining perfect data integrity and lightning-fast processing speeds.
            </p>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
              <div className="bg-slate-800/50 rounded-lg p-3">
                <div className="text-blue-400 font-semibold">Ultra-Fast</div>
                <div className="text-slate-300">Process files in seconds</div>
              </div>
              <div className="bg-slate-800/50 rounded-lg p-3">
                <div className="text-green-400 font-semibold">Lossless</div>
                <div className="text-slate-300">100% data preservation</div>
              </div>
              <div className="bg-slate-800/50 rounded-lg p-3">
                <div className="text-purple-400 font-semibold">Secure</div>
                <div className="text-slate-300">Blockchain verification</div>
              </div>
            </div>
          </div>
        </div>

        <div className="text-center mb-6">
          <h3 className="text-2xl font-bold text-white mb-2">Upload Your File</h3>
          <p className="text-slate-300">Drag and drop your file to begin compression. Supports .pdf, .doc, and .docx formats.</p>
        </div>

        <Card className="bg-slate-800/80 backdrop-blur-sm border-slate-700 p-8">
          <div
            className={`relative border-2 border-dashed rounded-xl p-12 text-center transition-all duration-300 ${
              dragActive
                ? 'border-blue-400 bg-blue-400/10'
                : uploadComplete
                ? 'border-green-400 bg-green-400/10'
                : 'border-slate-600 hover:border-slate-500'
            }`}
            onDragEnter={handleDrag}
            onDragLeave={handleDrag}
            onDragOver={handleDrag}
            onDrop={handleDrop}
          >
            <input
              type="file"
              accept=".pdf,.doc,.docx"
              onChange={handleFileSelect}
              className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              disabled={isUploading}
            />

            <div className="space-y-4">
              {!uploadedFile ? (
                <>
                  <div className="mx-auto w-16 h-16 bg-blue-500/20 rounded-full flex items-center justify-center">
                    <Upload className="w-8 h-8 text-blue-400" />
                  </div>
                  <div>
                    <h3 className="text-xl font-semibold text-white mb-2">Drag & Drop</h3>
                    <p className="text-slate-300 mb-2">or <span className="text-blue-400 underline cursor-pointer">choose a file</span></p>
                    <p className="text-sm text-slate-400">Maximum file size 500MB • See more requirements</p>
                  </div>
                </>
              ) : (
                <div className="space-y-4">
                  <div className="mx-auto w-16 h-16 bg-purple-500/20 rounded-full flex items-center justify-center">
                    {uploadComplete ? (
                      <CheckCircle className="w-8 h-8 text-green-400" />
                    ) : (
                      <FileText className="w-8 h-8 text-purple-400" />
                    )}
                  </div>
                  <div>
                    <h3 className="text-xl font-semibold text-white mb-2">
                      {uploadComplete ? 'Success!' : isUploading ? 'Uploading...' : 'File Selected'}
                    </h3>
                    <p className="text-slate-300">{uploadedFile.name}</p>
                  </div>
                  {isUploading && (
                    <div className="w-full max-w-xs mx-auto">
                      <Progress value={uploadProgress} className="h-2" />
                      <p className="text-sm text-slate-400 mt-2">{uploadProgress}%</p>
                    </div>
                  )}
                </div>
              )}
            </div>
          </div>

          <div className="mt-8 space-y-6">
            {uploadedFile && (
              <div className="bg-slate-750 rounded-xl p-6 border border-slate-600">
                <div className="flex items-center justify-between mb-4">
                  <h4 className="text-lg font-semibold text-white">Compression Status</h4>
                  {uploadComplete && (
                    <div className="flex items-center gap-2 text-green-400">
                      <CheckCircle className="w-5 h-5" />
                      <span className="font-medium">Complete</span>
                    </div>
                  )}
                </div>
                
                <div className="space-y-3">
                  <div className="flex justify-between text-sm">
                    <span className="text-slate-300">Progress</span>
                    <span className="text-white font-mono">{uploadProgress}%</span>
                  </div>
                  
                  <div className="relative">
                    <Progress 
                      value={uploadProgress} 
                      className="h-3 bg-slate-700"
                    />
                    {uploadComplete && (
                      <div className="absolute right-1 top-1/2 transform -translate-y-1/2">
                        <CheckCircle className="w-4 h-4 text-green-400" />
                      </div>
                    )}
                  </div>
                  
                  <div className="flex justify-between items-center">
                    <span className="text-slate-400 text-sm">{status}</span>
                    {isUploading && (
                      <div className="flex space-x-1">
                        <div className="w-2 h-2 bg-blue-400 rounded-full animate-pulse"></div>
                        <div className="w-2 h-2 bg-blue-400 rounded-full animate-pulse" style={{animationDelay: '0.2s'}}></div>
                        <div className="w-2 h-2 bg-blue-400 rounded-full animate-pulse" style={{animationDelay: '0.4s'}}></div>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            )}

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <label className="text-sm font-medium text-slate-300 flex items-center gap-2">
                  <div className="w-4 h-4 bg-blue-500 rounded-full flex items-center justify-center">
                    <div className="w-2 h-2 bg-white rounded-full"></div>
                  </div>
                  Blockchain Verification
                </label>
                <div className="bg-slate-700/50 rounded-lg p-3">
                  <p className="text-white font-mono text-sm">Transaction Hash</p>
                  <p className="text-slate-300 text-sm">{uploadedFile ? txHash : '—'}</p>
                </div>
              </div>

              <div className="space-y-2">
                <label className="text-sm font-medium text-slate-300 flex items-center gap-2">
                  <div className="w-4 h-4 bg-green-500 rounded-full flex items-center justify-center">
                    <div className="w-2 h-2 bg-white rounded-full"></div>
                  </div>
                  File Information
                </label>
                <div className="bg-slate-700/50 rounded-lg p-3">
                  <p className="text-white font-mono text-sm">File Size</p>
                  <p className="text-slate-300 text-sm">
                    {uploadedFile ? `${(uploadedFile.size / 1024 / 1024).toFixed(2)} MB` : '—'}
                  </p>
                </div>
              </div>
            </div>

            <div className="flex justify-end gap-3 pt-4">
              <Button 
                variant="outline" 
                onClick={resetUpload}
                className="bg-slate-700 border-slate-600 text-white hover:bg-slate-600"
              >
                Reset
              </Button>
              <Button 
                className="bg-blue-600 hover:bg-blue-700 text-white"
                disabled={!uploadedFile || isUploading}
              >
                {uploadComplete ? 'Download Compressed File' : 'Start Compression'}
              </Button>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
};

export default Index;
