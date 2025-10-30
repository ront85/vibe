use eyre::Result;
use std::sync::Arc;
use std::time::Instant;

/// Maximum recording duration in seconds (5 minutes)
pub const MAX_RECORDING_DURATION_SECS: u64 = 300;

/// Maximum audio buffer size (5 minutes at 16kHz mono, 16-bit samples)
/// 16000 samples/sec * 300 sec = 4,800,000 samples
pub const MAX_AUDIO_BUFFER_SIZE: usize = 4_800_000;

#[derive(Debug, Clone, PartialEq)]
pub enum DictationState {
    Idle,
    Recording {
        start_time: Instant,
        audio_buffer: Arc<Vec<i16>>,
        microphone_device_id: Option<String>,
    },
    Processing {
        audio_data: Vec<i16>,
        duration_seconds: f64,
    },
    Error {
        message: String,
    },
}

impl Default for DictationState {
    fn default() -> Self {
        Self::Idle
    }
}

impl DictationState {
    /// Check if state allows starting a new recording
    pub fn can_start_recording(&self) -> bool {
        matches!(self, DictationState::Idle | DictationState::Error { .. })
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        matches!(self, DictationState::Recording { .. })
    }

    /// Check if currently processing
    pub fn is_processing(&self) -> bool {
        matches!(self, DictationState::Processing { .. })
    }

    /// Get recording duration if recording
    pub fn recording_duration(&self) -> Option<std::time::Duration> {
        if let DictationState::Recording { start_time, .. } = self {
            Some(start_time.elapsed())
        } else {
            None
        }
    }

    /// Check if recording has exceeded maximum duration
    pub fn has_exceeded_max_duration(&self) -> bool {
        if let Some(duration) = self.recording_duration() {
            duration.as_secs() >= MAX_RECORDING_DURATION_SECS
        } else {
            false
        }
    }

    /// Transition to Recording state
    pub fn start_recording(microphone_device_id: Option<String>) -> Self {
        DictationState::Recording {
            start_time: Instant::now(),
            audio_buffer: Arc::new(Vec::with_capacity(MAX_AUDIO_BUFFER_SIZE)),
            microphone_device_id,
        }
    }

    /// Transition to Processing state
    pub fn start_processing(audio_data: Vec<i16>, duration_seconds: f64) -> Self {
        DictationState::Processing {
            audio_data,
            duration_seconds,
        }
    }

    /// Transition to Error state
    pub fn error(message: String) -> Self {
        DictationState::Error { message }
    }

    /// Transition to Idle state
    pub fn idle() -> Self {
        DictationState::Idle
    }
}

/// Audio buffer for recording dictation
pub struct AudioBuffer {
    samples: Vec<i16>,
    max_size: usize,
}

impl AudioBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            samples: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Append audio samples to buffer
    pub fn append(&mut self, new_samples: &[i16]) -> Result<()> {
        if self.samples.len() + new_samples.len() > self.max_size {
            eyre::bail!("Audio buffer would exceed maximum size");
        }

        self.samples.extend_from_slice(new_samples);
        Ok(())
    }

    /// Get all samples from buffer
    pub fn get_samples(&self) -> &[i16] {
        &self.samples
    }

    /// Get number of samples in buffer
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.samples.clear();
    }

    /// Calculate RMS audio level for visualization (0.0 to 1.0)
    pub fn calculate_rms_level(&self, window_size: usize) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }

        let start = self.samples.len().saturating_sub(window_size);
        let window = &self.samples[start..];

        if window.is_empty() {
            return 0.0;
        }

        let sum_of_squares: f64 = window.iter().map(|&s| (s as f64).powi(2)).sum();
        let mean_square = sum_of_squares / window.len() as f64;
        let rms = mean_square.sqrt();

        // Normalize to 0.0-1.0 range (i16::MAX = 32767)
        (rms / i16::MAX as f64).min(1.0) as f32
    }

    /// Get duration in seconds (assuming 16kHz sample rate)
    pub fn duration_seconds(&self) -> f64 {
        self.samples.len() as f64 / 16000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        let state = DictationState::Idle;
        assert!(state.can_start_recording());
        assert!(!state.is_recording());

        let state = DictationState::start_recording(None);
        assert!(!state.can_start_recording());
        assert!(state.is_recording());

        let state = DictationState::start_processing(vec![0; 1000], 1.0);
        assert!(!state.can_start_recording());
        assert!(state.is_processing());

        let state = DictationState::error("Test error".to_string());
        assert!(state.can_start_recording()); // Can restart after error

        let state = DictationState::idle();
        assert!(state.can_start_recording());
    }

    #[test]
    fn test_recording_duration() {
        let state = DictationState::start_recording(None);
        std::thread::sleep(std::time::Duration::from_millis(100));

        if let Some(duration) = state.recording_duration() {
            assert!(duration.as_millis() >= 100);
        } else {
            panic!("Expected recording duration");
        }
    }

    #[test]
    fn test_max_duration_check() {
        let state = DictationState::Idle;
        assert!(!state.has_exceeded_max_duration());

        // Can't easily test actual timeout without waiting 5 minutes
        // This test just verifies the logic compiles and runs
    }

    #[test]
    fn test_audio_buffer_append() {
        let mut buffer = AudioBuffer::new(1000);
        let samples = vec![100i16; 500];

        buffer.append(&samples).unwrap();
        assert_eq!(buffer.len(), 500);

        buffer.append(&samples).unwrap();
        assert_eq!(buffer.len(), 1000);

        // Should fail to exceed max size
        assert!(buffer.append(&[1i16]).is_err());
    }

    #[test]
    fn test_audio_buffer_rms_calculation() {
        let mut buffer = AudioBuffer::new(1000);

        // Empty buffer should return 0.0
        assert_eq!(buffer.calculate_rms_level(100), 0.0);

        // Add some samples
        let samples = vec![1000i16; 100];
        buffer.append(&samples).unwrap();

        let rms = buffer.calculate_rms_level(100);
        assert!(rms > 0.0 && rms <= 1.0);
    }

    #[test]
    fn test_audio_buffer_duration() {
        let mut buffer = AudioBuffer::new(100000);

        // 16000 samples = 1 second at 16kHz
        let samples = vec![0i16; 16000];
        buffer.append(&samples).unwrap();

        assert!((buffer.duration_seconds() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_audio_buffer_clear() {
        let mut buffer = AudioBuffer::new(1000);
        buffer.append(&vec![1i16; 500]).unwrap();
        assert_eq!(buffer.len(), 500);

        buffer.clear();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_state_machine_prevents_invalid_transitions() {
        let state = DictationState::start_recording(None);
        assert!(!state.can_start_recording()); // Cannot start while already recording

        let state = DictationState::start_processing(vec![], 0.0);
        assert!(!state.can_start_recording()); // Cannot start while processing
    }
}
