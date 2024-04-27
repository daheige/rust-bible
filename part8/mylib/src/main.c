// main.c
// Created by daheige on 2024/4/27.

#include "mylib.h"

int main() {
    // 调用rust mylib库中的say_hello函数
    // 在编译的时候可以选择是用动态链接或静态链接的方式来生成c语言的二进制文件
    // 具体生成机制看bin/share-run.sh和static-run.sh即可
    say_hello();
}