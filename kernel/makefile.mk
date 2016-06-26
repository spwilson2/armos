#HOST?=$(shell ../default-host.sh)
#HOSTARCH:=$(shell ../target-triplet-to-arch.sh $(HOST))
.PHONY: bad_make
bad_make:
	@echo "This makefile can't be run, please use the top level Makefile."
	
# Default the host arch to arm.
HOSTARCH?=armv7-a

KERNEL_CFLAGS += -fpic -ffreestanding -nostartfiles

# Save this directory so we have it after includes.
BACKUP_DIR:=$(CURDIR)

# Help with recursion.
ARCHDIR:=$(CURDIR)/arch/$(HOSTARCH)

# Fake that archdir is the current directory when we include the file.
CURDIR:=$(ARCHDIR)
export CURDIR
include $(ARCHDIR)/make.config

# Reset the CURDIR back to here.
CURDIR:=$(BACKUP_DIR)

KERNEL_ARCH_SRC:= $(shell find $(CURDIR) -type f -iname '*.[cS]')
AUTO_LIBS:= $(addprefix -include,$(shell find $(CURDIR)/autoinclude/ -type f -iname '*.h'))
LIBS+= -nostdlib $(AUTO_LIBS)
OBJS+=$(KERNEL_OBJS)
DEPS+=$(SRC:.c=.d)

include $(DEPS_FILES)

CLEAN :=  $(CLEAN) $(OBJS) $(DEPS_FILES) kernel.elf

kernel.img: $(OBJS)
	$(CC) -T $(ARCHDIR)/linker.ld -o kernel.elf $(OBJS) $(LDFLAGS) $(LIBS)
	$(OBJCOPY) kernel.elf -O binary $@

#%.d: %.c
#	@set -e; rm -f $@; \
#	$(CC) -M $(CPPFLAGS) $< > $@.$$$$; \
#	sed 's,\($*\)\.o[ :]*,\1.o $@ : ,g' < $@.$$$$ > $@; \
#	rm -f $@.$$$$

%.o: %.c
	$(CC) -c $< -o $@ $(CFLAGS) $(KERNEL_CFLAGS) $(LIBS)

%.d: %.c
	$(CC) -MD -MP $@ $(CFLAGS) $(KERNEL_CFLAGS) $(LIBS)

%.o: %.S
	$(CC) -c $< -o $@ $(KERNEL_CFLAGS)
