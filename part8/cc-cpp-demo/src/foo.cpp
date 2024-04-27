// foo.cpp
// Created by daheige on 2024/4/27.
// 通过在我们的 C++源文件的顶部，添加extern "C"部分，防止 C++编译器的名称篡改
extern "C" {
// 函数签名
int multiply(int, int);
}

// 函数定义
int multiply(int x, int y) {
    return x * y;
}