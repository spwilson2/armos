#!/bin/sh

cd kernel && find . | grep -e ".*[.]o" -e ".*[.]img" -e ".*[.]elf" | xargs rm 

rm -rf bootloader raspbootin
