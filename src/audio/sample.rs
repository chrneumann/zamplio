use anyhow;
use cpal::FromSample;
use hound::WavReader;

// An audio sample.
#[derive(Clone)]
pub struct Sample {
    data: Vec<i32>,
}

impl Sample {
    /// Load the sample from file.
    pub fn load(path: &str) -> Result<Self, anyhow::Error> {
        let mut reader = WavReader::open(path)?;
        println! {"reader {:?}", reader.spec()}
        let mut data: Vec<i32> = vec![];

        for sample in reader.samples::<i32>() {
            data.push(sample?);
        }
        Ok(Sample { data })
    }

    /// Return the sample's data.
    pub fn get_data(&self) -> &Vec<i32> {
        &self.data
    }

    /// Return the sample's data at the given position.
    pub fn get_position(&self, position: usize) -> f32 {
        let sample = self.data[position];
        FromSample::<i32>::from_sample_(sample)
    }
}
