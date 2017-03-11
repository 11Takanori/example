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
