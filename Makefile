ARCH ?= armv7a

ifeq ($(ARCH),armv7a)
    TRIPLE ?= arm-none-eabi-
	RUST_TARGET ?=arm-unknown-linux-gnueabi
	#RUST_TARGET ?=arch.json
else
    $(error Unknown architecture $(ARCH))
endif

RUSTC ?= rustc

CROSS_LD := $(TRIPLE)ld
CROSS_AS := $(TRIPLE)as
CROSS_OBJDUMP := $(TRIPLE)objdump
CROSS_OBJCOPY := $(TRIPLE)objcopy

OBJDIR := .obj/$(ARCH)/

LINKSCRIPT := arch/$(ARCH)/linker.ld
TARGETSPEC := arch/$(ARCH)/target.json

LDFLAGS := --gc-sections

RUSTFLAGS := --target=$(RUST_TARGET) -C lto \
	-C no-prepopulate-passes -C no-stack-check -O \
	-Z no-landing-pads \
	-A dead_code
ARCH_ASFLAGS:= -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a

OBJS := boot.o kernel.o
OBJS := $(OBJS:%=$(OBJDIR)%)
BIN := ../kernel.$(ARCH).bin
SRCDIR := src/

CLEAN := $(OBJS) kernel.img kernel.elf


.PHONY: clean all run

all: kernel.img

run: kernel.img boot.sh
	bash boot.sh

clean:
	rm -rf .obj $(CLEAN)

kernel.img: $(OBJS)
	$(CROSS_LD) $(OBJS) $(LDFLAGS) -T $(SRCDIR)$(LINKSCRIPT) -o kernel.elf
	$(CROSS_OBJCOPY) kernel.elf -O binary $@

$(OBJDIR)kernel.o: src/main.rs Makefile
	@mkdir -p $(dir $@)
	$(RUSTC) $< $(RUSTFLAGS) --emit=obj,dep-info --out-dir $(OBJDIR)

#$(OBJDIR)%.o:%.rs
#	@mkdir -p $(dir $@)
#	$(RUSTC) $< $(RUSTFLAGS) --out-dir=$(OBJDIR) --emit=obj,dep-info -o $@

$(OBJDIR)%.o:src/arch/$(ARCH)/%.S
	@mkdir -p $(dir $@)
	@$(CROSS_AS) $< -c $(ARCH_ASFLAGS) -o $@ 

-include $(shell find $(OBJDIR) -name *.d )
