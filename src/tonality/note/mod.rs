mod note;
mod symbol;
mod tone;

pub use note::*;
pub use symbol::*;
pub use tone::*;

pub trait Divided {
    fn num_devided() -> usize;
    fn index(&self) -> usize;
}

pub trait Pitch {
    fn transpose(self, transpose: i32) -> Self;
}

impl<T: Divided + From<usize>> Pitch for T {
    fn transpose(self, transpose: i32) -> Self {
        let index = self.index();
        let devided = T::num_devided() as i32;
        let new_index = index as i32 + transpose;
        if new_index < 0 {
            ((devided + (new_index % devided)) as usize).into()
        } else {
            (new_index as usize).into()
        }
    }
}
