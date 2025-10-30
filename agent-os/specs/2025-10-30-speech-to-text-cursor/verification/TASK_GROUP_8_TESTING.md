# Task Group 8: Dictation Settings UI - Manual Testing Guide

**Status**: Implementation Complete
**Date**: 2025-10-30
**Component**: Dictation Settings Section in Settings Page

## Overview

This testing guide covers manual verification of the Dictation Settings UI component integrated into the Settings page. As there is no frontend test framework configured, all tests should be performed manually.

## Test Environment Setup

1. Start the Vibe application in development mode:
   ```bash
   cd desktop
   bunx tauri dev
   ```

2. Navigate to Settings by clicking the settings icon in the application

3. Scroll down to find the "Dictation" section

## Test Cases

### Test 8.1.1: Settings Component Renders All Controls

**Objective**: Verify that all dictation settings controls are visible and properly rendered.

**Steps**:
1. Open Settings
2. Scroll to the "Dictation" section
3. Verify the following controls are present:
   - Microphone device dropdown
   - Whisper model dropdown (showing: Tiny, Base, Small, Medium, Large)
   - Keyboard shortcut input field
   - "Show floating widget" toggle
   - "Audio feedback (beeps)" toggle
   - Usage instructions alert box

**Expected Result**:
- All controls are visible
- Labels have info tooltips (i icon)
- Default values are displayed correctly
- UI matches the existing settings page styling

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.2: Microphone Selection Saves Correctly

**Objective**: Verify microphone device selection persists correctly.

**Steps**:
1. Click on the microphone dropdown
2. Note the currently selected device
3. Select a different microphone device
4. Verify "Saved" toast notification appears
5. Close and reopen Settings
6. Verify the selected microphone is still selected

**Expected Result**:
- Dropdown shows all available input devices
- Selection triggers toast notification
- Selection persists after app restart (localStorage)
- Default devices are marked with "(Default)"

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.3: Model Selection Updates Settings

**Objective**: Verify Whisper model selection works and persists.

**Steps**:
1. Note the currently selected model (default should be "Small")
2. Open the model dropdown
3. Select a different model (e.g., "Medium - Slower, better accuracy")
4. Verify "Saved" toast notification appears
5. Close Settings and reopen
6. Verify the selected model persists

**Expected Result**:
- All 5 models are shown with descriptions:
  - Tiny - Fastest, lowest accuracy
  - Base - Fast, basic accuracy
  - Small - Balanced (recommended)
  - Medium - Slower, better accuracy
  - Large - Slowest, best accuracy
- Selection saves to localStorage
- Default is "small"

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.4: Keyboard Shortcut Customization Works

**Objective**: Verify keyboard shortcut input and conflict detection.

**Steps**:
1. Note the current keyboard shortcut
2. Click in the shortcut input field
3. Type a new shortcut (e.g., "Cmd+Option+D" on macOS)
4. Press Enter or click outside the field
5. Verify "Saved" toast appears
6. Try entering a conflicting shortcut (e.g., "Cmd+S")
7. Verify warning message appears below the input

**Expected Result**:
- Default shortcut is platform-specific:
  - macOS: Cmd+Shift+Space
  - Windows/Linux: Ctrl+Alt+S
- Custom shortcuts save on Enter or blur
- Conflicting shortcuts show warning:
  - "Warning: 'Cmd+S' conflicts with common system shortcuts..."
- Warning disappears when valid shortcut entered
- Hint text shows: "Default: [platform default]. Press Enter to save."

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.5: Toggle Floating Widget Updates State

**Objective**: Verify floating widget toggle immediately affects widget visibility.

**Steps**:
1. Ensure floating widget is visible (check bottom center of screen)
2. Toggle "Show floating widget" OFF
3. Verify:
   - Toast notification "Disabled" appears
   - Floating widget disappears immediately
4. Toggle "Show floating widget" ON
5. Verify:
   - Toast notification "Enabled" appears
   - Floating widget appears immediately
6. Close and reopen app
7. Verify widget visibility matches last saved state

**Expected Result**:
- Widget appears/disappears immediately on toggle
- Toast feedback on each toggle
- State persists across app restarts
- Default state is enabled (ON)

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.6: Toggle Audio Feedback

**Objective**: Verify audio feedback toggle works.

