# This is an implementation of using non-recursive make
ifndef BASEDIR
BASEDIR:=$(CURDIR)
else
$(error This make file should not be included!)
endif

HOST?=armv7-a
HOST_PRFX?=arm-none-eabi-

CC := $(HOST_PRFX)gcc
LD := $(HOST_PRFX)ld
OBJCOPY :=$(HOST_PRFX)objcopy

CFLAGS?= -O0 -std=gnu99 -Wall -Wextra
CPPFLAGS?=
LDFLAGS?=
LIBS?=

#CLFLAGS:=$(CFLAGS)

CLEAN:= kernel.img

.PHONY: all

all: kernel.img

# Recursively build items. Set the CURDIR to trick the included file.
CURDIR:=$(CURDIR)/kernel
include $(CURDIR)/makefile.mk
CURDIR:=$(BASEDIR)

#dir:= libc
#include $(dir)/Make.mk


.PHONY: clean run

run: kernel.img boot.sh
	bash boot.sh

clean:
	-rm -f $(CLEAN)
