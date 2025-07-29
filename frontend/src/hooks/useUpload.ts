import { useState, useCallback, useRef } from 'react';
import { uploadFile, validateFile, type UploadProgress, type CompressionResponse, type UploadOptions } from '@/lib/api';

export interface UploadState {
  isUploading: boolean;
  progress: number;
  status: string;
  error: string | null;
  uploadComplete: boolean;
  uploadResponse: CompressionResponse | null;
  canCancel: boolean;
}

export interface UploadActions {
  startUpload: (file: File) => Promise<void>;
  cancelUpload: () => void;
  resetUpload: () => void;
  retryUpload: () => Promise<void>;
}

export const useUpload = (): UploadState & UploadActions => {
  const [state, setState] = useState<UploadState>({
    isUploading: false,
    progress: 0,
    status: 'Ready',
    error: null,
    uploadComplete: false,
    uploadResponse: null,
    canCancel: false,
  });

  const currentFile = useRef<File | null>(null);
  const currentXhr = useRef<XMLHttpRequest | null>(null);

  const updateState = useCallback((updates: Partial<UploadState>) => {
    setState(prev => ({ ...prev, ...updates }));
  }, []);

  const startUpload = useCallback(async (file: File) => {
    // Validate file first
    const validation = validateFile(file);
    if (!validation.valid) {
      updateState({
        error: validation.error!,
        status: 'Validation failed',
        isUploading: false,
      });
      return;
    }

    // Reset state for new upload
    updateState({
      isUploading: true,
      progress: 0,
      status: 'Initializing upload...',
      error: null,
      uploadComplete: false,
      uploadResponse: null,
      canCancel: true,
    });

    currentFile.current = file;

    const uploadOptions: UploadOptions = {
      onProgress: (progress: UploadProgress) => {
        updateState({
          progress: progress.percentage,
          status: `Uploading: ${progress.percentage}%`,
        });
      },
      onError: (error: string) => {
        updateState({
          error,
          status: 'Upload failed',
          isUploading: false,
          canCancel: false,
        });
      },
      onSuccess: (response: CompressionResponse) => {
        updateState({
          uploadComplete: true,
          uploadResponse: response,
          status: 'Upload successful!',
          isUploading: false,
          canCancel: false,
          progress: 100,
        });
      },
      retryCount: 3,
      retryDelay: 1000,
    };

    try {
      await uploadFile(file, uploadOptions);
    } catch (error) {
      // Error is already handled by onError callback
      console.error('Upload error:', error);
    }
  }, [updateState]);

  const cancelUpload = useCallback(() => {
    if (currentXhr.current) {
      currentXhr.current.abort();
      currentXhr.current = null;
    }
    
    updateState({
      isUploading: false,
      status: 'Upload cancelled',
      canCancel: false,
    });
  }, [updateState]);

  const resetUpload = useCallback(() => {
    if (currentXhr.current) {
      currentXhr.current.abort();
      currentXhr.current = null;
    }

    updateState({
      isUploading: false,
      progress: 0,
      status: 'Ready',
      error: null,
      uploadComplete: false,
      uploadResponse: null,
      canCancel: false,
    });

    currentFile.current = null;
  }, [updateState]);

  const retryUpload = useCallback(async () => {
    if (currentFile.current) {
      await startUpload(currentFile.current);
    }
  }, [startUpload]);

  return {
    ...state,
    startUpload,
    cancelUpload,
    resetUpload,
    retryUpload,
  };
}; 