$ cargo new hello_cargo
$ cd hello_cargo
#第一行命令新建了名为 hello_cargo 的目录。我们将项目命名为 hello_cargo，同时 Cargo 在一个同名目录中创建项目文件。
#
#进入 hello_cargo 目录并列出文件。将会看到 Cargo 生成了两个文件和一个目录：一个 Cargo.toml 文件，一个 src 目录，以及位于 src 目录中的 main.rs 文件。
#
#它也在 hello_cargo 目录初始化了一个 Git 仓库，并带有一个 .gitignore 文件。如果在现有的 Git 仓库中运行 cargo new，则不会生成 Git 文件；你可以使用 cargo new --vcs=git 来覆盖此行为。

$ cargo build

$ ./target/debug/hello_cargo

$ cargo run
# 也可以使用 cargo run 在一个命令中同时编译代码并运行生成的可执行文件

# Cargo 还提供了一个名为 cargo check 的命令。该命令快速检查代码确保其可以编译，但并不产生可执行文件：
$ cargo check
# 通常 cargo check 要比 cargo build 快得多，因为它省略了生成可执行文件的步骤。如果你在编写代码时持续的进行检查，cargo check 会加速开发！

$ cargo build --release
$ cargo run --release
$ cargo build -r
$ cargo run -r

Profiles

#Profiles provide a way to alter the compiler settings, influencing things like optimizations and debugging symbols.
#
#Cargo has 4 built-in profiles: dev, release, test, and bench.
#The profile is automatically chosen based on which command is being run if a profile is not specified on the command-line.
#In addition to the built-in profiles, custom user-defined profiles can also be specified.
#
#Profile settings can be changed in Cargo.toml with the [profile] table.
#Within each named profile, individual settings can be changed with key/value pairs like this:
#
#[profile.dev]
#opt-level = 1               # Use slightly better optimizations.
#overflow-checks = false     # Disable integer overflow checks.
#
#Cargo only looks at the profile settings in the Cargo.toml manifest at the root of the workspace.
#Profile settings defined in dependencies will be ignored.
#
#Additionally, profiles can be overridden from a config definition.
#Specifying a profile in a config file or environment variable will override the settings from Cargo.toml.

Default profiles

#dev
#
#The dev profile is used for normal development and debugging.
#It is the default for build commands like cargo build, and is used for cargo install --debug.
#
#The default settings for the dev profile are:
#
#[profile.dev]
#opt-level = 0
#debug = true
#split-debuginfo = '...'  # Platform-specific.
#debug-assertions = true
#overflow-checks = true
#lto = false
#panic = 'unwind'
#incremental = true
#codegen-units = 256
#rpath = false
#
#release
#
#The release profile is intended for optimized artifacts used for releases and in production.
#This profile is used when the --release flag is used, and is the default for cargo install.
#
#The default settings for the release profile are:
#
#[profile.release]
#opt-level = 3
#debug = false
#split-debuginfo = '...'  # Platform-specific.
#debug-assertions = false
#overflow-checks = false
#lto = false
#panic = 'unwind'
#incremental = false
#codegen-units = 16
#rpath = false

参考
#https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
#https://doc.rust-lang.org/cargo/reference/profiles.html?highlight=overflow-checks
