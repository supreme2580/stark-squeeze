export interface UploadProgress {
  loaded: number;
  total: number;
  percentage: number;
}

export interface CompressionResponse {
  success: boolean;
  file_url?: string;
  ipfs_cid?: string;
  compression_ratio?: number;
  original_size?: number;
  compressed_size?: number;
  error?: string;
  mapping_file?: string;
  upload_timestamp?: number;
  file_type?: string;
}

export interface ServerStatus {
  status: string;
  dictionary_loaded: boolean;
  dictionary_size?: number;
  uptime: string;
  total_files_processed: number;
}

export interface UploadOptions {
  onProgress?: (progress: UploadProgress) => void;
  onError?: (error: string) => void;
  onSuccess?: (response: CompressionResponse) => void;
  retryCount?: number;
  retryDelay?: number;
}

const API_BASE = import.meta.env.VITE_API_BASE_URL || 'https://stark-squeeze.onrender.com';

class ApiError extends Error {
  constructor(
    message: string,
    public status?: number,
    public response?: unknown
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

// File validation
export const validateFile = (file: File): { valid: boolean; error?: string } => {
  // Check file size (500MB limit)
  const maxSize = 500 * 1024 * 1024; // 500MB
  if (file.size > maxSize) {
    return {
      valid: false,
      error: `File size exceeds 500MB limit. Current size: ${(file.size / 1024 / 1024).toFixed(2)}MB`
    };
  }

  // Check file type
  const allowedTypes = [
    'application/pdf',
    'application/msword',
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    'text/plain',
    'application/json',
    'text/csv',
    'application/xml',
    'text/xml'
  ];

  const allowedExtensions = ['.pdf', '.doc', '.docx', '.txt', '.json', '.csv', '.xml'];

  const hasValidType = allowedTypes.includes(file.type);
  const hasValidExtension = allowedExtensions.some(ext => 
    file.name.toLowerCase().endsWith(ext)
  );

  if (!hasValidType && !hasValidExtension) {
    return {
      valid: false,
      error: `Unsupported file type. Allowed types: ${allowedExtensions.join(', ')}`
    };
  }

  return { valid: true };
};

// Check server status
export const checkServerStatus = async (): Promise<ServerStatus> => {
  try {
    const response = await fetch(`${API_BASE}/status`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
      mode: 'cors',
    });
    if (!response.ok) {
      throw new ApiError(`Server responded with ${response.status}`, response.status);
    }
    return await response.json();
  } catch (error) {
    if (error instanceof ApiError) {
      throw error;
    }
    // Check if it's a CORS error
    if (error instanceof TypeError && error.message.includes('Failed to fetch')) {
      throw new ApiError('CORS error: Server is not configured to allow cross-origin requests. Please contact the server administrator.');
    }
    throw new ApiError('Failed to connect to server. Please check if the server is running.');
  }
};

// Upload file with real progress tracking
export const uploadFile = async (
  file: File,
  options: UploadOptions = {}
): Promise<CompressionResponse> => {
  const {
    onProgress,
    onError,
    onSuccess,
    retryCount = 3,
    retryDelay = 1000
  } = options;

  // Validate file first
  const validation = validateFile(file);
  if (!validation.valid) {
    const error = validation.error!;
    onError?.(error);
    throw new ApiError(error);
  }

  const uploadWithRetry = async (attempt: number): Promise<CompressionResponse> => {
    try {
      const formData = new FormData();
      formData.append('file', file);

      const xhr = new XMLHttpRequest();

      return new Promise((resolve, reject) => {
        xhr.upload.addEventListener('progress', (event) => {
          if (event.lengthComputable) {
            const progress: UploadProgress = {
              loaded: event.loaded,
              total: event.total,
              percentage: Math.round((event.loaded / event.total) * 100)
            };
            onProgress?.(progress);
          }
        });

        xhr.addEventListener('load', () => {
          if (xhr.status >= 200 && xhr.status < 300) {
            try {
              const response: CompressionResponse = JSON.parse(xhr.responseText);
              if (response.success) {
                onSuccess?.(response);
                resolve(response);
              } else {
                const error = response.error || 'Upload failed';
                onError?.(error);
                reject(new ApiError(error, xhr.status, response));
              }
            } catch (parseError) {
              const error = 'Failed to parse server response';
              onError?.(error);
              reject(new ApiError(error, xhr.status));
            }
          } else {
            let errorMessage = 'Upload failed';
            try {
              const errorResponse = JSON.parse(xhr.responseText);
              errorMessage = errorResponse.error || errorMessage;
            } catch {
              errorMessage = `Server error: ${xhr.status}`;
            }
            onError?.(errorMessage);
            reject(new ApiError(errorMessage, xhr.status));
          }
        });

        xhr.addEventListener('error', () => {
          const error = 'Network error occurred during upload';
          onError?.(error);
          reject(new ApiError(error));
        });

        xhr.addEventListener('abort', () => {
          const error = 'Upload was cancelled';
          onError?.(error);
          reject(new ApiError(error));
        });

        xhr.open('POST', `${API_BASE}/compress`);
        // Add CORS headers for cross-origin requests
        xhr.withCredentials = false;
        xhr.send(formData);
      });
    } catch (error) {
      if (attempt < retryCount) {
        // Wait before retrying
        await new Promise(resolve => setTimeout(resolve, retryDelay));
        return uploadWithRetry(attempt + 1);
      }
      throw error;
    }
  };

  return uploadWithRetry(1);
};

// Cancel upload (if needed for future implementation)
export const cancelUpload = (xhr: XMLHttpRequest) => {
  xhr.abort();
};

// Download compressed file
export const downloadCompressedFile = async (fileId: string): Promise<Blob> => {
  try {
    const response = await fetch(`${API_BASE}/files/${fileId}`, {
      method: 'GET',
      mode: 'cors',
    });
    if (!response.ok) {
      throw new ApiError(`Download failed: ${response.status}`, response.status);
    }
    return await response.blob();
  } catch (error) {
    if (error instanceof ApiError) {
      throw error;
    }
    throw new ApiError('Failed to download file');
  }
};

// Health check
export const healthCheck = async (): Promise<boolean> => {
  try {
    const response = await fetch(`${API_BASE}/health`, {
      method: 'GET',
      mode: 'cors',
    });
    return response.ok;
  } catch (error) {
    console.error('Health check failed:', error);
    
    // Try without CORS mode as fallback
    try {
      const response = await fetch(`${API_BASE}/health`, {
        method: 'GET',
      });
      return response.ok;
    } catch (fallbackError) {
      console.error('Fallback health check also failed:', fallbackError);
      return false;
    }
  }
}; 