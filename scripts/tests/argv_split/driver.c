/* SPDX-License-Identifier: GPL-2.0 */
/* Host allocator transport; both algorithms keep the original kernel ctype. */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <linux/slab.h>
#include <linux/string.h>

extern char **c_argv_split(gfp_t flags, const char *source, int *argc);
extern void c_argv_free(char **argv);
static char **(*volatile split_call)(gfp_t, const char *, int *) = argv_split;
static void (*volatile free_call)(char **) = argv_free;

#define CHECK(condition) do { if (!(condition)) { \
	fprintf(stderr, "argv_split failure at line %d: %s\n", __LINE__, #condition); \
	exit(1); } } while (0)

struct allocation {
	void *pointer;
	size_t size;
	gfp_t flags;
	int freed;
};
struct state {
	struct allocation allocations[2];
	unsigned int attempts, frees, freed[2], null_frees, helper_calls;
	size_t accounted;
	unsigned int failure, mutation;
	char *source;
	size_t source_len;
};
static struct state states[2];
static unsigned int active;

void *host_alloc(size_t size, gfp_t flags)
{
	struct state *s = &states[active];
	unsigned int index = s->attempts++;
	struct allocation *a;

	CHECK(index < 2);
	a = &s->allocations[index];
	a->size = size;
	a->flags = flags;
	if (s->mutation == index + 1 && s->source)
		memset(s->source, index ? ' ' : 'M', s->source_len);
	if (s->failure == index + 1 || size > KMALLOC_MAX_SIZE)
		return NULL;
	CHECK(size != 0);
	a->pointer = malloc(size);
	CHECK(a->pointer != NULL);
	CHECK((uintptr_t)a->pointer % _Alignof(void *) == 0);
	memset(a->pointer, flags & __GFP_ZERO ? 0 : 0xa5, size);
	if (flags & __GFP_ACCOUNT)
		s->accounted += size;
	return a->pointer;
}

void kfree(const void *pointer)
{
	struct state *s = &states[active];
	if (!pointer) {
		s->null_frees++;
		return;
	}
	for (unsigned int i = 0; i < 2; ++i) {
		struct allocation *a = &s->allocations[i];
		if (a->pointer != pointer)
			continue;
		CHECK(!a->freed && s->frees < 2);
		a->freed = 1;
		s->freed[s->frees++] = i;
		if (a->flags & __GFP_ACCOUNT)
			s->accounted -= a->size;
		free(a->pointer);
		return;
	}
	CHECK(!"free of an argument pointer instead of its hidden allocation owner");
}

void *host_realloc(const void *old, size_t size, unsigned long align, gfp_t flags, int node)
{
	void *result;
	CHECK(old == NULL && node == NUMA_NO_NODE && align == _Alignof(void *));
	states[active].helper_calls++;
	result = host_alloc(size, flags);
	/* The actual NULL-old kernel krealloc path has this diagnostic no-op. */
	if (result)
		kfree(NULL);
	return result;
}

