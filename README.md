<p align="center"><img alt="Logo" src="logo.png"></p>

## About
[![CI](https://github.com/obhq/obliteration/actions/workflows/main.yml/badge.svg)](https://github.com/obhq/obliteration/actions/workflows/main.yml) [![Discord](https://img.shields.io/discord/1121284256816185444?color=740d03&label=Obliteration&logo=discord)](https://discord.gg/Qsdaxj6tnH)

Obliteration is an experimental PS4 emulator for Windows, Linux and macOS. **The project is under development and cannot run any games yet**. The reason it take so long is because we decided to go with the correct path without stubbing as much as possible.

This project started as a hard-fork from [Kyty](https://github.com/InoriRus/Kyty). Then we decided to rewrite the whole project from scratch by using Kyty and [Uplift](https://github.com/idc/uplift) as a reference to help us getting started with the project. How Obliteration work is totally different from Kyty because we decided to go with the same path as Uplift (and Spine?) instead of Kyty. This is the reason why Obliteration required firmware files from the PS4 in order to work while Kyty does not.

## Get a daily build

You can download binaries from the latest commits [here](https://github.com/obhq/obliteration/actions/workflows/main.yml). You **MUST** sign in to GitHub otherwise you will not be able to download files.

## Screenshots

![Screenshots](screenshots.png)

Thanks to [VocalFan](https://github.com/VocalFan) for the awesome icon!

## Obliteration discussion

We have a Discord server for discussion about Obliteration and its development. You can join the server [here](https://discord.gg/Qsdaxj6tnH).

## Features

- [x] Cross-platform with native binary for each platform.
- [x] Built-in FTP client to pull the decrypted firmware from jailbroken PS4.
- [x] Built-in PKG file supports for Fake PKG.
- [x] Game library.
- [x] Emulate system calls instead of user-space libraries.

## System requirements

- Windows 10, Linux or macOS 11+.
- x86-64 CPU. We want to support non-x86 but currently we don't have any developers who are willing to work on this.
- A jailbroken PS4 with FTP server that supports SELF decryption.

### Windows-specific requirements

- [Microsoft Visual C++ 2022 Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist). If there is an error related to `msvcp140.dll`, `vcruntime140.dll`, or `vcruntime140_1.dll` that means you need to install this manually. It's likely your system already has it, so try to run Obliteration first.

### Linux-specific requirements

Obliteration supports only 4KB/8KB/16KB pages. Most people should not have any problem with this because 4KB is the default for most distros.

## Building from source

### Windows prerequisites

- Visual Studio 2022
  - `Desktop development with C++` workload is required
- Rust on the latest stable channel
- CMake 3.21+
  - Make sure you have `Add CMake to the system PATH` selected when installing

### Linux prerequisites

- GCC 9.4+
- Rust on the latest stable channel
- CMake 3.21+

### macOS prerequisites

- macOS 12+
- Homebrew
- Clang 13+
- Rust on the latest stable channel
- CMake 3.21+

### Install Qt 6

You need to install Qt 6 on your system before you proceed. The minimum version is 6.5.

#### Windows-specific requirements

You need `Qt Online Installer` for open-source to install Qt, downloaded from https://www.qt.io. The installer will ask you to sign in with a Qt account, which you can create for free. You need to check `Custom installation` and do not check `Qt for desktop development` that is using the MinGW toolchain. Make sure you have checked the `MSVC 2019 64-bit` component in the `Select Components` page for the version you wish to install and uncheck all of the other components.

Once installation is completed you need to set the `CMAKE_PREFIX_PATH` environment variable to the full path of the installed version (e.g. `C:\Qt\6.5.1\msvc2019_64`). To set an environment variable:

1. Open a run dialog with <kbd>Win</kbd> + <kbd>R</kbd>.
2. Enter `sysdm.cpl` then click `OK`.
3. Go to the `Advanced` tab then click on `Environment Variables...`.
4. Click `New...` to create a new environment variable. Just create for either `User variables` or `System variables`, not both.
5. Restart your terminal or IDE to load the new PATH.

#### Install Qt with Homebrew (macOS only)

```sh
brew install qt@6
```

### Configure build system

```sh
cmake --preset PRESET .
```

The value of `PRESET` will depend on your platform and the build configuration you want to use. The current available presets are:

- windows-release
- windows-debug
- linux-release
- linux-debug
- mac-release
- mac-debug

If all you want is to use the emulator, choose `[YOUR-PLATFORM]-release` for optimized outputs. But if you want to edit the code, choose `*-debug`.

### Build

```sh
cmake --build --preset PRESET
```

You can use `-j` to enable parallel building (e.g. `cmake --build --preset PRESET -j 2`). Each parallel build on Linux consumes a lot of memory so don't use the number of your CPU cores otherwise your system might crash due to out of memory. On Windows and macOS it seems like it is safe to use the number of your CPU cores.

## Development

Before proceeding, make sure the build preset you are using is `*-debug`. We recommend Visual Studio Code as a code editor with the following extensions:

- [C/C++](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [CMake](https://marketplace.visualstudio.com/items?itemName=twxs.cmake)
- [CMake Tools](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cmake-tools)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

Then open this folder with VS Code. It will ask which CMake preset to use and you need to choose the same one that you were using when building. Everything should work out of the box (e.g. code completion, debugging, etc).

### macOS debugging issues

If you can't launch or debug Obliteration from VS Code, try [this](https://github.com/vadimcn/codelldb/discussions/456#discussioncomment-874122) solution.

### Get a homebrew application for testing

If you don't have a PS4 application for testing you can download PS Scene Quiz for free [here](https://pkg-zone.com/details/LAPY10010).

### Rules for Rust sources

- Use unsafe code only when you know what you are doing. When you do try to wrap it in a safe function so other people who are not familiar with unsafe code can have a safe life.
- Don't chain method calls without an intermediate variable if the result code is hard to follow. We encourage code readability as a pleasure when writing so try to make it easy to read and understand for other people.
- Do not blindly cast an integer. Make sure the value can fit in a destination type. We don't have any plans to support non-64-bit systems so the pointer size and its related types like `usize` are always 64-bits.
- Beware of deadlock and memory leak. Rust can protect us from most mistakes, except those two. Deadlock can be happen easily in Rust because Rust requires us to wrap the data we want to protect with a mutex and get a mutable reference through it. The problem with this is it become natural for you to lock the mutex to operate on the inner data, which can easily cause a deadlock if you are not aware when there are another locks being active. The only cases for memory leak you need to aware is when working with `Arc`. Just make sure you don't create a reference cycle that will never be dropped.

### Rules for C++ sources

Just follow how Qt is written (e.g. coding style, etc.). Always prefers Qt classes over `std` when possible so you don't need to handle exceptions. Do not use the Qt `ui` file to design the UI because it will break on a high-DPI screen.

### Starting point

The application consists of 2 binaries:

1. Main application. This is what users will see when they launch Obliteration. Its entry point is inside `src/main.cpp`.
2. Emulator kernel. This is where emulation takes place. Its entry point is inside `src/kernel/src/main.rs`.

### Debugging the kernel

Create `.kernel-debug` in the root of the repository. The contents of this file is YAML and the kernel will deserialize it to the `Args` struct in `src/kernel/src/main.rs` when passing the `--debug` flag to the kernel. See `Args` struct for available options.

We already provide a launch configuration for VS Code so all you need to do is choose `Debug - Kernel` as the configuration and start debugging.

### UI Icons

We use icons from https://materialdesignicons.com for UI (e.g. on the menu and toolbar).

### Additional informations

[PS4 Developer Wiki](https://www.psdevwiki.com/ps4) has a lot of useful information about the PS4 internal. We also have a PS4 reverse engineering [project](https://github.com/obhq/reverse-engineering).

## Code contribution

If you want to make some contributions but don't know what to work on you can look for `TODO` comment or `todo!` macro invocation in the source code. You can also take a look on the unassigned issues.

## License

- `src/ansi_escape.hpp`, `src/ansi_escape.cpp`, `src/log_formatter.hpp` and `src/log_formatter.cpp` are licensed under GPL-3.0 only.
- `src/param`, `src/pfs` and `src/pkg` are licensed under LGPL-3.0 license.
- All other source code is licensed under MIT license.
- All release binaries are under GPL-3.0 license.
