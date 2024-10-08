# CatOS
A small kernel AMD64 kernel written in Rust.

> [!NOTE] 
> This project is based off of https://os.phil-opp.com/, and as such, some aspects of the projects will be similar to it. 

# Accquiring CatOS
<!--- maybe add a section about releases.. i couldn't get it to compile :( -mxtlrr --->
## From source
You'll need two things:
- Qemu (on Arch Linux as `qemu-system-full`)
- Rust

both of which are available in package managers. Along with this, you'll need to install
`bootimage` if you haven't already:
```sh
$ cd ~    # Super important. See rust-osdev/bootimage issue 64
$ sudo cargo install bootimage
# Also install llvm-tools-preview with 'rustup component add llvm-tools-preview', if not done
# already.
```

### Compilation
```sh
# Cloning the repository
$ git clone https://github.com/Cyteon/catos
$ cd catos
# Here is where you'll install llvm-tools-preview. See above
$ cargo build
```
The output file is in `target/x86_64-cat_os/debug`.

## The Tests
CatOS comes with its own test suite. You can use
```
$ cargo test
```

to run the tests. You *will* need to have qemu for this.

# Running CatOS

## Real Hardware
The kernel is a floppy disk image, so you can follow [this](https://wiki.osdev.org/Bootable_CD). To make
the 1.44 MB floppy, you can
```sh
# Generate a 1.44 mb empty disk image
$ dd if=/dev/zero of=flp.img bs=512 count=2880
# Copy the kernel image
$ cp target/x86_64-cat_os/debug/bootimage-cat_os.bin kernel.elf
$ dd if=flp.img of=kernel.elf conv=notrunc
```

## QEMU (with Cargo)
```bash
$ cargo run
```
