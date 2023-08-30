#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch<T> {
    class: T,
    oct: usize,
}

impl<T> Pitch<T> {
    pub fn new(class: T, oct: usize) -> Self {
        Self { class, oct }
    }

    pub fn class(&self) -> &T {
        &self.class
    }

    pub fn oct(&self) -> usize {
        self.oct
    }

    pub fn as_ref(&self) -> Pitch<&T> {
        Pitch::new(self.class(), self.oct())
    }
}

// impl<T: PitchClassLike> Pitch<T> {
//     pub fn frequency(&self, ref_freq: f32) -> f32 {
//         ref_freq
//             * self.oct() as f32
//             * 2.0_f32.powf(self.class().step() as f32 / self.class().chroma_size() as f32)
//     }
// }
