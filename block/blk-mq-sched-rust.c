// SPDX-License-Identifier: GPL-2.0
/*
 * Macro and static-inline boundaries only. Scheduling, traversal, resource
 * ownership and failure unwinds are implemented by blk-mq-sched.rs.
 */
#include "blk-mq-sched-rust.h"

bool lupos_sched_test_bit(unsigned int bit, const unsigned long *state)
{
	return test_bit(bit, state);
}
void lupos_sched_set_bit(unsigned int bit, unsigned long *state)
{
	set_bit(bit, state);
}
void lupos_sched_clear_bit(unsigned int bit, unsigned long *state)
{
	clear_bit(bit, state);
}
void lupos_sched_mb(void) { smp_mb(); }
void lupos_sched_init_list(struct list_head *head) { INIT_LIST_HEAD(head); }
bool lupos_sched_list_empty_careful(const struct list_head *head)
{
	return list_empty_careful(head);
}
void lupos_sched_list_add(struct list_head *entry, struct list_head *head)
{
	list_add(entry, head);
}
void lupos_sched_list_add_tail(struct list_head *entry, struct list_head *head)
{
	list_add_tail(entry, head);
}
void lupos_sched_list_cut_before(struct list_head *list, struct list_head *head,
		struct list_head *entry)
{
	list_cut_before(list, head, entry);
}
void lupos_sched_list_splice_init(struct list_head *list, struct list_head *head)
{
	list_splice_init(list, head);
}
void lupos_sched_list_splice_tail_init(struct list_head *list, struct list_head *head)
{
	list_splice_tail_init(list, head);
}
void lupos_sched_spin_lock(spinlock_t *lock) { spin_lock(lock); }
void lupos_sched_spin_unlock(spinlock_t *lock) { spin_unlock(lock); }
unsigned long lupos_sched_jiffies(void) { return jiffies; }
bool lupos_sched_need_resched(void) { return need_resched(); }
int lupos_sched_get_budget(struct request_queue *q)
{
	return blk_mq_get_dispatch_budget(q);
}
void lupos_sched_put_budget(struct request_queue *q, int token)
{
	blk_mq_put_dispatch_budget(q, token);
}
void lupos_sched_set_budget(struct request *rq, int token)
{
	blk_mq_set_rq_budget_token(rq, token);
}
bool lupos_sched_get_driver_tag(struct request *rq) { return blk_mq_get_driver_tag(rq); }
bool lupos_sched_hctx_stopped(struct blk_mq_hw_ctx *hctx) { return blk_mq_hctx_stopped(hctx); }
bool lupos_sched_queue_quiesced(struct request_queue *q) { return blk_queue_quiesced(q); }
struct blk_mq_ctx *lupos_sched_get_ctx(struct request_queue *q) { return blk_mq_get_ctx(q); }
struct blk_mq_hw_ctx *lupos_sched_map_queue(blk_opf_t opf, struct blk_mq_ctx *ctx)
{
	return blk_mq_map_queue(opf, ctx);
}
struct blk_mq_hw_ctx *lupos_sched_queue_hctx(struct request_queue *q, int index)
{
	return queue_hctx(q, index);
}
struct blk_mq_ctx *lupos_sched_read_dispatch_from(struct blk_mq_hw_ctx *hctx)
{
	return READ_ONCE(hctx->dispatch_from);
}
void lupos_sched_write_dispatch_from(struct blk_mq_hw_ctx *hctx, struct blk_mq_ctx *ctx)
{
	WRITE_ONCE(hctx->dispatch_from, ctx);
}
bool lupos_sched_rq_mergeable(struct request *rq) { return rq_mergeable(rq); }
unsigned int lupos_sched_debugfs_lock(struct request_queue *q) { return blk_debugfs_lock(q); }
void lupos_sched_debugfs_unlock(struct request_queue *q, unsigned int flags)
{
	blk_debugfs_unlock(q, flags);
}
void lupos_sched_debugfs_lock_nomemsave(struct request_queue *q) { blk_debugfs_lock_nomemsave(q); }
void lupos_sched_debugfs_unlock_nomemrestore(struct request_queue *q) { blk_debugfs_unlock_nomemrestore(q); }
void lupos_sched_debugfs_register(struct request_queue *q) { blk_mq_debugfs_register_sched(q); }
void lupos_sched_debugfs_register_hctx(struct request_queue *q, struct blk_mq_hw_ctx *hctx)
{
	blk_mq_debugfs_register_sched_hctx(q, hctx);
}
void lupos_sched_debugfs_unregister(struct request_queue *q) { blk_mq_debugfs_unregister_sched(q); }
void lupos_sched_debugfs_unregister_hctx(struct blk_mq_hw_ctx *hctx)
{
	blk_mq_debugfs_unregister_sched_hctx(hctx);
}
void lupos_sched_assert_update_locked(struct blk_mq_tag_set *set)
{
	lockdep_assert_held_write(&set->update_nr_hwq_lock);
}
/* Separate WARN_ON_ONCE storage, matching the two original call sites. */
void lupos_sched_warn_missing_free(void) { WARN_ON_ONCE(1); }
void lupos_sched_warn_missing_alloc(void) { WARN_ON_ONCE(1); }
struct elv_change_ctx *lupos_sched_alloc_ctx(void)
{
	return kzalloc_obj(struct elv_change_ctx);
}
struct elevator_tags *lupos_sched_alloc_tags_storage(unsigned int nr_tags)
{
	struct elevator_tags *et;
	gfp_t gfp = GFP_NOIO | __GFP_ZERO | __GFP_NOWARN | __GFP_NORETRY;

	return kmalloc_flex(*et, tags, nr_tags, gfp);
}
int lupos_sched_xa_insert(struct xarray *xa, unsigned long index, void *entry)
{
	return xa_insert(xa, index, entry, GFP_KERNEL);
}
