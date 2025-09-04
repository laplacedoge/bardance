use std::{fs::File, io, path::Path};

use symphonia::core::{audio::SampleBuffer, codecs::{DecoderOptions, CODEC_TYPE_NULL}, errors::Error, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint};

pub fn decode_audio(path: &Path) -> Result<Vec<f32>, String> {
    let source = File::open(path)
        .map_err(|e| format!("Failed to open audio file: {e}"))?;

    let stream = MediaSourceStream::new(Box::new(source), Default::default());

    let mut hint = Hint::new();
    if let Some(e) = path.extension().and_then(|s| s.to_str()) {
        hint.with_extension(e);
    }

    let metadata_options: MetadataOptions = Default::default();
    let format_options: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, stream, &format_options, &metadata_options)
        .map_err(|_| format!("Unsupported format"))?;

    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or(format!("No supported audio tracks"))?;
    let track_id = track.id;

    let decoder_options: DecoderOptions = Default::default();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_options)
        .map_err(|_| format!("Unsupported codec"))?;

    let mut complete_samples: Vec<f32> = vec![];
    let mut segmented_samples: Option<SampleBuffer<f32>> = None;
    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(Error::IoError(e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
                break;
            },
            Err(e) => return Err(format!("Failed to get packet: {e}")),
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                if segmented_samples.is_none() {
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    segmented_samples = Some(SampleBuffer::new(duration, spec));
                }

                if let Some(buf) = &mut segmented_samples {
                    let num_samples = audio_buf.frames();
                    buf.copy_planar_ref(audio_buf);
                    let samples = &buf.samples()[..num_samples];
                    complete_samples.extend_from_slice(samples);
                }
            },
            Err(Error::DecodeError(e)) =>
                return Err(format!("Failed to decode: {e}")),
            Err(_) => break,
        }
    }

    Ok(complete_samples)
}
