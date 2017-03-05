#include <stdio.h>

int max3(int a, int b, int c)
{
    int max = a;

    if (b > max) max = b;
    if (c > max) max = c;

    return max;
}

int max4(int a, int b, int c, int d)
{
    int max = a;

    if (b > max) max = b;
    if (c > max) max = c;
    if (d > max) max = d;

    return max;
}

int min3(int a, int b, int c)
{
    int min = a;

    if (b < min) min = b;
    if (c < min) min = c;

    return min;
}

int min4(int a, int b, int c, int d)
{
    int min = a;

    if (b < min) min = b;
    if (c < min) min = c;
    if (d < min) min = d;

    return min;
}

int main(void)
{
    printf("max3(%d,%d,%d) = %d\n", 2, 1, 3, max3(2, 1, 3));
    printf("max4(%d,%d,%d,%d) = %d\n", 2, 4, 1, 3, max4(2, 4, 1, 3));
    printf("min3(%d,%d,%d) = %d\n", 2, 1, 3, min3(2, 1, 3));
    printf("min4(%d,%d,%d,%d) = %d\n", 2, 4, 1, 3, min4(2, 4, 1, 3));

    return 0;
}