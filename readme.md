# lib_utils

This is library with handy functions used across my projects

## Features

There are some features you can enable:

- **b32** - This is a base32 encoding/decoding library
- **b58** - This is a base58 encoding/decoding library
- **b64** - This is a base64 encoding/decoding library
- **cuuid** - This is a uuid library that encodes/decodes uuid to/from BaseX string, requires also one of the following features: **b32**, **b58**, **b64**
- **envs** - This is a library that loads environment variables into config, awailable across the project
- **time** - This is a library that provides UTC time functions

By default enabled features are: **envs**

## Usage

Add this into your `Cargo.toml`

```toml
[dependencies]
grapple_utils = { version = "0.1.0", features = ["envs", "time", "b32", "b58", "b64", "cuuid"] }
`
```

## Examples

There are some examples in `examples` folder.  
Some of them requires features, so run them with:

```bash
cargo run --example <example_name> --features <feature_name>
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
