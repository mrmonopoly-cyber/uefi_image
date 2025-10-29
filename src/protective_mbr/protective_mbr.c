#include "protective_mbr.h"
#include <stddef.h>
#include <stdint.h>

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
  uint8_t boot_code[424];
  uint8_t unique_mbr_disk_signature[4];
  uint8_t unknown[2];
  partitionRecord record[4];
  uint16_t signature;
}ProtectiveMbrBase;

#define PROTECTIVE_MBR_BASE_DEFAULT \
    {\
      .boot_code = {0},\
      .unique_mbr_disk_signature = {0},\
      .unknown = {0},\
      .record = \
      {\
        {\
          .BootIndicator =1,\
          .StartingCHS[0] = 0x00,\
          .StartingCHS[1] = 0x02,\
          .StartingCHS[2] = 0x00,\
          .OSType = 0xEE,\
          .EndingCHS[0] = 0xFF,\
          .EndingCHS[1] = 0xFF,\
          .EndingCHS[2] = 0xFF,\
          .StartingLBA = 0x00000001,\
        },\
        {0},\
        {0},\
        {0},\
      },\
      .signature = 0xAA55,\
    }

typedef struct
{
  ProtectiveMbrBase base;
  uint8_t reserved[]; //INFO: Logical Block Size - 512. The rest of the logical block, if any, is reserved. Set to zero.
}ProtectiveMbrCommon;

typedef struct
{
  ProtectiveMbrBase base;
  uint8_t reserved[0];
}ProtectiveMbrLB512;

//public

static int _write_protective_mbr(FILE* image, const ProtectiveMbrCommon* const mbr, const uint32_t size)
{
  uint32_t err=0;
  err=fwrite(mbr, 1, size, image);

  if (err!=size)
  {
    return -1;
  }

  return 0;
}

ProtectiveMbrStatus add_protective_mbr(FILE* image, const MbrLogicalBlockSize logical_block_size)
{
  if (!image)
  {
    return ProtectiveMbrStatus_invalid_parameter;
  }

  int err=0;

  switch (logical_block_size)
  {
    case MbrLogicalBlockSize_512:
      ProtectiveMbrLB512 mbr=
      {
        .base = PROTECTIVE_MBR_BASE_DEFAULT,
        .reserved = {},
      };
      err = _write_protective_mbr(image,(ProtectiveMbrCommon *) &mbr, sizeof(mbr));
      break;
    default:
      return ProtectiveMbrStatus_invalid_parameter;
  }

  if (err<0)
  {
    return ProtectiveMbrStatus_error_writing_image;
  }

  return ProtectiveMbrStatus_success;
}
