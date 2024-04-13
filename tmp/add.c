#include <stdio.h>

int add(int a, int b) {
    return a + b;
}

int main() {
    printf("1 + 1 = %d", add(1, 1));
    return 0;
}