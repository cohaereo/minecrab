cargo build --release && \
./verify_shaders.sh && \
RUST_LOG=info,wgpu_hal=warn,wgpu_core=warn,winit=warn ./target/release/minecrab