default: run_ci

set export

original_rustflags := env_var_or_default('RUSTFLAGS', '--cfg hotshot_example')
original_rustdocflags := env_var_or_default('RUSTDOCFLAGS', '--cfg hotshot_example')
original_target_dir := env_var_or_default('CARGO_TARGET_DIR', 'target')

run_ci: lint build test

async := "async-std"

# Run arbitrary cargo commands, with e.g.
#     just async=async-std cargo check
# or
#     just async=tokio cargo test --tests test_consensus_task
# Defaults to async-std.

@cargo *ARGS:
  echo setting async executor to {{async}}
  export RUSTDOCFLAGS='-D warnings --cfg async_executor_impl="{{async}}" --cfg async_channel_impl="{{async}}" {{original_rustdocflags}}' RUSTFLAGS='--cfg async_executor_impl="{{async}}" --cfg async_channel_impl="{{async}}" {{original_rustflags}}' CARGO_TARGET_DIR='{{original_target_dir}}/{{async}}' && cargo {{ARGS}}

@tokio target *ARGS:
  echo setting executor to tokio
  export RUSTDOCFLAGS='-D warnings --cfg async_executor_impl="tokio" --cfg async_channel_impl="tokio" {{original_rustdocflags}}' RUSTFLAGS='--cfg async_executor_impl="tokio" --cfg async_channel_impl="tokio" {{original_rustflags}}' CARGO_TARGET_DIR='{{original_target_dir}}/tokio' && just {{target}} {{ARGS}}

@async_std target *ARGS:
  echo setting executor to async-std
  export RUST_MIN_STACK=4194304 RUSTDOCFLAGS='-D warnings --cfg async_executor_impl="async-std" --cfg async_channel_impl="async-std" {{original_rustdocflags}}' RUSTFLAGS='--cfg async_executor_impl="async-std" --cfg async_channel_impl="async-std" {{original_rustflags}}' CARGO_TARGET_DIR='{{original_target_dir}}/async-std' && just {{target}} {{ARGS}}

@async-std target *ARGS:
  echo setting executor to async-std
  export RUST_MIN_STACK=4194304 RUSTDOCFLAGS='-D warnings --cfg async_executor_impl="async-std" --cfg async_channel_impl="async-std" {{original_rustdocflags}}' RUSTFLAGS='--cfg async_executor_impl="async-std" --cfg async_channel_impl="async-std" {{original_rustflags}}' CARGO_TARGET_DIR='{{original_target_dir}}/async-std' && just {{target}} {{ARGS}}

build:
  cargo build --workspace --examples --bins --tests --lib --benches

test *ARGS:
  echo Testing {{ARGS}}
  cargo test --verbose --lib --bins --tests --benches --workspace --no-fail-fast {{ARGS}} -- --test-threads=1 --nocapture --skip crypto_test

check:
  echo Checking
  cargo check --workspace --bins --tests --examples

lint: 
  echo linting
  cargo fmt --check
  cargo clippy --workspace --examples --bins --tests -- -D warnings

fmt:
  echo Running cargo fmt
  cargo fmt
