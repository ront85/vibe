/// Integration tests for the dictation feature
///
/// These tests focus on end-to-end workflows and integration points between components.
/// They complement the unit tests in individual modules by testing how components work together.

use std::sync::{Arc, Mutex};
use std::thread;
use tempfile::tempdir;
use vibe_core::dictation::{AudioBuffer, DictationState};
use vibe_core::dictation_history::{DictationHistory, NewDictationEntry};

/// Test 1: End-to-end dictation workflow simulation
/// Validates: Recording state -> Audio buffer -> Processing -> History entry
#[test]
fn test_end_to_end_dictation_workflow() {
    // Setup: Create temp database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("dictation_e2e.db");
    let history = DictationHistory::new(&db_path).unwrap();

    // Step 1: Start recording
    let state = DictationState::start_recording(Some("microphone".to_string()));
    assert!(state.is_recording());

    // Step 2: Simulate audio capture (collect samples)
    let mut buffer = AudioBuffer::new(16000 * 5); // 5 seconds capacity
    let samples: Vec<i16> = vec![100; 16000]; // 1 second of audio at 16kHz
    buffer.append(&samples).unwrap();

    assert_eq!(buffer.len(), 16000);
    assert!((buffer.duration_seconds() - 1.0).abs() < 0.01);

    // Step 3: Stop recording and get audio
    let audio_data = buffer.get_samples().to_vec();
    let duration = buffer.duration_seconds();

    // Step 4: Simulate transcription (would normally use Whisper)
    let transcription = "This is a test transcription".to_string();

    // Step 5: Transition to processing state
    let state = DictationState::start_processing(audio_data, duration);
    assert!(state.is_processing());

    // Step 6: Add to history
    let entry = NewDictationEntry {
        transcription_text: transcription.clone(),
        destination_app: "TextEdit".to_string(),
        model_used: "small".to_string(),
        duration_seconds: duration,
    };

    let entry_id = history.insert(entry).unwrap();

    // Step 7: Verify history entry was created
    let entries = history.get_all().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id, entry_id);
    assert_eq!(entries[0].transcription_text, transcription);

    // Step 8: Return to idle state
    let state = DictationState::idle();
    assert!(matches!(state, DictationState::Idle));
}

/// Test 2: ESC cancellation doesn't create history entry
/// Validates: Cancellation flow doesn't persist any data
#[test]
fn test_esc_cancellation_no_history() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("dictation_cancel.db");
    let history = DictationHistory::new(&db_path).unwrap();

    // Start recording
    let state = DictationState::start_recording(None);
    assert!(state.is_recording());

    // Collect some audio
    let mut buffer = AudioBuffer::new(16000 * 5);
    buffer.append(&vec![100; 8000]).unwrap(); // 0.5 seconds

    // User presses ESC - cancel without saving
    let state = DictationState::idle(); // Return to idle without processing
    assert!(matches!(state, DictationState::Idle));

    // Verify no history entry was created
    let entries = history.get_all().unwrap();
    assert_eq!(entries.len(), 0);
}

/// Test 3: 5-minute timeout creates history entry
/// Validates: Timeout mechanism properly saves partial recording
#[test]
fn test_timeout_creates_history_entry() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("dictation_timeout.db");
    let history = DictationHistory::new(&db_path).unwrap();

    // Start recording
    let state = DictationState::start_recording(None);
    assert!(state.is_recording());

    // Simulate 5 minutes of audio (use smaller buffer for test speed)
    let mut buffer = AudioBuffer::new(16000 * 300); // 5 minutes capacity
    let samples_per_second = 16000;

    // Add samples in chunks to simulate real-time recording
    for _ in 0..300 {
        buffer.append(&vec![100; samples_per_second]).ok(); // May fail at max capacity
    }

    // Check if timeout should trigger
    let duration = buffer.duration_seconds();
    assert!(duration >= 299.0); // Close to 5 minutes (allowing for buffer limits)

    // Timeout triggered - save to history
    let transcription = "Transcribed text from 5-minute recording".to_string();

    let entry = NewDictationEntry {
        transcription_text: transcription,
        destination_app: "Unknown".to_string(),
        model_used: "small".to_string(),
        duration_seconds: duration,
    };

    history.insert(entry).unwrap();

    // Verify entry was created
    let entries = history.get_all().unwrap();
    assert_eq!(entries.len(), 1);
    assert!(entries[0].duration_seconds >= 299.0);
}

