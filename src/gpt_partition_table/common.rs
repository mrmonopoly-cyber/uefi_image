use core::fmt::Display;

pub type GUID = [u8;16];
pub type LBA = u64;

pub const EMPTY_GUID : GUID = [0;16];

#[repr(C,packed(1))]
#[derive(Debug,Clone, Copy)]
pub struct UnicodeString<const N:usize>
{
    buffer: [u16;N],
}

pub enum UnicodeStringError {
    TooLongInputString,
}

impl<const N:usize> UnicodeString<N> {
    pub fn new(str: &str) -> Result<Self,UnicodeStringError> {
        if str.len() >= N/2 {
            Err(UnicodeStringError::TooLongInputString)
        }else{
            let mut self_c = Self{buffer:[0;N]};
            let mut i=0;
            for c in str.bytes()  {
                self_c.buffer[i] = c.into();
                i+=1;
            }
            Ok(self_c)
        }
    }
    
}

impl<const N:usize> Display for UnicodeString<N>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self)
    }
    // add code here
}
