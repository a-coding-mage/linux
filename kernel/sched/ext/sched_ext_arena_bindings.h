/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_EXT_ARENA_BINDINGS_H
#define LUPOS_SCHED_EXT_ARENA_BINDINGS_H

/* Configured native headers are the sole layout and constant authority. */
#include <linux/bpf.h>
#include <linux/errno.h>
#include <linux/find.h>
#include <linux/genalloc.h>
#include <linux/mm.h>
#include <linux/numa.h>
#include <linux/rhashtable.h>
#include <linux/seq_buf.h>
#include "internal.h"
#include "arena.h"

#define LUPOS_SCX_ARENA_ENOMEM ENOMEM
#define LUPOS_SCX_ARENA_EINVAL EINVAL
#define LUPOS_SCX_ARENA_PAGE_SIZE PAGE_SIZE
#define LUPOS_SCX_ARENA_PAGE_SHIFT PAGE_SHIFT

/*
 * sch storage belongs to the ext owner. These leaves borrow it; only
 * set_pool writes, and only to the existing arena_pool slot. The caller
 * pins sch/map/pool, excludes slot mutation except init/destroy, and must
 * never hold a Rust reference claiming exclusivity over native storage.
 */
bool lupos_scx_arena_has_map(const struct scx_sched *sch);
struct gen_pool *lupos_scx_arena_pool(const struct scx_sched *sch);
void lupos_scx_arena_set_pool(struct scx_sched *sch, struct gen_pool *pool);
/*
 * Use the original low-32-bit rebasing inline without accessing payload.
 * A held map reference does not establish exclusive/resident payload pages.
 * Payload access must retain native fault recovery and internal.h's bounds
 * requirements; recovery also needs an applicable BPF program on the stack.
 * The conversion alone validates neither an object extent nor that context.
 */
void *lupos_scx_arena_to_kaddr(struct scx_sched *sch, const void *bpf_ptr);

/*
 * Native genalloc owns allocation, synchronization and flexible storage.
 * A non-null pool is live throughout every call. Add/alloc/free follow the
 * configured genalloc context requirements; create/add may sleep. Destroy
 * requires exclusive teardown after all users drain and all bits are clear;
 * it does not wait for an RCU grace period. for_each_chunk synchronously calls
 * Rust with rcu_read_lock held; the callback must not sleep, retain pointers
 * or access arena payload.
 */
struct gen_pool *lupos_scx_arena_pool_create(int order);
void lupos_scx_arena_pool_destroy(struct gen_pool *pool);
int lupos_scx_arena_pool_add(struct gen_pool *pool, unsigned long addr,
			   size_t size);
unsigned long lupos_scx_arena_pool_alloc(struct gen_pool *pool, size_t size);
void lupos_scx_arena_pool_free(struct gen_pool *pool, unsigned long addr,
			     size_t size);
void lupos_scx_arena_for_each_chunk(struct gen_pool *pool);
int lupos_scx_arena_pool_order(const struct gen_pool *pool);
unsigned long lupos_scx_arena_chunk_start(const struct gen_pool_chunk *chunk);
unsigned long lupos_scx_arena_chunk_end(const struct gen_pool_chunk *chunk);
/*
 * Search only a pinned chunk of the retiring pool, with external bitmap
 * writers excluded. size is exactly the registered chunk bytes >> pool
 * order; offset may be past size, as permitted by native find_next_*.
 */
unsigned long lupos_scx_arena_chunk_next_set(const struct gen_pool_chunk *chunk,
					   unsigned long size,
					   unsigned long offset);
unsigned long lupos_scx_arena_chunk_next_zero(const struct gen_pool_chunk *chunk,
					    unsigned long size,
					    unsigned long offset);

/*
 * Sleepable allocation and non-sleepable rollback retain native BPF cfg.
 * Rollback may defer; failed native free-span allocation retains pages until
 * arena destruction. This void interface does not guarantee immediate release.
 */
void *lupos_scx_arena_pages_alloc(struct scx_sched *sch, u32 page_cnt);
void lupos_scx_arena_pages_free(struct scx_sched *sch, void *bpf_ptr,
			       u32 page_cnt);
void lupos_scx_arena_might_sleep(void);

/* Rust callback continuation, invoked synchronously by a native C adapter. */
void lupos_scx_arena_clear_chunk(struct gen_pool *pool,
				struct gen_pool_chunk *chunk, void *data);

#endif /* LUPOS_SCHED_EXT_ARENA_BINDINGS_H */
