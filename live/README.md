# DCD Live cho Geode

DCDLive là một clickbot thế hệ mới cho Geometry Dash, được phát triển bằng Rust và tích hợp với Geode SDK.

## Tổng quan dự án

DCDLive là một công cụ clickbot tiên tiến được thiết kế để nâng cao trải nghiệm chơi Geometry Dash. Dự án được xây dựng với kiến trúc hybrid, kết hợp Rust core với C++ wrapper để tích hợp với Geode mod framework.

## Tính năng chính

- **Dễ sử dụng**: Giao diện người dùng trực quan với egui
- **Hỗ trợ chế độ Platformer**: Tương thích đầy đủ với cả chế độ thường và platformer
- **Tích hợp ClickpackDB**: Tải xuống clickpack trực tiếp trong game từ bộ sưu tập 300+ clickpack
- **Biến đổi âm thanh nâng cao**: 
  - Thay đổi pitch ngẫu nhiên
  - Điều chỉnh âm lượng động
  - Timing có thể tùy chỉnh
- **Tương thích bot**: Hoạt động với mọi loại bot
- **Thuật toán âm thanh thực tế**: Mô phỏng âm thanh click tự nhiên
- **Phím tắt có thể cấu hình**: Tùy chỉnh keybind theo ý muốn

## Hướng dẫn build

### Build cho Geode (khuyến nghị)

1. Đảm bảo `crate-type` trong `Cargo.toml` được đặt thành `staticlib`:
   ```toml
   [lib]
   crate-type = ["staticlib"]
   ```

2. Build thư viện Rust:
   ```bash
   cargo build --release --features geode
   ```

3. Build Geode wrapper ([tài liệu](https://docs.geode-sdk.org/getting-started/create-mod/#build)):
   ```bash
   cd ..
   cmake --build build --config Release
   ```

### Build như DLL

1. Đặt `crate-type` trong `Cargo.toml` thành `cdylib`:
   ```toml
   [lib]
   crate-type = ["cdylib"]
   ```

2. Build:
   ```bash
   cargo build --release
   ```

## Cấu trúc dự án

```
zcblive/
├── live/                    # Core Rust library
│   ├── src/
│   │   ├── lib.rs          # Entry point và FFI exports
│   │   ├── bot.rs          # Logic chính của bot
│   │   ├── clickpack.rs    # Xử lý clickpack
│   │   ├── game.rs         # Tích hợp game (chỉ DLL)
│   │   ├── hooks.rs        # Memory hooks (chỉ DLL)
│   │   └── utils.rs        # Utilities
│   ├── egui_gl_hook/       # OpenGL hook cho UI
│   ├── gfmod/              # FMOD bindings
│   └── Cargo.toml
├── src/
│   └── main.cpp            # C++ wrapper cho Geode
├── CMakeLists.txt          # Build configuration
└── mod.json               # Geode mod metadata
```

## Dependencies và yêu cầu

### Rust Dependencies chính
- `egui`: UI framework
- `kittyaudio`: Audio playback
- `retour`: Memory hooking (chỉ DLL)
- `windows`: Windows API bindings
- `serde`: Serialization cho config
- `egui_clickpack_db`: ClickpackDB integration

### System Requirements
- Windows (Win64)
- Geometry Dash 2.2074
- Geode SDK 4.0.1+

## Tùy chọn cấu hình

### Features
- `geode`: Build cho Geode (vô hiệu hóa DLL-specific code)
- `dll`: Build như DLL với memory hooks

### Cấu hình chính
- **Pitch settings**: Biến đổi pitch ngẫu nhiên
- **Volume settings**: Điều chỉnh âm lượng và spam detection
- **Timing settings**: Delay và offset cho clicks
- **Shortcuts**: Phím tắt tùy chỉnh
- **UI scale**: Tỷ lệ giao diện (0.3x - 5.0x)

## Hướng dẫn sử dụng

### Phím tắt mặc định
- **Phím 1**: Mở/đóng menu
- **Phím 2**: Bật/tắt clickbot
- **Phím 3**: Bật/tắt noise

### Truy cập ClickpackDB
1. Mở menu DCDLive
2. Chuyển đến tab "Clickpack"
3. Nhấn "Open ClickpackDB..."
4. Duyệt và tải xuống clickpack

### Cấu hình Audio
- **Global volume**: Âm lượng tổng thể
- **Volume variation**: Biến đổi âm lượng ngẫu nhiên
- **Spam detection**: Giảm âm lượng khi spam click
- **Platformer volume**: Âm lượng riêng cho chế độ platformer

## Chi tiết kỹ thuật

### Kiến trúc
- **Core**: Rust library với FFI exports
- **UI**: egui với OpenGL rendering
- **Audio**: kittyaudio với FMOD fallback
- **Integration**: C++ wrapper cho Geode hooks

### Memory Management
- Static detours cho DLL mode
- Safe FFI boundaries với proper error handling
- Automatic config saving mỗi 5 giây

### Performance
- LTO optimization trong release builds
- Minimal overhead với lazy initialization
- Efficient audio mixing với configurable buffer sizes

## Cộng đồng

Tham gia Discord server để được hỗ trợ, chia sẻ clickpack và video:
- https://discord.gg/clickbot
- https://discord.gg/BRVVVzxESu

## Phát triển

### Build từ source
```bash
# Clone repository
git clone <repository-url>
cd zcblive

# Build cho Geode
cd live
cargo build --release --features geode
cd ..
cmake --build build --config Release
```

### Debugging
- Console output có thể được bật trong Options
- Debug panel hiển thị click times và audio parameters
- Log output qua `simple_logger`

---

**Lưu ý**: `--features geode` vô hiệu hóa compilation của nhiều DLL-specific code để tối ưu hóa cho Geode environment.
