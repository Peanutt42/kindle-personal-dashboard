# Kindle Personal Dashboard

## Requirements

- Jailbroken Kindle
  
  see <https://kindlemodding.org/jailbreaking/index.html>
- Kindle SDK along with koxtoolchain

  see <https://kindlemodding.org/kindle-dev/gtk-tutorial/prerequisites.html> for documentation (i suggest building the toolchain and sdk in a debain vm / container for less headaches)
  
  (should be installed in `~/x-tools/`, can be overriden by adding `-DKINDLE_XTOOLS_DIR=/my/custom/path/to/x-tools/` when configuring with cmake)
- C++ compiler capable of C++-17
- rust (with cargo installed), with edition 2024 support
- rust target `arm-unknown-linux-gnueabi`
  
  (run `rustup target add arm-unknown-linux-gnueabi` if not yet installed)

## Supported Devices

Currently only tested on Kindle Paperwhite 2 (6. Generation).
Compiling for different models only requires a slightly adjusted `./cmake/kindle*_toolchain.cmake`.

## Building from source

if using nix / NixOS, you only need to setup the Kindle Sdk, everything else is in the nixShell:
```bash
nix develop
```

there are currently two cmake profiles:

- `local` (for local host development)
- `local_release` (optimized variant)
- `kindlepw2` (for Kindle Paperwhite 2)
- `kindlepw2_debug` (unoptimized debug variant)

1. Configure

```bash
cmake --preset=YOURPRESET
```

2. Compiling

```bash
cmake --build --preset=YOURPRESET -j
```

## Installing on a Kindle

After compiling, the executable should be in `./build_<preset>/kindle_personal_dashboard`.
Copy it to the kindles `/mnt/us/extensions` rootfs (or simply into the `extensions` directory when plugging the Kindle in over USB).
At last, launch kterm and execute `kindle_personal_dashboard`.

## Example usage for Kindle Paperwhite 2:

```bash
cmake --preset=kindlepw2
cmake --build --preset=kindlepw2 -j
scp ./build_kindlepw2/kindle_personal_dashboard root@YOUR_KINDLES_IP:/mnt/us/extensions/
```

Tada!

## Development

There are some quality of life improvements when using justfile:

```bash
$ just --list
Available recipes:
    build preset="local"                     # compile with preset
    configure preset="local"                 # configure cmake with preset
    run                                      # runs locally on host machine
    upload kindle_ip kindle_type="kindlepw2" # compiles and uploads binary to kindle over ssh
```

So for quick testing locally: `just run`

and for quick testing on actual Kindle: `just upload 192.168.0.xyz`

To run clang-format manually:

```bash
clang-format -i ./include/**.hpp ./src/**.cpp
```

To run clang-tidy manually:

```bash
clang-tidy -p ./build/ ./src/**.cpp
```
