clear \
cargo clean \
RUSTC=$HOME/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc     RUSTFLAGS="-C instrument-coverage"     cargo build --bin stdlib \
echo "{some: 'thing'}" | target/debug/stdlib \
LLVM_PROFILE_FILE="stdlib.profraw" target/debug/stdlib - \
llvm-profdata merge -sparse stdlib.profraw -o stdlib.profdata \
llvm-cov show -Xdemangler=rustfilt target/debug/stdlib -instr-profile=stdlib.profdata -show-line-counts-or-regions -show-instantiations -name=add_quoted_string \
RUSTFLAGS="-C instrument-coverage"     cargo test --tests \
llvm-profdata merge -sparse default_*.profraw -o stdlib.profdata \
llvm-cov report --use-color --ignore-filename-regex='~/.cargo/registry' --instr-profile=stdlib.profdata --object target/debug/deps/stdlib-9ebd730377f4c477 --object target/debug/deps/stdlib-91d7b12bec96a30b --show-instantiation-summary --Xdemangler=rustfilt | less -R \

