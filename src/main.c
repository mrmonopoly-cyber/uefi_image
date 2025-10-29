#include <stdio.h>

#include "protective_mbr/protective_mbr.h"

int main(void)
{
  const char image_name[] = "BOOTX64.efi";
  int err=0;

  FILE* uefi_image = fopen(image_name, "wb");

  if (!uefi_image)
  {
    fprintf(stderr, "failed creating image: %s\n", image_name); 
    return 1;
  }

  if ((err=add_protective_mbr(uefi_image))!=ProtectiveMbrStatus_success)
  {
    fprintf(stderr, "failed adding protective mbr: %d\n", err); 
    return 1;
  }


  return 0;
}
