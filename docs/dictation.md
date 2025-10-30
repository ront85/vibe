# Speech-to-Text Dictation

## Overview

Vibe's dictation feature enables system-wide speech-to-text with automatic pasting at your cursor location. Simply hold a keyboard shortcut, speak, and release - your words appear instantly wherever your cursor is focused.

**Key Features:**
- Push-to-talk recording (hold to record, release to transcribe)
- Automatic pasting at cursor location in any application
- Floating widget with real-time audio visualization
- Searchable history of all dictations
- Fully offline with ultimate privacy
- Automatic punctuation
- 5-minute maximum recording time

## Getting Started

### 1. Enable Dictation

Open Vibe settings and navigate to the **Dictation** section. The feature is disabled by default to avoid accidental activations.

### 2. Grant Permissions

Dictation requires two system permissions:

#### macOS
1. **Accessibility Permission**: Required for global keyboard shortcuts and text pasting
   - Go to System Preferences → Security & Privacy → Privacy → Accessibility
   - Click the lock icon and add Vibe to the list
   - Enable the checkbox next to Vibe

2. **Microphone Permission**: Required for voice recording
   - Go to System Preferences → Security & Privacy → Privacy → Microphone
   - Enable the checkbox next to Vibe

#### Windows
1. **Microphone Permission**: Required for voice recording
   - Go to Settings → Privacy → Microphone
   - Enable "Allow apps to access your microphone"
   - Enable Vibe in the list

#### Linux
1. **Audio Access**: Ensure PulseAudio or PipeWire is running
2. **Input Group**: Add your user to the `input` group for keyboard hooks
   ```bash
   sudo usermod -a -G input $USER
   # Log out and back in for changes to take effect
   ```

### 3. Choose a Whisper Model

Select a Whisper model in the dictation settings:
- **Tiny** (75 MB): Fastest, lowest accuracy
- **Small** (250 MB): Recommended - balanced speed and accuracy
- **Base** (150 MB): Good accuracy, slower than small
- **Medium** (750 MB): High accuracy, requires more RAM
- **Large** (1.5 GB): Highest accuracy, slowest

**Recommendation**: Start with "Small" model for best balance.

## Using Dictation

### Method 1: Keyboard Shortcut (Push-to-Talk)

1. **Press and hold** the keyboard shortcut:
   - **macOS**: Cmd+Shift+Space
   - **Windows/Linux**: Ctrl+Alt+S

2. A high beep confirms recording has started

3. **Speak clearly** into your microphone (up to 5 minutes)

4. **Release the keyboard shortcut** when finished

5. A low beep confirms recording stopped

6. Transcription happens automatically (< 2 seconds typically)

7. Text appears at your cursor location

### Method 2: Floating Widget (Click-to-Record)

1. Click the floating widget at the bottom center of your screen

2. Speak into your microphone

3. Click again to stop recording

4. Text appears at your cursor location

### Canceling a Recording

Press **ESC** at any time during recording to cancel without transcribing or pasting.

## Floating Widget

The floating widget provides visual feedback during dictation:

### Collapsed State (Idle)
- Appears as a small black pill (5px × 30px) at screen bottom center
- 50% opacity
- Hover to see tooltip: "Click or hold [shortcut] to start dictating"

### Recording State
- Expands to show waveform visualization
- 3-5 white vertical bars animate based on your voice level
- Close button (X) on left to cancel
- Microphone icon on right

### Processing State
- Shows animated horizontal bar while transcribing
- Typically lasts 0.5-2 seconds

### Disabling the Widget
If you prefer keyboard-only operation, disable the floating widget in settings.

## Settings Configuration

### Microphone Selection
Choose which microphone to use for dictation. Vibe will detect all available input devices.

**Tip**: Select "Default" to use your system's default microphone.

### Whisper Model
Choose the Whisper model for transcription (see "Choose a Whisper Model" above).

### Keyboard Shortcut
Customize the global keyboard shortcut for dictation.

**Default Shortcuts:**
- macOS: Cmd+Shift+Space
- Windows/Linux: Ctrl+Alt+S

**Changing the Shortcut:**
1. Click in the keyboard shortcut field
2. Press your desired key combination
3. Vibe will warn you if the shortcut conflicts with system shortcuts

**Note**: Avoid common shortcuts like Cmd+S (Save), Cmd+C (Copy), etc.

### Show Floating Widget
Toggle the floating widget on/off. When disabled, dictation only works via keyboard shortcut.

### Audio Feedback
Toggle audio beeps (high beep on start, low beep on stop). Disable for silent operation.

## Dictation History

View all your past dictations in the **Dictation History** tab.

### Features

**Search**: Filter history by text content or destination app
- Real-time search as you type
- Highlights matching text

**Copy**: Click the copy button to copy transcription to clipboard

**Edit**: Click edit to modify the transcribed text
- Updates are saved immediately
- Original timestamp preserved

**Delete**: Remove entries you no longer need
- Confirmation dialog before deletion

**Automatic Cleanup**: Entries older than 30 days are deleted automatically

### History Metadata
Each entry shows:
- Timestamp (date and time)
- Destination application (where text was pasted)
- Transcribed text
- Model used
- Recording duration

## Tips for Best Results

