/* SPDX-License-Identifier: GPL-2.0 */
#ifndef BLK_MQ_SCHED_RUST_H
#define BLK_MQ_SCHED_RUST_H

/* Canonical layouts and real C boundaries for the Rust scheduler owner. */
#include <linux/jiffies.h>
#include <linux/list_sort.h>
#include "blk.h"
#include "blk-mq-sched.h"
#include "blk-mq-debugfs.h"

/* Bind the Clang-evaluated kernel frequency instead of the UAPI HZ macro. */
enum { LUPOS_SCHED_HZ = HZ };
enum { LUPOS_SCHED_XA_PRESENT = XA_PRESENT };

bool lupos_sched_test_bit(unsigned int bit, const unsigned long *state);
void lupos_sched_set_bit(unsigned int bit, unsigned long *state);
void lupos_sched_clear_bit(unsigned int bit, unsigned long *state);
void lupos_sched_mb(void);
void lupos_sched_init_list(struct list_head *head);
bool lupos_sched_list_empty_careful(const struct list_head *head);
void lupos_sched_list_add(struct list_head *entry, struct list_head *head);
void lupos_sched_list_add_tail(struct list_head *entry, struct list_head *head);
void lupos_sched_list_cut_before(struct list_head *list, struct list_head *head,
		struct list_head *entry);
void lupos_sched_list_splice_init(struct list_head *list, struct list_head *head);
void lupos_sched_list_splice_tail_init(struct list_head *list, struct list_head *head);
void lupos_sched_spin_lock(spinlock_t *lock);
void lupos_sched_spin_unlock(spinlock_t *lock);
unsigned long lupos_sched_jiffies(void);
bool lupos_sched_need_resched(void);
int lupos_sched_get_budget(struct request_queue *q);
void lupos_sched_put_budget(struct request_queue *q, int token);
void lupos_sched_set_budget(struct request *rq, int token);
bool lupos_sched_get_driver_tag(struct request *rq);
bool lupos_sched_hctx_stopped(struct blk_mq_hw_ctx *hctx);
bool lupos_sched_queue_quiesced(struct request_queue *q);
struct blk_mq_ctx *lupos_sched_get_ctx(struct request_queue *q);
struct blk_mq_hw_ctx *lupos_sched_map_queue(blk_opf_t opf, struct blk_mq_ctx *ctx);
struct blk_mq_hw_ctx *lupos_sched_queue_hctx(struct request_queue *q, int index);
struct blk_mq_ctx *lupos_sched_read_dispatch_from(struct blk_mq_hw_ctx *hctx);
void lupos_sched_write_dispatch_from(struct blk_mq_hw_ctx *hctx, struct blk_mq_ctx *ctx);
bool lupos_sched_rq_mergeable(struct request *rq);
unsigned int lupos_sched_debugfs_lock(struct request_queue *q);
void lupos_sched_debugfs_unlock(struct request_queue *q, unsigned int flags);
void lupos_sched_debugfs_lock_nomemsave(struct request_queue *q);
void lupos_sched_debugfs_unlock_nomemrestore(struct request_queue *q);
void lupos_sched_debugfs_register(struct request_queue *q);
void lupos_sched_debugfs_register_hctx(struct request_queue *q, struct blk_mq_hw_ctx *hctx);
void lupos_sched_debugfs_unregister(struct request_queue *q);
void lupos_sched_debugfs_unregister_hctx(struct blk_mq_hw_ctx *hctx);
void lupos_sched_assert_update_locked(struct blk_mq_tag_set *set);
void lupos_sched_warn_missing_free(void);
void lupos_sched_warn_missing_alloc(void);
struct elv_change_ctx *lupos_sched_alloc_ctx(void);
struct elevator_tags *lupos_sched_alloc_tags_storage(unsigned int nr_tags);
int lupos_sched_xa_insert(struct xarray *xa, unsigned long index, void *entry);

#endif
