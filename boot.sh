#/bin/bash

if [ ! -d raspbootin ]; then

git submodule update --init --recursive

fi

if $!; then

cd raspbootin &&
make &&
cd .. &&

sudo raspbootin/raspbootcom/raspbootcom /dev/ttyUSB0 kernel.img
reset

fi
