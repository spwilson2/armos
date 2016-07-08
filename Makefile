DEBUG:=debug

ARCH ?=armv7a
TARGET_TRIPLET:=arm-unknown-linux-gnueabi
RUST_OS:=target/$(TARGET_TRIPLET)/$(DEBUG)/librustos.a

ARCH_TOOL_PFX:=arm-none-eabi-

ARCH_LD:=$(ARCH_TOOL_PFX)ld
ARCH_AS:=$(ARCH_TOOL_PFX)gcc
ARCH_CCFLAGS:= -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a -mtune=cortex-a7 -fpic -ffreestanding -nostartfiles -fno-exceptions
ARCH_LDFLAGS:=

ASSEMBLY_SOURCE_FILES := $(wildcard src/arch/$(ARCH)/*.S)
ASSEMBLY_OBJS:= $(patsubst src/arch/$(ARCH)/%.S, \
		build/arch/$(ARCH)/%.o, $(ASSEMBLY_SOURCE_FILES))

LINKER_SCRIPT := src/arch/$(ARCH)/linker.ld

kernel:=target/kernel-$(ARCH).bin

.PHONY: all cargo

all: $(kernel)

clean:
	@cargo clean
	@rm -rf build

$(kernel): $(ASSEMBLY_OBJS) $(LINKER_SCRIPT) $(RUST_OS)
	$(ARCH_LD) --gc-sections $(ASSEMBLY_OBJS) $(RUST_OS) $(ARCH_LDFLAGS) -T $(LINKER_SCRIPT) -o $@

$(RUST_OS): cargo

cargo:
	@cargo build --target=$(TARGET_TRIPLET)

#%.o:%.S
#	$(ARCH_AS) $< $(ARCH_CCFLAGS) -o $@

# compile assembly files
build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.S
	@mkdir -p $(shell dirname $@)
	@$(ARCH_AS) $< -c $(ARCH_CCFLAGS) -o $@ 
