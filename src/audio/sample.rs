use anyhow;
use hound::WavReader;

// An audio sample.
pub struct Sample<T>
where
    T: hound::Sample,
{
    data: Vec<T>,
}

impl<T> Sample<T>
where
    T: hound::Sample,
{
    /// Load the sample from file.
    pub fn load(path: &str) -> Result<Self, anyhow::Error> {
        let mut reader = WavReader::open(path)?;
        println! {"reader {:?}", reader.spec()}
        let mut data: Vec<T> = vec![];

        for sample in reader.samples::<T>() {
            data.push(sample?);
        }
        Ok(Sample { data })
    }

    /// Return the sample's data.
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}
