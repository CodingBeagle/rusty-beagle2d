# Application Binary Interface (ABI)

The ABI is an interface between two binary program modules, typically an operating system and an application being run by a user.

Adhering to an ABI is usually the job of a compiler / linker. Caring about the ABI might come into play if you call from one programming language to another using foreign function calls.

Good link: https://github.com/rust-lang/rustup#how-rustup-works

Rust can use two ABIs on Windows:
- The native (MSVC) ABI used by Visual Studio
- The GNU ABI used by the GCC toolchain

If you are interested in interopting with software produced by Visual Studio (such as libraries compiled with MSVC), then you need to use the MSVC toolchain. Since the MSVC ABI provides the best interoperation with other Windows software, it is recommended for most purposes. It makes sense to use the native ABI when developing for that environment.

For the MSVC toolchain, you need to have the Visual C++ Build Tools 2019 installed, because rustc uses it's linker!

# LLDB

LLDB (https://lldb.llvm.org/) is a next generation, high-performance debugger.

LLDB support for Windows is apparently not quite there yet (?), but in active development.

