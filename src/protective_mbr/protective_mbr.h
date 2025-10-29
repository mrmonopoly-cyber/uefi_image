#pragma once

#include <stdint.h>
#include <stdio.h>

typedef enum
{
  ProtectiveMbrStatus_invalid_parameter=0,
  ProtectiveMbrStatus_error_writing_image,
  ProtectiveMbrStatus_success, 
}ProtectiveMbrStatus;
ProtectiveMbrStatus add_protective_mbr(FILE* image, const uint32_t logical_block_size);
