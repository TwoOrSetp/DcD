# ZCB Live - Rust Audio Engine

ZCB Live is a next-generation Geometry Dash clickbot written in Rust, featuring advanced audio processing, real-time UI, and comprehensive clickpack support. This is the core Rust library that powers the ZCB Live clickbot.

## Architecture Overview

The codebase is structured as a modular Rust library with the following key components:

### Core Modules

- **`lib.rs`** - Main entry point and DLL/Geode integration layer
- **`bot.rs`** - Core bot logic, UI rendering, and state management
- **`clickpack.rs`** - Audio clickpack loading and sound management
- **`game.rs`** - Geometry Dash game object abstractions and memory structures
- **`hooks.rs`** - Function hooking for game integration (DLL mode only)
- **`utils.rs`** - Utility functions and helpers

### Key Components and Responsibilities

#### 1. Bot (`bot.rs`)
The central component that manages the entire clickbot system:

- **Configuration Management**: Loads/saves settings from `.zcb/config.json`
- **Audio Engine**: Manages kittyaudio mixer for low-latency sound playback
- **UI System**: Renders egui-based interface with multiple tabs (Clickpack, Audio, Options)
- **Clickpack Management**: Handles loading and switching between different sound packs
- **Timing System**: Processes click timing and categorizes clicks (hard, soft, micro, etc.)
- **Device Management**: Handles audio output device selection and switching
- **Toast Notifications**: Provides user feedback through toast messages

Key features:
- Real-time audio processing with configurable buffer sizes
- Pitch randomization and volume control
- Spam detection and volume adjustment
- Player-specific sound routing (Player 1/2, Left/Right for platformer)
- Hotkey support for toggling bot and menu
- ClickpackDB integration for downloading sound packs

#### 2. Clickpack System (`clickpack.rs`)
Sophisticated audio management system:

- **Sound Classification**: Categorizes sounds into 8 types based on timing:
  - HardClick/HardRelease (>2.0s interval)
  - Click/Release (>0.15s interval)
  - SoftClick/SoftRelease (>0.025s interval)
  - MicroClick/MicroRelease (<0.025s interval)

- **Multi-Player Support**: Separate sound sets for:
  - Player1/Player2 (jump sounds)
  - Left1/Left2, Right1/Right2 (platformer movement)

- **Flexible Loading**: Supports various clickpack folder structures
- **Fallback System**: Intelligent sound selection with preference ordering
- **Noise Support**: Background noise/whitenoise playback

#### 3. Game Integration (`game.rs`, `hooks.rs`)
Provides seamless integration with Geometry Dash:

- **Memory Structures**: Safe abstractions for game objects (PlayLayer, PlayerObject, etc.)
- **Function Hooking**: Intercepts game functions to detect player actions
- **Dual Hook System**: Primary and alternate hooking methods for compatibility
- **Event Detection**: Captures level start/reset, player death, button presses

#### 4. Cross-Platform Support (`lib.rs`)
Supports both deployment methods:

- **Geode Mod**: Integrates with Geode SDK as a static library
- **Standalone DLL**: Works as an injectable DLL with OpenGL hooking
- **Conditional Compilation**: Feature flags control platform-specific code

### Main Functions and Their Purposes

#### Core Bot Functions
- `Bot::init()` - Initializes audio system, loads config, sets up hooks
- `Bot::on_action()` - Processes player input and plays appropriate sounds
- `Bot::draw_ui()` - Renders the main UI interface
- `Bot::preload_clickpack()` - Loads clickpack sounds in background threads
- `Bot::on_reset()/on_death()` - Handles level events and button releases

#### Audio Processing
- `Bot::get_pitch()` - Calculates randomized pitch values
- `Bot::play_noise()` - Manages background noise playback
- `Clickpack::get_random_click()` - Selects appropriate sound based on timing and player

#### Configuration Management
- `Config::load()/save()` - Persistent configuration storage
- `Env::load()/save()` - Environment settings and clickpack preferences

### How Components Work Together

1. **Initialization Flow**:
   - `lib.rs` provides entry points for both Geode and DLL modes
   - `Bot::init()` sets up audio system and loads configuration
   - Game hooks are installed (DLL mode) or Geode callbacks are registered
   - Clickpacks are preloaded in background threads

2. **Runtime Operation**:
   - Game hooks detect player actions and call `Bot::on_action()`
   - Bot calculates timing, selects appropriate sound, and plays it
   - UI system continuously renders interface and processes user input
   - Configuration changes are automatically saved every 5 seconds

3. **Audio Pipeline**:
   - Sounds are loaded from clickpack directories into memory
   - kittyaudio mixer handles low-latency playback with configurable buffering
   - Pitch and volume are applied in real-time based on user settings
   - Multiple sounds can play simultaneously with proper mixing

### Dependencies and External Libraries

#### Core Dependencies
- **`egui`** (0.27) - Immediate mode GUI framework for the interface
- **`kittyaudio`** (0.1.9) - Low-latency audio playback engine
- **`windows`** (0.58.0) - Windows API bindings for system integration
- **`serde`** + **`serde_json`** - Configuration serialization
- **`anyhow`** - Error handling and propagation

#### Specialized Libraries
- **`retour`** (0.3.1) - Function hooking library (DLL mode only)
- **`egui_gl_hook`** - Custom OpenGL integration for egui rendering
- **`egui-keybind`** - Hotkey binding UI components
- **`egui-modal`** - Modal dialog system
- **`egui-notify`** - Toast notification system
- **`egui_clickpack_db`** - ClickpackDB integration for downloading packs

#### Utility Libraries
- **`fastrand`** - Fast random number generation for pitch/timing
- **`parking_lot`** - High-performance synchronization primitives
- **`once_cell`** - Thread-safe lazy initialization
- **`ureq`** - HTTP client for clickpack downloads

