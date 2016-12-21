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

#TODO: Use cargo for importing other libraries.
CARGO:= xargo build --manifest-path
CARGOFLAGS := --target=$(RUST_TARGET) --release

#RUSTFLAGS := --target=$(RUST_TARGET) -A dead_code \
#	-Z no-landing-pads -L $(OBJDIR) \
#	-C no-prepopulate-passes -C panic=abort \

ifdef $(DEBUG)
RUSTFLAGS := --target=$(RUST_TARGET) -A dead_code \
	-Z no-landing-pads -L $(OBJDIR) \
	-C panic=abort \
	-C no-stack-check \
	-C debuginfo=2 \
	-C opt-level=0
else
RUSTFLAGS := --target=$(RUST_TARGET) -A dead_code \
	-Z no-landing-pads -L $(OBJDIR) \
	-C panic=abort \
	-C debuginfo=0 \
	-C opt-level=3 \
	--cfg debug
endif


#RUSTFLAGS := --target=$(RUST_TARGET) -C lto -O
#RUSTFLAGS := --target=$(RUST_TARGET) -C lto
ARCH_ASFLAGS:= -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a

SRCS := boot.S
OBJS := $(SRCS:%.S=%.o)
OBJS := $(OBJS:%=$(OBJDIR)%)
BIN := ../kernel.$(ARCH).bin
SRCDIR := src/
RUSTSRC := $(shell find $(SRCDIR) -name *.rs)

CLEAN := $(OBJS) kernel.img kernel.elf

.PHONY: clean all run boot debug 

all: $(OBJDIR) kernel.img
debug: all 

debug: DEBUG=true


boot: kernel.img
	bash boot.sh

run: kernel.img boot.sh
	bash boot.sh
clean:
	rm -rf .obj $(CLEAN)
	cargo clean --manifest-path crates/rlibc/Cargo.toml

kernel.img: kernel.elf 
	$(CROSS_OBJCOPY) kernel.elf -O binary $@

kernel.elf: $(OBJS) $(OBJDIR)kernel.lib $(SRCDIR)$(LINKSCRIPT)
	$(CROSS_LD) $^ $(LDFLAGS) -T $(SRCDIR)$(LINKSCRIPT) -o kernel.elf

$(OBJDIR)kernel.lib: src/main.rs $(RUSTSRC) Makefile $(OBJDIR)librlibc.rlib
	$(RUSTC) $(RUSTFLAGS) -o $@ $< -L $(OBJDIR)

$(OBJDIR)librlibc.rlib: crates/rlibc/src/lib.rs Makefile
	$(CARGO) crates/rlibc/Cargo.toml $(CARGOFLAGS)
	mv crates/rlibc/target/$(RUST_TARGET)/release/librlibc.rlib $@

$(OBJDIR)%.o:src/arch/$(ARCH)/%.S Makefile
	@$(CROSS_AS) $< -c $(ARCH_ASFLAGS) -o $@ 


$(shell mkdir -p $(OBJDIR))

#-include $(shell find $(OBJDIR) -name *.d )
-include $(wildcard $(OBJDIR)*.d )
