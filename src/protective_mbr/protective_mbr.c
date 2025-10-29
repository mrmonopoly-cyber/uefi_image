#include "protective_mbr.h"

ProtectiveMbrStatus add_protective_mbr(FILE* image)
{
  if (!image)
  {
    return ProtectiveMbrStatus_invalid_parameter;
  }

  //TODO: fill the image

  return ProtectiveMbrStatus_success;
}
