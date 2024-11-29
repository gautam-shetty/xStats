#!/bin/bash

TARGET_DIR="tmp/AgriCraft"
OUTPUT_DIR="tmp/op"

cargo run -- -t "$TARGET_DIR" -o "$OUTPUT_DIR"