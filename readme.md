# lib_utils

This is library with handy functions used across my projects

## Features

There are some features you can enable:

- **b32** - This is a base32 encoding/decoding library
- **b58** - This is a base58 encoding/decoding library
- **b64** - This is a base64 encoding/decoding library
- **cuuid** - This is a uuid library that encodes/decodes uuid to/from BaseX string, requires also one of the following features: **b32**, **b58**, **b64**
- **envs** - This is a library that loads environment variables. With it you can easily load configs
- **time** - This is a library that provides UTC time functions

By default enabled features are: **envs**

## Usage

Add this into your `Cargo.toml`

```toml
[dependencies]
grapple_utils = { version = "0.2.0", features = ["envs", "time", "b32", "b58", "b64", "cuuid"] }
```

### BaseX encoding-decoding

All schemes shares the same interface, so you just enable requested BaseX as feature, for example `b64`  
After this all you need is use it like this:

```rust
use grapple_utils::b64u; // or b64 for not URL-safe

let b64_str = b64u::encode("my string");
let decoded_bytes = b64u::decode(&b64_str)?;
let decoded_str = b64u::decode_to_string(&b64_str)?;
```

### Uuid BaseX encofing-decoding

In order to use it, you need to add one of the baseX features and `cuuid` feature as well

From existing Uuid

```rust
use grapple_utils::cuuid::CUuid;
use uuid::Uuid;

// Create id
let id_v4 = Uuid::new_v4();
let id_v7 = Uuid::now_v7();

// Encode
let encoded: String = CUuid::B64.from(id_v4);
let encoded: String = CUuid::B64.from(id_v7);
```

Create and encode

```rust
use grapple_utils::cuuid::CUuid;
use uuid::Uuid;

// Encode
let encoded: String = CUuid::B64.new_v4();
let encoded: String = CUuid::B64.now_v7();
```

Decode

```rust
use grapple_utils::cuuid::CUuid;
use uuid::Uuid;

let encoded: String = CUuid::B64.new_v4();
let id: Uuid = CUuid::B64.decode(&encoded);
```

### Environment variables reading

For development you can create `./.cargo/config.toml` file and set your variables there

```toml
# config.toml

[env]
# For single value
MY_VALUE = "my value"

# For array
MY_ARRAY = "ENV1:VALUE1,ENV2:VALUE2"
```

Single value loading

```rust
use grapple_utils::envs;

// If you just need to read string, use this:
let value: String = envs::get("YOUR_ENV")?;

// You also can parse value from string. Type must implement `FromStr` trait
let value: i32 = envs::get_parse("YOUR_ENV")?;

// If your value is BaseX encoded string, you can read it as
// -- BaseX decoded string bytes. Add feature `b64` to enable method
let value: Vec<u8> = envs::get_b64u_as_u8s("YOUR_ENV")?;

// -- BaseX decoded string. Add feature `b64` to enable method
let value: String = envs::get_b64u_as_s("YOUR_ENV")?;
```

Array of values loading

```rust
use grapple_utils::envs;

// If you need to load keys from configuration, or just an array of values, use this
let keys: HashMap<String, String> = envs::get_keys("YOUR_ENV")?;

// There are also a way to parse it as well. Type must implement `FromStr` trait
let keys: HashMap<String, i32> = envs::get_keys_parse("YOUR_ENV")?;

// If your keys encoded as BaseX, you can read it as
// -- BaseX decoded string bytes. Add feature `b64` to enable method
let keys: HashMap<String, Vec<u8>> = envs::get_keys_b64u_as_u8s("YOUR_ENV")?;

// -- BaseX decoded string. Add feature `b64` to enable method
let keys: HashMap<String, String> = envs::get_keys_b64u_as_s("YOUR_ENV")?;
```

## Examples

There are some examples in `examples` folder.  
Some of them requires features, so run them with:

```bash
cargo run --example <example_name> --features <feature_name>
```

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request. I wrote this library for my own use, so it may not fit everyone's needs, but your input is appreciated!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
