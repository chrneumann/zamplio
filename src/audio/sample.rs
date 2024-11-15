use anyhow;
use hound::WavReader;

// An audio sample.
pub struct Sample {
    data: Vec<i16>,
}

impl Sample {
    /// Load the sample from file.
    pub fn load(path: &str) -> Result<Self, anyhow::Error> {
        let mut reader = WavReader::open(path)?;
        println! {"reader {:?}", reader.spec()}
        let mut data: Vec<i16> = vec![];

        for sample in reader.samples::<i16>() {
            data.push(sample?);
        }
        Ok(Sample { data })
    }

    /// Return the sample's data.
    pub fn data(&self) -> &Vec<i16> {
        &self.data
    }
}
