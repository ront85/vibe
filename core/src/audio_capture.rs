use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SizedSample, Stream, StreamConfig};
use eyre::{bail, Context, OptionExt, Result};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Sample rate for dictation audio (16kHz mono, Whisper compatible)
pub const DICTATION_SAMPLE_RATE: u32 = 16000;

/// Maximum recording duration in seconds (5 minutes)
pub const MAX_RECORDING_DURATION_SECS: u64 = 300;

/// Ring buffer capacity for 5 minutes at 16kHz
/// 16000 samples/sec * 300 sec = 4,800,000 samples
pub const RING_BUFFER_CAPACITY: usize = 4_800_000;

/// Window size for RMS calculation (250ms at 16kHz = 4000 samples)
const RMS_WINDOW_SIZE: usize = 4000;

/// Audio feedback beep parameters
pub const START_BEEP_FREQUENCY: f32 = 750.0; // Hz
pub const STOP_BEEP_FREQUENCY: f32 = 500.0; // Hz
pub const BEEP_DURATION_MS: u64 = 100; // milliseconds

/// Shared audio buffer for recording
#[derive(Clone)]
pub struct AudioCaptureBuffer {
    samples: Arc<Mutex<Vec<i16>>>,
    start_time: Arc<Mutex<Option<Instant>>>,
}

impl AudioCaptureBuffer {
    pub fn new() -> Self {
        Self {
            samples: Arc::new(Mutex::new(Vec::with_capacity(RING_BUFFER_CAPACITY))),
            start_time: Arc::new(Mutex::new(None)),
        }
    }

    /// Start recording (reset buffer and set start time)
    pub fn start_recording(&self) {
        let mut samples = self.samples.lock().unwrap();
        samples.clear();

        let mut start_time = self.start_time.lock().unwrap();
        *start_time = Some(Instant::now());
    }

    /// Append audio samples to buffer
    pub fn append_samples(&self, new_samples: &[i16]) -> Result<()> {
        let mut samples = self.samples.lock().unwrap();

        // Check if adding would exceed capacity
        if samples.len() + new_samples.len() > RING_BUFFER_CAPACITY {
            bail!("Audio buffer would exceed maximum capacity");
        }

        samples.extend_from_slice(new_samples);
        Ok(())
    }

    /// Get all samples from buffer (for transcription)
    pub fn get_samples(&self) -> Vec<i16> {
        let samples = self.samples.lock().unwrap();
        samples.clone()
    }

    /// Get number of samples in buffer
    pub fn len(&self) -> usize {
        let samples = self.samples.lock().unwrap();
        samples.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        let samples = self.samples.lock().unwrap();
        samples.is_empty()
    }

    /// Calculate real-time RMS level for visualization (0.0 to 1.0)
    pub fn calculate_rms_level(&self) -> f32 {
        let samples = self.samples.lock().unwrap();

        if samples.is_empty() {
            return 0.0;
        }

        // Take last RMS_WINDOW_SIZE samples for real-time calculation
        let start = samples.len().saturating_sub(RMS_WINDOW_SIZE);
        let window = &samples[start..];

        if window.is_empty() {
            return 0.0;
        }

        let sum_of_squares: f64 = window.iter().map(|&s| (s as f64).powi(2)).sum();
        let mean_square = sum_of_squares / window.len() as f64;
        let rms = mean_square.sqrt();

        // Normalize to 0.0-1.0 range (i16::MAX = 32767)
        (rms / i16::MAX as f64).min(1.0) as f32
    }

    /// Get recording duration
    pub fn recording_duration(&self) -> Duration {
        let start_time = self.start_time.lock().unwrap();
        if let Some(start) = *start_time {
            start.elapsed()
        } else {
            Duration::from_secs(0)
        }
    }

    /// Check if recording has exceeded max duration
    pub fn has_exceeded_max_duration(&self) -> bool {
        self.recording_duration().as_secs() >= MAX_RECORDING_DURATION_SECS
    }

    /// Get duration in seconds (based on sample count)
    pub fn duration_seconds(&self) -> f64 {
        let samples = self.samples.lock().unwrap();
        samples.len() as f64 / DICTATION_SAMPLE_RATE as f64
    }

