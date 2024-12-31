#!/bin/bash

# Run cargo build
cargo build

# Check if the build was successful
if [ $? -eq 0 ]; then
    echo "Build succeeded, proceeding with publish..."

    # Publish api-response-macros
    cargo publish --no-verify --registry crates-io --allow-dirty -p api-response-macros

    code=$?

    if [ $code -eq 0 ]; then
        echo "crate api-response-macros published successfully."
    elif [ $code -eq 101 ]; then
        echo "crate api-response-macros already exists on crates.io index."
    else
        echo "Failed to publish crate api-response-macros."
        exit $code
    fi

    # Publish api-response
    cargo publish --no-verify --registry crates-io --allow-dirty -p api-response

    # Check if the second publish succeeded
    if [ $? -eq 0 ]; then
        echo "crate api-response published successfully."
    elif [ $? -eq 101 ]; then
        echo "crate api-response already exists on crates.io index."
    else
        echo "Failed to publish crate api-response."
    fi
else
    echo "Build failed, publish aborted."
fi
