## Tech stack

Define your technical stack below. This serves as a reference for all team members and helps maintain consistency across the project.

### Framework & Runtime
- **Application Framework:** Tauri v2 (desktop app framework)
- **Backend Language:** Rust (stable)
- **Frontend Language:** TypeScript
- **Package Manager:** Bun (primary), Cargo (Rust)
- **Build Tool:** Vite

### Frontend
- **JavaScript Framework:** React
- **CSS Framework:** TailwindCSS + DaisyUI
- **UI Components:** Custom components
- **Routing:** react-router-dom
- **i18n:** i18next + react-i18next

### Core Libraries
- **Transcription Engine:** whisper-rs (Whisper.cpp bindings, custom fork)
- **Speaker Diarization:** pyannote-rs
- **Media Processing:** ffmpeg (pre-built binaries)
- **GPU Acceleration:** Optional features for CUDA, Vulkan, Metal, CoreML, ROCm

### Testing & Quality
- **Rust Testing:** cargo test with built-in test framework
- **Frontend Linting:** ESLint
- **Rust Linting:** cargo clippy
- **Formatting:** cargo fmt (Rust), Prettier (TypeScript)

### Deployment & Infrastructure
- **Platforms:** Windows (NSIS), macOS (DMG/App), Linux (Deb/RPM/AppImage)
- **CI/CD:** GitHub Actions
- **Distribution:** GitHub Releases with auto-updater

### Third-Party Services
- **AI Summarization:** Claude API, Ollama (optional)
- **Model Storage:** Hugging Face (model downloads)
- **Crash Reporting:** Built-in crash reporter with backtrace serialization
