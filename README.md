## Danmakw
WGPU based danmaku renderer

### Run the example
```bash
cargo run --release --example winit # winit example
cargo run --release --example gtk_vulkan_dmabuf # gtk-vulkan-dmabuf example
cargo run --release --example gtk_wgpu_gles_framebuffer # gtk-wgpu-dles-framebuffer example
```

### Known issues
- The gtk_vulkan_dmabuf example is not working on Windows. It seems slower (~3-10x) than the winit example (Winit example can get ~10000 FPS (No Vsync) on my machine).
- The gtk_wgpu_gles_framebuffer example needs a "scaleY(-1)" to flip the image.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.