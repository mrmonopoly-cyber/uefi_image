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

//INFO: Logical Block Size - 512. The rest of the logical block, if any, is reserved. Set to zero.
#define RESERVED_FIELD_SIZE(LBS) LBS - 512 

static int _write_protective_mbr(FILE* image, const ProtectiveMbrBase* const mbr, const uint32_t lbl_size)
{
  uint32_t err=0;
  uint8_t zero=0;

  err=fwrite(mbr, 1, sizeof(*mbr), image);

  if (err!=sizeof(*mbr))
  {
    return -1;
  }

  err=fwrite(&zero, 1,RESERVED_FIELD_SIZE(lbl_size), image);

  return 0;
}

//public

ProtectiveMbrStatus add_protective_mbr(FILE* image, const uint32_t logical_block_size)
{
  ProtectiveMbrBase mbr = PROTECTIVE_MBR_BASE_DEFAULT;

  if (!image)
  {
    return ProtectiveMbrStatus_invalid_parameter;
  }

  if (_write_protective_mbr(image,(ProtectiveMbrBase *) &mbr, logical_block_size)<0)
  {
    return ProtectiveMbrStatus_error_writing_image;
  }

  return ProtectiveMbrStatus_success;
}
