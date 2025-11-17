mod common;
mod header;
mod partiotion_entry_array;

use std::fmt::Display;

use header::*;
use partiotion_entry_array::*;

use crate::image_write::ImageWriteError;

use self::common::{UnicodeString, EMPTY_GUID};

const MINIMUM_SIZE_PARTITION_ENTRY_ARRAY : usize = 16384; //INFO: 16KB

pub enum GptPartitionTableError{
    UnsupportedBlockSize,
    InvalidNumOfPartitionEntries(usize),
    GptPartitionEntryArrayTooSmall(usize),
}

struct GptBlockStructure<const N:usize> {
    first_usable_block: u64,
    partiotion_entry_array_blocks: [GptPartiotionEntry;N],
}

impl<const N:usize> GptBlockStructure<N> {
    fn new(block_size:u32) -> Result<Self,GptPartitionTableError>
    {
        let size_of_partitio_array = (block_size as usize) * N;

        if size_of_partitio_array < MINIMUM_SIZE_PARTITION_ENTRY_ARRAY {
            return Err(GptPartitionTableError::GptPartitionEntryArrayTooSmall(size_of_partitio_array));
        }

        let no_name = UnicodeString::<PARITION_NAME_LENGTH>::new("").ok().unwrap();
        let partition_type = partition_type_guid::UNUSED_ENTRY_GUID;
        let unused_partition = GptPartiotionEntry::new(partition_type, 0, 0, no_name).expect("");

        match (block_size, N) {
            (512,32..) | (4096,4..) => {
                Ok(Self {
                    first_usable_block: N as u64 + 2,
                    partiotion_entry_array_blocks: [unused_partition;N],
                })
            },
            (512,_) | (4096,_) => Err(GptPartitionTableError::InvalidNumOfPartitionEntries(N)),
            _ => Err(GptPartitionTableError::UnsupportedBlockSize),
        }
    }
}

#[repr(C,packed(1))]
pub struct GptPartitionTable<const N:usize>{
    header: GptHeader,
    entry_array: [GptPartiotionEntry;N],
}

impl<const N:usize> GptPartitionTable<N> {

    pub fn new(block_size: u32) -> Result<Self,GptPartitionTableError>{
        let gpt_partion_structure = GptBlockStructure::<N>::new(block_size)?;
        let header =GptHeader::new(
            block_size,
            EMPTY_GUID,
            gpt_partion_structure.first_usable_block,
            N as u32)
            .expect("basic gpt partition table");  //TODO: parameters

        Ok(Self
        {
            header,
            entry_array: gpt_partion_structure.partiotion_entry_array_blocks,
        })
    }
}

impl<const N:usize> crate::ImageWrite for GptPartitionTable<N>{
    fn write_to_image(&self, image: &mut std::fs::File) -> Result<(), ImageWriteError> {
        self.header.write_to_image(image)?;
        for entry in self.entry_array
        {
            entry.write_to_image(image)?;
        }

        Ok(())
    }
}

impl Display for GptPartitionTableError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GptPartitionTableError::UnsupportedBlockSize =>
                        write!(f,"UnsupportedBlockSize"),
            GptPartitionTableError::InvalidNumOfPartitionEntries(v) =>
                        write!(f,"InvalidNumOfPartitionEntries: {}",v),
            GptPartitionTableError::GptPartitionEntryArrayTooSmall(v) =>
                        write!(f,"GptPartitionEntryArrayTooSmall: {}, at least: {}",v, MINIMUM_SIZE_PARTITION_ENTRY_ARRAY),
        }
    }
}

impl Default for GptPartitionTable<34>{
    fn default() -> Self {
        match Self::new(512) {
            Ok(b) => b,
            Err(e) => {
                println!("{}",e);
                todo!()
            },
        }
    }
}
