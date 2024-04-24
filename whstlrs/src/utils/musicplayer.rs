use core::time::Duration;
use rodio::{OutputStream, source::Source};

struct Flute {
  sample_rate: u32
}

impl Flute {
    fn new(sample_rate:u32) -> Flute {
       return Flute {
       	      sample_rate: sample_rate
	      };
    }

    fn get_sample(&mut self) -> f32 {
       return sample;
    }

}

impl Source for Flute {
fn channels(&self) -> u16 {
   return 1;
   }

fn sample_rate(&self) -> u32 {
   return self.sample_rate;
   }
fn current_frame_len(&self) -> Option<usize> {
   	  return None;
	  }

fn total_duration(&self) -> Option<Duration> {
return None;
}

impl Iterator for Flute {
     type Item = f32;

     fn nect(&mut self) -> Option<Self::Item> {
     	return Some(self.get_sample());
}

fn main() {
   let (_stream, stream_handle) = OutputStream::try_default().unwrap();
   let _ result = stream_handle.play_raw(flute.convert_samples());
   std::thread::sleep(std::time::Duration::from_secs(5));
}
