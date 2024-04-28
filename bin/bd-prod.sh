#!/usr/bin/env bash
# bd = build & deploy

cargo build --release && rm -rf ../rust-reinze/plugins/libreinze_lib_fun_*.so && cp target/release/libreinze_lib_fun.so ../rust-reinze/plugins/libreinze_lib_fun_$(date "+%Y%m%dT%H%M%S").so
