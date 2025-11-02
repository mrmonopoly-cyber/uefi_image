use super::common::*;

#[derive(Debug)]
#[repr(C,packed(1))]
pub struct GptHeaderData
{
    signature: u64,
    revision: u32,
    header_size: u32,
    header_crc32: u32,
    reserved: u32,
    my_lba: u64,
    alternate_lba: u64,
    first_usable_lba: u64,
    last_usable_lba: u64,
    disk_guid: GUID,
    partition_entry_lba: u64,
    number_of_partition_entries: u32,
    sizeof_partitionentry: u32,
    partition_entry_array_crc32: u32,
}


impl GptHeaderData{
    pub fn new(padding_size: u32,
        alternate_lba: u64,
        disk_guid: GUID,
        first_usable_lba: u64,
        last_usable_lba: u64,
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
pub struct GptHeader
{
    data: GptHeaderData,
    padding_size: u32, //INFO: Block Size -92
}

#[derive(Debug)]
pub enum GptHeaderError {
    InvalidBlockSize,
}

impl GptHeader {
    fn first_usable_lba_from_lbs(block_size: u32) -> Result<u64,GptHeaderError>{
        match block_size {
            512 => Ok(34),
            _ => Err(GptHeaderError::InvalidBlockSize)
        }
    }

    pub fn new(block_size: u32, disk_guid: GUID) -> Result<Self, GptHeaderError> {
        let padding_size = block_size - 92;
        let first_usable_lba = Self::first_usable_lba_from_lbs(block_size)?;

        if block_size < 512 {
            Err(GptHeaderError::InvalidBlockSize)
        }else{
            Ok(Self {
                data: GptHeaderData::new(
                          padding_size,
                          0,
                          disk_guid,
                          first_usable_lba,
                          0),
                padding_size,
            })
        }
    }
    
}

impl Default for GptHeader{
    fn default() -> Self {
        Self::new(512).expect("GptHeader default constructor error")
    }
}
