# This is an implementation of using non-recursive make
#

HOST?=armv7-a
HOST_PRFX?=arm-none-eabi-

CC := $(HOST_PRFX)gcc
LD := $(HOST_PRFX)ld
OBJCOPY :=$(HOST_PRFX)objcopy

CFLAGS?= -O0 -std=gnu99
CPPFLAGS?=
LDFLAGS?=
LIBS?=

#CLFLAGS:=$(CFLAGS)

CLEAN:= kernel.img

.PHONY: all

all: kernel.img

# Recursively build items.
DIR:= kernel
include $(DIR)/makefile.mk

#dir:= libc
#include $(dir)/Make.mk


.PHONY: clean

run: kernel.img boot.sh
	bash boot.sh

clean:
	-rm -f $(CLEAN)
