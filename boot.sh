#/bin/bash

if [ ! -d bootloader ]; then

git submodule update --init --recursive &&
mv raspbootin bootloader

fi

if $!; then

cd bootloader &&
make &&
cd .. &&
bootloader/raspbootcom/raspbootcom /dev/ttyUSB0 kernel/kernel.img

fi

exit
