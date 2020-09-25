
const BUFFER_SIZE: usize = 5;

#[derive(Debug)]
pub struct SampleBuffer {
    samples: [Sample; BUFFER_SIZE]
}

impl SampleBuffer {

    pub fn new() -> Self {
        Self {
            samples: [
                Sample::default(),
                Sample::default(),
                Sample::default(),
                Sample::default(),
                Sample::default()
            ]
        }
    }
    
    /// Add new item to the buffer, while forgetting about the oldest one
    pub fn push(&mut self, item: Sample) {

        for i in 0..BUFFER_SIZE - 1 {
            self.samples[i] = self.samples[i + 1];
        }

        self.samples[BUFFER_SIZE - 1] = item;
    }

    pub fn mean_sample(&self) -> Sample {

        let mut total_x: i16 = 0;
        let mut total_y: i16 = 0;
        let mut total_z: i16 = 0;

        for i in 0..BUFFER_SIZE - 1 {
            total_x += self.samples[i].x;
            total_y += self.samples[i].y;
            total_z += self.samples[i].z;
        }

        // NOTE (cikzh): Rounding by truncation
        let mean_x = total_x / BUFFER_SIZE as i16;
        let mean_y = total_y / BUFFER_SIZE as i16;
        let mean_z = total_z / BUFFER_SIZE as i16;
        
        Sample { x: mean_x, y: mean_y, z: mean_z }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Sample {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Default for Sample {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}
