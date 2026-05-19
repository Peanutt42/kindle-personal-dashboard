# Kindle Personal Dashboard

## Supported Devices

Currently only tested on Kindle Paperwhite 2 (6. Generation).
Compiling for different models only requires a slightly adjusted `./cmake/kindle*_toolchain.cmake`.

If you need instructions on how to jailbreak your Kindle, go to <https://kindlemodding.org/jailbreaking/index.html>.

## Building from source: Requirements

### Using nix

You only need to setup the Kindle Sdk, everything else is provided by the nixShell:

```bash
nix develop
```

### Bare-bones requirements

- Kindle SDK along with koxtoolchain

  see <https://kindlemodding.org/kindle-dev/gtk-tutorial/prerequisites.html> for documentation (i suggest building the toolchain and sdk in a debain vm / container for less headaches)
  
  (should be installed in `~/x-tools/`, can be overriden by adding `-DKINDLE_XTOOLS_DIR=/my/custom/path/to/x-tools/` when configuring with cmake)

- C++ compiler capable of C++-17
- rust (with cargo installed), with edition 2024 support
- rust target `arm-unknown-linux-gnueabi` if you want to crosscompile to the Kindle Paperwhite 2
  
  (run `rustup target add arm-unknown-linux-gnueabi` if not yet installed)

- clang-tidy if you want to lint

## Building from source: Compiling

These are the possible cmake profiles:

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

The output will be in `./build/` or `./build_release` for `local*` presets and `./build_YOURPRESET/` otherwise.

## Installing on a Kindle

After compiling, copy the contents of `./build_<preset>/bundle/*` into `extensions/kindle-personal-dashboard/` (`/mnt/us/extensions/kindle-personal-dashboard/` over ssh).

It should look like this:

```
$ tree /mnt/us/extensions/kindle-personal-dashboard/
/mnt/us/extensions/kindle-personal-dashboard/
├── bin
│   └── kindle_personal_dashboard
├── config.xml
└── menu.json
```

Then you can launch using the KUAL menu `Kindle Personal Dashboard`.

### Example usage for Kindle Paperwhite 2:

```bash
cmake --preset=kindlepw2
cmake --build --preset=kindlepw2 -j
scp -r ./build_kindlepw2/bundle/* root@YOUR_KINDLES_IP:/mnt/us/extensions/kindle-personal-dashboard/
```

Thats it!

## Development

- quality of life improvements when using justfile:

	```bash
	$ just --list
	Available recipes:
    	build preset="local"                     # compile with preset
    	configure preset="local" *cmake_args=""  # configure cmake with preset
    	lint preset="local"                      # lints using run-clang-tidy
    	run preset="local"                       # runs locally on host machine
    	upload kindle_ip kindle_type="kindlepw2" # compiles and uploads binary to kindle over ssh
	```

	So for quick testing locally: `just run`

	and for quick testing on actual Kindle: `just upload 192.168.0.xyz`

- clang-format (manually, normally automatically by your IDE)

	```bash
	clang-format -i ./include/**.hpp ./src/**.cpp
	```

- clang-tidy

    enable by setting the `KPD_ENABLE_CLANG_TIDY` definition to `ON` when configuring cmake:

    ```bash
    cmake --preset=YOURPRESET -DKPD_ENABLE_CLANG_TIDY=ON
    ```

    or using just:

    ```bash
    just configure YOURPRESET -DKPD_ENABLE_CLANG_TIDY=ON
    ```

- address sanitizer

	```bash
	just configure local -DKPD_ENABLE_ASAN=On
	ASAN_OPTIONS=detect_leaks=1 LSAN_OPTIONS=suppressions=./tools/asan/gtk.supp just run
	```
