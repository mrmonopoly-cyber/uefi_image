mod common;
mod header;
mod partiotion_entry_array;

use std::fmt::Display;

use header::*;
use partiotion_entry_array::*;

use self::common::UnicodeString;

pub enum GptPartitionTableError{

}

#[repr(C,packed(1))]
pub struct GptPartitionTable<const N:usize>{
    header: GptHeader,
    entry_array: [GptPartiotionEntry;N],
}

impl<const N:usize> GptPartitionTable<N> {
    pub fn new() -> Self{
        let no_name = UnicodeString::<PARITION_NAME_LENGTH>::new("").ok().unwrap();
        let partition_type = PartitionTypeGuid::UNUSED_ENTRY_GUID;
        let unused_partition = GptPartiotionEntry::new(partition_type, no_name).expect("");
        let header =GptHeader::new(512, [0;16], 0).expect("basic gpt partition table");  //TODO: parameters
        Self
        {
            header,
            entry_array: [unused_partition;N],
        }
    }
}

impl<const N:usize> crate::ImageWrite<GptHeaderError> for GptPartitionTable<N>{
    fn write_to_image(&self, image: &mut std::fs::File) -> Result<(), GptHeaderError> {
        todo!();
    }
}

impl Display for GptPartitionTableError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self)
    }
}
