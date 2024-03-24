#include <stdio.h>

extern int ret_five();

void callme() {
    printf("Called");
}

int main() {
    ret_five(); // should call callme

    return 0;
}