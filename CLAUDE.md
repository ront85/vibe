# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Vibe is an offline audio/video transcription desktop application powered by OpenAI's Whisper model. It provides multilingual transcription (nearly every language), batch processing, speaker diarization, AI summarization (Claude API/Ollama), and exports to multiple formats (SRT, VTT, TXT, HTML, PDF, JSON, DOCX). The app is fully offline with ultimate privacy and supports GPU acceleration across macOS, Windows, and Linux (Nvidia/AMD/Intel via Vulkan/CoreML).

Key capabilities:
- Transcribe audio/video files and download from YouTube/Vimeo/Facebook/Twitter
- Real-time preview with speaker diarization
- System audio & microphone transcription
- **Speech-to-text dictation at cursor** (push-to-talk with automatic pasting)
- CLI support and optional HTTP API with Swagger docs
- Auto-updates and cross-platform support

## Technology Stack

**Desktop App (Tauri v2):**
- **Backend:** Rust with Tauri 2.x
- **Frontend:** React + TypeScript + Vite + TailwindCSS/DaisyUI
- **Package Manager:** Bun (primary)
- **i18n:** i18next + react-i18next with locales in `desktop/src-tauri/locales/`

**Cargo Workspace Structure:**
```
Cargo.toml (workspace root)
├── core/                    # vibe_core library - core transcription engine
└── desktop/src-tauri/       # Tauri app wrapper
```

**Core Dependencies:**
- `whisper-rs` - Whisper.cpp bindings (custom fork)
- `pyannote-rs` - Speaker diarization
- `ffmpeg` - Audio/video processing (pre-built via scripts)
- `cpal` - Cross-platform audio capture for dictation
- `rusqlite` - SQLite database for dictation history
- Various `tauri-plugin-*` packages for file system, dialogs, updater, etc.

**Additional Components:**
- `landing/` - SvelteKit website
- `scripts/` - Build automation scripts

## Development Commands

**Setup:**
```bash
cd desktop
bun install
bun scripts/pre_build.js  # Downloads ffmpeg, openblas, platform dependencies
```

**Development:**
```bash
# From desktop/ directory
bun run dev              # Frontend only
bunx tauri dev           # Full Tauri app with hot reload
```

**Building:**
```bash
# From desktop/ directory
bunx tauri build         # Production build for current platform
```

**Testing:**
```bash
# Test core library (use --release for performance)
cargo test -p vibe_core --release -- --nocapture

# Test dictation feature specifically
cargo test --test dictation_integration_tests --release -- --nocapture

# Test all packages
cargo test -- --nocapture

# Enable detailed logging
export RUST_LOG=trace
cargo test -p vibe_core --release -- --nocapture
```

**Linting:**
```bash
# Rust
cargo fmt                # Format
cargo clippy             # Lint

# Frontend (from desktop/)
bun run lint
```

**Pre-build Script Options:**
```bash
bun scripts/pre_build.js --vulkan    # Vulkan SDK setup
bun scripts/pre_build.js --openblas  # OpenBLAS setup
bun scripts/pre_build.js --amd       # AMD/ROCm support
```

## Architecture

**Core Library (`core/src/`):**
- `lib.rs` - Public API exports
- `audio.rs` - Audio device handling and processing
- `audio_capture.rs` - Real-time microphone capture for dictation
- `config.rs` - Configuration management
- `dictation.rs` - Dictation state machine and audio buffering
- `dictation_history.rs` - SQLite-based history storage
- `dictation_transcribe.rs` - Dictation-specific transcription logic
- `downloader.rs` - Model downloading from Hugging Face
- `transcribe.rs` - Whisper transcription engine
- `transcript.rs` - Transcript processing and formatting

**Tauri App (`desktop/src-tauri/src/`):**
- `main.rs` - App entry point and Tauri builder
- `cli.rs` - CLI argument parsing and console attachment
- `cmd/` - Tauri command handlers (frontend-backend bridge)
  - `cmd/dictation.rs` - Dictation commands (paste, history, etc.)
  - `cmd/permissions.rs` - Permission check commands
