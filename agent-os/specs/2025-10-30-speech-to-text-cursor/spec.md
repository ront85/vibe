# Specification: Speech-to-Text at Cursor

## Goal

Add real-time speech-to-text dictation feature that allows users to speak and automatically paste transcribed text at cursor location in any application, similar to Whisper Flow.

## User Stories

-   As a user, I want to hold a keyboard shortcut to dictate text that automatically appears where my cursor is, so that I can write faster without typing
-   As a user, I want to see a visual floating widget that shows audio levels when recording, so that I know the dictation is working

## Specific Requirements

**Push-to-Talk Recording**

-   Hold keyboard shortcut to start recording (key down), release to stop and transcribe (key up)
-   Maximum 5-minute recording session with auto-stop on timeout
-   ESC key cancels recording without transcription
-   Audio feedback: high beep (750Hz, 100ms) on start, low beep (500Hz, 100ms) on stop
-   Audio buffer management with real-time microphone capture
-   Works system-wide in any application where cursor is located

**Floating Widget UI**

-   Fixed position: center horizontally, 100px from screen bottom
-   Collapsed state: 5px height x 30px width, 50% opacity black, pill-shaped
-   Expanded state: 30px height when active/hovered, 100% opacity black
-   Click to start/stop recording as alternative to keyboard shortcut
-   Real-time audio level visualization with white vertical bars (3-5 bars)
-   Processing state shows horizontal bar animation (loading indicator)
-   Close button (left) and record/stop button (right) when active
-   Option in settings to disable floating widget entirely (keyboard-only mode)

**Keyboard Shortcuts**

-   Platform-specific defaults: macOS Cmd+Shift+Space, Windows/Linux Ctrl+Alt+S
-   User-configurable in settings with conflict detection
-   Rapid press handling: ignore subsequent presses during active session
-   Global hotkey registration works system-wide across all applications
-   CRITICAL: Original macOS default Cmd+S conflicts with Save - changed to Cmd+Shift+Space

**Transcription Processing**

-   Batch mode: transcribe complete audio after user releases key
-   User-selectable Whisper model in settings (default: small)
-   Automatic punctuation enabled via Whisper configuration
-   Share language settings with main app
-   Immediate paste at cursor location after transcription (no preview)
-   Fail silently if no speech detected (empty transcription)
-   Leverage existing whisper-rs integration and transcribe.rs infrastructure

**History Management**

-   New "Dictation" tab in main application showing chronological list
-   Store all transcriptions with metadata: timestamp, destination app name, transcribed text, model used, duration
-   30-day retention with automatic cleanup of older entries
-   Search functionality: filter by text content, app name, or date range
-   Edit and copy capabilities for historical transcriptions
-   SQLite database in local app data directory (offline only, privacy-first)

**Settings Configuration**

-   New "Dictation" section in settings page
-   Microphone device selection using existing AudioDeviceInput component
-   Whisper model selection dropdown (tiny, base, small, medium, large)
-   Keyboard shortcut customization with conflict warnings
-   Toggle to enable/disable floating widget
-   Audio feedback enable/disable option
-   All settings persist across app restarts

**System Integration**

-   Request accessibility permissions on first use (macOS) with clear explanation dialog
-   Request microphone permissions on first use (all platforms)
-   Platform-specific text pasting: direct input simulation preferred over clipboard
-   Destination app name detection for history metadata
-   Handle permission denials gracefully with remediation instructions

**Text Pasting Mechanism**

-   Direct text input simulation using platform-specific APIs preserves clipboard
-   macOS: AXUIElement or CGEvent text input
-   Windows: SendInput API with KEYEVENTF_UNICODE
-   Linux: X11 XTest or Wayland input simulation
-   Maintain focus in target application during paste
-   If cursor not in text field, fail silently (user retrieves from history)

**Edge Cases Handling**

