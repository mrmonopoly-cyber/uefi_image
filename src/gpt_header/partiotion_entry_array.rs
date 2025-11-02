use super::common::*;

#[repr(u16)]
#[derive(Debug,Clone,Copy)]
pub enum PartitionTypeGuid {
    UnusedEntry,
    EFISystemPartition,
    PartitionContainingAlegacyMBR
}

const PARITION_NAME_LENGTH: usize = 72;

struct GptPartiotionEntryArray<const N:usize>{
    entries : [GptPartiotionEntry;N],
}

#[allow(non_snake_case)]
#[repr(C,packed(1))]
#[derive(Debug)]
pub struct GptPartiotionEntryData
{
    PartitionTypeGUID: PartitionTypeGuid,
    UniquePartitionGUID: GUID,
    StartingLBA: u64,
    EndingLBA: u64,
    Attributes: u64,
    PartitionName: [u8;PARITION_NAME_LENGTH],
}

pub struct GptPartiotionEntry
{
    data: GptPartiotionEntryData,
    padding: u32, //INFO: SizeOf PartitionEntry - 128
}

impl GptPartiotionEntry {
    pub fn new(partition_type: PartitionTypeGuid, partion_name: [u8;PARITION_NAME_LENGTH]) -> Result<Self,()> {
        Ok(Self{
            data: GptPartiotionEntryData {
                PartitionTypeGUID: partition_type,
                UniquePartitionGUID: (),
                StartingLBA: (),
                EndingLBA: (),
                Attributes: (),
                PartitionName: partion_name,
            }
        })
    }
    
}
