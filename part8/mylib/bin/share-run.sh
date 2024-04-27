#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)
sh $root_dir/bin/build.sh

mkdir -p $root_dir/src/cbin
cd $root_dir/src
echo "gen cdylib libmylib begin..."
# 通过动态链接的方式，生成main二进制文件
gcc -o $root_dir/src/cbin/main main.c -I./src -L../target/debug -lmylib
echo "gen cdylib libmylib success"
echo "run $root_dir/src/cbin/main begin..."
$root_dir/src/cbin/main
