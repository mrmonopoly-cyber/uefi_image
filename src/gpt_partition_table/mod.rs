mod common;
mod header;
mod partiotion_entry_array;

use std::fmt::Display;

use header::*;
use partiotion_entry_array::*;

pub enum GptPartitionTableError{

}

#[repr(C,packed(1))]
pub struct GptPartitionTable<const N:usize>{
    header: GptHeader,
    entry_array: [GptPartiotionEntry;N],
}

impl<const N:usize> GptPartitionTable<N> {
    pub fn new() -> Self{
        let unused_partition = GptPartiotionEntry::new(PartitionTypeGuid::UnusedEntry, [0;72]).expect("");
        let header =GptHeader::new(512, [0;128], 0).expect("basic gpt partition table");  //TODO: parameters
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
