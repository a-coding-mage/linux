// SPDX-License-Identifier: GPL-2.0
/* Compare entire forests, not just connectivity, against the original C. */
#include <linux/union_find.h>

extern struct uf_node *c_uf_find(struct uf_node *);
extern void c_uf_union(struct uf_node *, struct uf_node *);
extern void rust_node_init(struct uf_node *);
extern int rust_static_init(void);

#define COUNT 257
static struct uf_node reference[COUNT], translated[COUNT];
static struct uf_node *(*volatile find_call)(struct uf_node *) = uf_find;
static void (*volatile union_call)(struct uf_node *, struct uf_node *) = uf_union;

static int compare(void)
{
	for (unsigned int i = 0; i < COUNT; ++i) {
		if (reference[i].parent - reference != translated[i].parent - translated)
			return 1;
		if (reference[i].rank != translated[i].rank)
			return 2;
	}
	return 0;
}

static void initialize(void)
{
	for (unsigned int i = 0; i < COUNT; ++i) {
		uf_node_init(&reference[i]);
		rust_node_init(&translated[i]);
	}
}

int main(void)
{
	static const unsigned int ranks[] = { 0, 1, 2, 0x7fffffffU, 0x80000000U, ~0U };
	unsigned int state = 0x739178c1;
	if (!rust_static_init())
		return 3;
	/* Include the unsigned-rank comparisons and defined unsigned overflow. */
	for (unsigned int a = 0; a < sizeof(ranks) / sizeof(*ranks); ++a) {
		for (unsigned int b = 0; b < sizeof(ranks) / sizeof(*ranks); ++b) {
			initialize();
			reference[0].rank = translated[0].rank = ranks[a];
			reference[1].rank = translated[1].rank = ranks[b];
			c_uf_union(&reference[0], &reference[1]);
			union_call(&translated[0], &translated[1]);
			if (compare())
				return 4;
		}
	}
	/* Long paths distinguish the original path splitting from full compression. */
	for (unsigned int start = 0; start < COUNT; ++start) {
		initialize();
		for (unsigned int i = 0; i + 1 < COUNT; ++i) {
			reference[i].parent = &reference[i + 1];
			translated[i].parent = &translated[i + 1];
		}
		if (c_uf_find(&reference[start]) - reference !=
		    find_call(&translated[start]) - translated || compare())
			return 5;
	}
	for (unsigned int round = 0; round < 64; ++round) {
		initialize();
		for (unsigned int operation = 0; operation < 1024; ++operation) {
			unsigned int a, b;
			state ^= state << 13; state ^= state >> 17; state ^= state << 5;
			a = state % COUNT;
			state ^= state << 13; state ^= state >> 17; state ^= state << 5;
			b = state % COUNT;
			if (state & 1) {
				c_uf_union(&reference[a], &reference[b]);
				union_call(&translated[a], &translated[b]);
			} else if (c_uf_find(&reference[a]) - reference !=
				   find_call(&translated[a]) - translated) {
				return 6;
			}
			if (compare())
				return 7;
		}
	}
	return 0;
}