    /// Clear the buffer
    pub fn clear(&self) {
        let mut samples = self.samples.lock().unwrap();
        samples.clear();

        let mut start_time = self.start_time.lock().unwrap();
        *start_time = None;
    }
}

impl Default for AudioCaptureBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Audio capture stream handle
pub struct AudioCaptureStream {
    stream: Stream,
    buffer: AudioCaptureBuffer,
}

impl AudioCaptureStream {
    /// Create and start audio capture stream
    pub fn start(device_id: Option<String>) -> Result<Self> {
        let host = cpal::default_host();

        // Get the device
        let device = if let Some(device_id) = device_id {
            let device_index: usize = device_id.parse().context("Failed to parse device ID")?;
            host.devices()
                .context("Failed to enumerate devices")?
                .nth(device_index)
                .ok_or_eyre("Failed to get device by ID")?
        } else {
            host.default_input_device().ok_or_eyre("No default input device")?
        };

        let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());
        tracing::debug!("Starting audio capture on device: {}", device_name);

        // Get supported config
        let supported_config = device
            .default_input_config()
            .context("Failed to get default input config")?;

        tracing::debug!("Device config: {:?}", supported_config);

        // Create buffer
        let buffer = AudioCaptureBuffer::new();
        buffer.start_recording();

        // Build stream based on sample format
        let buffer_clone = buffer.clone();
        let stream = match supported_config.sample_format() {
            cpal::SampleFormat::I16 => build_input_stream::<i16>(&device, &supported_config.into(), buffer_clone)?,
            cpal::SampleFormat::I32 => build_input_stream::<i32>(&device, &supported_config.into(), buffer_clone)?,
            cpal::SampleFormat::F32 => build_input_stream::<f32>(&device, &supported_config.into(), buffer_clone)?,
            format => bail!("Unsupported sample format: {:?}", format),
        };

        stream.play().context("Failed to start audio stream")?;
        tracing::debug!("Audio capture stream started");

        Ok(Self { stream, buffer })
    }

    /// Get the audio buffer
    pub fn buffer(&self) -> &AudioCaptureBuffer {
        &self.buffer
    }

    /// Stop the stream and return captured samples
    pub fn stop(self) -> Result<Vec<i16>> {
        tracing::debug!("Stopping audio capture stream");
        self.stream.pause().context("Failed to pause stream")?;
        Ok(self.buffer.get_samples())
    }
}

