// foo.cpp 具体函数实现
// Created by daheige on 2024/4/27.
#include "foo.h"

using namespace std;

// 函数定义
int multiply(int x, int y) {
    return x * y;
}

// 实现函数interop_sort
void interop_sort(int numbers[], size_t size) {
    int *start = &numbers[0]; // 数组的第一个元素
    int *end = &numbers[0] + size; // 数组的最后一个元素
    // 从大到小进行排序
    std::sort(start, end, [](int x, int y) { return x > y; });
}