-   Microphone disconnected during recording: stop and transcribe captured audio if any
-   Transcription failure: show error notification, preserve audio buffer for retry
-   5-minute timeout: auto-stop and transcribe with optional toast notification
-   Empty transcription: fail silently, no paste or notification
-   System audio device change: refresh device list in settings
-   Widget persistence across app restarts: remember enabled/disabled state
-   Multi-monitor support: show widget on primary monitor
-   Model not downloaded: prompt user to download, fallback to smallest available

**Performance Requirements**

-   Transcription latency: less than 2 seconds from release to paste (small model, typical 10-30s clips)
-   Widget animations: 60fps with less than 16ms frame time
-   Audio waveform updates: 30-60fps for smooth visualization
-   History search: less than 100ms response time for typical history size
-   Minimal startup impact: don't load Whisper models until first use
-   Efficient ring buffer for audio capture with minimal memory overhead

## Visual Design

**`planning/visuals/image.png`**

-   Full Whisper Flow interface showing history tab with chronological transcription list
-   Left sidebar navigation pattern for tabs (Home, Dictionary, Snippets, Style, Notes)
-   Toast notification "Transcript copied" at bottom right corner
-   Clean typography with timestamps and full transcribed text visible
-   Light background with good contrast for readability
-   Successful completion feedback pattern to replicate

**`planning/visuals/image copy.png`**

-   Collapsed floating widget in idle state: small black bar at screen bottom
-   Minimal visual footprint to avoid distraction
-   Positioned at bottom center as specified
-   Semi-transparent appearance when idle
-   Pill-shaped with rounded corners

**`planning/visuals/image copy 2.png`**

-   Expanded widget showing hover tooltip: "Click or hold Ctrl + s to start dictating"
-   Animated dots indicating idle state
-   Instruction text appears on hover to guide user
-   Black background with white text for high contrast
-   Larger height when active to accommodate controls and visualization

## Existing Code to Leverage

**core/src/audio.rs - Audio Processing**

-   Use existing ffmpeg integration via find_ffmpeg_path() and normalize() functions
-   Leverage audio format conversion (16kHz mono, pcm_s16le) for Whisper compatibility
-   Reuse parse_wav_file() for audio buffer handling
-   Build upon established patterns for audio device management

**core/src/transcribe.rs - Whisper Integration**

-   Use create_context() for model loading with GPU device configuration
-   Leverage transcribe() function with progress callbacks and abort handling
-   Reuse setup_params() for Whisper configuration (language, temperature, sampling strategy)
-   Adopt existing FullParams configuration patterns for punctuation and timestamps
-   Utilize convert_integer_to_float_audio() for sample format conversion

**desktop/src-tauri/src/cmd/audio.rs - Audio Device Enumeration**

-   Reuse get_audio_devices() command for microphone selection in settings
-   Leverage AudioDevice struct and device enumeration patterns
-   Adopt cpal integration patterns for cross-platform audio capture
-   Use existing WavWriterHandle pattern for real-time audio recording

**desktop/src-tauri/src/setup.rs - App Initialization**

-   Follow ModelContext pattern for managing Whisper context state
-   Use existing app.manage(Mutex) pattern for thread-safe dictation state
-   Adopt STATIC_APP pattern for global app handle access
-   Leverage established directory creation patterns for history database

**desktop/src-tauri/src/cmd/mod.rs - Tauri Commands**

-   Follow set_progress_bar() pattern for UI progress updates
-   Use existing event listener patterns (abort_transcribe, abort_download)
-   Adopt error handling and logging patterns (eyre, Context, log_error)
-   Reuse State management patterns with Mutex for shared state access

## Out of Scope

-   Real-time streaming transcription (batch mode only in this spec)
-   Custom vocabulary or user dictionaries
-   Voice commands or special keywords
-   Multi-language detection within single recording
-   Cloud sync of transcription history
-   Speaker identification in dictation mode
-   Audio playback of recorded dictation
-   Export history to external formats
-   Integration with external note-taking apps
-   Custom widget positioning or theming
