# Spectra

A small Rust CLI that compresses images using a truncated Singular Value
Decomposition (SVD). Each RGB channel is treated as a matrix, decomposed, and
rebuilt from only its top `k` singular values. The alpha channel is preserved
unchanged.

## How it works

For a channel matrix `A`, the thin SVD gives:

```text
A = U * S * V^T
```

Keeping only the first `k` components produces a rank-`k` approximation:

```text
A_k = U_k * S_k * V_k^T
```

`U_k` is scaled by the retained singular values, then multiplied by `V_k^T` to
reconstruct the channel. Lower `k` means stronger compression and more detail
loss; higher `k` means better quality.

## Implemented steps

1. Added a Nix flake that provides the Rust toolchain and build inputs.
2. Added CLI parsing with `clap` for input, output, and rank.
3. Load and decode the input image, then convert it to RGBA.
4. Split the image into red, green, blue, and alpha channel matrices.
5. Convert each RGB channel into a `faer::Mat<f64>` matrix.
6. Compute a thin SVD for each RGB channel.
7. Reconstruct each channel using only the top `k` singular values.
8. Convert the reconstructed channels back to `u8` values with rounding and
   clamping, rebuild an `RgbaImage`, and save it to the output path.

## Project layout

```text
src/main.rs  # image loading, channel conversion, SVD compression, output
src/cli.rs   # command-line arguments
flake.nix    # x86_64-linux development shell and package definition
```

## Getting started

Enter the development shell:

```sh
nix develop
```

Build and run:

```sh
cargo build
cargo run -- --input photo.jpg --output compressed.png --rank 50
```

Short flags are also available:

```sh
cargo run -- -i photo.jpg -o compressed.png -k 32
```

`--rank` defaults to `50`; `--output` defaults to `compressed.png` in the
current directory.

## Limitations

- The alpha channel is copied unchanged.
- `k` is not validated against the image dimensions; it must not exceed the
  smaller matrix dimension.
- The saved file is still a normal image format. This project reduces matrix
  rank, not encoded file size, so the output file may not be smaller.

## License

MIT
