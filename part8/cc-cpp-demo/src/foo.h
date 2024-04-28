#include <iostream>

// 头文件定义
// 添加extern "C"部分，防止 C++编译器的名称篡改
extern "C" {
// 函数签名
int multiply(int, int);
void interop_sort(int numbers[], size_t size);
}
