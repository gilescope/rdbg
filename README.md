# rdbg

[![Build Status](https://travis-ci.org/starfleetcadet75/rdbg.svg?branch=master)](https://travis-ci.org/starfleetcadet75/rdbg)
[![Lines of Code](https://tokei.rs/b1/github/starfleetcadet75/rdbg)](https://github.com/starfleetcadet75/rdbg)
[![license](https://img.shields.io/github/license/mashape/apistatus.svg?maxAge=2592000)]()

A debugger written in Rust. See the [wiki](https://github.com/starfleetcadet75/rdbg/wiki) for a complete list of supported features.

_**NOTE:** rdbg is in the very early stages of development. This may or may not go anywhere._

## Build

```
$ git clone https://github.com/starfleetcadet75/rdbg.git
$ cd rdbg
$ cargo build --release
```

## Run Locally

`cargo run --bin rdbg -- "tests/hello_world"`

## Run Remotely

*WIP*: This is what it will look like...
Start the server on the target: `cargo run --bin rdbg-server -- "tests/hello_world"`
Connect from the client: `cargo run --bin rdbg -- -r localhost:2159`

## Contributing

Contributions are always welcome. Look through the open issues on GitHub or grep for TODO in the code.

## License

See the [LICENSE file](LICENSE.md).

## References

[ptrace man page](http://man7.org/linux/man-pages/man2/ptrace.2.html)
[Writing a Linux Debugger](https://blog.tartanllama.xyz/writing-a-linux-debugger-setup.html)
[Programmatic access to the call stack in C++](https://eli.thegreenplace.net/2015/programmatic-access-to-the-call-stack-in-c)
[Playing with ptrace](https://www.linuxjournal.com/article/6100)
[Loading and ptrace'ing a process in Rust](http://system.joekain.com/2015/07/15/rust-load-and-ptrace.html)
[Threads and fork(): think twice before mixing them](http://www.linuxprogrammingblog.com/threads-and-fork-think-twice-before-using-them)
[Write yourself an strace in 70 lines of code](https://blog.nelhage.com/2010/08/write-yourself-an-strace-in-70-lines-of-code)
[Debugging with the PTrace Utility](http://www.secretmango.com/jimb/Whitepapers/ptrace/ptrace.html)
[nix-rust](https://github.com/nix-rust/nix)
[gtrace](https://github.com/geofft/gtrace)
