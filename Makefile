LD:=arm-none-eabi-ld
CC:=arm-none-eabi-gcc

all: target/arm-none-eabi/debug/kernel

target/arm-none-eabi/debug/libkernel.a: src/* src/*/* src/*/*/*
	CC=$(CC) xargo build --target arm-none-eabi --all -v

target/arm-none-eabi/debug/kernel: linkers/linker.ld target/arm-none-eabi/debug/libkernel.a
	#$(CC) --gc-sections -z max-page-size=0x1000 -T $< -o $@ target/arm-none-eabi/debug/libkernel.a asm/start.s
	$(CC) -Wl,--gc-sections -ggdb -nostdlib -nostartfiles -z max-page-size=0x1000 -T $< -o $@  asm/start.s target/arm-none-eabi/debug/libkernel.a 
	arm-none-eabi-objcopy --only-keep-debug $@ $@.sym
	#arm-none-eabi-objcopy $@
	arm-none-eabi-objcopy --strip-debug $@

gdb-nox: target/arm-none-eabi/debug/kernel
	qemu-system-arm -M realview-pbx-a9 -m 512M -kernel $< -nographic -gdb tcp::1234 -S -d guest_errors

clean: 
	rm -rf target
