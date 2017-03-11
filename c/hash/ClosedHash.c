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

Bucket *Search(const ClosedHash *h, const Member *x)
{
    int i;
    int key = hash(x->no, h->size);
    Bucket *p = &h->table[key];

    for (i = 0; p->stat != Empty && i < h->size; i++) {
        if (p->stat == Occupied && p->data.no == x->no)
            return p;
        key = rehash(key, h->size);
        p = &h->table[key];
    }
    return NULL;
}

int Add(ClosedHash *h, const Member *x)
{
    int i;
    int key = hash(x->no, h->size);
    Bucket *p = &h->table[key];

    if (Search(h, x))
        return 1;
    for (i = 0; i < h->size; i++) {
        if (p->stat == Empty || p->stat == Deleted) {
            SetBucket(p, x, Occupied);
            return 0;
        }
        key = rehash(key, h->size);
        p = &h->table[key];
    }
    return 2;
}
