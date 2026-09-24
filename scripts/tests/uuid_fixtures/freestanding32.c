/* SPDX-License-Identifier: GPL-2.0-only */
/* Private i386 Linux syscall transport; no production C or kernel type shim. */
typedef __SIZE_TYPE__ size_t;
extern int main(void);
static long call3(long n, long a, long b, long c)
{
	long result;
	__asm__ volatile("int $0x80" : "=a"(result) : "a"(n), "b"(a), "c"(b), "d"(c) : "memory");
	return result;
}
__attribute__((noreturn)) void abort(void)
{
	call3(1, 97, 0, 0);
	__builtin_unreachable();
}
int puts(const char *s)
{
	size_t n = 0;
	while (s[n]) n++;
	call3(4, 1, (long)s, n);
	call3(4, 1, (long)"\n", 1);
	return 0;
}
void *mmap(void *p, size_t n, int prot, int flags, int fd, long offset)
{
	long args[] = {(long)p, n, prot, flags, fd, offset};
	long r = call3(90, (long)args, 0, 0);
	return r < 0 && r >= -4095 ? (void *)-1 : (void *)r;
}
int mprotect(void *p, size_t n, int prot) { return call3(125, (long)p, n, prot); }
int munmap(void *p, size_t n) { return call3(91, (long)p, n, 0); }
void *memcpy(void *dst, const void *src, size_t n)
{
	unsigned char *d = dst;
	const unsigned char *s = src;
	for (size_t i = 0; i < n; i++) d[i] = s[i];
	return dst;
}
void *memset(void *dst, int v, size_t n)
{
	unsigned char *d = dst;
	for (size_t i = 0; i < n; i++) d[i] = v;
	return dst;
}
int memcmp(const void *a, const void *b, size_t n)
{
	const unsigned char *x = a, *y = b;
	for (size_t i = 0; i < n; i++) if (x[i] != y[i]) return x[i] - y[i];
	return 0;
}
int bcmp(const void *a, const void *b, size_t n) { return memcmp(a,b,n); }
void *memmove(void *dst, const void *src, size_t n)
{
	unsigned char *d = dst;
	const unsigned char *s = src;
	if (d < s) return memcpy(dst,src,n);
	while (n) { n--; d[n] = s[n]; }
	return dst;
}
__attribute__((noreturn)) void suite(void)
{
	call3(1, main(), 0, 0);
	__builtin_unreachable();
}
__asm__(".text\n.globl _start\n_start:\n andl $-16,%esp\n call suite\n ud2\n");
