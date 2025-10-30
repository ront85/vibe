# Spec Initialization: Speech-to-Text at Cursor

**Feature Name:** Speech-to-Text at Cursor (like Whisper Flow)

**Description:**
Add a real-time speech-to-text feature that allows users to dictate text and have it automatically pasted where their cursor is located. This is similar to the Whisper Flow application.

**Key Requirements from User:**
1. **Keyboard shortcut trigger** - User can configure a keyboard combination in settings
2. **Floating button with frequency display** - A small floating window/button that shows:
   - Visual indication when speaking (frequency/audio level visualization)
   - Clickable button to start/stop listening (alternative to keyboard shortcut)
3. **Microphone selection** - User can choose which microphone to use in settings
4. **Transcription behavior** - When activated (via keyboard combo or button):
   - Listens to audio from selected microphone
   - Transcribes speech to text using Whisper
   - Automatically pastes the transcribed text where the cursor is currently positioned
5. **Currently missing** - This feature does not exist in Vibe yet

**Current Vibe Context:**
- Desktop app using Tauri v2
- Backend: Rust
- Frontend: React + TypeScript
- Already has Whisper integration (whisper-rs)
- Already has audio device handling (audio.rs)
- Supports microphone transcription for file creation
- Cross-platform: macOS, Windows, Linux

**Date Created:** 2025-10-30
