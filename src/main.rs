use std::env::args;
use std::fs::OpenOptions;
use std::process::exit;

use self::image_write::ImageWrite;

mod protective_mbr;
mod gpt_partition_table;
mod image_write;

fn main() {
    let mut disk_image_name = String::from("test.img");
    for arg in args().skip(1)
    {
        println!("{}",arg);
        disk_image_name = arg;
    }

    let mut uefi_image = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&disk_image_name)
        .unwrap_or_else(|err|{
            println!("failed creating image {} with errror: {}",disk_image_name, err);
            exit(1);
        });

    let mbr = protective_mbr::ProtectiveMbr::default();
    let gpt_table = gpt_partition_table::GptPartitionTable::default();

    mbr.write_to_image(&mut uefi_image).unwrap_or_else(|err|{
        println!("failed writing protective mbr to image {} with errror: {}",disk_image_name, err);
        exit(1);
    });


    gpt_table.write_to_image(&mut uefi_image).unwrap_or_else(|err|{
        println!("failed writing gpt table to image {} with errror: {}",disk_image_name, err);
        exit(1);
    });
}
