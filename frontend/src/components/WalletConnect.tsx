import React from 'react';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { useWallet } from '@/contexts/WalletContext';
import { Wallet, LogOut, User, FileText, HardDrive } from 'lucide-react';

const WalletConnect: React.FC = () => {
  const { isConnected, address, userData, userFiles, connectWallet, disconnectWallet } = useWallet();

  const formatAddress = (addr: string) => {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  if (!isConnected) {
    return (
      <Card className="bg-slate-800/80 backdrop-blur-sm border-slate-700 p-6">
        <div className="text-center space-y-4">
          <div className="mx-auto w-16 h-16 bg-blue-500/20 rounded-full flex items-center justify-center">
            <Wallet className="w-8 h-8 text-blue-400" />
          </div>
          <div>
            <h3 className="text-xl font-semibold text-white mb-2">Connect Your Wallet</h3>
            <p className="text-slate-300 mb-4">
              Connect your Starknet wallet to access your files and compression history
            </p>
          </div>
          <Button 
            onClick={connectWallet}
            className="bg-blue-600 hover:bg-blue-700 text-white"
          >
            <Wallet className="w-4 h-4 mr-2" />
            Connect Wallet
          </Button>
        </div>
      </Card>
    );
  }

  return (
    <div className="space-y-6">
      {/* Wallet Info Card */}
      <Card className="bg-slate-800/80 backdrop-blur-sm border-slate-700 p-6">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <Avatar className="w-12 h-12">
              <AvatarFallback className="bg-blue-600 text-white">
                <User className="w-6 h-6" />
              </AvatarFallback>
            </Avatar>
            <div>
              <h3 className="text-lg font-semibold text-white">Connected Wallet</h3>
              <p className="text-slate-300 font-mono text-sm">{formatAddress(address!)}</p>
            </div>
          </div>
          <Button 
            variant="outline" 
            onClick={disconnectWallet}
            className="bg-transparent border-slate-600 text-slate-300 hover:bg-slate-700 hover:text-white hover:border-slate-500"
          >
            <LogOut className="w-4 h-4 mr-2" />
            Disconnect
          </Button>
        </div>
      </Card>

      {/* User Stats */}
      {userData && (
        <Card className="bg-slate-800/80 backdrop-blur-sm border-slate-700 p-6">
          <h4 className="text-lg font-semibold text-white mb-4 text-left">Your Statistics</h4>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="bg-slate-700/50 rounded-lg p-4">
              <div className="flex items-center space-x-3">
                <FileText className="w-5 h-5 text-blue-400" />
                <div className="text-left">
                  <p className="text-slate-300 text-sm">Files Uploaded</p>
                  <p className="text-white font-semibold">{userData.files_count}</p>
                </div>
              </div>
            </div>
            <div className="bg-slate-700/50 rounded-lg p-4">
              <div className="flex items-center space-x-3">
                <HardDrive className="w-5 h-5 text-green-400" />
                <div className="text-left">
                  <p className="text-slate-300 text-sm">Total Size</p>
                  <p className="text-white font-semibold">{formatFileSize(userData.total_uploaded_size)}</p>
                </div>
              </div>
            </div>
            <div className="bg-slate-700/50 rounded-lg p-4">
              <div className="flex items-center space-x-3">
                <Wallet className="w-5 h-5 text-purple-400" />
                <div className="text-left">
                  <p className="text-slate-300 text-sm">Member Since</p>
                  <p className="text-white font-semibold">
                    {new Date(userData.created_at).toLocaleDateString()}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </Card>
      )}

      {/* User Files */}
      {userFiles.length > 0 && (
        <Card className="bg-slate-800/80 backdrop-blur-sm border-slate-700 p-6">
          <h4 className="text-lg font-semibold text-white mb-4 text-left">Your Files</h4>
          <div className="space-y-3">
            {userFiles.map((file, index) => (
              <div key={index} className="bg-slate-700/50 rounded-lg p-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <FileText className="w-5 h-5 text-blue-400" />
                    <div className="text-left">
                      <p className="text-white font-medium">{file.filename}</p>
                      <p className="text-slate-300 text-sm">
                        {formatFileSize(file.original_size)} → {formatFileSize(file.compressed_size)}
                      </p>
                    </div>
                  </div>
                  <div className="flex items-center space-x-2">
                    <Badge variant={file.visibility === 'public' ? 'default' : 'secondary'}>
                      {file.visibility}
                    </Badge>
                    <p className="text-slate-400 text-sm">
                      {new Date(file.created_at).toLocaleDateString()}
                    </p>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </Card>
      )}
    </div>
  );
};

export default WalletConnect; 