#include <stdio.h>

void callme() {
    printf("called");
}

extern int call();

void main() {
    printf("calling call ...\n");
    printf("got %d", call());
}