### MMIO

Thanks to [BrianSidebotham](https://github.com/BrianSidebotham/arm-tutorial-rp) for getting the right compile flags for the raspi-2.

arm-none-eabi-gcc -O0 -DRPI2 -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a -mtune=cortex-a7 -nostartfiles -g armc-02.c -o kernel.elf

RPI2 Flag is given so set the base of the GPIO. In this case else is the A,B,B+.

```
#ifdef RPI2
    #define GPIO_BASE       0x3F200000UL
#else
    #define GPIO_BASE       0x20200000UL
#endi
```

This also means the new UART Address is 0x3F201000


### Booting
go to https://github.com/raspberrypi/firmware boot directory and grab
bootcode.bin and start.elf the minimum number of files on the sd card.
Then just add your own kernel as kernel7.img

Addtionally to avoid the sd card write shuffle use [raspbootin](https://github.com/mrvn/raspbootin)!
