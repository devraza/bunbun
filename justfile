set export
RUSTFLAGS := "-Zlocation-detail=none"

target := `rustc -Vv | awk 'FNR == 5 {print $2}'`

# Build for a small executable
tiny:
  cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target {{target}} --release

# Normal build, in this file for no good reason
build:
  cargo build --release
