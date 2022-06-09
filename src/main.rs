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
    channels: 2,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
    };
     let mut writer = hound::WavWriter::create("sine.wav",spec).unwrap();
         writer.write_sample(0).unwrap();
          writer.finalize().unwrap();
        writer.write_sample(0).expect("can't write once");   //Its channels is two (= stereo) so  
         writer.write_sample(0).expect("can't write twice"); //have to write twice
         writer.finalize().expect("can't finalize.");

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
     onkai(C5,100);
     onkai(C5,100);
     onkai(G5,100);
     onkai(G5,100);
     onkai(A5,100);
     onkai(A5,100);
     onkai(G5,100);

     let mut reader1 = hound::WavReader::open("sine.wav").expect("cannot open");

     let mut data1 = reader1.samples::<i16>();
     let mut r_left = data1.by_ref().step_by(2);

     let mut reader2 = hound::WavReader::open("sine.wav").expect("cannot open");
     let mut data2 = reader2.samples::<i16>();
     let mut r_right = data2.skip(1).step_by(2);

    //calc

     let vec_l: Vec<i16> = r_left.map(|x| (x.unwrap() as f32 * 0.5) as i16 ).collect();
     let vec_r: Vec<i16> = r_right.map(|x| x.unwrap()).collect();

     let mut wav_buffer: Vec<i16> = [].to_vec();

     for i in 0..vec_l.len() {
         wav_buffer.push(vec_l[i]);
         wav_buffer.push(vec_r[i]);
     }

     let wav_iter = wav_buffer.iter();

    let mut writer = hound::WavWriter::create("sine2.wav",spec).unwrap();

     for item in wav_iter.map(|x| *x) {
          writer.write_sample(item).unwrap();
     }

     writer.finalize().unwrap();
