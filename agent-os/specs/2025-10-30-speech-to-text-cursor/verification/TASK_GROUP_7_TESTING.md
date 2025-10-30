# Task Group 7: FloatingWidget Component - Testing Guide

## Components Created

1. **WaveformBars.tsx** - Audio visualization component
2. **FloatingWidget.tsx** - Main floating widget component
3. **Dictation.tsx** - Dictation settings provider

## Manual Testing Required

Since there is no automated frontend testing framework configured in the project, the following manual tests should be performed:

### Test 1: Widget Renders in Idle State
**Steps:**
1. Run the app with `bunx tauri dev`
2. Verify the floating widget appears at the bottom center of the screen
3. Check that it's displayed as a small pill (5px × 30px)
4. Verify it has 50% opacity (semi-transparent)

**Expected Result:**
- Widget is visible but subtle
- Positioned correctly (center horizontally, 100px from bottom)
- Pill-shaped with rounded corners

### Test 2: Widget Expands on Hover
**Steps:**
1. Hover mouse over the idle widget
2. Observe the transition animation

**Expected Result:**
- Widget expands smoothly to 30px height
- Opacity increases to 100%
- Tooltip appears showing "Click or hold [shortcut] to start dictating"
- Three animated dots appear in the widget
- Animation is smooth (60fps target)

### Test 3: Click to Start Recording Works
**Steps:**
1. Click the widget when in idle state
2. Observe state change

**Expected Result:**
- Widget expands to full size (30px height)
- Background remains black at 100% opacity
- Close button (X) appears on the left
- Stop button (square) appears on the right
- Waveform bars appear in the center
- Backend `start_dictation` command is invoked

### Test 4: Audio Level Visualization Updates
**Steps:**
1. Start recording (click widget or use keyboard shortcut)
2. Speak into the microphone
3. Observe the waveform bars

**Expected Result:**
- 5 white vertical bars are visible
- Bars animate smoothly based on audio levels
- Each bar moves slightly differently (visual variety)
- Animation runs at 30-60fps
- Bars respond to voice volume in real-time

### Test 5: State Transitions Render Correctly
**Steps:**
1. Test all state transitions:
   - Idle → Recording (click widget)
   - Recording → Processing (click stop button)
   - Processing → Idle (after transcription completes)
   - Recording → Idle (click cancel button)

**Expected Result:**
- Each state has distinct visual appearance
- Transitions are smooth with no visual glitches
- Processing state shows horizontal sliding bar animation
- Error state (if applicable) shows red background

### Test 6: Cancel Button Works
**Steps:**
1. Start recording
2. Click the X (cancel) button on the left

**Expected Result:**
- Recording stops immediately
- Widget returns to idle state
- Audio levels reset to 0
- Backend `cancel_dictation` command is invoked
- No transcription is performed

### Test 7: Settings Integration
**Steps:**
1. Open settings (when implemented in Task Group 8)
2. Toggle "Show floating widget" to OFF
3. Verify widget disappears
4. Toggle back to ON
5. Verify widget reappears

**Expected Result:**
- Widget visibility controlled by settings
- Changes take effect immediately
- Setting persists across app restarts

### Test 8: Keyboard Shortcut Display
**Steps:**
1. Hover over widget
2. Check tooltip text

**Expected Result:**
- Tooltip shows correct platform-specific shortcut:
  - macOS: "Click or hold Cmd+Shift+Space to start dictating"
  - Windows/Linux: "Click or hold Ctrl+Alt+S to start dictating"

### Test 9: Multi-Monitor Support
**Steps:**
1. If available, test on a multi-monitor setup
2. Move app between monitors

**Expected Result:**
- Widget appears on the primary monitor
- Widget stays centered horizontally on the active monitor

### Test 10: Performance Check
**Steps:**
1. Open browser DevTools (right-click widget, Inspect)
2. Go to Performance tab
3. Start profiling
4. Perform various interactions (hover, recording, etc.)
5. Stop profiling

**Expected Result:**
- Animations run at 60fps (16.67ms per frame or less)
- No dropped frames during state transitions
- Minimal CPU usage when idle
- Smooth animation during recording with audio visualization

## Backend Integration Points

The FloatingWidget expects the following Tauri commands to be available:
- `start_dictation` - Starts recording
- `stop_dictation` - Stops recording and begins transcription
- `cancel_dictation` - Cancels recording without transcription

The FloatingWidget listens for these events:
- `dictation_state_change` with payload: "idle" | "recording" | "processing" | "error"
- `audio_level_update` with payload: number (0.0 to 1.0)

**Note:** These backend implementations are part of subsequent task groups (3, 4, 5) and need to be completed for full functionality.

## Known Limitations

1. No automated tests (project doesn't have frontend testing framework)
2. Backend commands not yet implemented (will be added in later task groups)
3. Widget positioning may need adjustment based on platform-specific title bar heights
4. Audio level events from backend not yet implemented

## Files Created

- `/Users/rontiso/Development/vibe/desktop/src/components/FloatingWidget.tsx`
- `/Users/rontiso/Development/vibe/desktop/src/components/WaveformBars.tsx`
- `/Users/rontiso/Development/vibe/desktop/src/providers/Dictation.tsx`
- Updated: `/Users/rontiso/Development/vibe/desktop/src/App.tsx`
- Updated: `/Users/rontiso/Development/vibe/desktop/src-tauri/locales/en-US/common.json`

## Next Steps

Task Group 8: Dictation Settings Section will provide the UI controls to configure:
- Microphone selection
- Model selection
- Keyboard shortcut customization
- Widget visibility toggle
- Audio feedback toggle
