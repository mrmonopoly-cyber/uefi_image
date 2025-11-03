use std::fs::File;

pub trait ImageWrite<T> {
    fn write_to_image(&self, image: &mut File) -> Result<(), T>;
}
