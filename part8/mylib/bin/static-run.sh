#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

sh $root_dir/bin/build.sh

cd $root_dir/src
echo "gcc static bin begin..."
# 通过静态链接 libmylib.a 文件的方式，生成二进制文件main-hello
# 由于libmylib.a 缺少了一些pthread, dl类函数，需要链接进来
gcc -o $root_dir/src/cbin/main-hello main.c -I./src ../target/debug/libmylib.a -lpthread -ldl
echo "gcc static bin success"

echo "run $root_dir/src/cbin/main-hello to exec main bin"
$root_dir/src/cbin/main-hello