/// Build input stream for a specific sample type
fn build_input_stream<T>(
    device: &cpal::Device,
    config: &StreamConfig,
    buffer: AudioCaptureBuffer,
) -> Result<Stream>
where
    T: Sample + SizedSample + FromSample<T>,
    i16: FromSample<T>,
{
    let source_sample_rate = config.sample_rate.0;
    let source_channels = config.channels as usize;

    tracing::debug!(
        "Building stream: {} Hz, {} channels -> {} Hz mono",
        source_sample_rate,
        source_channels,
        DICTATION_SAMPLE_RATE
    );

    let err_fn = |err| {
        tracing::error!("Audio stream error: {}", err);
    };

    // Calculate resampling ratio
    let resample_ratio = source_sample_rate as f32 / DICTATION_SAMPLE_RATE as f32;
    let mut sample_accumulator: f32 = 0.0;

    let stream = device.build_input_stream(
        config,
        move |data: &[T], _: &_| {
            // Convert to i16 and downsample/downmix to 16kHz mono
            let mut resampled_samples = Vec::new();

            for chunk in data.chunks(source_channels) {
                // Mix channels to mono (average)
                let mono_sample: f32 = chunk
                    .iter()
                    .map(|&s| {
                        let s_i16: i16 = i16::from_sample(s);
                        s_i16 as f32
                    })
                    .sum::<f32>()
                    / source_channels as f32;

                sample_accumulator += 1.0;

                // Resample: only keep samples at 16kHz rate
                if sample_accumulator >= resample_ratio {
                    sample_accumulator -= resample_ratio;
                    resampled_samples.push(mono_sample as i16);
                }
            }

            // Append to buffer
            if !resampled_samples.is_empty() {
                if let Err(e) = buffer.append_samples(&resampled_samples) {
                    tracing::warn!("Failed to append samples: {}", e);
                }
            }
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

/// Generate audio feedback beep (simple sine wave)
pub fn play_beep(frequency: f32, duration_ms: u64, enabled: bool) -> Result<()> {
    if !enabled {
        return Ok(());
    }

    tracing::debug!("Playing beep: {} Hz, {} ms", frequency, duration_ms);

    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_eyre("No default output device")?;

    let config = device.default_output_config().context("Failed to get output config")?;
    let sample_rate = config.sample_rate().0 as f32;

    let duration_samples = (sample_rate * duration_ms as f32 / 1000.0) as usize;

    // Generate sine wave samples
    let mut samples: Vec<f32> = Vec::with_capacity(duration_samples);
    for i in 0..duration_samples {
        let t = i as f32 / sample_rate;
        let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin() * 0.2; // Reduced amplitude
        samples.push(sample);
    }

    // Play the beep
    let samples = Arc::new(Mutex::new(samples));
    let samples_clone = samples.clone();

    let err_fn = |err| {
        tracing::error!("Beep playback error: {}", err);
    };

    let mut sample_index = 0;
    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &_| {
            let samples = samples_clone.lock().unwrap();
            for output_sample in data.iter_mut() {
                if sample_index < samples.len() {
                    *output_sample = samples[sample_index];
                    sample_index += 1;
                } else {
                    *output_sample = 0.0;
                }
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    // Wait for beep to finish
    std::thread::sleep(Duration::from_millis(duration_ms + 50));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_buffer_creation() {
        let buffer = AudioCaptureBuffer::new();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_audio_buffer_append() {
        let buffer = AudioCaptureBuffer::new();
        buffer.start_recording();

        let samples = vec![100i16; 1000];
        buffer.append_samples(&samples).unwrap();

        assert_eq!(buffer.len(), 1000);
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_audio_buffer_max_capacity() {
        let buffer = AudioCaptureBuffer::new();
        buffer.start_recording();

        // Fill to capacity
        let chunk_size = 10000;
        let num_chunks = RING_BUFFER_CAPACITY / chunk_size;

        for _ in 0..num_chunks {
            let samples = vec![100i16; chunk_size];
            buffer.append_samples(&samples).unwrap();
        }

        assert_eq!(buffer.len(), num_chunks * chunk_size);

        // Try to exceed capacity
        let extra = vec![100i16; 1];
        assert!(buffer.append_samples(&extra).is_err());
    }

    #[test]
    fn test_rms_calculation() {
        let buffer = AudioCaptureBuffer::new();
        buffer.start_recording();

        // Empty buffer
        assert_eq!(buffer.calculate_rms_level(), 0.0);

        // Add samples
        let samples = vec![1000i16; 5000];
        buffer.append_samples(&samples).unwrap();

        let rms = buffer.calculate_rms_level();
        assert!(rms > 0.0 && rms <= 1.0);
    }

    #[test]
    fn test_duration_calculation() {
        let buffer = AudioCaptureBuffer::new();
        buffer.start_recording();

        // 16000 samples = 1 second at 16kHz
        let samples = vec![0i16; 16000];
        buffer.append_samples(&samples).unwrap();

        let duration = buffer.duration_seconds();
        assert!((duration - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_timeout_detection() {
        let buffer = AudioCaptureBuffer::new();

        // Not started
        assert!(!buffer.has_exceeded_max_duration());

        buffer.start_recording();

        // Just started
        assert!(!buffer.has_exceeded_max_duration());

        // Would need to wait 5 minutes to test actual timeout
        // This test just verifies the logic compiles
    }

    #[test]
    fn test_buffer_clear() {
        let buffer = AudioCaptureBuffer::new();
        buffer.start_recording();

        let samples = vec![100i16; 1000];
        buffer.append_samples(&samples).unwrap();

        assert_eq!(buffer.len(), 1000);

        buffer.clear();

        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_get_samples() {
        let buffer = AudioCaptureBuffer::new();
        buffer.start_recording();

        let original_samples = vec![100i16, 200, 300, 400, 500];
        buffer.append_samples(&original_samples).unwrap();

        let retrieved = buffer.get_samples();
        assert_eq!(retrieved, original_samples);
    }

    // Note: Audio stream tests require actual audio devices and would be platform-specific
    // Manual testing required for:
    // - Starting audio capture on different devices
    // - Real-time RMS calculation accuracy
    // - 5-minute timeout mechanism
    // - Audio feedback beep playback
}
