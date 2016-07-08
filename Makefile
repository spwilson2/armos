ARCH ?= armv7a


ifeq ($(ARCH),armv7a)
    TRIPLE ?= arm-none-eabi-
	RUST_TARGET ?=arm-unknown-linux-gnueabi
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

RUSTFLAGS := -O --target=$(RUST_TARGET) --crate-type=lib 
ARCH_ASFLAGS:= -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a

OBJS := boot.o rust-kernel.o
OBJS := $(OBJS:%=$(OBJDIR)%)
BIN := ../kernel.$(ARCH).bin
SRCDIR := src/

.PHONY: clean all

all: kernel.bin

clean:
	rm -rf .obj kernel.bin

kernel.bin: $(OBJS)
	$(CROSS_LD) $(OBJS) $(LDFLAGS) -T $(SRCDIR)$(LINKSCRIPT) -o $@

$(OBJDIR)rust-kernel.o: src/main.rs Makefile
	@mkdir -p $(dir $@)
	$(RUSTC) $< $(RUSTFLAGS) --emit=obj,dep-info -o $@

$(OBJDIR)%.o:%.rs
	@mkdir -p $(dir $@)
	$(RUSTC) $< $(RUSTFLAGS) --out-dir=$(OBJDIR) --emit=obj,dep-info -o $@

$(OBJDIR)%.o:src/arch/$(ARCH)/%.S
	@mkdir -p $(dir $@)
	@$(CROSS_AS) $< -c $(ARCH_ASFLAGS) -o $@ 

-include $(shell find $(OBJDIR) -name *.d )
