#!/bin/bash

mkdir boot
mount /dev/sde1 boot && cp kernel.img boot/kernel7.img && umount boot && rmdir boot
