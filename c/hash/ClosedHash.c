#include <stdio.h>
#include <stdlib.h>
#include "Member.h"
#include "ClosedHash.h"

static int hash(int key, int size)
{
    return key % size;
}

static int rehash(int key, int size)
{
    return (key + 1) % size;
}

static void SetBucket(Bucket *n, const Member *x, Status stat)
{
    n->data = *x;
    n->stat = stat;
}

int Initialize(ClosedHash *h, int size)
{
    int i;
    if ((h->table = calloc(size, sizeof(Bucket))) == NULL) {
        h->size = 0;
        return 0;
    }
    h->size = size;
    for (i = 0; i < size; i++)
        h->table[i].stat = Empty;
    return 1;
}