static void exercise(const char *input, size_t length, gfp_t flags,
		     unsigned int failure, unsigned int mutation, int null_argc)
{
	char **vectors[2];
	int counts[2] = { 7391, 7391 };
	unsigned int argc = 0;

	memset(states, 0, sizeof(states));
	for (active = 0; active < 2; ++active) {
		struct state *s = &states[active];
		s->failure = failure;
		s->mutation = mutation;
		if (input) {
			s->source = malloc(length + 1);
			CHECK(s->source != NULL);
			memcpy(s->source, input, length);
			s->source[length] = 0;
			s->source_len = strlen(s->source);
		}
		vectors[active] = active ? split_call(flags, s->source, null_argc ? NULL : &counts[active]) :
			c_argv_split(flags, s->source, null_argc ? NULL : &counts[active]);
		if (input && !mutation)
			CHECK(!memcmp(s->source, input, length));
		if (!vectors[active] || null_argc)
			CHECK(counts[active] == 7391);
	}
	CHECK(!vectors[0] == !vectors[1]);
	CHECK(counts[0] == counts[1]);
	CHECK(states[0].attempts == states[1].attempts);
	for (unsigned int i = 0; i < states[0].attempts; ++i) {
		CHECK(states[0].allocations[i].size == states[1].allocations[i].size);
		CHECK(states[0].allocations[i].flags == flags);
		CHECK(states[1].allocations[i].flags == flags);
	}
	if (vectors[0]) {
		CHECK(states[0].attempts == 2);
		for (active = 0; active < 2; ++active) {
			CHECK(vectors[active] == (char **)states[active].allocations[1].pointer + 1);
			CHECK(vectors[active][-1] == states[active].allocations[0].pointer);
		}
		CHECK(!memcmp(states[0].allocations[0].pointer, states[1].allocations[0].pointer,
			      states[0].allocations[0].size));
		while (vectors[0][argc]) {
			CHECK(argc + 2 < states[0].allocations[1].size / sizeof(char *));
			CHECK(vectors[1][argc] != NULL);
			CHECK(vectors[0][argc] - (char *)states[0].allocations[0].pointer ==
			      vectors[1][argc] - (char *)states[1].allocations[0].pointer);
			CHECK(!strcmp(vectors[0][argc], vectors[1][argc]));
			argc++;
		}
		CHECK(vectors[1][argc] == NULL);
		CHECK(states[0].allocations[1].size == (argc + 2) * sizeof(char *));
		if (!null_argc)
			CHECK(counts[0] == (int)argc);
		for (active = 0; active < 2; ++active) {
			/* Public pointers have no ownership: argv_free must ignore them. */
			for (unsigned int i = 0; i <= argc; ++i)
				vectors[active][i] = (char *)(uintptr_t)(0x100 + i);
			if (active)
				free_call(vectors[active]);
			else
				c_argv_free(vectors[active]);
			CHECK(states[active].frees == 2);
			CHECK(states[active].freed[0] == 0 && states[active].freed[1] == 1);
		}
	}
	CHECK(states[0].frees == states[1].frees);
	CHECK(!memcmp(states[0].freed, states[1].freed, sizeof(states[0].freed)));
	CHECK(states[0].null_frees == 0);
	CHECK(states[1].null_frees == !!vectors[1]);
	CHECK(states[0].helper_calls == 0);
	CHECK(states[1].helper_calls == (states[1].attempts == 2));
	for (active = 0; active < 2; ++active) {
		CHECK(states[active].accounted == 0);
		for (unsigned int i = 0; i < 2; ++i)
			CHECK(!states[active].allocations[i].pointer || states[active].allocations[i].freed);
		free(states[active].source);
	}
}

int main(void)
{
	static const char *cases[] = { "", " \t\r\n\v\f", "word", " a b  c ",
		"'two words' \\\"still split\\\"", "a\240b", "a\0ignored tail" };
	static const gfp_t flags[] = { 0, __GFP_ZERO, __GFP_ACCOUNT,
		__GFP_ZERO | __GFP_ACCOUNT, (gfp_t)0xa535c947U };
	char bytes[512];
	uint32_t random = 0x748291;
	for (unsigned int f = 0; f < sizeof(flags) / sizeof(*flags); ++f) {
		for (unsigned int failure = 0; failure <= 2; ++failure) {
			for (unsigned int mutation = 0; mutation <= 2; ++mutation) {
				for (unsigned int null_argc = 0; null_argc < 2; ++null_argc) {
					exercise(NULL, 0, flags[f], failure, mutation, null_argc);
					for (unsigned int i = 0; i < sizeof(cases) / sizeof(*cases); ++i)
						exercise(cases[i], strlen(cases[i]), flags[f], failure, mutation, null_argc);
				}
			}
		}
	}
	for (unsigned int byte = 0; byte < 256; ++byte) {
		bytes[0] = byte; bytes[1] = 'a'; bytes[2] = byte; bytes[3] = 'b'; bytes[4] = 0;
		exercise(bytes, 4, flags[byte % 5], byte % 3, 0, byte & 1);
		exercise(bytes, 4, flags[byte % 5], 0, 0, byte & 1);
	}
	for (unsigned int i = 0; i < 512; ++i) {
		for (unsigned int j = 0; j < sizeof(bytes); ++j) {
			random = random * 1664525U + 1013904223U;
			bytes[j] = random >> 24;
		}
		exercise(bytes, sizeof(bytes), flags[i % 5], i % 3, (i / 3) % 3, i & 1);
	}
	/* The real configured slab maximum bounds the snapshot, not strlen(). */
	char *large = malloc(KMALLOC_MAX_SIZE + 64);
	CHECK(large != NULL);
	memset(large, 'a', KMALLOC_MAX_SIZE + 64);
	exercise(large, KMALLOC_MAX_SIZE + 64, __GFP_ZERO | __GFP_ACCOUNT, 0, 0, 0);
	for (size_t i = 0; i < KMALLOC_MAX_SIZE + 64; ++i)
		large[i] = i & 1 ? ' ' : 'a';
	/* Its pointer vector exceeds KMALLOC_MAX_SIZE: release the snapshot. */
	exercise(large, KMALLOC_MAX_SIZE + 64, __GFP_ACCOUNT, 0, 0, 0);
	free(large);
	puts("argv_split: snapshot, flags, hidden owner and failures passed");
	return 0;
}
