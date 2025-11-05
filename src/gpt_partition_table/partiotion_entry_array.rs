use super::common::*;

#[derive(Debug,Clone,Copy)]
pub enum PartitionTypeGuid {
    UnusedEntry(GUID),
    EFISystemPartition(GUID),
    PartitionContainingAlegacyMBR(GUID)
}

impl PartitionTypeGuid{
    pub const UNUSED_ENTRY_GUID: PartitionTypeGuid= PartitionTypeGuid::UnusedEntry([0;16]);
    pub const EFI_SYSTEM_PARTITION_GUID: PartitionTypeGuid = PartitionTypeGuid::UnusedEntry([
        0xC1,0x2A,0x73,0x28,0xF8,0x1F,0x11,0xD2,0xBA,0x4B,0x00,0xA0,0xC9,0x3E,0xC90,0x3B
    ]);
    pub const LEGACY_MBR_GUID : PartitionTypeGuid = PartitionTypeGuid::UnusedEntry([
        0x02,0x4D,0xEE,0x41,0x33,0xE7,0x11,0xD3,0x9D,0x69,0x00,0x08,0xC7,0x81,0xF3,0x9F
    ]);
}

pub const PARITION_NAME_LENGTH: usize = 36;

#[allow(non_snake_case)]
#[repr(C,packed(1))]
#[derive(Debug,Clone, Copy)]
pub struct GptPartiotionEntryData
{
    PartitionTypeGUID: PartitionTypeGuid,
    UniquePartitionGUID: GUID,
    StartingLBA: LBA,
    EndingLBA: LBA,
    Attributes: u64,
    PartitionName: UnicodeString<PARITION_NAME_LENGTH>, //INFO: unicode string
}

#[derive(Clone,Copy)]
pub struct GptPartiotionEntry
{
    data: GptPartiotionEntryData,
    padding: u32, //INFO: SizeOf PartitionEntry - 128
}

impl GptPartiotionEntry {
    pub fn new(partition_type: PartitionTypeGuid, partion_name: UnicodeString<PARITION_NAME_LENGTH>) -> Result<Self,()> {
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
