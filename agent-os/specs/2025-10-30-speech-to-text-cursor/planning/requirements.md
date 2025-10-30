# Spec Requirements: Speech-to-Text at Cursor

## Initial Description
Add a real-time speech-to-text feature that allows users to dictate text and have it automatically pasted where their cursor is located. This is similar to the Whisper Flow application.

Key capabilities:
- Keyboard shortcut trigger (user configurable)
- Floating button with frequency display showing audio level visualization
- Microphone selection in settings
- Listens, transcribes with Whisper, and auto-pastes at cursor position
- Currently missing from Vibe

## Requirements Discussion

### First Round Questions

**Q1: I'm assuming you want push-to-talk behavior (hold keyboard shortcut to record, release to stop) rather than toggle behavior (press once to start, press again to stop). Is that correct?**
**Answer:** Yes, push-to-talk (hold to record, release to stop)

**Q2: Should the transcription happen in real-time while speaking, or wait until you finish speaking (batch mode)?**
**Answer:** Batch mode - wait until finished speaking

**Q3: I assume there should be a maximum recording time (like 5 minutes) to prevent runaway sessions. Is that acceptable?**
**Answer:** Yes, 5 minutes maximum

**Q4: For the floating button position - should it be fixed at a specific location (like center bottom of screen), or draggable by the user?**
**Answer:** Fixed at center bottom of screen, 100px from bottom edge

**Q5: What size should the floating button be? I'm thinking something small and unobtrusive (maybe 5px height × 30px wide when collapsed, expanding when active)?**
**Answer:** Yes, 5px height × 30px wide (collapsed), expands to 30px height when active

**Q6: Should there be a way to cancel the recording (like pressing ESC) without pasting anything?**
**Answer:** Yes, ESC key cancels recording

**Q7: Should there be a preview before pasting, or immediately paste after transcription?**
**Answer:** No preview - immediate paste

**Q8: Should this work system-wide (in any application), or only when Vibe is focused?**
**Answer:** System-wide (works in any application)

**Q9: For punctuation - should Whisper automatically add punctuation, or transcribe exactly what's spoken?**
**Answer:** Automatic punctuation via Whisper

**Q10: Should users be able to select which Whisper model to use for this feature (trading off speed vs accuracy)?**
**Answer:** Yes, user-selectable Whisper model

**Q11: Should language settings be separate for this feature, or share the same language settings as the main app?**
**Answer:** Share settings with main app

**Q12: Should there be a history/log of all dictation transcriptions?**
**Answer:** Yes, log all transcriptions

**Q13: Is there anything you specifically DON'T want included in this feature?**
**Answer:** None

### Follow-up Questions

**Follow-up 1: For the waveform visualization - should it be a simple bar style (like audio levels) or more detailed frequency spectrum?**
**Answer:** Simple bars (audio level visualization)

**Follow-up 2: What color scheme for the floating widget? I'm thinking white bars on a black or dark gray button background?**
**Answer:** White bars on black button background

**Follow-up 3: You mentioned expanding to 30px height when active - should that be the maximum height or can it expand further?**
**Answer:** 30px is the maximum expanded height

**Follow-up 4: For the button states - should it show different visual states (idle, hovered, recording, processing)?**
**Answer:**
- Idle: Half transparent (50% opacity black)
- Hovered or Active: Full black (100% opacity) with white bars
- Active shows: Close button on left, record/stop button on right

**Follow-up 5: Should the button itself be clickable to start/stop recording, or just serve as a visual indicator?**
**Answer:** Yes, button is clickable as alternative to keyboard shortcut

**Follow-up 6: Should the keyboard shortcut be displayed on the button?**
**Answer:** No shortcut indicator on the button itself

**Follow-up 7: What should the default keyboard shortcut be for each platform?**
**Answer:**
- macOS: Cmd+S
- Windows/Linux: Ctrl+Alt+S

