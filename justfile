msrv := ```
    cargo metadata --format-version=1 \
    | jq -r 'first(.packages[] | select(.source == null and .rust_version)) | .rust_version' \
    | sed -E 's/^1\.([0-9]{2})$/1\.\1\.0/'
```
msrv_rustup := "+" + msrv

_list:
    @just --list

# Clippy check workspace.
clippy:
    cargo clippy --workspace --no-default-features
    cargo clippy --workspace --all-features
    cargo hack --feature-powerset --depth=3 clippy --workspace

# Run workspace test suite.
test toolchain="":
    cargo {{ toolchain }} nextest run --workspace --all-features
    cargo {{ toolchain }} test --doc --workspace --all-features
    RUSTDOCFLAGS="-D warnings" cargo {{ toolchain }} doc --workspace --all-features --no-deps

# Downgrade dev-dependencies necessary to run MSRV checks/tests.
[private]
downgrade-msrv:
    @echo "No MSRV downgrades needed"

# Run workspace test suite using MSRV.
test-msrv: downgrade-msrv (test msrv_rustup)

# Run workspace test suite, capturing coverage.
test-coverage toolchain="": (test-coverage-codecov toolchain) (test-coverage-lcov toolchain)

# Run workspace test suite, capturing coverage info in Codecov format.
test-coverage-codecov toolchain="":
    cargo {{ toolchain }} llvm-cov --workspace --all-features --codecov --output-path codecov.json

# Run workspace test suite, capturing coverage info in Lcov format.
test-coverage-lcov toolchain="":
    cargo {{ toolchain }} llvm-cov --workspace --all-features --lcov --output-path lcov.info

# Build workspace documentation.
doc *args:
    RUSTDOCFLAGS="--cfg=docsrs -D warnings" cargo +nightly doc --no-deps --workspace --all-features {{ args }}

# Build workspace documentation, open it, and watch for changes.
doc-watch: (doc "--open")
    cargo watch -- @just doc

# Check project.
check: && clippy
    just --unstable --fmt --check
    fd --hidden --extension=md --extension=yml --exec-batch prettier --check
    fd --hidden --extension=toml --exec-batch taplo format --check
    fd --hidden --extension=toml --exec-batch taplo lint
    cargo +nightly fmt -- --check

# Format project.
fmt:
    just --unstable --fmt
    cargo rdme --force
    fd --hidden --extension=toml --exec-batch taplo format
    cargo +nightly fmt
    fd --hidden --extension=md --extension=yml --exec-batch prettier --write
