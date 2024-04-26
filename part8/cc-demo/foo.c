// foo.c
// Created by daheige on 2024/4/25.

#include <stdio.h>

void print_app_info() {
    // 如果没有定义WELCOME宏
#ifdef WELCOME
    printf("welcome to ");
#endif

    // 输出build.rs中定义的APP_NAME和VERSION
    printf("%s - version:%s\n", APP_NAME, VERSION);
}

void foo() {
    printf("hello,world\n");
    printf("cc demo\n");
}

void greet(const char *name) {
    printf("hello,%s!\n", name);
}
