#include "protective_mbr.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct
{
  uint8_t BootIndicator;
  uint8_t StartingCHS[3];
  uint8_t OSType;
  uint8_t EndingCHS[3];
  uint32_t StartingLBA;
  uint32_t SizeInLBA;
}partitionRecord;

typedef struct
{
  uint8_t boot_code[440];
  uint8_t unique_mbr_disk_signature[4];
  uint8_t unknown[2];
  partitionRecord record[4];

  uint16_t signature;
  uint8_t reserved[]; //INFO: Logical Block Size - 512. The rest of the logical block, if any, is reserved. Set to zero.
}ProtectiveMbr;

ProtectiveMbrStatus add_protective_mbr(FILE* image, const uint32_t logical_block_size)
{
  size_t err=0;
  ProtectiveMbr *mbr=NULL;
  const uint32_t mbr_size = sizeof(*mbr) + logical_block_size - 512;

  if (!image)
  {
    return ProtectiveMbrStatus_invalid_parameter;
  }
  mbr = calloc(1,mbr_size);

  mbr->record[0].BootIndicator =1;
  mbr->record[0].StartingCHS[0] = 0x00;
  mbr->record[0].StartingCHS[1] = 0x02;
  mbr->record[0].StartingCHS[2] = 0x00;
  mbr->record[0].OSType = 0xEE;
  mbr->record[0].StartingLBA = 0x00000001;


  err=fwrite(mbr, 1, mbr_size, image);
  free(mbr);

  if (err!=mbr_size)
  {
    return ProtectiveMbrStatus_error_writing_image;
  }


  return ProtectiveMbrStatus_success;
}
