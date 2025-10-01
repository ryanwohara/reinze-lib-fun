#!/usr/bin/env bash
# bd = build & deploy

[[ $(uname -s) == "Darwin" ]] && export extension=.dylib || export extension=.so

cargo update && \
  cargo build --release && \
  rm -rf ../rust-reinze/plugins/libreinze_lib_fun_* && \
  cp "target/release/libreinze_lib_fun${extension}" "../rust-reinze/plugins/libreinze_lib_fun_$(date "+%Y%m%dT%H%M%S")${extension}"
