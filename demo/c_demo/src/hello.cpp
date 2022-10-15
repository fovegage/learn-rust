//
// Created by 高哲 on 10/14/22.
//

// src/hello.c

//#include <stdio.h>
//
//void hello() {
//    printf("Hello, World!\n");
//}

extern "C" {
int multiply(int x, int y);
}

int multiply(int x, int y) {
    return x*y;
}
