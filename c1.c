#include <stdio.h>
#include <stdlib.h>

void one(){
    printf("part 1: \n");

    int x = 42;
    int* ptr = &x;
    printf("x = %i\n", x);
    printf("x = %i = *ptr\n", *ptr);
    *ptr = 100;
    printf("x = %i\n\n", x);
}

void two(){
    printf("part 2: \n");
    
    int arr[5] = {1,2,3,4,5};
    int* ptr = arr;
    printf("[");
    for(int i = 0; i < 5; ++i) {
        printf("%i%s", *(i + ptr), (i == 4 ? "" : ", "));
    }
    printf("]\n\n");
}

void three(){
    printf("part 3: \n");
    
    printf("enter length of array: ");
    int len = 0;
    if (scanf("%i", &len) != 1 || len <= 0) {
        printf("invalid length.\n");
        return;
    }

    printf("enter %i numbers with a space in between: ", len);
    int* arr = malloc(len * sizeof(int));
    if (arr == NULL) {
        printf("malloc error\n");
        return;
    }

    for(int i = 0; i < len; ++i) {
        if (scanf("%i", &arr[i]) != 1) {
            printf("invalid input.\n");
            free(arr);
            return;
        }
    }

    printf("numbers entered: ");
    for(int i = 0; i < len; ++i) {
        printf("%i ", arr[i]);
    }
    
    int total = 0;
    for(int i = 0; i < len; ++i) {
        total += arr[i];
    }
    printf("\naverage: %.2f\n", (float)total / len);
    free(arr);
}

int main(){
    one(); two(); three(); return 0;
}