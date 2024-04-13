#include <stdio.h>

extern int add(int a, int b);

int main() {
    printf("1 + 1 = %d", add(1, 1));
    return 0;
}