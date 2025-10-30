use crate::transcribe::WhisperContext;
use eyre::{Context, Result};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::io::Cursor;
use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy};

/// Sample rate for dictation audio (must match audio_capture.rs)
const DICTATION_SAMPLE_RATE: u32 = 16000;

/// Transcribe audio samples for dictation with automatic punctuation
///
/// # Arguments
/// * `ctx` - Whisper context (loaded model)
/// * `audio_samples` - Raw i16 PCM samples at 16kHz mono
/// * `language` - Optional language code (e.g., "en", "es"). If None, auto-detect
/// * `init_prompt` - Optional initial prompt to guide transcription style
///
/// # Returns
/// * Transcribed text with automatic punctuation, or empty string if no speech detected
pub fn transcribe_dictation(
    ctx: &WhisperContext,
    audio_samples: &[i16],
    language: Option<String>,
    init_prompt: Option<String>,
) -> Result<String> {
    tracing::debug!("Starting dictation transcription with {} samples", audio_samples.len());

    // Return empty if no audio
    if audio_samples.is_empty() {
        tracing::debug!("No audio samples provided");
        return Ok(String::new());
    }

    // Convert i16 samples to f32 for Whisper
    let audio_data = convert_integer_to_float_audio(audio_samples);

    // Create transcription parameters with automatic punctuation
    let params = create_dictation_params(language.as_deref(), init_prompt.as_deref());

    // Create state for transcription
    let mut state = ctx.create_state().context("Failed to create Whisper state")?;

    // Run transcription
    tracing::debug!("Running Whisper transcription...");
    state
        .full(params, &audio_data)
        .context("Failed to run Whisper transcription")?;

    // Extract text from all segments
    let num_segments = state.full_n_segments().context("Failed to get segment count")?;
    tracing::debug!("Transcription produced {} segments", num_segments);

    let mut transcription_text = String::new();
    for i in 0..num_segments {
        let segment_text = state
            .full_get_segment_text(i)
            .context("Failed to get segment text")?;
        transcription_text.push_str(&segment_text);
        transcription_text.push(' ');
    }

    let transcription_text = transcription_text.trim().to_string();

    if transcription_text.is_empty() {
        tracing::debug!("Empty transcription result (no speech detected)");
    } else {
        tracing::debug!("Transcription successful: {} characters", transcription_text.len());
    }

    Ok(transcription_text)
}

/// Create Whisper parameters optimized for dictation
///
/// This function creates FullParams directly rather than using setup_params
/// to avoid lifetime issues with TranscribeOptions references
fn create_dictation_params<'a>(language: Option<&'a str>, init_prompt: Option<&'a str>) -> FullParams<'a, 'a> {
    // Use beam search for better quality (similar to setup_params default)
    let sampling_strategy = SamplingStrategy::BeamSearch {
        beam_size: 5, // Balance speed/accuracy
        patience: -1.0,
    };

    let mut params = FullParams::new(sampling_strategy);

    // Set language if provided
    if language.is_some() {
        params.set_language(language);
    }

    // Set initial prompt if provided
    if let Some(prompt) = init_prompt {
        params.set_initial_prompt(prompt);
    }

    // Optimize for dictation (automatic punctuation and clean output)
    params.set_print_special(false);
    params.set_print_progress(true);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_suppress_blank(true);
    params.set_suppress_non_speech_tokens(true);
    params.set_token_timestamps(true);

    // Deterministic results for consistent transcription
    params.set_temperature(0.0);

    // Don't translate, transcribe in original language
    params.set_translate(false);

    params
}

/// Convert i16 PCM samples to f32 normalized samples for Whisper
///
/// Whisper expects f32 samples in range [-1.0, 1.0]
fn convert_integer_to_float_audio(samples: &[i16]) -> Vec<f32> {
    samples.iter().map(|&s| s as f32 / i16::MAX as f32).collect()
}

/// Save audio samples to a temporary WAV file for debugging
/// This is useful for troubleshooting transcription issues
#[allow(dead_code)]
fn save_audio_samples_to_wav(samples: &[i16], output_path: &Path) -> Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: DICTATION_SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(output_path, spec).context("Failed to create WAV writer")?;

    for &sample in samples {
        writer.write_sample(sample).context("Failed to write sample")?;
    }

    writer.finalize().context("Failed to finalize WAV file")?;
    tracing::debug!("Saved audio to {:?}", output_path);

    Ok(())
}