**Follow-up 8: Should the keyboard shortcuts be different per platform or consistent across all?**
**Answer:** Platform-specific

**Follow-up 9: For the transcription history - where should users access it (new tab, separate window, settings panel)?**
**Answer:** New tab in app

**Follow-up 10: Should the history be searchable, or just a chronological list? Should users be able to edit or copy from history?**
**Answer:** Yes - search, edit, copy capabilities

**Follow-up 11: Should there be a limit on how long history is kept (30 days, 90 days, forever)?**
**Answer:** Last 30 days

**Follow-up 12: Should the history include metadata like timestamp and which app it was pasted into?**
**Answer:** Yes, include timestamps and destination app name

**Follow-up 13: For system-wide functionality - should the app request/prompt for accessibility permissions on first use?**
**Answer:** Yes, prompt on first use with clear explanation for accessibility/input permissions

**Follow-up 14: Should the dictation settings be in a separate section or integrated into existing settings?**
**Answer:** Separate "Dictation" section

**Follow-up 15: Which Whisper model should be the default for this feature?**
**Answer:** Small (balanced speed/accuracy)

**Follow-up 16: Should there be an option to disable the floating button entirely and only use keyboard shortcuts?**
**Answer:** Yes, option to disable floating button entirely

**Follow-up 17: If the cursor is not in a text field when transcription completes, what should happen?**
**Answer:** Do nothing (user can retrieve from history)

**Follow-up 18: Should there be any audio feedback when starting/stopping recording?**
**Answer:**
- High beep when starting recording
- Lower beep when stopping recording

**Follow-up 19: What happens if the transcription result is empty (no speech detected)?**
**Answer:** Fail silently

**Follow-up 20: Should there be visual feedback during processing (after release but before paste)?**
**Answer:** Yes, bars animate horizontally (loading animation)

