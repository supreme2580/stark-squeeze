import { useState, useEffect } from 'react';
import { Upload, FileText, CheckCircle, Zap, AlertCircle, X, RotateCcw, Wifi, WifiOff } from 'lucide-react';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import WalletConnect from '@/components/WalletConnect';
import { useWallet } from '@/contexts/WalletContext';
import { useUpload } from '@/hooks/useUpload';
import { useToast } from '@/hooks/use-toast';
import { validateFile, checkServerStatus, healthCheck } from '@/lib/api';

const Index = () => {
  const { isConnected } = useWallet();
  const { toast } = useToast();
  const [dragActive, setDragActive] = useState(false);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [validationError, setValidationError] = useState<string | null>(null);
  const [serverStatus, setServerStatus] = useState<'connected' | 'disconnected' | 'checking'>('checking');
  
  const {
    isUploading,
    progress,
    status,
    error,
    uploadComplete,
    uploadResponse,
    canCancel,
    startUpload,
    cancelUpload,
    resetUpload,
    retryUpload,
  } = useUpload();

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
      handleFileSelection(file);
    }
  };

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const file = e.target.files[0];
      handleFileSelection(file);
    }
  };

  const handleFileSelection = (file: File) => {
    // Clear previous errors
    setValidationError(null);
    
    // Validate file
    const validation = validateFile(file);
    if (!validation.valid) {
      setValidationError(validation.error!);
      toast({
        title: "File Validation Failed",
        description: validation.error!,
        variant: "destructive",
      });
      return;
    }

    setSelectedFile(file);
    
    // Show success toast
    toast({
      title: "File Selected",
      description: `${file.name} (${(file.size / 1024 / 1024).toFixed(2)}MB)`,
      variant: "success",
    });
  };

  const handleStartUpload = async () => {
    if (!selectedFile) return;

    try {
      await startUpload(selectedFile);
    } catch (error) {
      console.error('Upload failed:', error);
    }
  };

  // Check server status on component mount
  useEffect(() => {
    const checkServer = async () => {
      try {
        setServerStatus('checking');
        const isHealthy = await healthCheck();
        if (isHealthy) {
          setServerStatus('connected');
          toast({
            title: "Server Connected",
            description: "Successfully connected to compression server",
            variant: "success",
          });
        } else {
          setServerStatus('disconnected');
          toast({
            title: "Server Disconnected",
            description: "Unable to connect to compression server",
            variant: "destructive",
          });
        }
      } catch (error) {
        setServerStatus('disconnected');
        toast({
          title: "Server Error",
          description: "Failed to connect to compression server",
          variant: "destructive",
        });
      }
    };

    checkServer();
  }, [toast]);

  // Show toast notifications for upload state changes
  useEffect(() => {
    if (uploadComplete && uploadResponse) {
      toast({
        title: "Upload Successful!",
        description: `File compressed successfully. Compression ratio: ${uploadResponse.compression_ratio?.toFixed(1)}%`,
        variant: "success",
      });
    }
  }, [uploadComplete, uploadResponse, toast]);

  useEffect(() => {
    if (error) {
      toast({
        title: "Upload Failed",
        description: error,
        variant: "destructive",
      });
    }
  }, [error, toast]);

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
          
          {/* Wallet Connection Section */}
          <div className="mb-8">
            <WalletConnect />
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
          <div className="flex items-center justify-center gap-2 mb-4">
            <div className={`flex items-center gap-2 px-3 py-1 rounded-full text-sm ${
              serverStatus === 'connected' 
                ? 'bg-green-500/20 text-green-400 border border-green-500/30'
                : serverStatus === 'disconnected'
                ? 'bg-red-500/20 text-red-400 border border-red-500/30'
                : 'bg-yellow-500/20 text-yellow-400 border border-yellow-500/30'
            }`}>
              {serverStatus === 'connected' ? (
                <Wifi className="w-4 h-4" />
              ) : serverStatus === 'disconnected' ? (
                <WifiOff className="w-4 h-4" />
              ) : (
                <div className="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin" />
              )}
              <span className="font-medium">
                {serverStatus === 'connected' ? 'Server Connected' : 
                 serverStatus === 'disconnected' ? 'Server Disconnected' : 'Checking Connection'}
              </span>
            </div>
          </div>
          
          <h3 className="text-2xl font-bold text-white mb-2">Upload Your File</h3>
          <p className="text-slate-300">
            {isConnected 
              ? "Drag and drop your file to begin compression. Supports .pdf, .doc, and .docx formats."
              : "Please connect your wallet to upload and compress files."
            }
          </p>
        </div>

        <Card className="bg-slate-800/80 backdrop-blur-sm border-slate-700 p-8">
          <div
            className={`relative border-2 border-dashed rounded-xl p-12 text-center transition-all duration-300 ${
              dragActive
                ? 'border-blue-400 bg-blue-400/10'
                : uploadComplete
                ? 'border-green-400 bg-green-400/10'
                : 'border-slate-600 hover:border-slate-500'
            } ${!isConnected ? 'opacity-50 cursor-not-allowed' : ''}`}
            onDragEnter={isConnected ? handleDrag : undefined}
            onDragLeave={isConnected ? handleDrag : undefined}
            onDragOver={isConnected ? handleDrag : undefined}
            onDrop={isConnected ? handleDrop : undefined}
          >
            <input
              type="file"
              accept=".pdf,.doc,.docx"
              onChange={handleFileSelect}
              className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              disabled={isUploading || !isConnected}
            />

            <div className="space-y-4">
              {!selectedFile ? (
                <>
                  <div className="mx-auto w-16 h-16 bg-blue-500/20 rounded-full flex items-center justify-center">
                    <Upload className="w-8 h-8 text-blue-400" />
                  </div>
                  <div>
                    <h3 className="text-xl font-semibold text-white mb-2">
                      {isConnected ? 'Drag & Drop' : 'Connect Wallet First'}
                    </h3>
                    <p className="text-slate-300 mb-2">
                      {isConnected 
                        ? <>or <span className="text-blue-400 underline cursor-pointer">choose a file</span></>
                        : 'Connect your wallet to start compressing files'
                      }
                    </p>
                    <p className="text-sm text-slate-400">
                      {isConnected ? 'Maximum file size 500MB • See more requirements' : 'Wallet connection required'}
                    </p>
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
                    <p className="text-slate-300">{selectedFile.name}</p>
                  </div>
                  {isUploading && (
                    <div className="w-full max-w-xs mx-auto">
                      <Progress value={progress} className="h-2" />
                      <p className="text-sm text-slate-400 mt-2">{progress}%</p>
                    </div>
                  )}
                </div>
              )}
            </div>
          </div>

          <div className="mt-8 space-y-6">
            {selectedFile && (
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
                    <span className="text-white font-mono">{progress}%</span>
                  </div>
                  
                  <div className="relative">
                    <Progress 
                      value={progress} 
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
                  <p className="text-slate-300 text-sm">
                    {uploadResponse?.ipfs_cid ? `IPFS: ${uploadResponse.ipfs_cid.slice(0, 10)}...` : '—'}
                  </p>
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
                    {selectedFile ? `${(selectedFile.size / 1024 / 1024).toFixed(2)} MB` : '—'}
                  </p>
                </div>
              </div>
            </div>

            {/* Error Display */}
            {error && (
              <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-4">
                <div className="flex items-center gap-2 text-red-400 mb-2">
                  <AlertCircle className="w-5 h-5" />
                  <span className="font-medium">Upload Error</span>
                </div>
                <p className="text-red-300 text-sm">{error}</p>
                <div className="flex gap-2 mt-3">
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={retryUpload}
                    className="bg-red-500/20 border-red-500/30 text-red-300 hover:bg-red-500/30"
                  >
                    <RotateCcw className="w-4 h-4 mr-2" />
                    Retry
                  </Button>
                </div>
              </div>
            )}

            {/* Action Buttons */}
            <div className="flex justify-end gap-3 pt-4">
              <Button 
                variant="outline" 
                onClick={resetUpload}
                className="bg-slate-700 border-slate-600 text-white hover:bg-slate-600"
              >
                Reset
              </Button>
              
              {isUploading && canCancel && (
                <Button
                  variant="outline"
                  onClick={cancelUpload}
                  className="bg-red-500/20 border-red-500/30 text-red-300 hover:bg-red-500/30"
                >
                  <X className="w-4 h-4 mr-2" />
                  Cancel
                </Button>
              )}
              
              <Button 
                className="bg-blue-600 hover:bg-blue-700 text-white"
                disabled={!selectedFile || isUploading || !isConnected || serverStatus !== 'connected'}
                onClick={handleStartUpload}
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
