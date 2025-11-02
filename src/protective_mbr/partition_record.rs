#[repr(C,packed(1))]
#[derive(Default,Clone, Copy)]
pub struct PartitionRecord{
    boot_indicator :u8,
    starting_chs : [u8;3],
    os_type: u8,
    ending_chs: [u8;3],
    staring_lba : u32,
    size_in_lba : u32,
}

impl PartitionRecord {
    pub fn uefi_partition_record() -> Self {
        Self {
            boot_indicator: 1,
            starting_chs: [0x00,0x02,0x00],
            os_type: 0xEE,
            ending_chs: [0xFF,0xFF,0xFF],
            staring_lba: 0x00000001,
            size_in_lba: 0, //TODO: not yet implemented
        }
    }
}
