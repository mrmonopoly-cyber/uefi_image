use bytemuck::{bytes_of, Pod, Zeroable};

use crate::image_write::ImageWriteError;

use super::common::*;

use std::fmt::Display;

#[derive(Debug,Clone, Copy)]
#[repr(C,packed(1))]
pub struct GptHeaderData
{
    signature: u64,
    revision: u32,
    header_size: u32,
    header_crc32: u32,
    reserved: u32,
    my_lba: u64,
    alternate_lba: LBA,
    first_usable_lba: LBA,
    last_usable_lba: LBA,
    disk_guid: GUID,
    partition_entry_lba: LBA,
    number_of_partition_entries: u32,
    sizeof_partitionentry: u32,
    partition_entry_array_crc32: u32,
}


impl GptHeaderData{
    pub fn new(padding_size: u32,
        alternate_lba: LBA,
        disk_guid: GUID,
        first_usable_lba: LBA,
        last_usable_lba: LBA,
        number_of_partition_entries: u32,
        ) -> Self
    {
        let header_size : u32 = core::mem::size_of::<Self>().try_into().unwrap();
        let header_size = header_size + padding_size;

        Self {
            signature: 0x5452415020494645,
            revision: 0x00010000,
            header_size,
            header_crc32: 0,
            reserved: 0,
            my_lba: 1,
            alternate_lba,
            first_usable_lba,
            last_usable_lba,
            disk_guid,
            partition_entry_lba: 2, //TODO: for later
            number_of_partition_entries,
            sizeof_partitionentry: Default::default(), //INFO: 128 * 2 * n
            partition_entry_array_crc32: Default::default()
        }
    }
}

#[repr(C,packed(1))]
#[derive(Debug,Clone, Copy)]
pub struct GptHeader
{
    data: GptHeaderData,
    padding_size: u32, //INFO: Block Size -92
}

#[derive(Debug)]
pub enum GptHeaderError {
    InvalidBlockSize,
    ImageWriteFailed,
}

impl GptHeader {
    pub fn new(
        block_size: u32,
        disk_guid: GUID,
        first_usable_lba: u64,
        number_of_partition_entries: u32)
        -> Result<Self, GptHeaderError>
    {
        let padding_size = block_size - 92;
        let last_usable_lba = first_usable_lba + u64::from(number_of_partition_entries);

        if block_size < 512 {
            Err(GptHeaderError::InvalidBlockSize)
        }else{
            Ok(Self {
                data: GptHeaderData::new(
                          padding_size,
                          last_usable_lba + 1,
                          disk_guid,
                          first_usable_lba,
                          last_usable_lba,
                          number_of_partition_entries),
                padding_size,
            })
        }
    }
    
}

impl Display for GptHeaderError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            GptHeaderError::InvalidBlockSize => write!(f,"InvalidBlockSize"),
            GptHeaderError::ImageWriteFailed => write!(f,"ImageWriteFailed"),
        }
    }
}

impl crate::ImageWrite for GptHeader{
    fn write_to_image(&self, image: &mut std::fs::File) -> Result<(), ImageWriteError> {
        let bytes = bytes_of(&self.data);
        let padding = vec![0_u8;self.padding_size as usize];

        Self::try_write(image, bytes)?;
        Self::try_write(image, &padding)?;

        Ok(())
    }
}

unsafe impl Zeroable for GptHeaderData{}
unsafe impl Pod for GptHeaderData{}
