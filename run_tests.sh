#!/bin/bash

run_tests() {
    local features="$1"
    echo "Running tests with features: $features"
    cargo test --release --no-default-features --features "$features"
    
    # Check if test was successfull
    if [ $? -ne 0 ]; then
        echo "Tests failed for features: $features"
        exit 1
    fi
}

# Time
run_tests "time"

# Encoding
run_tests "b32"
run_tests "b58"
run_tests "b64"
run_tests "b32,b58,b64"

# Envs
run_tests "envs"
run_tests "envs,b32"
run_tests "envs,b58"
run_tests "envs,b64"
run_tests "envs,b32,b58,b64"

# Cuuid
run_tests "cuuid"
run_tests "cuuid,b32"
run_tests "cuuid,b58"
run_tests "cuuid,b64"
run_tests "cuuid,b32,b58,b64"

# Test all
echo "Running tests with all features"
cargo test --all-features

echo "All tests completed successfully."