**Steps**:
1. Toggle "Audio feedback (beeps)" OFF
2. Verify "Disabled" toast appears
3. Toggle "Audio feedback (beeps)" ON
4. Verify "Enabled" toast appears
5. Close and reopen Settings
6. Verify toggle state persists

**Expected Result**:
- Toggle saves to localStorage
- Toast feedback on each change
- Default state is enabled (ON)
- State persists across restarts

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.7: Info Tooltips Display Correctly

**Objective**: Verify all info tooltips show helpful information.

**Steps**:
1. Hover over each info icon (i) next to labels
2. Read the tooltip content
3. Verify tooltips are clear and helpful

**Expected Tooltips**:
- **Microphone**: "Select the microphone device to use for speech dictation"
- **Dictation model**: "Choose the Whisper model for transcription. Larger models are more accurate but slower. Small is recommended for balanced performance."
- **Keyboard shortcut**: "Customize the keyboard shortcut for starting dictation. Hold the keys to record, release to transcribe. Avoid common system shortcuts."
- **Show floating widget**: "Show or hide the floating widget at the bottom of the screen. You can still use keyboard shortcuts even if hidden."
- **Audio feedback**: "Play audio beeps when starting and stopping recording to provide feedback"

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.8: Usage Instructions Display

**Objective**: Verify usage instructions are clear and helpful.

**Steps**:
1. Scroll to the bottom of the Dictation section
2. Read the usage instructions alert box

**Expected Content**:
- Title: "How to use:"
- Three bullet points:
  1. "Hold the keyboard shortcut to record, release to transcribe and paste"
  2. "Press ESC to cancel recording without pasting"
  3. "Maximum recording time is 5 minutes"
- Alert should be styled as an info alert (blue/informational)

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.9: Responsiveness and Layout

**Objective**: Verify settings section adapts to different window sizes.

**Steps**:
1. Resize the Settings window to minimum width
2. Verify all controls remain usable
3. Resize to maximum width
4. Verify layout remains consistent with other settings sections

**Expected Result**:
- All controls remain accessible at minimum width
- No horizontal scrolling required
- Layout matches existing settings sections
- Consistent spacing and alignment

**Status**: [ ] Pass / [ ] Fail

---

### Test 8.1.10: Integration with DictationProvider

**Objective**: Verify settings integrate correctly with global dictation state.

**Steps**:
1. Change all settings in the Dictation section
2. Navigate away from Settings
3. Check that FloatingWidget reflects the settings (visible/hidden)
4. Return to Settings
5. Verify all values are still correct

**Expected Result**:
- All settings use DictationProvider context
- Changes propagate to other components
- No localStorage inconsistencies
- Settings survive page navigation

**Status**: [ ] Pass / [ ] Fail

---

## Browser DevTools Testing

### Console Errors

**Steps**:
1. Open browser DevTools (F12)
2. Navigate to Console tab
3. Clear console
4. Open Settings and interact with Dictation section
5. Change all settings

**Expected Result**:
- No errors in console
- No warnings related to dictation settings
- Successful Tauri invoke logs (if verbose logging enabled)

**Status**: [ ] Pass / [ ] Fail

---

## Cross-Platform Testing

### macOS

- [ ] Default shortcut shows: "Cmd+Shift+Space"
- [ ] Shortcut input accepts Cmd key combinations
- [ ] All settings save correctly

### Windows

- [ ] Default shortcut shows: "Ctrl+Alt+S"
- [ ] Shortcut input accepts Ctrl/Alt combinations
- [ ] All settings save correctly

### Linux

- [ ] Default shortcut shows: "Ctrl+Alt+S"
- [ ] Shortcut input accepts Ctrl/Alt combinations
- [ ] All settings save correctly

---

## Known Limitations

1. **No automated tests**: Frontend test framework not configured, all testing is manual
2. **Shortcut conflict detection**: Only checks against hardcoded common shortcuts, not exhaustive
3. **Device refresh**: Audio devices list refreshes on dropdown focus, but may not detect hot-plugged devices immediately

---

## Test Summary

**Total Test Cases**: 10 manual tests + cross-platform verification

**Completed Tests**: _____ / 10

**Overall Status**: [ ] All Pass / [ ] Some Failures / [ ] Not Tested

---

## Issues Found

| Test # | Issue Description | Severity | Status |
|--------|------------------|----------|--------|
|        |                  |          |        |

---

## Sign-off

**Tester**: _________________________

**Date**: _________________________

**Notes**:
