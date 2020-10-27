use std::f32::consts::PI;
use std::i16;
use hound;

fn main(){
let (A4 , As4 , B4 , C5 ,Cs5 , D5 , Ds5, E5 ,F5 , Fs5 , G5 , Gs5 ,A5)
    =
    (440.000, 466.164, 493.883,523.251,  554.365, 587.330,
     622.254, 659.255,   698.456,  739.989, 783.991, 830.609, 880.000);

     let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
    };
     let mut writer = hound::WavWriter::create("sine.wav",spec).unwrap();
         writer.write_sample(0).unwrap();
          writer.finalize().unwrap();

     fn onkai(takasa:f32, hayasa_csec:i32){
     let mut writer = hound::WavWriter::append("sine.wav").unwrap();
     for t in (0 .. hayasa_csec*441).map(|x| x as f32 / 44100.0) {
         let sample = (t * takasa * 2.0 * PI).sin();
         let amplitude = i16::MAX as f32;
          writer.write_sample((sample * amplitude) as i16).unwrap();
         }
     for t in (0 .. 100).map(|x| x as f32 / 44100.0) {
          writer.write_sample(0).unwrap();
        }
         writer.finalize().unwrap();
     }

     onkai(C5,30);
     onkai(C5,30);
     onkai(G5,30);
     onkai(G5,30);
     onkai(A5,30);
     onkai(A5,30);
     onkai(G5,30);
}
