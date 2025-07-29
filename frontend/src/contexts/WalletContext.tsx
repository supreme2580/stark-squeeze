import React, { createContext, useContext, useEffect, useState } from 'react';

// Mock data types
interface UserData {
  wallet_address: string;
  files_count: number;
  total_uploaded_size: number;
  created_at: number;
  last_activity: number;
}

interface FileData {
  file_hash: string;
  wallet_address: string;
  filename: string;
  original_size: number;
  compressed_size: number;
  file_format: string;
  created_at: number;
  visibility: 'public' | 'private';
}

interface WalletContextType {
  isConnected: boolean;
  address: string | undefined;
  userData: UserData | null;
  userFiles: FileData[];
  connectWallet: () => Promise<void>;
  disconnectWallet: () => void;
  refreshUserData: () => void;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

// Mock data for demonstration
const mockUserData: UserData = {
  wallet_address: '',
  files_count: 0,
  total_uploaded_size: 0,
  created_at: Date.now(),
  last_activity: Date.now(),
};

const mockUserFiles: FileData[] = [
  {
    file_hash: '0x1234567890abcdef',
    wallet_address: '',
    filename: 'document.pdf',
    original_size: 2048576,
    compressed_size: 512000,
    file_format: 'pdf',
    created_at: Date.now() - 86400000,
    visibility: 'public',
  },
  {
    file_hash: '0xabcdef1234567890',
    wallet_address: '',
    filename: 'presentation.pptx',
    original_size: 10485760,
    compressed_size: 2097152,
    file_format: 'pptx',
    created_at: Date.now() - 172800000,
    visibility: 'private',
  },
];

export const WalletProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isConnected, setIsConnected] = useState(false);
  const [address, setAddress] = useState<string | undefined>(undefined);
  const [userData, setUserData] = useState<UserData | null>(null);
  const [userFiles, setUserFiles] = useState<FileData[]>([]);

  const connectWallet = async () => {
    try {
      // Mock wallet connection - in real implementation, this would connect to actual wallet
      const mockAddress = '0x' + Math.random().toString(16).substr(2, 40);
      setAddress(mockAddress);
      setIsConnected(true);
      
      // Update user data with connected wallet address
      const updatedUserData = {
        ...mockUserData,
        wallet_address: mockAddress,
        last_activity: Date.now(),
      };
      setUserData(updatedUserData);

      // Update user files with connected wallet address
      const updatedUserFiles = mockUserFiles.map(file => ({
        ...file,
        wallet_address: mockAddress,
      }));
      setUserFiles(updatedUserFiles);
    } catch (error) {
      console.error('Failed to connect wallet:', error);
    }
  };

  const disconnectWallet = () => {
    setIsConnected(false);
    setAddress(undefined);
    setUserData(null);
    setUserFiles([]);
  };

  const refreshUserData = () => {
    if (address) {
      const updatedUserData = {
        ...mockUserData,
        wallet_address: address,
        last_activity: Date.now(),
      };
      setUserData(updatedUserData);

      const updatedUserFiles = mockUserFiles.map(file => ({
        ...file,
        wallet_address: address,
      }));
      setUserFiles(updatedUserFiles);
    }
  };

  const value: WalletContextType = {
    isConnected,
    address,
    userData,
    userFiles,
    connectWallet,
    disconnectWallet,
    refreshUserData,
  };

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (context === undefined) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
}; 