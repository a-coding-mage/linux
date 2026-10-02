/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef _BPF_LPM_TRIE_RUST_H
#define _BPF_LPM_TRIE_RUST_H

#include <linux/bpf.h>
#include <linux/btf.h>
#include <linux/err.h>
#include <linux/slab.h>
#include <linux/spinlock.h>
#include <linux/vmalloc.h>
#include <linux/btf_ids.h>
#include <asm/rqspinlock.h>
#include <linux/bpf_mem_alloc.h>

/* These private declarations are copied verbatim from lpm_trie.c. Keep the
 * retained C provider unchanged; bindgen derives every Rust field/layout from
 * these declarations and this build's real kernel headers and configuration.
 */
struct lpm_trie_node;

struct lpm_trie_node {
	struct lpm_trie_node __rcu	*child[2];
	u32				prefixlen;
	u32				flags;
	u8				data[];
};

struct lpm_trie {
	struct bpf_map			map;
	struct lpm_trie_node __rcu	*root;
	struct bpf_mem_alloc		ma;
	size_t				n_entries;
	size_t				max_prefixlen;
	size_t				data_size;
	rqspinlock_t			lock;
};

/* Evaluate configuration-dependent macros in C, never guess slab limits. */
enum {
	LUPOS_LPM_VAL_SIZE_MAX = KMALLOC_MAX_SIZE - 256 - sizeof(struct lpm_trie_node),
	LUPOS_LPM_CREATE_FLAG_MASK = BPF_F_NO_PREALLOC | BPF_F_NUMA_NODE | BPF_F_ACCESS_MASK,
	LUPOS_LPM_NUMA_NO_NODE = NUMA_NO_NODE,
	LUPOS_LPM_TRIE_SIZE = sizeof(struct lpm_trie),
	LUPOS_LPM_TRIE_ALIGN = __alignof__(struct lpm_trie),
	LUPOS_LPM_MAP_OFFSET = offsetof(struct lpm_trie, map),
	LUPOS_LPM_ROOT_OFFSET = offsetof(struct lpm_trie, root),
	LUPOS_LPM_MA_OFFSET = offsetof(struct lpm_trie, ma),
	LUPOS_LPM_ENTRIES_OFFSET = offsetof(struct lpm_trie, n_entries),
	LUPOS_LPM_MAX_PREFIX_OFFSET = offsetof(struct lpm_trie, max_prefixlen),
	LUPOS_LPM_DATA_SIZE_OFFSET = offsetof(struct lpm_trie, data_size),
	LUPOS_LPM_LOCK_OFFSET = offsetof(struct lpm_trie, lock),
	LUPOS_LPM_NODE_SIZE = sizeof(struct lpm_trie_node),
	LUPOS_LPM_NODE_ALIGN = __alignof__(struct lpm_trie_node),
	LUPOS_LPM_CHILD_OFFSET = offsetof(struct lpm_trie_node, child),
	LUPOS_LPM_PREFIX_OFFSET = offsetof(struct lpm_trie_node, prefixlen),
	LUPOS_LPM_FLAGS_OFFSET = offsetof(struct lpm_trie_node, flags),
	LUPOS_LPM_NODE_DATA_OFFSET = offsetof(struct lpm_trie_node, data),
	LUPOS_LPM_KEY_SIZE = sizeof(struct bpf_lpm_trie_key_u8),
	LUPOS_LPM_KEY_ALIGN = __alignof__(struct bpf_lpm_trie_key_u8),
	LUPOS_LPM_KEY_DATA_OFFSET = offsetof(struct bpf_lpm_trie_key_u8, data),
};

void lupos_lpm_lock_init(rqspinlock_t *lock);
int lupos_lpm_lock_irqsave(rqspinlock_t *lock, unsigned long *flags);
void lupos_lpm_unlock_irqrestore(rqspinlock_t *lock, unsigned long flags);
struct lpm_trie_node *lupos_lpm_dereference_bpf(struct lpm_trie_node __rcu *const *slot);
struct lpm_trie_node *lupos_lpm_dereference(struct lpm_trie_node __rcu *const *slot);
struct lpm_trie_node *lupos_lpm_dereference_protected(struct lpm_trie_node __rcu *const *slot);
struct lpm_trie_node *lupos_lpm_access_pointer(struct lpm_trie_node __rcu *const *slot);
void lupos_lpm_assign_pointer(struct lpm_trie_node __rcu **slot, struct lpm_trie_node *node);
void lupos_lpm_init_pointer(struct lpm_trie_node __rcu **slot, struct lpm_trie_node *node);
size_t lupos_lpm_read_entries(const struct lpm_trie *trie);
bool lupos_lpm_flags_access_ok(u32 flags);
struct lpm_trie_node **lupos_lpm_alloc_stack(size_t count);

/* Rust owns all provider callbacks. The C table is registration metadata. */
struct bpf_map *lupos_trie_alloc(union bpf_attr *attr);
void lupos_trie_free(struct bpf_map *map);
int lupos_trie_get_next_key(struct bpf_map *map, void *key, void *next_key);
void *lupos_trie_lookup_elem(struct bpf_map *map, void *key);
long lupos_trie_update_elem(struct bpf_map *map, void *key, void *value, u64 flags);
long lupos_trie_delete_elem(struct bpf_map *map, void *key);
int lupos_trie_check_btf(struct bpf_map *map, const struct btf *btf,
		       const struct btf_type *key_type, const struct btf_type *value_type);
u64 lupos_trie_mem_usage(const struct bpf_map *map);

#endif