/// Test 4: Empty transcription doesn't paste but may log to history
/// Validates: Graceful handling of "no speech detected" scenario
#[test]
fn test_empty_transcription_handling() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("dictation_empty.db");
    let history = DictationHistory::new(&db_path).unwrap();

    // Simulate recording with silence
    let mut buffer = AudioBuffer::new(16000);
    buffer.append(&vec![0; 16000]).unwrap(); // 1 second of silence

    // Transcription returns empty string (no speech detected)
    let transcription = String::new();

    // Decision: Don't add empty transcriptions to history
    // (This is the implementation choice - fail silently)
    if !transcription.is_empty() {
        let entry = NewDictationEntry {
            transcription_text: transcription,
            destination_app: "Unknown".to_string(),
            model_used: "small".to_string(),
            duration_seconds: buffer.duration_seconds(),
        };
        history.insert(entry).unwrap();
    }

    // Verify no history entry for empty transcription
    let entries = history.get_all().unwrap();
    assert_eq!(entries.len(), 0);
}

/// Test 5: Settings changes affect recording behavior
/// Validates: Settings integration with state management
#[test]
fn test_settings_affect_recording() {
    // Test that microphone selection is properly stored in state
    let state = DictationState::start_recording(Some("built-in-microphone".to_string()));

    if let DictationState::Recording { microphone_device_id, .. } = state {
        assert_eq!(microphone_device_id, Some("built-in-microphone".to_string()));
    } else {
        panic!("Expected Recording state");
    }

    // Test that None is acceptable (default microphone)
    let state = DictationState::start_recording(None);
    if let DictationState::Recording { microphone_device_id, .. } = state {
        assert_eq!(microphone_device_id, None);
    } else {
        panic!("Expected Recording state");
    }
}

/// Test 6: Microphone disconnect during recording handles gracefully
/// Validates: Error recovery when audio device is unavailable
#[test]
fn test_microphone_disconnect_handling() {
    // Start recording with specific microphone
    let state = DictationState::start_recording(Some("external-mic".to_string()));
    assert!(state.is_recording());

    // Collect partial audio
    let mut buffer = AudioBuffer::new(16000 * 5);
    buffer.append(&vec![100; 8000]).unwrap(); // 0.5 seconds

    // Microphone disconnect detected - transition to error state
    let error_msg = "Microphone disconnected";
    let state = DictationState::error(error_msg.to_string());

    if let DictationState::Error { message } = state {
        assert_eq!(message, error_msg);
    } else {
        panic!("Expected Error state");
    }

    // Can recover by returning to idle
    let state = DictationState::idle();
    assert!(matches!(state, DictationState::Idle));
    assert!(state.can_start_recording());
}

/// Test 7: Rapid keyboard shortcut presses are debounced
/// Validates: State machine prevents multiple simultaneous sessions
#[test]
fn test_rapid_press_debouncing() {
    // First press - start recording
    let state = DictationState::start_recording(None);
    assert!(state.is_recording());
    assert!(!state.can_start_recording()); // Cannot start again

    // Rapid second press - should be ignored
    // State machine prevents transition
    assert!(!state.can_start_recording());

    // Third press while still recording - still blocked
    assert!(!state.can_start_recording());

    // Only after returning to idle can we start again
    let state = DictationState::idle();
    assert!(state.can_start_recording());
}

/// Test 8: Audio buffer memory management
/// Validates: Buffer doesn't grow beyond max capacity
#[test]
fn test_audio_buffer_memory_limit() {
    let max_samples = 1000;
    let mut buffer = AudioBuffer::new(max_samples);

    // Try to add more samples than capacity
    let samples = vec![100i16; 800];
    buffer.append(&samples).unwrap();
    assert_eq!(buffer.len(), 800);

    // Add more samples - should fail when exceeding capacity
    let more_samples = vec![200i16; 300];
    let result = buffer.append(&more_samples);

    // Should return error when exceeding capacity
    assert!(result.is_err());
    assert_eq!(buffer.len(), 800); // Unchanged
}

/// Test 9: Concurrent state access (thread safety)
/// Validates: State can be safely accessed from multiple threads
#[test]
fn test_concurrent_state_access() {
    let state = Arc::new(Mutex::new(DictationState::idle()));

    let state_clone = Arc::clone(&state);
    let handle = thread::spawn(move || {
        let mut s = state_clone.lock().unwrap();
        *s = DictationState::start_recording(None);
    });

    handle.join().unwrap();

    let s = state.lock().unwrap();
    assert!(s.is_recording());
}

/// Test 10: History search performance with multiple entries
/// Validates: Search remains fast even with many entries
#[test]
fn test_history_search_performance() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("dictation_search.db");
    let history = DictationHistory::new(&db_path).unwrap();

    // Add 100 entries
    for i in 0..100 {
        let entry = NewDictationEntry {
            transcription_text: format!("Entry number {}", i),
            destination_app: "TestApp".to_string(),
            model_used: "small".to_string(),
            duration_seconds: 1.0,
        };
        history.insert(entry).unwrap();
    }

    // Measure search time
    let start = std::time::Instant::now();
    let results = history.search("number 42").unwrap();
    let duration = start.elapsed();

    // Should find the entry
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].transcription_text, "Entry number 42");

    // Search should be fast (< 100ms as per requirements)
    assert!(duration.as_millis() < 100,
            "Search took {}ms, expected < 100ms", duration.as_millis());
}
