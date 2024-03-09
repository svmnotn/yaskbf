# Yet Another Split Keyboard Firmware

## Dependencies

#### 1. `cargo objcopy`:

```console
$ rustup component add llvm-tools-preview
```

#### 2. `uf2conv`:

``` console
$ cargo install cargo-binutils uf2conv
```

## Run

The `nice!nano` v2 needs to be in bootloader and connected

Then, run:

```console
$ cargo br
$ uf2conv right.bin --base 0x26000 --family 0xADA52840 --output right.uf2
$ cargo bl
$ uf2conv left.bin --base 0x26000 --family 0xADA52840 --output left.uf2
```

then copy the firmware for the connected side onto the nice!nano drive, it will disconnect and reboot once written.
