#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)
proto_dir=$root_dir/proto
rust_grpc_dir=$root_dir/src/rust_grpc
mkdir -p $rust_grpc_dir

protoExec=$(which "protoc")
if [ -z $protoExec ]; then
    echo 'Please install protoc'
    echo "Please look readme.md to install proto3"
    echo "if you use centos7,please look readme.md"
    exit 0
fi

genRust=$(which "protoc-gen-rust")
if [ -z $genRust ]; then
    echo "please install "$genRust"use: cargo install protobuf-codegen && cargo install protobuf"
    exit 0
fi

grpcGenRust=$(which "protoc-gen-rust-grpc")
if [ -z $grpcGenRust ]; then
    echo "please install "$grpcGenRust"use: cargo install grpc-compiler"
    exit 0
fi

echo "\n\033[0;32mGenerating codes...\033[39;49;0m\n"

echo "generating rust stubs..."

cd $proto_dir
mod_arr=() # 需要写入mod.rs的模块名称列表
for file in $proto_dir/*.proto; do
  echo "generating rust stubs from: $file"

  # 获取文件的相对路径
  file_name=$(basename $file)
  file_name=${file_name%.*}
  echo "current proto file_name: "$file_name
  mod_arr+=($file_name)
  # protoc-gen-rust 插件生成rust代码
  #$protoExec --proto_path=$proto_dir --rust_out=$rust_grpc_dir $file

  # rust grpc代码生成
  #$protoExec --proto_path=$proto_dir --rust-grpc_out=$rust_grpc_dir $file
done

# 采用protoc_rust_grpc::Codegen 工具替代protoc工具

echo "\033[0;32m[DONE]\033[39;49;0m\n"

echo "\033[0;32mrebuild the mod.rs \033[39;49;0m\n"
echo "// @generated by protoc-gen-rust.Do not edit it!!!" > $rust_grpc_dir/mod.rs
for mod in ${mod_arr[@]};do
  echo "pub mod "$mod";" >> $rust_grpc_dir/mod.rs
  echo "pub mod "$mod"_grpc;" >> $rust_grpc_dir/mod.rs
done

echo "\033[0;32m[DONE]\033[39;49;0m\n"

echo "\033[0;32m[Notice]\033[39;49;0m"
echo "please add: mod rust_grpc; to $root_dir/src/main.rs"
echo "\033[0;32mCongratulations! You can begin your rust GRPC journey~\033[39;49;0m"
exit 0
