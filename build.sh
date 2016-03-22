#!/bin/bash

cd kernel

arm-none-eabi-gcc -O0 -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a -mtune=cortex-a7 -nostartfiles -fpic -ffreestanding -c boot.S -o boot.o &&

arm-none-eabi-gcc -O0 -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a -mtune=cortex-a7 -nostartfiles -fpic -ffreestanding -std=gnu99 -c kernel.c -o kernel.o -Wall -Wextra &&

arm-none-eabi-gcc -T linker.ld -o myos.elf -ffreestanding -O0 -nostdlib boot.o kernel.o &&
arm-none-eabi-objcopy myos.elf -O binary kernel.img

