# Yet Another Split Keyboard Firmware

## Dependencies

#### 0. Setup cross compilation

```console
rustup target add thumbv7em-none-eabihf
```

#### 1. `cargo objcopy`:

```console
rustup component add llvm-tools-preview
```

#### 2. `flip-link`:

```console
cargo install flip-link
```

#### 3. `probe-rs`:

```console
cargo install probe-rs --features cli
```

#### 4. `uf2conv`:

```console
cargo install cargo-binutils uf2conv
```

## Run

The `nice!nano` v2 needs to be in bootloader and connected

Then, run:

### For the right side

```console
cargo br
```
followed by
```console
uf2conv right.bin --base 0x27000 --family 0xADA52840 --output right.uf2
```

### For the left side
```console
cargo bl
```
followed by
```console
uf2conv left.bin --base 0x27000 --family 0xADA52840 --output left.uf2
```

then copy the firmware for the connected side onto the nice!nano drive, it will disconnect and reboot once written.
