# Cloud Provider Integration

Kit Config provides native support for cloud provider configurations, making it easy to integrate with AWS, GCP, and DigitalOcean services.

## AWS Configuration

The AWS source loads configuration from environment variables or key-value maps:

### Environment Variables
- `AWS_CONFIG_MAP` - JSON string containing AWS configuration key-value pairs
- `AWS_PREFIX` - Optional prefix for AWS environment variables (e.g., "MYAPP_AWS_")

### Usage
```rust
use kit_config::{ConfigLoader, sources::AWSSource};

let config = ConfigLoader::builder()
    .add_source(AWSSource::new())
    .build();
```

## GCP Configuration

The GCP source loads configuration from environment variables or key-value maps:

### Environment Variables
- `GCP_CONFIG_MAP` - JSON string containing GCP configuration key-value pairs
- `GCP_PREFIX` - Optional prefix for GCP environment variables (e.g., "MYAPP_GCP_")

### Usage
```rust
use kit_config::{ConfigLoader, sources::GCPSource};

let config = ConfigLoader::builder()
    .add_source(GCPSource::new())
    .build();
```

## DigitalOcean Configuration

The DO source loads configuration from environment variables or key-value maps:

### Environment Variables
- `DO_CONFIG_MAP` - JSON string containing DigitalOcean configuration key-value pairs
- `DO_PREFIX` - Optional prefix for DigitalOcean environment variables (e.g., "MYAPP_DO_")

### Usage
```rust
use kit_config::{ConfigLoader, sources::DOSource};

let config = ConfigLoader::builder()
    .add_source(DOSource::new())
    .build();
```

## Key-Value Map Support

All cloud provider sources support key-value map configuration through environment variables:

```rust
// Set environment variable with JSON configuration
// export AWS_CONFIG_MAP='{"region": "us-west-2", "profile": "default"}'

use kit_config::{ConfigLoader, sources::AWSSource};

let config = ConfigLoader::builder()
    .add_source(AWSSource::new().with_key_value_map("AWS_CONFIG_MAP"))
    .build();
```

## Variable Prefix Handling

All cloud provider sources support variable prefix handling through the `with_prefix` method:

```rust
use kit_config::{ConfigLoader, sources::AWSSource};

let config = ConfigLoader::builder()
    .add_source(AWSSource::new().with_prefix("MYAPP_AWS_"))
    .build();
```

With the prefix "MYAPP_AWS_", the source will look for environment variables like `MYAPP_AWS_REGION` instead of just `AWS_REGION`.

## Configuration Format

Cloud provider configurations should be valid JSON objects that map configuration keys to values. The values can be strings, numbers, booleans, or nested objects.

## Custom Cloud Configuration

To use custom cloud configurations, set the appropriate environment variable with a JSON string containing your configuration:

```bash
export AWS_CONFIG_MAP='{"region": "us-east-1", "timeout": 30}'
```

The configuration will be automatically parsed and made available through the config loader.