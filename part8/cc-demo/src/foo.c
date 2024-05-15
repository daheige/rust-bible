// foo.c
// Created by daheige on 2024/4/25.

#include <stdio.h>
#include <string.h>

void print_app_info() {
    // 如果没有定义WELCOME宏
    if (strcmp(WELCOME, "YES") == 0) {
        printf("welcome to ");
    }

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
