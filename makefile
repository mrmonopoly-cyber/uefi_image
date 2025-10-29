# USER PARAMETERS
FEATURES ?= 
SRC ?= .
CC ?= gcc

# MAKE PARAMETERS
OUT := uefi_image
UEFI_IMAGE_SRC := $(SRC)/src
UEFI_IMAGE_DEPS := $(shell find $(UEFI_IMAGE_SRC) -type f -name "*.c")
BUILD := build
UEFI_IMAGE_OBJS := $(patsubst $(UEFI_IMAGE_SRC)/%.c,$(BUILD)/%.o,$(UEFI_IMAGE_DEPS))
WARNINGS := -Wall -Wextra 
CFLAGS := $(WARNINGS)

.PHONY : all

all: build_uefi_image

build_uefi_image: $(UEFI_IMAGE_OBJS)
	$(CC) $(CFLAGS) $(FEATURES) $(UEFI_IMAGE_OBJS) -o $(OUT)

build: build_uefi_image

$(BUILD)/%.o: $(UEFI_IMAGE_SRC)/%.c
	@mkdir -p $(dir $@)
	$(CC) -o $@ $< -c $(CFLAGS) $(FEATURES)

clang-lsp:
	bear -- make build

clean:
	rm -rf $(BUILD) $(OUT)
