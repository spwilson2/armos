#/bin/bash

if [ ! -d raspbootin ]; then

git submodule update --init --recursive

fi

if $!; then

cd raspbootin &&
make &&
cd .. &&

# Use tmux otherwise the terminal is gonna get fuuuuked
tmux new 'sudo raspbootin/raspbootcom/raspbootcom /dev/ttyUSB0 kernel.img'

fi