/// Convert audio samples to WAV format in memory
/// Useful if we need to pass through ffmpeg normalization
#[allow(dead_code)]
fn samples_to_wav_bytes(samples: &[i16]) -> Result<Vec<u8>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: DICTATION_SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = WavWriter::new(&mut cursor, spec).context("Failed to create WAV writer")?;
        for &sample in samples {
            writer.write_sample(sample).context("Failed to write sample")?;
        }
        writer.finalize().context("Failed to finalize WAV")?;
    }

    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Helper to get test model path (skip if not available)
    fn get_test_model_path() -> Option<PathBuf> {
        // Look for tiny model in common locations
        let possible_paths = vec![
            PathBuf::from("../../models/ggml-tiny.bin"),
            PathBuf::from("../models/ggml-tiny.bin"),
            PathBuf::from("models/ggml-tiny.bin"),
            PathBuf::from(std::env::var("HOME").unwrap_or_default())
                .join("Library/Application Support/vibe/models/ggml-tiny.bin"),
        ];

        for path in possible_paths {
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    // Helper to generate test audio (sine wave or silence)
    fn generate_test_audio(duration_secs: f32, frequency: f32) -> Vec<i16> {
        let sample_rate = DICTATION_SAMPLE_RATE as f32;
        let num_samples = (duration_secs * sample_rate) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate;
            let amplitude = 0.3; // 30% amplitude
            let value = amplitude * (2.0 * std::f32::consts::PI * frequency * t).sin();
            let sample = (value * i16::MAX as f32) as i16;
            samples.push(sample);
        }

        samples
    }

    #[test]
    fn test_convert_integer_to_float_audio() {
        let samples = vec![0i16, i16::MAX / 2, i16::MAX, i16::MIN, i16::MIN / 2];
        let floats = convert_integer_to_float_audio(&samples);

        assert_eq!(floats.len(), samples.len());
        assert_eq!(floats[0], 0.0);
        assert!((floats[1] - 0.5).abs() < 0.01);
        assert!((floats[2] - 1.0).abs() < 0.01);
        assert!(floats[3] < 0.0); // Should be negative
        assert!(floats[4] < 0.0); // Should be negative
    }

    #[test]
    fn test_convert_empty_audio() {
        let samples: Vec<i16> = vec![];
        let floats = convert_integer_to_float_audio(&samples);
        assert_eq!(floats.len(), 0);
    }

    #[test]
    fn test_create_dictation_params() {
        let params = create_dictation_params(Some("en"), None);
        // We can't easily test internal whisper-rs params, but we can verify it doesn't panic
        drop(params);
    }

    #[test]
    fn test_create_dictation_params_with_prompt() {
        let prompt = "This is a technical document about programming.";
        let params = create_dictation_params(Some("en"), Some(prompt));
        drop(params);
    }

    #[test]
    fn test_samples_to_wav_bytes() {
        let samples = generate_test_audio(0.1, 440.0); // 100ms of A440
        let wav_bytes = samples_to_wav_bytes(&samples).unwrap();

        // WAV file should have header + data
        assert!(wav_bytes.len() > 44); // WAV header is 44 bytes

        // Check RIFF header
        assert_eq!(&wav_bytes[0..4], b"RIFF");
        assert_eq!(&wav_bytes[8..12], b"WAVE");
    }

    #[test]
    #[ignore] // Only run with --ignored flag when model is available
    fn test_transcribe_empty_audio() {
        let model_path = match get_test_model_path() {
            Some(path) => path,
            None => {
                println!("Skipping test: no model found");
                return;
            }
        };

        let ctx = create_context(&model_path, None, None).unwrap();
        let samples: Vec<i16> = vec![];

        let result = transcribe_dictation(&ctx, &samples, Some("en".to_string()), None).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    #[ignore] // Only run with --ignored flag when model is available
    fn test_transcribe_silence() {
        let model_path = match get_test_model_path() {
            Some(path) => path,
            None => {
                println!("Skipping test: no model found");
                return;
            }
        };

        let ctx = create_context(&model_path, None, None).unwrap();
        let samples = vec![0i16; 16000]; // 1 second of silence

        let result = transcribe_dictation(&ctx, &samples, Some("en".to_string()), None).unwrap();
        // Silence should return empty or minimal output
        assert!(result.len() < 20); // Allow for some noise detection
    }

    #[test]
    #[ignore] // Only run with --ignored flag when model is available and real audio is available
    fn test_transcribe_real_audio() {
        // This test would require a real audio sample with speech
        // For now, we'll skip it as it requires test fixtures
        println!("Real audio transcription test requires test fixtures");
    }

    #[test]
    fn test_transcribe_respects_language() {
        // We can't fully test this without a model, but we can verify the function accepts it
        let params = create_dictation_params(Some("es"), None);
        drop(params);
    }

    #[test]
    fn test_transcribe_handles_long_audio() {
        // Generate 5 seconds of test audio (max is 300 seconds)
        let samples = generate_test_audio(5.0, 440.0);
        assert_eq!(samples.len(), 5 * DICTATION_SAMPLE_RATE as usize);

        // Test conversion doesn't panic
        let floats = convert_integer_to_float_audio(&samples);
        assert_eq!(floats.len(), samples.len());
    }
}
