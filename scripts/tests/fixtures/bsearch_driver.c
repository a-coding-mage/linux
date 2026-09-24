/* SPDX-License-Identifier: GPL-2.0-only */
/* Private independent original-C differential fixture; never production. */
#include <linux/bsearch.h>
extern void *original_bsearch(const void *, const void *, size_t, size_t, cmp_func_t);
extern void *rust_consumer(const void *, const void *, size_t, size_t, cmp_func_t);
extern void *wrong_context(const void *, const void *, size_t, size_t, cmp_func_t);
extern void *wrong_signature(const void *, const void *, size_t, size_t, cmp_func_t);
static const void *trace[128];
static unsigned int count;
static int direction;
struct item { unsigned char tag; long value; unsigned short tail; };
/* Padding is deliberately not initialized or inspected. Key type differs. */
static int compare(const void *key, const void *value)
{
    const struct item *v = value;
    long k = *(const int *)key;
    trace[count++] = value;
    return (k > v->value) - (k < v->value);
}
static int fixed(const void *key, const void *value)
{
    (void)key;
    trace[count++] = value;
    return direction;
}
static unsigned int bad_compare(const void *key, const void *value)
{
    return (unsigned int)compare(key, value);
}
static int check(const void *key, const void *base, size_t n, size_t size, cmp_func_t cmp)
{
    const void *expected[128];
    count = 0;
    void *want = original_bsearch(key, base, n, size, cmp);
    unsigned int total = count;
    for (unsigned int i = 0; i < total; ++i) expected[i] = trace[i];
    /* Volatile outer loads retain actual protected indirect calls. */
    void *(*volatile choices[])(const void *, const void *, size_t, size_t, cmp_func_t) = {
        bsearch, rust_consumer
    };
    for (unsigned int choice = 0; choice < 2; ++choice) {
        count = 0;
        if (choices[choice](key, base, n, size, cmp) != want || count != total) return 10;
        for (unsigned int i = 0; i < total; ++i)
            if (trace[i] != expected[i]) return 11;
    }
    return 0;
}
int main(int argc, char **argv)
{
    if (argc != 2) return 95;
    struct item values[257];
    for (int i = 0; i < 257; ++i) { values[i].tag = 1; values[i].value = i / 3 - 20; values[i].tail = 7; }
    int key = 0;
    if (argv[1][0] == 'b') {
        cmp_func_t volatile cmp = (cmp_func_t)bad_compare;
        bsearch(&key, values, 257, sizeof(*values), cmp);
        return 0;
    }
    if (argv[1][0] == 'c') { wrong_context(&key, values, 257, sizeof(*values), compare); return 0; }
    if (argv[1][0] == 's') { wrong_signature(&key, values, 257, sizeof(*values), compare); return 0; }
    if (check(NULL, NULL, 0, (size_t)-1, NULL)) return 12;
    for (size_t n = 0; n <= 257; ++n) {
        for (key = -22; key < 68; ++key)
            if (check(&key, values, n, sizeof(*values), compare)) return 13;
    }
    /* Same-type aliases, including a key inside the searched storage, are C-defined. */
    for (direction = -1; direction <= 1; ++direction) {
        if (check(values, values, 257, 0, fixed)) return 14;
        if (check(values, values, (size_t)-1, 0, fixed)) return 15;
    }
    /* Unsigned multiplication wraps to zero; all visited pointers stay in object. */
    direction = 0;
    if (check(values, values, ((size_t)1 << (sizeof(size_t)*8-1)), 4, fixed)) return 16;
    return 0;
}
