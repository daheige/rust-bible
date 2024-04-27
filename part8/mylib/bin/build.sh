#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

# 构建rust库文件
echo "cargo build begin..."
cd $root_dir
cargo build

exit 0
