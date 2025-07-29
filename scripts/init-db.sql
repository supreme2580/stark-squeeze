-- Stark Squeeze Database Initialization Script
-- This script sets up the initial database schema for the application

-- Create extensions if needed
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create tables for user data
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet_address VARCHAR(66) UNIQUE NOT NULL,
    files_count INTEGER DEFAULT 0,
    total_uploaded_size BIGINT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create tables for file data
CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    file_hash VARCHAR(66) NOT NULL,
    wallet_address VARCHAR(66) NOT NULL REFERENCES users(wallet_address),
    filename VARCHAR(255) NOT NULL,
    original_size BIGINT NOT NULL,
    compressed_size BIGINT NOT NULL,
    file_format VARCHAR(10) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    visibility VARCHAR(10) DEFAULT 'public' CHECK (visibility IN ('public', 'private')),
    ipfs_hash VARCHAR(66),
    compression_ratio DECIMAL(5,2)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_files_wallet_address ON files(wallet_address);
CREATE INDEX IF NOT EXISTS idx_files_created_at ON files(created_at);
CREATE INDEX IF NOT EXISTS idx_users_wallet_address ON users(wallet_address);

-- Create a function to update user statistics
CREATE OR REPLACE FUNCTION update_user_stats()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE users 
        SET files_count = files_count + 1,
            total_uploaded_size = total_uploaded_size + NEW.original_size,
            last_activity = CURRENT_TIMESTAMP
        WHERE wallet_address = NEW.wallet_address;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE users 
        SET files_count = files_count - 1,
            total_uploaded_size = total_uploaded_size - OLD.original_size,
            last_activity = CURRENT_TIMESTAMP
        WHERE wallet_address = OLD.wallet_address;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update user statistics
DROP TRIGGER IF EXISTS trigger_update_user_stats ON files;
CREATE TRIGGER trigger_update_user_stats
    AFTER INSERT OR DELETE ON files
    FOR EACH ROW
    EXECUTE FUNCTION update_user_stats();

-- Insert some sample data for testing
INSERT INTO users (wallet_address, files_count, total_uploaded_size) 
VALUES ('0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef', 2, 3145728)
ON CONFLICT (wallet_address) DO NOTHING;

INSERT INTO files (file_hash, wallet_address, filename, original_size, compressed_size, file_format, visibility, compression_ratio)
VALUES 
    ('0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef', '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef', 'document.pdf', 2048576, 512000, 'pdf', 'public', 75.0),
    ('0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890', '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef', 'presentation.pptx', 10485760, 2097152, 'pptx', 'private', 80.0)
ON CONFLICT DO NOTHING; 