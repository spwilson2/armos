
#TARGET:=arm-eabi
TOOLCHAIN_PREFIX:=arm-none-eabi-
BUILD_ARCH:=armv7-a
BUILD_DIR:=
TOOL_DIR:=../tools/bin

#TODO:
#CROSS_CC:=$(TOOL_DIR)/$(TOOLCHAIN_PREFIX)gcc
#CROSS_LD:=$(TOOL_DIR)/$(TOOLCHAIN_PREFIX)ld
#CROSS_AS:=$(TOOL_DIR)/$(TOOLCHAIN_PREFIX)gcc
#CROSS_OBJCOPY:=$(TOOL_DIR)/$(TOOLCHAIN_PREFIX)objcopy

CC:=$(TOOLCHAIN_PREFIX)gcc
LD:=$(TOOLCHAIN_PREFIX)ld
AS:=$(TOOLCHAIN_PREFIX)as
OBJCOPY:=$(TOOLCHAIN_PREFIX)objcopy

TARGET_SYSROOT:=$(BUILD_DIR)/$(BUILD_ARCH)/sysroot

PROJDIRS := kernel

SRCFILES := $(shell find $(PROJDIRS) -type f -name "\*.c")
HDRFILES := $(shell find $(PROJDIRS) -type f -name "\*.h")
DEPFILES := $(patsubst %.c,%.d,$(SRCFILES))

HOSTARCH ?= $(BUILD_ARCH)

HOSTARCH := armv7-a

OLD_CURDIR := $(CURDIR)
CURDIR := $(CURDIR)/kernel

include kernel/Makefile

CURDIR := $(OLD_CURDIR)

.PHONY: all boot

all: kernel

kernel: kernel.img


boot:
	bash boot.sh


.PHONY: clean todo

todo:
	-@for file in $(ALLFILES:Makefile=); do fgrep -H -e TODO -e FIXME $$file; done; true

clean:
	rm -rf $(OBJS) kernel.img
