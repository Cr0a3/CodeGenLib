#include <stdio.h>

extern int call();

void callme() {
    printf("called\n");
}

int main() {
    if ( 5 == call() ) {
        printf("worked\n");
    }

    return 0;
}