**Follow-up 21: What should happen if the user presses the keyboard shortcut rapidly multiple times?**
**Answer:** Ignore (don't queue or cancel)

### Existing Code to Reference

**Similar Features Identified:**
- Audio capture: `core/src/audio.rs` - handles microphone device enumeration and audio capture
- Whisper integration: `core/src/transcribe.rs` - transcription engine
- Settings management: Tauri settings system already in place
- Microphone transcription: Existing feature for file creation can be referenced

**Technical Components to Leverage:**
- `whisper-rs` bindings (already integrated)
- Audio device handling from `audio.rs`
- Configuration system from `config.rs`
- Frontend: React components with TailwindCSS/DaisyUI styling

## Visual Assets

### Files Provided:
- `image.png`: Screenshot of Whisper Flow application showing the main interface with transcription history, sidebar navigation (Home, Dictionary, Snippets, Style, Notes), and a "Transcript copied" toast notification at bottom right
- `image copy.png`: Close-up of the collapsed floating button in idle state - appears as a small black bar at screen bottom
- `image copy 2.png`: Close-up of the expanded floating button showing "Click or hold Ctrl + s to start dictating" tooltip with animated dots indicating idle state

### Visual Insights:

**From Reference Application (Whisper Flow):**
1. **Floating Widget Design:**
   - Compact horizontal bar that sits at bottom center of screen
   - Black background with rounded corners
   - Expands vertically when activated
   - Shows instructional text on hover ("Click or hold Ctrl + s to start dictating")
   - Animated dots/bars for audio visualization during recording

2. **UI State Transitions:**
   - Collapsed idle state: Minimal visual footprint (thin bar)
   - Hover state: Shows tooltip with instructions
   - Active/recording state: Expands to show audio visualization
   - Processing state: Animated loading indicator

3. **Design Patterns:**
   - Clean, minimal aesthetic
   - High contrast (white on black)
   - Unobtrusive positioning (doesn't block content)
   - Toast notifications for completion ("Transcript copied")

4. **History Interface:**
   - Chronological list with timestamps
   - Shows full transcribed text
   - Clean typography with good readability
   - Sidebar navigation for easy access

**Fidelity Level:** High-fidelity screenshots (actual reference application)

## Requirements Summary

### Functional Requirements

#### Recording & Transcription
- **Push-to-talk behavior:** Hold keyboard shortcut to record, release to stop
- **Batch transcription:** Wait until user finishes speaking, then transcribe complete audio
- **5-minute session timeout:** Prevent runaway recording sessions
- **System-wide text pasting:** Works in any application where cursor is located
- **Automatic punctuation:** Whisper automatically adds punctuation to transcription
- **ESC cancellation:** Cancel recording without pasting by pressing ESC
- **Immediate paste:** No preview step - paste transcription directly at cursor
- **User-selectable Whisper model:** Choose model in settings (default: small)
- **Shared language settings:** Use same language configuration as main app
- **Empty transcription handling:** Fail silently if no speech detected

#### Floating Widget
- **Fixed position:** Center bottom of screen, 100px from bottom edge
- **Collapsed dimensions:** 5px height × 30px wide
- **Expanded dimensions:** 30px height × 30px wide
- **Click-to-record:** Alternative to keyboard shortcut
- **Interactive elements:** Close button (left), Record/Stop button (right) when active
- **Waveform visualization:** Simple audio level bars (white on black)
- **State-based appearance:**
  - Idle: 50% opacity black
  - Hovered/Active: 100% opacity black with white bars
- **No shortcut indicator:** Button does not display keyboard shortcut text

#### Keyboard Shortcuts
- **Platform-specific defaults:**
  - macOS: Cmd+S
  - Windows/Linux: Ctrl+Alt+S
- **Configurable in settings:** Users can customize shortcuts
- **Hold-to-record behavior:** Recording starts on key down, stops on key up
- **Rapid press handling:** Ignore subsequent presses during active session

#### Transcription History
- **30-day retention:** Keep history for last 30 days
- **Searchable interface:** Search through past transcriptions
- **Edit/copy capabilities:** Users can edit and copy historical transcriptions
- **Metadata tracking:** Include timestamp and destination app name for each entry
- **Access location:** New tab in main application
- **Chronological display:** List most recent transcriptions first

#### Settings & Configuration
- **Dedicated "Dictation" section:** Separate settings area for this feature
- **Microphone selection:** Choose which microphone to use for dictation
- **Whisper model selection:** Choose transcription model (default: small)
- **Keyboard shortcut customization:** Configure platform-specific shortcuts
- **Toggle floating widget:** Option to disable floating button entirely (keyboard-only mode)
- **Language settings:** Share language configuration with main app

#### System Integration
- **System-wide keyboard hook:** Capture keyboard shortcuts globally
- **Accessibility permissions:** Request on first use (macOS)
- **Input simulation permissions:** Request on first use (Windows/Linux)
- **Permission prompting:** Clear explanation of why permissions are needed
- **Clipboard/text pasting API:** Simulate text input at cursor location
- **Destination app detection:** Capture name of application where text is pasted

### UI/UX Specifications

#### Widget Design - Idle State
- **Dimensions:** 5px height × 30px wide
- **Position:** Center horizontally, 100px from bottom of screen
- **Appearance:** 50% opacity black background
- **Shape:** Rounded corners (pill-shaped)
- **Visibility:** Semi-transparent to minimize distraction

#### Widget Design - Hovered State
- **Opacity change:** Transitions to 100% opacity black
- **Visual feedback:** Subtle scale or glow effect (optional)
- **Tooltip:** "Click or hold [shortcut] to start dictating"
- **Cursor:** Changes to pointer to indicate clickable

#### Widget Design - Active/Recording State
- **Dimensions:** 30px height × 30px wide (or wider for controls)
- **Background:** 100% opacity black
- **Waveform bars:** White vertical bars showing audio level
- **Left control:** Close/Cancel button (X icon)
- **Right control:** Record/Stop button (microphone icon with stop indicator)
- **Animation:** Bars animate in real-time with audio input

#### Widget Design - Processing State
- **Dimensions:** Remains at 30px height
- **Background:** 100% opacity black
- **Animation:** White bars animate horizontally (loading/processing indicator)
- **User feedback:** Visual indication that transcription is in progress
- **No interaction:** Controls disabled during processing

#### Visual Feedback Elements
- **Waveform visualization:**
  - Simple vertical bars (not complex frequency spectrum)
  - White color on black background
  - 3-5 bars updating in real-time
  - Height varies with audio input level

- **Loading animation:**
  - Horizontal bar movement (left to right)
  - Smooth animation at 60fps
  - Indicates transcription in progress

- **State transitions:**
  - Smooth opacity fades (0.2s duration)
  - Height expansion animation (0.3s ease-out)
  - Button appearance/disappearance with fade

#### Audio Feedback
- **Recording start:** High-pitched beep (750Hz, 100ms duration)
- **Recording stop:** Lower-pitched beep (500Hz, 100ms duration)
- **Volume:** Match system volume, but not too loud
- **User control:** Option to disable audio feedback in settings (optional)

#### History Interface
- **Layout:** Full-width tab in main application
- **List view:** Chronological entries with most recent first
- **Entry components:**
  - Timestamp (time and date)
  - Destination app name/icon
  - Full transcribed text
  - Action buttons: Copy, Edit, Delete

- **Search functionality:**
  - Search bar at top of history view
  - Filter by text content, app name, or date range
  - Real-time search results

- **Empty state:** Friendly message when no history exists

### Technical Requirements

#### Audio Processing
- **Microphone selection:** Use existing `audio.rs` device enumeration
- **Real-time audio capture:** Stream audio from selected microphone
- **Audio buffer management:** Accumulate audio until release (max 5 minutes)
- **Format conversion:** Ensure audio format compatible with Whisper (16kHz, mono)
- **Audio level detection:** Calculate RMS or peak levels for waveform visualization

#### Whisper Integration
- **Model loading:** Use existing `transcribe.rs` infrastructure
- **Async transcription:** Process audio in background thread (don't block UI)
- **Model selection:** Support user-selected model (tiny, base, small, medium, large)
- **Language parameter:** Pass language from shared app settings
- **Punctuation:** Enable automatic punctuation in Whisper configuration
- **Error handling:** Catch transcription failures gracefully

#### System Integration - Keyboard Hooks
- **Global hotkey registration:** Platform-specific keyboard hook APIs
  - macOS: Carbon or Cocoa event taps
  - Windows: RegisterHotKey API
  - Linux: X11 or Wayland input capture
- **Key down/up events:** Distinguish between press and release
- **Modifier key handling:** Support complex shortcuts (Cmd+Shift+S, etc.)
- **Conflict detection:** Warn if shortcut already in use by system/app

#### System Integration - Text Pasting
- **Clipboard simulation:**
  - Option 1: Copy to clipboard and send Cmd/Ctrl+V
  - Option 2: Direct text input simulation (more reliable)
- **Platform-specific APIs:**
  - macOS: NSPasteboard or CGEvent text input
  - Windows: SendInput API or clipboard
  - Linux: X11 XTest or Wayland input simulation
- **Cursor position detection:** Determine if cursor is in text field
- **Focus preservation:** Maintain focus in target application

#### System Integration - Permissions
- **macOS permissions:**
  - Accessibility (required for keyboard hooks and text input)
  - Microphone (required for audio capture)
  - Prompt with clear explanation of purpose

- **Windows permissions:**
  - Low-level keyboard hook (usually no prompt)
  - Microphone access (Windows 10+ privacy settings)

- **Linux permissions:**
  - Input device access (may require user group membership)
  - Microphone access (PulseAudio/PipeWire)

- **Permission flow:**
  - Check permissions on feature first use
  - Show dialog explaining why permissions are needed
  - Provide link to system settings if denied
  - Gracefully degrade if permissions not granted

#### Frontend Implementation
- **Framework:** React with TypeScript
- **Styling:** TailwindCSS + DaisyUI
- **State management:** React hooks (useState, useEffect)
- **Tauri commands:** Invoke Rust backend for audio/transcription
- **Floating window:** Tauri window API or overlay component
- **Animations:** CSS transitions and keyframe animations

#### Backend Implementation (Rust)
- **Tauri commands:**
  - `start_dictation()` - Initialize recording
  - `stop_dictation()` - Stop and transcribe
  - `cancel_dictation()` - Cancel without transcribing
  - `get_audio_levels()` - Real-time audio level for visualization
  - `paste_text(text: String)` - Paste at cursor
  - `get_dictation_history()` - Retrieve history

- **State management:** Arc<Mutex<DictationState>> for thread-safe state
- **Audio streaming:** Continuous audio capture during recording
- **Platform-specific modules:** Separate modules for macOS/Windows/Linux text pasting

#### Data Storage
- **History database:** SQLite or JSON file in app data directory
- **Schema:**
  - id (primary key)
  - timestamp (ISO 8601)
  - transcription_text (full text)
  - destination_app (app name/bundle ID)
  - model_used (Whisper model)
  - duration_seconds (recording length)

- **Cleanup:** Automatic deletion of entries older than 30 days
- **Privacy:** Store locally only, no cloud sync

### Edge Cases & Error Handling

1. **Cursor not in text field:**
   - Detection: Attempt to paste, catch failure
   - Behavior: Do nothing (silently fail)
   - User recovery: User can retrieve transcription from history tab

2. **Empty transcription (no speech detected):**
   - Detection: Whisper returns empty string or silence
   - Behavior: Fail silently (no paste, no notification)
   - Optional: Log to history with note "No speech detected"

3. **Rapid shortcut presses:**
   - Detection: Key press while session active
   - Behavior: Ignore subsequent presses (debounce)
   - Prevent: State machine ensures single active session

4. **5-minute timeout reached:**
   - Detection: Recording duration exceeds 300 seconds
   - Behavior: Auto-stop recording and begin transcription
   - Notification: Optional toast notification "Recording limit reached"

5. **ESC pressed during recording:**
   - Detection: ESC key event during active session
   - Behavior: Cancel recording immediately (no transcription)
   - State: Return widget to idle state
   - Audio feedback: Optional cancellation sound

6. **Transcription failure:**
   - Causes: Model not loaded, invalid audio, system error
   - Behavior: Show error notification to user
   - Recovery: Audio remains in buffer, user can retry
   - Logging: Log error details for debugging

7. **Permissions denied:**
   - Detection: Permission check returns false
   - Behavior: Show informative dialog with remediation steps
   - Fallback: Disable feature until permissions granted
   - Settings: Provide button to open system settings

8. **Microphone disconnected during recording:**
   - Detection: Audio stream error
   - Behavior: Stop recording, show error notification
   - Recovery: Transcribe audio captured so far (if any)

9. **System audio device changed:**
   - Detection: Audio device enumeration change
   - Behavior: Refresh device list in settings
   - Active session: Continue with current device until release

10. **App loses focus during recording:**
    - Behavior: Continue recording (system-wide feature)
    - Widget visibility: Widget remains visible on all desktops/spaces

11. **Multiple monitors:**
    - Widget position: Show on primary monitor (or monitor with cursor)
    - Settings: Option to choose which monitor (future enhancement)

12. **Extremely long transcription:**
    - Detection: Transcription text exceeds reasonable length
    - Behavior: Paste normally, may take longer
    - History: Store full text (no truncation)

13. **Clipboard contains sensitive data:**
    - Behavior: Text pasting via input simulation preserves clipboard
    - Alternative: If using clipboard method, restore original clipboard content after paste

14. **Whisper model not downloaded:**
    - Detection: Model file missing
    - Behavior: Prompt user to download model
    - Fallback: Use smallest available model automatically

### Platform Considerations

#### macOS
- **Keyboard shortcut:** Cmd+S (default)
- **Shortcut conflict:** **CRITICAL ISSUE** - Cmd+S is universal Save shortcut
  - Likely to be intercepted by active application before reaching Vibe
  - Strong recommendation: Change default to Cmd+Shift+Space or Cmd+Option+Space
  - Users may experience frustration with Save command not working

- **Accessibility API:**
  - Requires TCC (Transparency, Consent, and Control) approval
  - Use AXUIElement for text input simulation
  - CGEvent for keyboard event simulation

- **Permissions flow:**
  - Microphone: Standard macOS prompt
  - Accessibility: Manual approval in System Preferences > Security & Privacy
  - Provide clear instructions with screenshots

- **Window management:**
  - Floating widget should appear above all windows (NSFloatingWindowLevel)
  - Respect full-screen mode and Spaces

#### Windows
- **Keyboard shortcut:** Ctrl+Alt+S (default)
- **Less conflict risk:** Not a standard system shortcut
- **Keyboard hook:** RegisterHotKey or low-level keyboard hook (WH_KEYBOARD_LL)
- **Text input:** SendInput API with KEYEVENTF_UNICODE
- **Permissions:** Windows 10+ microphone privacy settings
- **Window management:** HWND_TOPMOST for always-on-top floating widget

#### Linux
- **Keyboard shortcut:** Ctrl+Alt+S (default)
- **Display server:** Support both X11 and Wayland
  - X11: XGrabKey for hotkeys, XTest for input simulation
  - Wayland: Limited support, may require compositor-specific protocols

- **Permissions:**
  - Input group membership may be required
  - PulseAudio/PipeWire for microphone access

- **Window management:** X11 override-redirect or Wayland layer-shell
- **Desktop environments:** Test with GNOME, KDE, XFCE

### Performance Requirements

1. **Transcription latency:** < 2 seconds from release to paste (small model, typical length)
2. **Widget responsiveness:** 60fps animations, < 16ms frame time
3. **Audio buffer:** Efficient ring buffer, minimal memory overhead
4. **Waveform update rate:** 30-60fps (sufficient for smooth visualization)
5. **History search:** < 100ms for typical history size (< 1000 entries)
6. **Startup impact:** Minimal - don't load Whisper models until first use

### Accessibility Requirements

1. **Keyboard-only operation:** Full functionality without mouse (already covered)
2. **Screen reader support:** Announce widget state changes (optional enhancement)
3. **High contrast mode:** Widget respects system high contrast settings (optional)
4. **Customizable shortcuts:** Essential for users with motor impairments
5. **Visual feedback:** Complement audio feedback with visual indicators

## Open Questions & Critical Decisions

### CRITICAL ISSUE: Keyboard Shortcut Conflict

**Problem:** Cmd+S on macOS is the universal Save shortcut used by virtually every application. This will likely cause conflicts:
- Applications will intercept Cmd+S before Vibe receives it
- Users attempting to save will trigger dictation instead
- Creates frustration and breaks expected behavior

**Recommended Alternatives:**
1. **Cmd+Shift+Space** (Primary recommendation)
   - Not commonly used by applications
   - Space bar association with voice/speech is intuitive
   - Easy to press with one hand

2. **Cmd+Option+Space** (Alternative)
   - Similar to Spotlight (Cmd+Space)
   - Less likely to conflict

3. **Ctrl+Shift+Space** (Cross-platform consistent)
   - Same shortcut on all platforms
   - Avoids platform-specific conflicts

4. **Let user choose during onboarding**
   - Present conflict warning
   - Offer recommended alternatives
   - Allow custom configuration

**Action Required:** Decision needed before specification finalized

### Secondary Questions

1. **Multi-monitor support:**
   - Should widget appear on monitor with active cursor?
   - Or always on primary monitor?
   - User configurable in settings?

2. **Widget persistence:**
   - Remember enabled/disabled state across app restarts?
   - Remember last used microphone?
   - Default: enabled or disabled on fresh install?

3. **Notification preferences:**
   - Toast notification on paste completion?
   - Sound-only feedback?
   - User preference in settings?

4. **Model auto-download:**
   - Auto-download default model (small) on first use?
   - Or require explicit user action?
   - Background download vs blocking?

5. **History privacy:**
   - Option to disable history entirely?
   - Clear history button?
   - Export history to file?

6. **Punctuation customization:**
   - Always use automatic punctuation?
   - Or toggle in settings?
   - Language-specific punctuation rules?

7. **Widget customization:**
   - Allow users to change colors?
   - Adjust size/position?
   - Or keep simple and consistent?

### Existing Code Reuse Opportunities

**Confirmed Components to Leverage:**
- `core/src/audio.rs` - Audio device enumeration and capture
- `core/src/transcribe.rs` - Whisper transcription engine
- `core/src/config.rs` - Configuration management
- `desktop/src-tauri/src/setup.rs` - App initialization patterns
- `desktop/src/components/` - React component patterns
- TailwindCSS + DaisyUI styling conventions

**Patterns to Investigate:**
- Settings page structure and layout
- Tauri command patterns for async operations
- Error handling and user notification patterns
- Model download and management UI

**Recommendation for Spec Writer:**
- Review existing settings implementation for consistency
- Study microphone selection UI from existing transcription features
- Reference Tauri window management code for floating widget
- Check i18n patterns for adding new translatable strings

## Success Criteria

### Functional Success
- User can activate dictation via keyboard shortcut from any application
- Transcription accurately reflects spoken content with proper punctuation
- Text is pasted at cursor location in target application
- History tab displays all transcriptions with search/edit/copy functionality
- Widget provides clear visual feedback for all states (idle/recording/processing)

### Performance Success
- Transcription latency < 2 seconds for small model with typical 10-30 second clips
- Widget animations maintain 60fps with no stuttering
- Audio visualization updates smoothly in real-time (30+ fps)
- No noticeable system performance impact when idle
- History search returns results in < 100ms

### Usability Success
- New users can discover and activate feature within 2 minutes
- Keyboard shortcuts feel natural and don't conflict with common shortcuts
- Widget is visible but unobtrusive, doesn't block content
- Permissions are requested with clear explanations
- Error states are handled gracefully with helpful guidance

### Quality Success
- No crashes or hangs during normal operation
- Graceful degradation when permissions denied
- Clear error messages for all failure modes
- Transcription accuracy matches main app performance
- Cross-platform consistency in behavior and appearance

### Integration Success
- Works reliably across common applications (browsers, text editors, chat apps)
- Respects existing Vibe settings (language, model preferences)
- Reuses existing audio/transcription infrastructure
- Follows established UI/UX patterns in Vibe
- Maintains Vibe's offline-first privacy guarantee

## Next Steps

1. **Resolve keyboard shortcut conflict** - Critical decision needed for macOS default
2. **Review and approve requirements** - Ensure all stakeholder agreement
3. **Create detailed UI mockups** - More refined design than reference screenshots
4. **Write technical specification** - Detailed implementation plan
5. **Break down into implementation tasks** - Sprint planning and estimation
6. **Set up development environment** - Ensure all dependencies available
7. **Implement core functionality** - Audio capture → Transcription → Paste
8. **Build floating widget UI** - React component with animations
9. **Add history feature** - Database, UI, search functionality
10. **Cross-platform testing** - Verify on macOS, Windows, Linux
11. **Permissions flow testing** - Smooth user experience for permission requests
12. **Performance optimization** - Meet latency and FPS targets
13. **Documentation** - User guide and technical docs
14. **Beta testing** - Gather user feedback before release
