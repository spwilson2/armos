#HOST?=$(shell ../default-host.sh)
#HOSTARCH:=$(shell ../target-triplet-to-arch.sh $(HOST))
.PHONY: bad_make
bad_make:
	@echo "This makefile can't be run, please use the top level Makefile."
	
# Default the host arch to arm.
HOSTARCH?=armv7-a

KERNEL_CFLAGS += -fpic -ffreestanding -nostartfiles

# Save this directory so we have it after includes.
KERNEL_BACKUP_CURDIR:=$(CURDIR)

# Help with recursion.

ARCHDIR:=$(CURDIR)/arch/$(HOSTARCH)

# Fake that archdir is the current directory when we include the file.
CURDIR:=$(ARCHDIR)
include $(ARCHDIR)/make.config

# Reset the CURDIR back to here.
CURDIR:=$(KERNEL_BACKUP_CURDIR)

LIBS := -nostdlib 

OBJS:= $(KERNEL_OBJS)

CLEAN :=  $(CLEAN) $(OBJS) kernel.elf

kernel.img: $(OBJS)
	$(CC) -T $(ARCHDIR)/linker.ld -o kernel.elf $(OBJS) $(LDFLAGS) $(LIBS)
	$(OBJCOPY) kernel.elf -O binary $@

%.o: %.c
	$(CC) -c $< -o $@ $(CFLAGS) $(KERNEL_CFLAGS)

%.o: %.S
	$(CC) -c $< -o $@ $(KERNEL_CFLAGS)
