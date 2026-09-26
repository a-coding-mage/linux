#ifndef _LINUX_TYPES_H
#define _LINUX_TYPES_H
typedef __SIZE_TYPE__ size_t;
typedef __UINTPTR_TYPE__ uintptr_t;
typedef unsigned int u32;
typedef unsigned long long u64;
typedef _Bool bool;
#define false 0
#define true 1
#define NULL ((void *)0)
#define __always_inline inline __attribute__((always_inline))
#define __attribute_const__ __attribute__((const))
typedef int (*cmp_func_t)(const void *a, const void *b);
typedef int (*cmp_r_func_t)(const void *a, const void *b, const void *priv);
typedef void (*swap_func_t)(void *a, void *b, int size);
typedef void (*swap_r_func_t)(void *a, void *b, int size, const void *priv);
#endif
