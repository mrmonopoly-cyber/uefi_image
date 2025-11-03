use super::common::*;

#[repr(u16)]
#[derive(Debug,Clone,Copy)]
pub enum PartitionTypeGuid {
    UnusedEntry,
    EFISystemPartition,
    PartitionContainingAlegacyMBR
}

const PARITION_NAME_LENGTH: usize = 72;

#[allow(non_snake_case)]
#[repr(C,packed(1))]
#[derive(Debug,Clone, Copy)]
pub struct GptPartiotionEntryData
{
    PartitionTypeGUID: PartitionTypeGuid,
    UniquePartitionGUID: GUID,
    StartingLBA: LBA,
    EndingLBA: LBA,
    Attributes: LBA,
    PartitionName: [u8;PARITION_NAME_LENGTH],
}

#[derive(Clone,Copy)]
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