- `dictation_settings.rs` - Dictation settings persistence
- `keyboard_hooks/` - Platform-specific global keyboard hooks
  - `keyboard_hooks/mod.rs` - Unified interface
  - `keyboard_hooks/macos.rs` - macOS Carbon/Cocoa event taps
  - `keyboard_hooks/windows.rs` - Windows RegisterHotKey API
  - `keyboard_hooks/linux.rs` - X11/Wayland keyboard hooks
- `text_input/` - Platform-specific text pasting
  - `text_input/mod.rs` - Unified interface
  - `text_input/macos.rs` - AXUIElement API
  - `text_input/windows.rs` - SendInput API
  - `text_input/linux.rs` - X11 XTest / Wayland
- `permissions/` - Permission checking and remediation
  - `permissions/mod.rs` - Unified interface
  - `permissions/macos.rs` - Accessibility permissions
  - `permissions/windows.rs` - Microphone permissions
  - `permissions/linux.rs` - Input group and audio access
- `setup.rs` - App initialization and global state
- `logging.rs` - Tracing/logging configuration
- `server.rs` - Optional HTTP API server (enable with `server` feature)
- Platform-specific modules: `screen_capture_kit.rs` (macOS), `gpu_preference.rs` (Windows)

**Frontend (`desktop/src/`):**
- `App.tsx` - Root component with routing
- `components/` - Reusable UI components
  - `components/FloatingWidget.tsx` - Dictation widget with waveform visualization
  - `components/WaveformBars.tsx` - Audio level visualization
  - `components/DictationHistoryEntry.tsx` - History list item
- `pages/` - Route-based page components
  - `pages/settings/DictationSettings.tsx` - Dictation configuration
  - `pages/history/DictationHistory.tsx` - Dictation history view
- `lib/` - Utilities and helpers
- `providers/` - React context providers
  - `providers/Dictation.tsx` - Dictation state provider

**Feature Flags (Cargo.toml):**
GPU backends are controlled via Cargo features:
- `cuda` - NVIDIA CUDA support
- `vulkan` - Vulkan GPU support (cross-platform)
- `metal` - Apple Metal (macOS)
- `coreml` - CoreML (macOS)
- `rocm` - AMD ROCm (Linux)
- `openblas` - OpenBLAS CPU acceleration
- `server` - Enable HTTP API server

## Dictation Feature Architecture

**Overview:**
The dictation feature enables system-wide speech-to-text with automatic pasting at cursor location, similar to Whisper Flow. It uses push-to-talk input, real-time audio capture, and offline Whisper transcription.

**Key Components:**

1. **State Management** (`core/src/dictation.rs`):
   - State machine: Idle → Recording → Processing → Idle/Error
   - Audio buffering with 5-minute maximum
   - RMS audio level calculation for visualization

2. **Audio Capture** (`core/src/audio_capture.rs`):
   - Real-time microphone recording at 16kHz mono
   - Audio feedback beeps (start/stop)
   - Device enumeration and selection

3. **Keyboard Hooks** (`desktop/src-tauri/src/keyboard_hooks/`):
   - Platform-specific global hotkey registration
   - Push-to-talk (key down/up detection)
   - Default shortcuts: macOS (Cmd+Shift+Space), Windows/Linux (Ctrl+Alt+S)
   - ESC cancellation support

4. **Transcription** (`core/src/dictation_transcribe.rs`):
   - Batch transcription after recording stops
   - Automatic punctuation enabled
   - Model selection (tiny, base, small, medium, large)
   - Empty transcription handling (silent failure)

5. **Text Pasting** (`desktop/src-tauri/src/text_input/`):
   - Direct text input (preserves clipboard)
   - Platform-specific: AXUIElement (macOS), SendInput (Windows), XTest (Linux)
   - Destination app detection for history metadata

6. **History Management** (`core/src/dictation_history.rs`):
   - SQLite database with 30-day retention
   - Full-text search with indexed queries
   - CRUD operations (create, read, update, delete)

7. **Floating Widget** (`desktop/src/components/FloatingWidget.tsx`):
   - Minimal collapsed state (5px × 30px)
   - Expanded state with waveform visualization
   - Click-to-record alternative to keyboard
   - Hardware-accelerated animations (60fps)

