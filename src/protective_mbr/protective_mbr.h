#pragma once

#include <stdio.h>
typedef enum
{
  ProtectiveMbrStatus_invalid_parameter,
  ProtectiveMbrStatus_success, 
}ProtectiveMbrStatus;
ProtectiveMbrStatus add_protective_mbr(FILE* image);
