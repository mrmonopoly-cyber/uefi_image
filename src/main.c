#include <stdint.h>
#include <stdio.h>

#include "protective_mbr/protective_mbr.h"

int main(int argc, char **argv)
{
  const char default_disk_image_name[] = "test.img";
  const char* disk_image_name_ptr = default_disk_image_name;

  if (argc < 2)
  {
    printf("using default image name: %s\n",default_disk_image_name);
  }else
  {
    disk_image_name_ptr = argv[1];
  }

  int err=0;

  FILE* uefi_image = fopen(disk_image_name_ptr, "wb");

  if (!uefi_image)
  {
    fprintf(stderr, "failed creating image: %s\n", disk_image_name_ptr); 
    return 1;
  }

  if ((err=add_protective_mbr(uefi_image,512))!=ProtectiveMbrStatus_success)
  {
    fprintf(stderr, "failed adding protective mbr: %d\n", err); 
    return 1;
  }


  return 0;
}
