# Obliteration

Obliteration is an experimental PS4 emulator based on [Kyty](https://github.com/InoriRus/Kyty). The project is still in an early stage so most of the games may not work.

## Building from source

#### Windows requirements

Only Windows 10 x64 is supported at the moment.

Toolchains:
- Visual Studio + clang-cl + ninja
- Eclipse CDT + mingw-w64 + gcc/clang + ninja/mingw32-make

Supported versions:
Tool                            | Version
:------------                   | :------------
cmake                           |3.12
Visual Studio 2019              |16.10.3
clang                           |12.0.1
clang-cl                        |11.0.0
gcc (MinGW-W64 x86_64-posix-seh)|10.2.0
ninja                           |1.10.1
MinGW-w64                       |8.0.0
Eclipse CDT                     |10.3.0
Qt                              |5.15.0

Define environment variable named Qt5_DIR pointing to the proper version of Qt

MSVC compiler (cl.exe) is not supported!

External dependencies:
* Qt 5.15.0

#### Linux requirements

Any x86-64 (some distro use AMD64 wording) should be working.

#### Install Vulkan SDK

Only version 1.2.198.1 has been tested but any later version should be fine.

For Windows just download from https://vulkan.lunarg.com/sdk/home. Once installed you need to restart your computer to make `VULKAN_SDK` environment variable effective.

For Linux it will be depend on your distro. For Arch Linux just install `vulkan-devel` and set `VULKAN_SDK` to `/usr`. For other distro try to find in its package repository first. If not found visit https://vulkan.lunarg.com/sdk/home to download and install it manually.

#### Clone the repository

You need to clone this repository with submodules like this:

```sh
git clone --recurse-submodules https://github.com/ultimaweapon/obliteration.git
```

#### Initialize VCPKG

Windows (CMD):

```sh
.\vcpkg\bootstrap-vcpkg.bat
```

Linux:

```sh
./vcpkg/bootstrap-vcpkg.sh
```

#### Install external dependencies

Windows (PowerShell):

```pwsh
.\vcpkg-restore.ps1
```

Linux:

```sh
./vcpkg-restore.sh
```

If the above command produced an error about Vulkan SDK that mean you have improper Vulkan SDK installed.

#### Build

Run CMake to configure the build system:

```sh
cmake -B build -S source -D CMAKE_BUILD_TYPE=Release -D KYTY_FINAL=1
```

Then start building:

```sh
cmake --build build
```

## Development

### Get a homebrew application for testing

If you don't have a PS4 application for testing you can download PS Scene Quiz for free [here](https://pkg-zone.com/details/LAPY10010). Once downloaded you need to extract the downloaded `pkg` to get the real content. You can use [LibOrbisPkg](https://github.com/OpenOrbis/LibOrbisPkg) to extract it.

## License

MIT
