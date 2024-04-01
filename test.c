#include <stdio.h>

void callme() {
    printf("called!");
}

extern int call();

void main() {
    if ( 5 == call() ) {
        printf("worked");
    }
}