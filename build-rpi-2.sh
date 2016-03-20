#!/bin/sh

arm-none-eabi-gcc -O0 -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a -mtune=cortex-a7 -nostartfiles -g $1 -o kernel.elf && arm-none-eabi-objcopy kernel.elf -O binary kernel.img
