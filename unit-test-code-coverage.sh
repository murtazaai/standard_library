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

#LLVM_PROFILE_FILE="standard-library.profraw" target/debug/standard-library - \
#llvm-profdata merge -sparse standard-library.profraw -o standard-library.profdata \
#llvm-cov show -Xdemangler=rustfilt target/debug/standard-library -instr-profile=standard-library.profdata -show-line-counts-or-regions -show-instantiations -name=add_quoted_string \
#RUSTFLAGS="-C instrument-coverage"     cargo test --tests \
#llvm-profdata merge -sparse default_*.profraw -o standard-library.profdata \
#llvm-cov report --use-color --ignore-filename-regex='~/.cargo/registry' --instr-profile=standard-library.profdata --object target/debug/deps/standard-library-9ebd730377f4c477 --object target/debug/deps/standard-library-91d7b12bec96a30b --show-instantiation-summary --Xdemangler=rustfilt | less -R \

