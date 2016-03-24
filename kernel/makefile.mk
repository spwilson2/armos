#HOST?=$(shell ../default-host.sh)
#HOSTARCH:=$(shell ../target-triplet-to-arch.sh $(HOST))
.PHONY: bad_make
bad_make:
	-echo 'This makefile can\'t be run, please use the top level Makefile.'
	
# Default the host arch to arm.
HOSTARCH?=armv7-a
# If we are running make from this directory.
DIR ?= $(CURDIR)

# Help with recursion.
d:= $(DIR)

ARCHDIR:=$(d)/arch/$(HOSTARCH)

include $(ARCHDIR)/make.config

LIBS := -nostdlib 
CFLAGS := $(CFLAGS) -Wall -Wextra
KERNEL_CFLAGS := $(CFLAGS) $(KERNEL_ARCH_CFLAGS) -fpic -ffreestanding -nostartfiles

OBJS:=\
	$(KERNEL_ARCH_OBJS) \

CLEAN :=  $(CLEAN) $(OBJS) kernel.elf

kernel.img: $(OBJS)
	$(CC) -T $(ARCHDIR)/linker.ld -o kernel.elf $(OBJS) $(LDFLAGS) $(LIBS)
	$(OBJCOPY) kernel.elf -O binary $@


%.o: %.c
	$(CC) -c $< -o $@ -std=gnu99 $(KERNEL_CFLAGS)

%.o: %.S
	$(CC) -c $< -o $@ $(KERNEL_CFLAGS)