### Build Instructions

#### For Geode Mod (Recommended)
Make sure `crate-type` in Cargo.toml is set to `staticlib`:

```bash
cargo build --release --features geode
```

Then build the Geode wrapper ([docs](https://docs.geode-sdk.org/getting-started/create-mod/#build)):

```bash
cd ..
cmake --build build --config Release
```

#### As Standalone DLL
Make sure `crate-type` in Cargo.toml is set to `cdylib`:

```bash
cargo build --release --features dll
```

### Feature Flags

- **`geode`** - Enables Geode SDK integration, disables DLL-specific code
- **`dll`** - Enables standalone DLL mode with function hooking
- **Default** - Basic functionality without platform-specific features

### Configuration

The bot stores configuration in `.zcb/` directory:
- `config.json` - User settings (audio, timing, shortcuts, etc.)
- `env.json` - Environment settings (selected clickpacks, devices)
- `clickpacks/` - Downloaded clickpack directories

### Known Issues and Warnings

#### Compilation Warnings (Resolved)
Recent improvements have addressed most compilation warnings:

1. **Deprecated PanicInfo**: ✅ Fixed - now uses `PanicHookInfo`
2. **Static Mut References**: ✅ Mitigated - added safer access patterns and suppressed warnings
   - Added `with_bot()` helper function for safer BOT access
   - Added proper documentation and safety comments
   - Warnings are suppressed with `#[allow(static_mut_refs)]` as the current usage is safe

#### Recent Improvements (v1.0.5+)
- ✅ **Dual Timing System**: Separate audio and sync timing for instant response + perfect recording
- ✅ **Instant Audio Response**: 0ms delay audio playback for immediate feedback
- ✅ **Predictive Timing**: Anticipates and pre-compensates for system latency
- ✅ **Input Tracking**: Monitors input patterns for optimal latency compensation
- ✅ **Three Timing Modes**: Responsive, Synchronized, and Hybrid modes for different use cases
- ✅ **Enhanced Recording Sync**: Advanced synchronization for perfect video recording
- ✅ **Pause Detection**: Automatic compensation for game pause/resume states
- ✅ **Timing Smoothing**: Reduces audio jitter during variable frame rate recording
- ✅ **Drift Compensation**: Corrects timing drift between game time and real time

#### Future Improvements
- Consider refactoring to use `Arc<Mutex<Bot>>` for full thread safety
- Migrate to `std::sync::OnceLock` when it becomes stable
- Further modularize the global state management
- Add automatic recording detection and sync mode switching

### Dual Timing System

The bot features an advanced dual timing system that separates audio playback timing from synchronization timing, providing both instant responsiveness and perfect recording accuracy:

#### Timing Modes
- **Responsive Mode**: Prioritizes immediate audio feedback for live gameplay (0ms delay)
- **Synchronized Mode**: Prioritizes perfect recording synchronization for video creation
- **Hybrid Mode**: Intelligently balances both based on current settings (recommended)

#### When to Use Each Mode
- **Responsive**: ✅ Live gameplay, ✅ Streaming, ✅ Practice sessions
- **Synchronized**: ✅ Recording videos, ✅ Creating content, ✅ Perfect sync required
- **Hybrid**: ✅ General use, ✅ Mixed recording/playing, ✅ Automatic optimization

#### Key Features
1. **Instant Audio Response**: Audio plays immediately upon button press (0ms delay)
2. **Predictive Timing**: Anticipates and pre-compensates for system latency
3. **Input Tracking**: Monitors input patterns for optimal compensation
4. **Dual Processing**: Separate timing calculations for audio vs synchronization

#### Enhanced Recording Synchronization
Advanced features for perfect video recording:
- **Pause Detection**: Automatically detects and compensates for game pauses
- **Drift Compensation**: Corrects timing drift between game time and real time
- **Smoothing**: Reduces timing jitter from frame rate variations
- **Recording Software Compatibility**: Accounts for recording software interference

#### Configuration Options
- **Timing Mode**: Choose between Responsive, Synchronized, or Hybrid
- **Instant Audio Response**: Enable/disable immediate audio feedback
- **Input Latency Compensation**: Fine-tune responsiveness (-0.1 to +0.1 seconds)
- **Time Smoothing Factor**: Control stability vs responsiveness (0.0-1.0)
- **Pause Detection Threshold**: Sensitivity for pause detection (0.01-1.0 seconds)
- **Timing Diagnostics**: Real-time monitoring of all timing systems

### Usage Guidelines

1. **First Launch**: The bot shows a welcome dialog with basic instructions
2. **Hotkeys**: Default hotkeys are `1` (toggle menu) and `2` (toggle bot)
3. **Clickpacks**: Use the ClickpackDB tab to browse and download sound packs
4. **Audio Settings**: Configure timing thresholds, volume, and pitch in the Audio tab
5. **Timing Setup**:
   - **For Live Play**: Set Timing Mode to "Responsive" + Enable "Instant Audio Response"
   - **For Recording**: Set Timing Mode to "Synchronized" + Enable "Enhanced Recording Sync"
   - **For Mixed Use**: Set Timing Mode to "Hybrid" (automatically optimizes)
   - **Fine-tuning**: Adjust "Input Latency Compensation" if audio feels delayed/early
6. **Compatibility**: Use alternate hook mode if experiencing conflicts with other mods

### Troubleshooting

- **Build Warnings**: The compilation warnings are non-critical and don't affect functionality
- **Audio Issues**: Try adjusting buffer size in Audio settings or switching output devices
- **Hook Conflicts**: Enable "Use alternate hook" in Options if experiencing issues with other mods
- **Performance**: Lower UI scale or disable unnecessary features if experiencing lag
