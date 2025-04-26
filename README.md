## Danmakw
WGPU based danmaku renderer

### Run the example
```bash
cargo run --release --example winit # winit example
cargo run --release --example gtk4 # gtk4 example
```

### Known issues
- The gtk4 example is not working on Windows. It seems slower (~3-10x) than the winit example (Winit example can get ~10000 FPS (No Vsync) on my machine).

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.