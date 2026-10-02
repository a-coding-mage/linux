// SPDX-License-Identifier: GPL-2.0-only
/* Macro/static-inline boundaries and registration; no trie implementation. */
#include "lpm_trie-rust.h"

void lupos_lpm_lock_init(rqspinlock_t *lock)
{
	raw_res_spin_lock_init(lock);
}

int lupos_lpm_lock_irqsave(rqspinlock_t *lock, unsigned long *flags)
{
	return raw_res_spin_lock_irqsave(lock, *flags);
}

void lupos_lpm_unlock_irqrestore(rqspinlock_t *lock, unsigned long flags)
{
	raw_res_spin_unlock_irqrestore(lock, flags);
}

struct lpm_trie_node *lupos_lpm_dereference_bpf(struct lpm_trie_node __rcu *const *slot)
{
	return rcu_dereference_check(*slot, bpf_rcu_lock_held());
}

struct lpm_trie_node *lupos_lpm_dereference(struct lpm_trie_node __rcu *const *slot)
{
	return rcu_dereference(*slot);
}

struct lpm_trie_node *lupos_lpm_dereference_protected(struct lpm_trie_node __rcu *const *slot)
{
	return rcu_dereference_protected(*slot, 1);
}

struct lpm_trie_node *lupos_lpm_access_pointer(struct lpm_trie_node __rcu *const *slot)
{
	return rcu_access_pointer(*slot);
}

void lupos_lpm_assign_pointer(struct lpm_trie_node __rcu **slot, struct lpm_trie_node *node)
{
	rcu_assign_pointer(*slot, node);
}

void lupos_lpm_init_pointer(struct lpm_trie_node __rcu **slot, struct lpm_trie_node *node)
{
	RCU_INIT_POINTER(*slot, node);
}

size_t lupos_lpm_read_entries(const struct lpm_trie *trie)
{
	return READ_ONCE(trie->n_entries);
}

bool lupos_lpm_flags_access_ok(u32 flags)
{
	return bpf_map_flags_access_ok(flags);
}

struct lpm_trie_node **lupos_lpm_alloc_stack(size_t count)
{
	return kmalloc_objs(struct lpm_trie_node *, count, GFP_ATOMIC | __GFP_NOWARN);
}

BTF_ID_LIST_SINGLE(trie_map_btf_ids, struct, lpm_trie)
const struct bpf_map_ops trie_map_ops = {
	.map_meta_equal = bpf_map_meta_equal,
	.map_alloc = lupos_trie_alloc,
	.map_free = lupos_trie_free,
	.map_get_next_key = lupos_trie_get_next_key,
	.map_lookup_elem = lupos_trie_lookup_elem,
	.map_update_elem = lupos_trie_update_elem,
	.map_delete_elem = lupos_trie_delete_elem,
	.map_lookup_batch = generic_map_lookup_batch,
	.map_update_batch = generic_map_update_batch,
	.map_delete_batch = generic_map_delete_batch,
	.map_check_btf = lupos_trie_check_btf,
	.map_mem_usage = lupos_trie_mem_usage,
	.map_btf_id = &trie_map_btf_ids[0],
};
