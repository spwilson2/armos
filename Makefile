LD:=arm-none-eabi-ld
CC:=arm-none-eabi-gcc

all: target/arm-none-eabi/debug/kernel

target/arm-none-eabi/debug/libkernel.a: src/* src/*/* src/*/*/*
	CC=$(CC) xargo build --target arm-none-eabi --all -v

target/arm-none-eabi/debug/kernel: linkers/linker.ld target/arm-none-eabi/debug/libkernel.a
	#$(CC) --gc-sections -z max-page-size=0x1000 -T $< -o $@ target/arm-none-eabi/debug/libkernel.a asm/start.s
	$(CC) -Wl,--gc-sections -nostdlib -nostartfiles -z max-page-size=0x1000 -T $< -o $@  asm/start.s target/arm-none-eabi/debug/libkernel.a 
	arm-none-eabi-objcopy --strip-debug $@
	arm-none-eabi-objcopy --only-keep-debug $@ $@.sym


clean: 
	rm -rf target
