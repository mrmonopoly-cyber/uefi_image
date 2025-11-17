use std::fmt::Display;
use std::fs::File;

pub mod partition_record;

use bytemuck::{bytes_of, Pod, Zeroable};
use partition_record::PartitionRecord;

use crate::image_write::{ImageWrite, ImageWriteError};

pub enum MbrWriteError{
    SystemError(std::io::Error),
    PartialWrite(usize)
}

impl Display for MbrWriteError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MbrWriteError::SystemError(error) => write!(f,"{}",error),
            MbrWriteError::PartialWrite(len) => write!(f,"{}",len),
        }
    }
}


const BOOT_CODE_SIZE : usize = 440;
const UNIQUE_MBR_DISK_SIGNATURE_SIZE : usize = 4;
const UNKNOWN_SIZE : usize = 2;
const RECORD_SIZE : usize = 4;

#[repr(C,packed(1))]
#[derive(Clone, Copy)]
pub struct ProtectiveMbrdata
{
    boot_code: [u8;BOOT_CODE_SIZE],
    unique_mbr_disk_signature: [u8;UNIQUE_MBR_DISK_SIGNATURE_SIZE],
    unknown: [u8;UNKNOWN_SIZE],
    record: [PartitionRecord;RECORD_SIZE],
    signature: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProtectiveMbr{
    data: ProtectiveMbrdata,
    padding_size: usize, //INFO: lbl size - 512
}

impl Default for ProtectiveMbr{ //INFO: LBL size 512
    fn default() -> Self {
        Self::new(512) 
    }
}



unsafe impl Zeroable for ProtectiveMbrdata{}
unsafe impl Pod for ProtectiveMbrdata{}

impl ProtectiveMbr {
    pub fn new(lbl_size: usize) -> Self {
        Self {
            data:
            ProtectiveMbrdata 
            {
                boot_code: [0;BOOT_CODE_SIZE],
                unique_mbr_disk_signature: [0;UNIQUE_MBR_DISK_SIGNATURE_SIZE],
                unknown: [0;UNKNOWN_SIZE],
                record: [
                    PartitionRecord::uefi_partition_record(),
                    PartitionRecord::default(),
                    PartitionRecord::default(),
                    PartitionRecord::default(),
                ],
                signature: 0xAA55,
            },
            padding_size: lbl_size - 512,
        }
    }
}

impl ImageWrite for ProtectiveMbr{
    fn write_to_image(&self, image: & mut File) -> Result<(),ImageWriteError> {

        let bytes = bytes_of(&self.data);
        let padding = vec![0u8;self.padding_size];

        Self::try_write(image, bytes)?;
        Self::try_write(image, &padding)?;

        Ok(())
    }
}