### Audio Quality
- Speak clearly and at a normal pace
- Use a quality microphone when possible
- Reduce background noise
- Position microphone 6-12 inches from your mouth

### Recording Tips
- Pause briefly between sentences for better punctuation
- Avoid "um", "uh", and filler words
- Speak in complete phrases rather than individual words
- Don't worry about capitalization - Whisper handles it automatically

### Performance
- Small model provides best speed/accuracy balance for most users
- Use GPU acceleration (Metal on macOS, CUDA on NVIDIA GPUs) for faster transcription
- Shorter recordings (10-30 seconds) transcribe faster than long ones
- First transcription may be slower due to model loading

### Battery Life (Laptops)
- Widget uses minimal battery when idle (< 0.1% per hour)
- Recording uses ~1-2% battery per minute
- Transcription is a short burst (< 0.5% per dictation)
- Close Vibe when not in use to maximize battery life

## Troubleshooting

### "No Text Appears After Dictation"

**Possible Causes:**
1. Cursor not in a text field
2. No speech detected (silent recording)
3. Target application doesn't accept text input
4. Permissions not granted (macOS: Accessibility)

**Solutions:**
- Click in a text field before dictating
- Check microphone levels in system settings
- Check dictation history to see if transcription succeeded
- Grant required permissions (see "Grant Permissions" above)

### "Keyboard Shortcut Doesn't Work"

**Possible Causes:**
1. Shortcut conflicts with system or app shortcut
2. Permissions not granted (macOS: Accessibility)
3. Another app has registered the same shortcut

**Solutions:**
- Try a different keyboard shortcut
- Check System Preferences → Keyboard → Shortcuts for conflicts
- Grant Accessibility permission (macOS)
- Close other dictation apps that may use the same shortcut

### "Poor Transcription Accuracy"

**Possible Causes:**
1. Background noise
2. Poor microphone quality
3. Speaking too fast or unclear
4. Model too small (tiny/base)

**Solutions:**
- Use a better microphone
- Reduce background noise
- Speak more clearly and at normal pace
- Try a larger model (small → medium)
- Check microphone input levels (should peak around 50-70%)

### "Transcription Takes Too Long"

**Expected Latency** (Small model):
- 10-second recording: ~0.5-1 second
- 30-second recording: ~1-2 seconds
- 5-minute recording: ~10-20 seconds

**Solutions if slower:**
- Use a smaller model (small → tiny)
- Enable GPU acceleration if available
- Close other CPU-intensive applications
- Upgrade hardware (faster CPU/GPU)

### "Widget Doesn't Appear"

**Solutions:**
1. Check that "Show Floating Widget" is enabled in settings
2. Restart Vibe application
3. Check if widget is hidden behind other windows
4. Try changing screen resolution or reconnecting external displays

### "Microphone Not Detected"

**Solutions:**
1. Check that microphone is connected and enabled in system settings
2. Grant microphone permission (see "Grant Permissions" above)
3. Restart Vibe after connecting microphone
4. Try selecting "Default" microphone in settings

## Privacy

Vibe's dictation feature is **fully offline** and respects your privacy:

- All transcription happens locally on your device
- No data sent to cloud services
- History stored in local SQLite database only
- No telemetry or usage tracking
- Audio is immediately discarded after transcription
- You can delete history entries at any time

## Platform-Specific Notes

### macOS
- Best experience with Accessibility permissions granted
- Metal GPU acceleration provides excellent performance on Apple Silicon (M1/M2)
- Keyboard shortcut changed from Cmd+S to Cmd+Shift+Space to avoid conflicts

### Windows
- Requires Windows 10 or later
- GPU acceleration available with NVIDIA GPUs (CUDA)
- Low-level keyboard hooks may trigger antivirus warnings (false positive)

### Linux
- **X11**: Full feature support
- **Wayland**: Limited support for global keyboard shortcuts
  - May require XWayland compatibility layer
  - Consider running X11 session for best experience
- Tested on Ubuntu and Fedora

## Keyboard Shortcuts Reference

| Action | macOS | Windows/Linux |
|--------|-------|---------------|
| Start/Stop Dictation | Cmd+Shift+Space | Ctrl+Alt+S |
| Cancel Recording | ESC | ESC |
| Open Settings | Cmd+, | Ctrl+, |
| Open History | (Navigate via app menu) | (Navigate via app menu) |

## Advanced Usage

### Customizing for Different Use Cases

**Writing**: Use medium/large model for best punctuation and accuracy

**Coding**: Dictation works but requires verbal commands (e.g., "open bracket", "semicolon")

**Quick Notes**: Use tiny/small model for fastest response

**Long-Form Content**: Break into shorter segments (< 30 seconds) for faster transcription

### Multi-Language Support

Vibe's dictation uses the same language setting as the main transcription feature:
1. Go to main Vibe settings
2. Set your preferred language
3. Dictation will automatically use this language

**Supported**: Nearly all languages supported by Whisper (99+ languages)

## Getting Help

If you encounter issues not covered in this guide:

1. Check `docs/debug.md` for general troubleshooting
2. Review system logs for error messages
3. File an issue on GitHub with details:
   - Operating system and version
   - Microphone model
   - Steps to reproduce the issue
   - Vibe version

## Changelog

See `docs/changelog.md` for version history and feature updates.

---

**Version**: 1.0
**Last Updated**: 2025-10-30
