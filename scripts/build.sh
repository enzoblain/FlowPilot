#!/bin/bash

set -e

cd "$(dirname "$0")/.."

CRATE_NAME="bindings"
LIB_NAME="bindings"

rustup target add \
  aarch64-apple-ios \
  aarch64-apple-ios-sim \
  x86_64-apple-ios \
  aarch64-apple-darwin

cargo build -p $CRATE_NAME --release --target aarch64-apple-ios
cargo build -p $CRATE_NAME --release --target aarch64-apple-ios-sim
cargo build -p $CRATE_NAME --release --target x86_64-apple-ios
cargo build -p $CRATE_NAME --release --target aarch64-apple-darwin

mkdir -p target/universal

lipo -create \
  target/aarch64-apple-ios-sim/release/lib${LIB_NAME}.a \
  target/x86_64-apple-ios/release/lib${LIB_NAME}.a \
  -output target/universal/lib${LIB_NAME}_sim.a

rm -rf Nexus.xcframework

xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/lib${LIB_NAME}.a \
  -headers crates/bindings/include \
  -library target/universal/lib${LIB_NAME}_sim.a \
  -headers crates/bindings/include \
  -library target/aarch64-apple-darwin/release/lib${LIB_NAME}.a \
  -headers crates/bindings/include \
  -output Nexus.xcframework

mkdir -p apps/nexus/Frameworks

rm -rf apps/nexus/Frameworks/Nexus.xcframework

mv Nexus.xcframework apps/nexus/Frameworks/