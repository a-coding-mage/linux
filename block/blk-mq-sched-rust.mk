# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile: use its canonical target/configuration flags.
targets += bindings/blk_mq_sched_generated.rs
always-$(CONFIG_RUST_BLK_MQ_SCHED) += bindings/blk_mq_sched_generated.rs

$(obj)/bindings/blk_mq_sched_generated.rs: private bindgen_target_flags = \
    --allowlist-function 'lupos_sched_.*|blk_mq_(run_hw_queue|delay_run_hw_queues|delay_run_hw_queue|dispatch_rq_list|dequeue_from_ctx|flush_busy_ctxs|free_map_and_rqs|alloc_map_and_rqs|free_rqs|tag_update_sched_shared_tags)|blk_bio_list_merge|elv_attempt_insert_merge|elevator_alloc|kobject_put|kfree|xa_(load|find|find_after|erase)|sbitmap_any_bit_set|list_sort' \
    --allowlist-var 'LUPOS_SCHED_.*|BLK_MQ_S_SCHED_RESTART|BLK_MQ_F_TAG_HCTX_SHARED|BLK_MQ_NO_HCTX_IDX|BLKDEV_DEFAULT_RQ|MAX_SCHED_RQ|ELEVATOR_FLAG_DYING|HZ|ENOMEM|ENOENT|EAGAIN|MAX_ERRNO' \
    --blocklist-type '__kernel_size_t|__kernel_ssize_t|__kernel_ptrdiff_t' \
    --wrap-unsafe-ops
$(obj)/bindings/blk_mq_sched_generated.rs: $(srctree)/block/blk-mq-sched-rust.h \
    $(srctree)/block/blk-mq-sched-rust.mk FORCE
	$(call if_changed_dep,bindgen)