**Data Flow:**
```
User presses keyboard shortcut
  ↓
Keyboard hook detects key down → Start recording
  ↓
Audio capture begins (16kHz mono) → Buffer audio samples
  ↓
Real-time audio levels → Update widget waveform visualization
  ↓
User releases key → Stop recording
  ↓
Audio buffer → Whisper transcription → Text result
  ↓
Text pasted at cursor (via platform API) + Saved to history
  ↓
Widget returns to idle state
```

**Platform Considerations:**
- **macOS**: Requires Accessibility permissions for keyboard hooks and text pasting
- **Windows**: Microphone permissions in Privacy settings
- **Linux**: X11 more reliable than Wayland for global hotkeys

**Testing:**
- 98 unit tests across all modules
- 10 integration tests for end-to-end workflows
- Performance targets: < 2s transcription, 60fps animations, < 100ms search

See `docs/dictation.md` for detailed user guide.

## Build Process

**Prerequisites:**
- Bun, Rust/Cargo, Clang/LLVM, CMake
- Platform-specific:
  - **Windows:** wget, 7zip
  - **Linux:** ffmpeg, libopenblas, GTK3, WebKit2GTK, libavutil/format/filter/device
  - **macOS:** Xcode (open once to download SDK)

**Pre-Build Script (`scripts/pre_build.js`):**
Must be run before first build. Downloads:
- Pre-built ffmpeg binaries for the platform
- OpenBLAS (Windows)
- Vulkan SDK (if `--vulkan` flag used)
- Platform-specific dependencies

**VSCode Development (Windows):**
rust-analyzer requires environment variables set in `.vscode/settings.json`:
- `FFMPEG_DIR`
- `OPENBLAS_PATH`
- `LIBCLANG_PATH`

**Release Optimization:**
The workspace `Cargo.toml` sets aggressive optimization for release builds:
- LTO enabled
- Single codegen unit
- Size optimization
- Panic=abort
- Strip symbols

## Testing

**Core Library Tests:**
Always use `--release` flag when testing `vibe_core` due to performance requirements of Whisper model:
```bash
cargo test -p vibe_core --release -- --nocapture
```

**Dictation Integration Tests:**
```bash
cargo test --test dictation_integration_tests --release -- --nocapture
```

**Logging:**
Set `RUST_LOG` environment variable for detailed output:
```bash
export RUST_LOG=trace  # or debug, info, warn, error
```

**CI/CD:**
GitHub Actions workflows in `.github/workflows/`:
- `test_core.yml` - Cross-platform testing (macOS Intel/ARM, Ubuntu, Windows)
- `lint_rust.yml` - cargo fmt & clippy enforcement
- `release.yml` - Multi-platform release builds
- `linux_special.yml` / `windows_special.yml` - Special GPU variant builds

## Special Features

**Deep Links:**
Custom protocol handler for model installation:
```
vibe://download/?url=<model_url>
```

**CLI Modes:**
```bash
vibe --help              # Show CLI help
vibe --server            # Start HTTP API server
vibe --portable          # Portable mode (data in app directory)
```

**HTTP API Server:**
Enable with `server` feature or `--server` CLI flag:
- Swagger UI available at `http://<host>:3022/docs`
- RESTful API for transcription operations

**i18n Support:**
Translation files located in `desktop/src-tauri/locales/`
- Managed via i18next/react-i18next
- VSCode i18n-ally extension configured for translation management

**Export Formats:**
Transcripts can be exported to: SRT, VTT, TXT, HTML, PDF, JSON, DOCX

**Crash Handling:**
Built-in crash reporter with backtrace serialization for debugging

## Important Documentation

- `docs/building.md` - Comprehensive build instructions with platform-specific details
- `docs/debug.md` - Troubleshooting guide for common issues
- `docs/models.md` - Model installation and management guide
- `docs/dictation.md` - Speech-to-text dictation feature guide
- `docs/changelog.md` - Release history
- Tauri config: `desktop/src-tauri/tauri.conf.json`

## Release Process

1. Increment version in `desktop/src-tauri/tauri.conf.json`
2. Create and push git tag:
   ```bash
   git tag -a v<version> -m "v<version>"
   git push --tags
   ```
3. GitHub Actions automatically builds for all platforms
4. Generates `latest.json` for auto-updater
5. Deploys landing page updates
