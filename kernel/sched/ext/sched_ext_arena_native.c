// SPDX-License-Identifier: GPL-2.0-only
/*
 * Arena-local native field, inline primitive and callback adapters.
 * Explicit unqualified runtime C boundary, not Rust coverage. No old
 * arena.c algorithm is included or called and no shared storage is defined.
 */
#error "SOURCE ONLY HOLD: sched_ext arena native ABI/protection admission is pending"

#include "sched_ext_arena_bindings.h"

bool lupos_scx_arena_has_map(const struct scx_sched *sch)
{
	return sch->arena_map != NULL;
}

struct gen_pool *lupos_scx_arena_pool(const struct scx_sched *sch)
{
	return sch->arena_pool;
}

void lupos_scx_arena_set_pool(struct scx_sched *sch, struct gen_pool *pool)
{
	sch->arena_pool = pool;
}

void *lupos_scx_arena_to_kaddr(struct scx_sched *sch, const void *bpf_ptr)
{
	/* Keep native normalization; payload bounds/fault handling are separate. */
	return scx_arena_to_kaddr(sch, bpf_ptr);
}

struct gen_pool *lupos_scx_arena_pool_create(int order)
{
	return gen_pool_create(order, NUMA_NO_NODE);
}

void lupos_scx_arena_pool_destroy(struct gen_pool *pool)
{
	gen_pool_destroy(pool);
}

int lupos_scx_arena_pool_add(struct gen_pool *pool, unsigned long addr,
			   size_t size)
{
	return gen_pool_add(pool, addr, size, NUMA_NO_NODE);
}

unsigned long lupos_scx_arena_pool_alloc(struct gen_pool *pool, size_t size)
{
	return gen_pool_alloc(pool, size);
}

void lupos_scx_arena_pool_free(struct gen_pool *pool, unsigned long addr,
			     size_t size)
{
	gen_pool_free(pool, addr, size);
}

/* Native C callback type; end-to-end CFI/KCFI admission remains pending. */
static void lupos_scx_arena_chunk_callback(struct gen_pool *pool,
					 struct gen_pool_chunk *chunk, void *data)
{
	lupos_scx_arena_clear_chunk(pool, chunk, data);
}

void lupos_scx_arena_for_each_chunk(struct gen_pool *pool)
{
	gen_pool_for_each_chunk(pool, lupos_scx_arena_chunk_callback, NULL);
}

int lupos_scx_arena_pool_order(const struct gen_pool *pool)
{
	return pool->min_alloc_order;
}

unsigned long lupos_scx_arena_chunk_start(const struct gen_pool_chunk *chunk)
{
	return chunk->start_addr;
}

unsigned long lupos_scx_arena_chunk_end(const struct gen_pool_chunk *chunk)
{
	return chunk->end_addr;
}

unsigned long lupos_scx_arena_chunk_next_set(const struct gen_pool_chunk *chunk,
					   unsigned long size,
					   unsigned long offset)
{
	/* bits is the inline flexible array after the native header fields. */
	return find_next_bit(chunk->bits, size, offset);
}

unsigned long lupos_scx_arena_chunk_next_zero(const struct gen_pool_chunk *chunk,
					    unsigned long size,
					    unsigned long offset)
{
	return find_next_zero_bit(chunk->bits, size, offset);
}

void *lupos_scx_arena_pages_alloc(struct scx_sched *sch, u32 page_cnt)
{
	return bpf_arena_alloc_pages_sleepable(sch->arena_map, NULL, page_cnt,
					      NUMA_NO_NODE, 0);
}

void lupos_scx_arena_pages_free(struct scx_sched *sch, void *bpf_ptr,
			       u32 page_cnt)
{
	/* Preserve native deferred rollback, including free-span failure behavior. */
	bpf_arena_free_pages_non_sleepable(sch->arena_map, bpf_ptr, page_cnt);
}

void lupos_scx_arena_might_sleep(void)
{
	might_sleep();
}
