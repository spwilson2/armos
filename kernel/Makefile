#HOST?=$(shell ../default-host.sh)
#HOSTARCH:=$(shell ../target-triplet-to-arch.sh $(HOST))
 
HOSTARCH ?= armv7-a

#Optional flags
CFLAGS ?= -Werror -Wall -Wextra -pedantic -Wshadow -Wpointer-arith -Wcast-align \

            #-Wwrite-strings -Wmissing-declarations -Wredundant-decls -o0 \
			#-Wnested-externs -Winline -Wno-long-long \
            #-Wuninitialized -Wconversion -Wstrict-prototypes -O0

CPPFLAGS?=
LDFLAGS?=
LIBS?=
 
DESTDIR?=
PREFIX?=/usr/local
EXEC_PREFIX?=$(PREFIX)
BOOTDIR?=$(EXEC_PREFIX)/boot
INCLUDEDIR?=$(PREFIX)/include
 
#Mandatory flags
CFLAGS:=$(CFLAGS) -fpic -ffreestanding -fbuiltin -Wall -Wextra
CPPFLAGS:=$(CPPFLAGS) -D__is_myos_kernel -Iinclude
LDFLAGS:=$(LDFLAGS)
LIBS:=$(LIBS) -nostdlib \

	#-lk -lgcc
 
ARCHDIR:=$(CURDIR)/arch/$(HOSTARCH)
 
include $(ARCHDIR)/make.config
 
CFLAGS:=$(CFLAGS) $(KERNEL_ARCH_CFLAGS)
CPPFLAGS:=$(CPPFLAGS) $(KERNEL_ARCH_CPPFLAGS)
LDFLAGS:=$(LDFLAGS) $(KERNEL_ARCH_LDFLAGS)
LIBS:=$(LIBS) $(KERNEL_ARCH_LIBS)
 
OBJS:=\
$(KERNEL_ARCH_OBJS) \

#kernel/kernel.o \
 
#CRTI_OBJ:=$(ARCHDIR)/crti.o
#CRTBEGIN_OBJ:=$(shell $(CC) $(CFLAGS) $(LDFLAGS) -print-file-name=crtbegin.o)
#CRTEND_OBJ:=$(shell $(CC) $(CFLAGS) $(LDFLAGS) -print-file-name=crtend.o)
#CRTN_OBJ:=$(ARCHDIR)/crtn.o
 
ALL_OUR_OBJS:=\
$(OBJS) \

#$(CRTI_OBJ) \
#$(CRTN_OBJ) \
 
OBJ_LINK_LIST:=\
$(OBJS) \

#$(CRTI_OBJ) \
#$(CRTBEGIN_OBJ) \
#$(CRTEND_OBJ) \
#$(CRTN_OBJ) \
 
all: kernel.img
 
.PHONY: all clean install install-headers install-kernel
 
kernel.img: $(OBJ_LINK_LIST) $(ARCHDIR)/linker.ld
	$(CC) -T $(ARCHDIR)/linker.ld -o $@ $(CFLAGS) $(OBJ_LINK_LIST) $(LDFLAGS) $(LIBS)
 
%.o: %.c
	$(CC) -c $< -o $@ -std=gnu11 $(CFLAGS) $(CPPFLAGS)
 
%.o: %.S
	$(CC) -c $< -o $@ $(CFLAGS) $(CPPFLAGS)
# 
#clean:
#	rm -f myos.kernel $(OBJS) $(ALL_OUR_OBJS) *.o */*.o */*/*.o
# 
#install: install-headers install-kernel
# 
#install-headers:
#	mkdir -p $(DESTDIR)$(INCLUDEDIR)
#	cp -RTv include $(DESTDIR)$(INCLUDEDIR)
# 
#install-kernel: myos.kernel
#	mkdir -p $(DESTDIR)$(BOOTDIR)
#	cp myos.kernel $(DESTDIR)$(BOOTDIR)
