# Nuke built-in rules and variables.
MAKEFLAGS += -rR

IMAGE_NAME = hadron

# Convenience macro to reliably declare user overridable variables.
DEFAULT_VAR = .if !defined($1) || $1 == "" $1 := $2 .endif

# Default value for HOST_CC
HOST_CC ?= cc

# Check if HOST_CC is undefined or empty, and set the default value
.if !defined(HOST_CC) || empty(HOST_CC)
    HOST_CC := cc
.endif


.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

.PHONY: run
run: $(IMAGE_NAME).iso
	qemu-system-x86_64 -M q35 -m 2G -cdrom $(IMAGE_NAME).iso -boot d

.PHONY: run-uefi
run-uefi: ovmf $(IMAGE_NAME).iso
	qemu-system-x86_64 -M q35 -m 2G -bios ovmf/OVMF.fd -cdrom $(IMAGE_NAME).iso -boot d

.PHONY: run-hdd
run-hdd: $(IMAGE_NAME).hdd
	qemu-system-x86_64 -M q35 -m 2G -hda $(IMAGE_NAME).hdd

.PHONY: run-hdd-uefi
run-hdd-uefi: ovmf $(IMAGE_NAME).hdd
	qemu-system-x86_64 -M q35 -m 2G -bios ovmf/OVMF.fd -hda $(IMAGE_NAME).hdd

ovmf:
	mkdir -p ovmf
	cd ovmf && fetch https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd -o OVMF.fd

limine:
	git clone https://github.com/limine-bootloader/limine.git --branch=v5.x-branch-binary --depth=1
	${MAKE} -C limine CC="${HOST_CC}"

.PHONY: kernel
kernel:
	${MAKE} -C kernel

$(IMAGE_NAME).iso: limine kernel
	rm -rf iso_root
	mkdir -p iso_root
	cp -v kernel/hadron.elf \
		limine.cfg limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -b limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
	./limine/limine bios-install $(IMAGE_NAME).iso
	rm -rf iso_root

$(IMAGE_NAME).hdd: limine kernel
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd
	gpart create -s gpt $(IMAGE_NAME).hdd
	gpart add -t efi -s 512k $(IMAGE_NAME).hdd
	gpart set -a active -i 1 $(IMAGE_NAME).hdd
	./limine/limine bios-install $(IMAGE_NAME).hdd
	mdconfig -a -t vnode -f $(IMAGE_NAME).hdd -u 0
	newfs_msdos -F 32 /dev/md0s1
	mkdir -p img_mount
	mount_msdosfs /dev/md0s1 img_mount
	mkdir -p img_mount/EFI/BOOT
	cp -v kernel/hadron.elf limine.cfg limine/limine-bios.sys img_mount/
	cp -v limine/BOOTX64.EFI img_mount/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI img_mount/EFI/BOOT/
	sync
	umount img_mount
	mdconfig -d -u 0
	rm -rf img_mount

.PHONY: clean
clean:
	rm -rf iso_root $(IMAGE_NAME).iso $(IMAGE_NAME).hdd
	${MAKE} -C kernel clean

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf
	${MAKE} -C kernel distclean
