use std::fmt::Display;
use std::fs::File;
use std::io::Write;

pub enum ImageWriteError {
    IncompleteWrite(usize),
    SystemError(std::io::Error),
}

pub trait ImageWrite {
    fn write_to_image(&self, image: &mut File) -> Result<(), ImageWriteError>;
    fn try_write(image: &mut File, bytes: &[u8]) -> Result<(), ImageWriteError>
    {
        let res = image.write(bytes);
        match res
        {
            Ok(len) => 
            {
                if len != bytes.len(){
                    Err(ImageWriteError::IncompleteWrite(len))
                }else{
                    Ok(())
                }
            },
            Err(e) => Err(ImageWriteError::SystemError(e))
        }
    }
}

impl Display for ImageWriteError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageWriteError::IncompleteWrite(v) => write!(f,"IncompleteWrite: {}",v),
            ImageWriteError::SystemError(error) => write!(f,"SystemError: {}",error),
        }
    }
}
