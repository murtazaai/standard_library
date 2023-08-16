# Clear screen
clear \
# Install rustfilt dependency
cargo install rustfilt
# Clean cargo cache
cargo clean \
# List rustup toolchains
rustup toolchain list \
# Set rust compiler path, rustflags and generate .profraw
RUSTC=$HOME/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc \
    RUSTFLAGS="-C instrument-coverage" \
    cargo build --lib
# List the generated .profraw
ls default_*.profraw
# These files will be used for generating .profdata files
# .profdata files will be used to view command line code coverage
# This procedure successfully works for binary only
# --lib i.e. library currently in the backlog!

# Backlog!

#LLVM_PROFILE_FILE="std.profraw" target/debug/std - \
#llvm-profdata merge -sparse std.profraw -o std.profdata \
#llvm-cov show -Xdemangler=rustfilt target/debug/std -instr-profile=std.profdata -show-line-counts-or-regions -show-instantiations -name=add_quoted_string \
#RUSTFLAGS="-C instrument-coverage"     cargo test --tests \
#llvm-profdata merge -sparse default_*.profraw -o std.profdata \
#llvm-cov report --use-color --ignore-filename-regex='~/.cargo/registry' --instr-profile=std.profdata --object target/debug/deps/std-9ebd730377f4c477 --object target/debug/deps/std-91d7b12bec96a30b --show-instantiation-summary --Xdemangler=rustfilt | less -R \

