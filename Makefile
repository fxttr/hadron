BUILD_DIR=build/
MOUNT_DIR=mnt/
BOOTLOADER_DIR=boot/Syndicate/
IMG_FILE=lumos.img
PROFILE=debug

all: link-kernel

compile-kernel:
	@echo "Compiling kernel"
	@cd kernel/
	@-cargo build
	@cd -

link-kernel: compile-kernel
	@echo "Linking kernel"
	@cd kernel/
	@ld -T kernel/src/link.ld ${BUILD_DIR}/s2.o target/x86_64-unknown-lumos/${PROFILE}/liblumos.a -o ${BUILD_DIR}/lumos
	@cd -

install-bootloader: ${IMG_FILE}
	@echo "Installing Syndicate"
	@dd if=loader.bin of=${IMG_FILE} seek=0 count=1 bs=512 conv=notrunc &>/dev/null

install: ${MOUNT_DIR} compile-kernel link-kernel install-bootloader
	@echo "Installing kernel image"
	@doas mount -o loop ${BUILD_DIR}/${IMG_FILE} ${MOUNT_DIR}
	@doas cp ${BUILD_DIR}/lumos ${MOUNT_DIR}/mkern.sys
	@sync
	@doas umount ${MOUNT_DIR}

run: run-clean install
	@echo "Starting..."
	@qemu-system-x86_64 -hda ${BUILD_DIR}/${IMG_FILE}

${MOUNT_DIR}:
	@mkdir $@

${IMG_FILE}:
	@echo "Creating OS disk"
	@dd if=/dev/zero of=${BUILD_DIR}/lumos.img count=50 bs=1M
	@parted -s -a optimal -- ${BUILD_DIR}/${IMG_FILE} mklabel msdos
	@parted -s -a optimal -- ${BUILD_DIR}/${IMG_FILE} mkpart primary 1MiB 100%
	@mkfs.fat -F 32 ${BUILD_DIR}/${IMG_FILE}
	

.PHONY: clean run-clean

clean:
	@echo "Soft cleaning"
	@rm -rf build/*
	@cargo clean

run-clean:
	@echo "Cleaning run targets"
	@-mdconfig -du md0
	@rm -rf *.img