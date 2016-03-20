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

This also means the new UART Address is 0x3F2010
