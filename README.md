# Ramen-OS

An OS all about Ramen

### Makefile targets

Running `make all` will compile the kernel (from the `kernel/` directory) and then generate a bootable ISO image.

Running `make all-hdd` will compile the kernel and then generate a raw image suitable to be flashed onto a USB stick or hard drive/SSD.

Running `make run` will build the kernel and a bootable ISO (equivalent to make all) and then run it using `qemu` (if installed).

Running `make run-hdd` will build the kernel and a raw HDD image (equivalent to make all-hdd) and then run it using `qemu` (if installed).

The `run-uefi` and `run-hdd-uefi` targets are equivalent to their non `-uefi` counterparts except that they boot `qemu` using a UEFI-compatible firmware.


Best intro to embedded Rust I've seen.  `embassy` works for a huge variety of chips:

https://www.youtube.com/watch?v=pDd5mXBF4tY&t=3s

https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust

## To-Do:
- [ ] Enable Scrolling for makros printing to screen
- [x] Parse Limine's memory map
- [ ] Implement physical frame allocator