BUILD_DIR=build/
MOUNT_DIR=mnt/
BOOTLOADER_DIR=boot/Syndicate/
IMG_FILE=disk0.img
PROFILE=debug

all: compile

compile:
	@echo "Compiling kernel"
	@-cargo build 

link:
	@echo "Linking kernel"
	@ld -T src/link.ld ${BUILD_DIR}/s2.o target/x86_64-unknown-lumos/${PROFILE}/liblumos.a -o ${BUILD_DIR}/lumos

install-bootloader: ${IMG_FILE}
	@echo "Installing Syndicate"
	@make -C ${BOOTLOADER_DIR}
	@dd if=${BOOTLOADER_DIR}/loader.bin of=${IMG_FILE} seek=0 count=1 bs=512 conv=notrunc &>/dev/null

install: ${MOUNT_DIR} compile link install-bootloader
	@echo "Installing kernel image"
	@mount -t msdosfs /dev/md0s1 ${MOUNT_DIR}
	@cp ${BUILD_DIR}/lumos ${MOUNT_DIR}/mkern.sys
	@sync
	@umount ${MOUNT_DIR}

run: run-clean install
	@echo "Starting..."
	@qemu-system-x86_64 -hda /dev/md0

${MOUNT_DIR}:
	@mkdir $@

${IMG_FILE}:
	@echo "Creating OS disk"
	@truncate -s 128m $@
	@mdconfig $@ &>/dev/null
	@gpart create -s mbr md0
	@gpart add -t \!11 md0
	@newfs_msdos -F32 -b 512 /dev/md0s1

.PHONY: clean run-clean

clean:
	@echo "Soft cleaning"
	@make -C ${BOOTLOADER_DIR} clean
	@rm -rf build/*
	@cargo clean

run-clean:
	@echo "Cleaning run targets"
	@-mdconfig -du md0
	@rm -rf *.img
