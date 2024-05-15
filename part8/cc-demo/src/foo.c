// foo.c
// Created by daheige on 2024/4/25.

#include <stdio.h>
#include <string.h>

void print_app_info() {
    // 如果没有定义WELCOME宏
#ifdef WELCOME
    // 调用strcmp函数比较WELCOME宏字符串是否为YES
    if (strcmp(WELCOME, "YES") == 0) {
        printf("welcome to ");
    }
#endif

    // 输出build.rs中定义的APP_NAME和VERSION
    printf("%s - current version:%s\n", APP_NAME, VERSION);
}

// 定义hello函数
void hello() {
    printf("hello,world\n");
}

// 定义greet函数
void greet(const char *name) {
    printf("hello,%s!\n", name);
}
