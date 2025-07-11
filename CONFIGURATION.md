# StarkSqueeze Configuration System

## Overview

The StarkSqueeze configuration system provides a centralized way to manage all settings and parameters used throughout the application. Instead of hardcoded values scattered across different modules, all configuration is now stored in a single `config.json` file and accessed through a type-safe configuration module.

## Configuration File Structure

The `config.json` file is organized into logical sections:

### Compression Settings
- **Target compression ratio**: The desired compression percentage (default: 90%)
- **Current compression ratio**: The actual compression being achieved (default: 80%)
- **Chunk size range**: Min/max/default chunk sizes for compression
- **Compression ratios**: Different compression schemes (5:1, 10:1, etc.)

### Dictionary Settings
- **ASCII combinations**: Settings for generating ASCII character dictionaries
- **Ultra-compressed**: Settings for ultra-compressed dictionary generation
- **Generation parameters**: Chunk sizes, flush intervals, performance estimates

### File Processing
- **ASCII conversion**: Chunk sizes and character ranges
- **Binary string conversion**: Format and bit settings

### Upload Settings
- **Hash configuration**: Algorithm and hash lengths
- **StarkNet settings**: Chunk sizes and optimization flags

### Server Configuration
- **Port and host**: Server binding settings
- **Endpoints**: API endpoint paths
- **Dictionary auto-generation**: Automatic dictionary creation

### CLI Settings
- **Progress bars**: Spinner and bar styles
- **Prompts**: Default values for user inputs

### Storage Settings
- **IPFS**: Gateway URLs and environment variables
- **Local storage**: File storage preferences

### Performance Settings
- **Memory management**: Chunk sizes for different operations
- **Compression thresholds**: Performance optimization parameters

### Validation Settings
- **File validation**: Size limits and extension restrictions
- **Compression validation**: Ratio limits and targets

### UI Settings
- **Colors**: Color schemes for different message types
- **Messages**: Standardized message strings

## Usage Examples

### Loading Configuration

```rust
use stark_squeeze::config::{get_config, load_config, save_config};

// Get the global configuration instance
let config = get_config();

// Access compression settings
let target_ratio = config.compression.target_compression_ratio;
let chunk_range = &config.compression.chunk_size_range;

// Access dictionary settings
let default_length = config.dictionary.ascii_combinations.default_length;
let output_file = &config.dictionary.ascii_combinations.output_file;

// Access UI settings
let success_color = &config.ui.colors.success;
let upload_message = &config.ui.messages.upload_complete;
```

### Updating Configuration

```rust
use stark_squeeze::config::{load_config, save_config, Config};

// Load current configuration
let mut config = load_config()?;

// Update settings
config.compression.target_compression_ratio = 95.0;
config.dictionary.ascii_combinations.default_length = 6;

// Save updated configuration
save_config(&config)?;
```

### CLI Integration

The CLI module now uses configuration for:

- Progress bar styles and templates
- Default values for user prompts
- UI colors and messages
- Performance settings

Example from the CLI:

```rust
let config = get_config();

// Use configured spinner style
let tick_strings: Vec<&str> = config.cli.progress.spinner_style.tick_strings
    .iter().map(|s| s.as_str()).collect();

spinner.set_style(
    ProgressStyle::default_spinner()
        .tick_strings(&tick_strings)
        .template(&config.cli.progress.spinner_style.template)
        .unwrap(),
);

// Use configured messages
spinner.finish_with_message(config.ui.messages.upload_complete.green().to_string());
```

## Configuration Migration

### Before (Hardcoded Values)

```rust
// Old way - hardcoded values scattered throughout code
let chunk_size = 5;
let compression_ratio = 80.0;
let spinner_style = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
let message = "Upload complete!";
```

### After (Configuration-Driven)

```rust
// New way - centralized configuration
let config = get_config();
let chunk_size = config.compression.chunk_size_range.default;
let compression_ratio = config.compression.current_compression_ratio;
let spinner_style = &config.cli.progress.spinner_style.tick_strings;
let message = &config.ui.messages.upload_complete;
```

## Benefits

1. **Centralized Management**: All settings in one place
2. **Type Safety**: Compile-time checking of configuration access
3. **Flexibility**: Easy to modify settings without code changes
4. **Consistency**: Uniform values across all modules
5. **Maintainability**: No more hunting for hardcoded values
6. **Documentation**: Self-documenting configuration structure

## Default Configuration

If `config.json` is not found, the system will:

1. Print a warning message
2. Use built-in default values
3. Continue operation normally

The default configuration provides sensible defaults for all settings.

## Configuration Validation

The configuration system includes:

- **Type checking**: All values are properly typed
- **Range validation**: Numeric values have appropriate ranges
- **Required fields**: Essential settings are always present
- **Fallback values**: Graceful degradation when settings are missing

## Performance Considerations

- **Lazy loading**: Configuration is loaded once and cached
- **Minimal overhead**: Configuration access is optimized
- **Memory efficient**: Shared configuration instance

## Future Enhancements

Planned improvements include:

- **Environment variable overrides**: Allow env vars to override config
- **Profile support**: Multiple configuration profiles
- **Hot reloading**: Runtime configuration updates
- **Validation schemas**: JSON schema validation
- **Configuration UI**: Web interface for editing settings

## Troubleshooting

### Common Issues

1. **Configuration not found**: Check that `config.json` exists in the project root
2. **Parse errors**: Validate JSON syntax in the configuration file
3. **Type mismatches**: Ensure configuration values match expected types
4. **Missing fields**: Add required configuration sections

### Debugging

Enable debug output to see configuration loading:

```rust
use stark_squeeze::config::load_config;

match load_config() {
    Ok(config) => println!("Configuration loaded successfully"),
    Err(e) => eprintln!("Configuration error: {}", e),
}
```

## Contributing

When adding new features:

1. **Add configuration**: Define new settings in the config structure
2. **Update defaults**: Provide sensible default values
3. **Document usage**: Add examples to this documentation
4. **Test integration**: Ensure configuration is used consistently

This configuration system ensures that StarkSqueeze is flexible, maintainable, and easy to customize for different use cases. 