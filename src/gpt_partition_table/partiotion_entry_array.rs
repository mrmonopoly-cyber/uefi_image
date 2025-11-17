use crate::image_write::ImageWriteError;
use bytemuck::{bytes_of, Pod, Zeroable};

use super::common::*;

#[allow(unused)]
pub mod partition_type_guid{
    use super::super::common::GUID;

    pub const UNUSED_ENTRY_GUID: GUID= [0;16];
    pub const EFI_SYSTEM_PARTITION_GUID: GUID= [
        0xC1,0x2A,0x73,0x28,0xF8,0x1F,0x11,0xD2,0xBA,0x4B,0x00,0xA0,0xC9,0x3E,0xC9,0x3B
    ];
    pub const LEGACY_MBR_GUID : GUID= [
        0x02,0x4D,0xEE,0x41,0x33,0xE7,0x11,0xD3,0x9D,0x69,0x00,0x08,0xC7,0x81,0xF3,0x9F
    ];
}

pub const PARITION_NAME_LENGTH: usize = 36;

#[allow(non_snake_case)]
#[repr(C,packed(1))]
#[derive(Debug,Clone, Copy)]
pub struct GptPartiotionEntryData
{
    PartitionTypeGUID: GUID,
    UniquePartitionGUID: GUID,
    StartingLBA: LBA,
    EndingLBA: LBA,
    Attributes: u64,
    PartitionName: UnicodeString<PARITION_NAME_LENGTH>, //INFO: unicode string
}

#[allow(unused)]
#[derive(Clone,Copy)]
pub struct GptPartiotionEntry
{
    data: GptPartiotionEntryData,
    padding: u32, //INFO: SizeOf PartitionEntry - 128
}

impl GptPartiotionEntry {
    pub fn new(partition_type: GUID,
        starting_lba: u64,
        ending_lba: u64,
        partion_name: UnicodeString<PARITION_NAME_LENGTH>) -> Result<Self,()> {
        Ok(Self{
            data: GptPartiotionEntryData {
                PartitionTypeGUID: partition_type,
                UniquePartitionGUID: EMPTY_GUID,
                StartingLBA: starting_lba,
                EndingLBA: ending_lba,
                Attributes: 0, //INFO: todo: not useful at the moment
                PartitionName: partion_name,
            },
            padding: 128 - (core::mem::size_of::<GptPartiotionEntryData>() as u32),
        })
    }
    
}

impl crate::ImageWrite for GptPartiotionEntry{
    fn write_to_image(&self, image: &mut std::fs::File) -> Result<(), ImageWriteError> {
        let bytes = bytes_of(&self.data);
        let padding = vec![0u8;self.padding as usize];

        Self::try_write(image, bytes)?;
        Self::try_write(image, &padding)?;

        Ok(())
    }
}

unsafe impl Zeroable for GptPartiotionEntryData{}
unsafe impl Pod for GptPartiotionEntryData{}
