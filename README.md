# Helloworld

This is a starter project :)

## Compiling with Sqlite support

First, please install the latest release of the [wasi-sdk](https://github.com/WebAssembly/wasi-sdk/releases/)

```bash
# on macos, use the following
wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-11/wasi-sdk-11.0-macos.tar.gz
```

then decompress it inside of a folder that you can access, for me thats on `$HOME/bin`

```bash
mv wasi-sdk-11.0-macos.tar.gz ./bin
tar xzf wasi-sdk-11.0-macos.tar.gz
```

Now that the folder is decompressed, it's time to compile the project. Run the following
command to build the project with sqlite support

```bash
CFLAGS_wasm32_wasi="--sysroot $HOME/bin/wasi-sdk-11.0/share/wasi-sysroot" CC_wasm32_wasi="$HOME/bin/wasi-sdk-11.0/bin/clang"  cargo build --target="wasm32-wasi"
```

It now works on wasi

## Example of compiling c to wasm

Inside of the wasi-sdk-11.0 folder, run the following command on your c file:

```bash
./bin/clang --target=wasm32-unknown-wasi --sysroot share/wasi-sysroot/ -O2 -s -o example.wasm example.c
```
