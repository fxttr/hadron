BUILD_DIR=build/
MOUNT_DIR=mnt/
BOOTLOADER_DIR=boot/Syndicate/
IMG_FILE=zen.img
PROFILE=debug

all: link-kernel

compile-kernel:
	@echo "Compiling kernel"
	@-cargo build

link-kernel: compile-kernel
	@echo "Linking kernel"
	@ld -T src/link.ld ${BUILD_DIR}/bootstrap.o target/x86_64-unknown-zen/${PROFILE}/libzen.a -o ${BUILD_DIR}/zen

install-bootloader: ${IMG_FILE}
	@echo "Installing Syndicate"
	@dd if=loader.bin of=${IMG_FILE} seek=0 count=1 bs=512 conv=notrunc &>/dev/null

install: ${MOUNT_DIR} compile-kernel link-kernel install-bootloader
	@echo "Installing kernel image"
	@doas mount -o loop ${BUILD_DIR}/${IMG_FILE} ${MOUNT_DIR}
	@doas cp ${BUILD_DIR}/zen ${MOUNT_DIR}/mkern.sys
	@sync
	@doas umount ${MOUNT_DIR}

run: run-clean install
	@echo "Starting..."
	@qemu-system-x86_64 -hda ${BUILD_DIR}/${IMG_FILE}

${MOUNT_DIR}:
	@mkdir $@

${IMG_FILE}:
	@echo "Creating OS disk"
	@dd if=/dev/zero of=${BUILD_DIR}/zen.img count=50 bs=1M
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