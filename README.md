# Hadron

Hadron is a minimal microkernel with L4 architecture.

This is still at the beginning of the development.
I am using this project mainly to learn more about os development and L4 microkernels.

## Features
At this moment the kernel doesn't have any features. I'm still in the phase of implementing the minimum features for a working x86_64 kernel.
Later I plan to implement a efficient IPC mechanism, a first ext4 implementation and port the kernel to RISC-V.

## Dependencies
If you use the Nix Package Manager, there is a flake.nix in the root directory. 
Else you need to have the following packages installed:
```
rustc
qemu
parted
xorriso
```
and a C-compiler of your choice.

## Build

The build of the kernel works via the included Makefile:
```
make
```
and you can run the kernel for example via UEFI:
```
make run-uefi
```
