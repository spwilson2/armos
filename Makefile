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

RUSTFLAGS := --target=$(RUST_TARGET) -C lto -C panic=abort

	#-C lto \
	#-C no-prepopulate-passes -C no-stack-check -C opt-level=3 \
	#-Z no-landing-pads \
	#-A dead_code
#RUSTFLAGS := --target=$(RUST_TARGET) -C lto -O
#RUSTFLAGS := --target=$(RUST_TARGET) -O
ARCH_ASFLAGS:= -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a

OBJS := boot.o
OBJS := $(OBJS:%=$(OBJDIR)%)
BIN := ../kernel.$(ARCH).bin
SRCDIR := src/
RUSTSRC := $(shell find $(SRCDIR) -name *.rs)

CLEAN := $(OBJS) kernel.img kernel.elf


.PHONY: clean all run

all: kernel.img

run: kernel.img boot.sh
	bash boot.sh

clean:
	rm -rf .obj $(CLEAN)

kernel.img: $(OBJS) $(OBJDIR)kernel.rlib
	$(CROSS_LD) $^ $(LDFLAGS) -T $(SRCDIR)$(LINKSCRIPT) -o kernel.elf
	$(CROSS_OBJCOPY) kernel.elf -O binary $@

#$(OBJDIR)kernel.rlib: src/main.rs $(RUSTSRC)
$(OBJDIR)kernel.rlib: src/main.rs $(RUSTSRC)
	echo $(RUSTSRC)
	@mkdir -p $(dir $@)
	$(RUSTC) $(RUSTFLAGS) -o $@ $<

#$(OBJDIR)%.o:%.rs
#	@mkdir -p $(dir $@)
#	$(RUSTC) $< $(RUSTFLAGS) --out-dir=$(OBJDIR) --emit=obj,dep-info -o $@

$(OBJDIR)%.o:src/arch/$(ARCH)/%.S
	@mkdir -p $(dir $@)
	@$(CROSS_AS) $< -c $(ARCH_ASFLAGS) -o $@ 

-include $(shell find $(OBJDIR) -name *.d )
