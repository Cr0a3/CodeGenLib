#include <stdio.h>

extern int add(int a, int b);

int main() {
    int a = 1;
    int b = 1;
    int c = add(a, b);

    printf("%d + %d = %d", a, b, c);
    return 0;
}