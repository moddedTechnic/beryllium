use std::ops::DerefMut;


pub trait Reversed {
    fn reversed(&self) -> Self;
}

impl<It, T> Reversed for It where It: DerefMut<Target = [T]> + Clone {
    fn reversed(&self) -> Self {
        let mut vec = self.clone();
        vec.reverse();
        vec
    }
}

