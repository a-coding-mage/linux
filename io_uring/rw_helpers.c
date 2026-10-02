// SPDX-License-Identifier: GPL-2.0
/* Header-only macros/inlines used by the Rust rw.c translation. */
#include "rw_bindings.h"
bool rust_rw_file_can_poll(struct io_kiocb *r) { return io_file_can_poll(r); }
__poll_t rust_rw_vfs_poll(struct file *f, struct poll_table_struct *p) { return vfs_poll(f, p); }
bool rust_rw_is_compat(struct io_ring_ctx *c) { return io_is_compat(c); }
bool rust_rw_vectored(unsigned int op) { return io_issue_defs[op].vectored; }
bool rust_rw_do_buffer_select(struct io_kiocb *r) { return io_do_buffer_select(r); }
void rust_rw_vec_reset(struct iou_vec *v, struct iovec *i, unsigned n) { io_vec_reset_iovec(v, i, n); }
void rust_rw_vec_kasan(struct iou_vec *v) { io_alloc_cache_vec_kasan(v); }
bool rust_rw_cache_put(struct io_alloc_cache *c, void *e) { return io_alloc_cache_put(c, e); }
void rust_rw_async_clear(struct io_kiocb *r) { io_req_async_data_clear(r, 0); }
void *rust_rw_async_alloc(struct io_alloc_cache *c, struct io_kiocb *r) { return io_uring_alloc_async_data(c, r); }
void rust_rw_iter_save(struct iov_iter *i, struct iov_iter_state *s) { iov_iter_save_state(i, s); }
u16 rust_rw_current_ioprio(void) { return get_current_ioprio(); }
int rust_rw_ioprio_check_cap(int ioprio) { return ioprio_check_cap(ioprio); }
unsigned long rust_rw_copy_from_user(void *d, const void __user *s, unsigned long n) { return copy_from_user(d, s, n); }
void rust_rw_assert_locked(struct io_ring_ctx *c) { lockdep_assert_held(&c->uring_lock); }
bool rust_rw_worker(void) { return io_wq_current_is_worker(); }
bool rust_rw_dying(struct io_ring_ctx *c) { return percpu_ref_is_dying(&c->refs); }
void rust_rw_end_write(struct kiocb *k) { kiocb_end_write(k); }
void rust_rw_start_write(struct kiocb *k) { kiocb_start_write(k); }
bool rust_rw_sb_start_write_trylock(struct super_block *s) { return sb_start_write_trylock(s); }
void rust_rw_sb_writers_release(struct super_block *s) { __sb_writers_release(s, SB_FREEZE_WRITE); }
void rust_rw_notify_modify(struct file *f) { fsnotify_modify(f); }
void rust_rw_notify_access(struct file *f) { fsnotify_access(f); }
void rust_rw_set_fail(struct io_kiocb *r) { req_set_fail(r); }
void rust_rw_set_res(struct io_kiocb *r, s32 v, u32 f) { io_req_set_res(r, v, f); }
bool rust_rw_has_async(struct io_kiocb *r) { return req_has_async_data(r); }
unsigned int rust_rw_put_kbuf(struct io_kiocb *r, int n, struct io_buffer_list *b) { return io_put_kbuf(r, n, b); }
bool rust_rw_recycle_kbuf(struct io_kiocb *r, struct io_buffer_list *b, unsigned f) { return io_kbuf_recycle(r, b, f); }
void rust_rw_task_work_add(struct io_kiocb *r, unsigned f) { __io_req_task_work_add(r, f); }
void rust_rw_store_complete(struct io_kiocb *r) { smp_store_release(&r->iopoll_completed, 1); }
bool rust_rw_load_complete(struct io_kiocb *r) { return smp_load_acquire(&r->iopoll_completed); }
bool rust_rw_iter_ubuf(const struct iov_iter *i) { return iter_is_ubuf(i); }
bool rust_rw_iter_bvec(const struct iov_iter *i) { return iov_iter_is_bvec(i); }
void __user *rust_rw_iter_iov_addr(const struct iov_iter *i) { return iter_iov_addr(i); }
size_t rust_rw_iter_iov_len(const struct iov_iter *i) { return iter_iov_len(i); }
bool rust_rw_wake_page_match(struct wait_page_queue *w, struct wait_page_key *k) { return wake_page_match(w, k); }
void rust_rw_list_del_init(struct list_head *e) { list_del_init(e); }
void rust_rw_list_init(struct list_head *e) { INIT_LIST_HEAD(e); }
void rust_rw_list_del(struct list_head *e) { list_del(e); }
int rust_rw_set_flags(struct kiocb *k, rwf_t f, int t) { return kiocb_set_rw_flags(k, f, t); }
u64 rust_rw_ktime_get_ns(void) { return ktime_get_ns(); }
void rust_rw_trace_short_write(struct io_ring_ctx *c, loff_t p, size_t w, size_t a) { trace_io_uring_short_write(c, p, w, a); }
bool rust_rw_is_uring_cmd(struct io_kiocb *r) { return io_is_uring_cmd(r); }
void rust_rw_timer_setup(struct hrtimer_sleeper *t) { hrtimer_setup_sleeper_on_stack(t, CLOCK_MONOTONIC, HRTIMER_MODE_REL); }
void rust_rw_timer_expires(struct hrtimer_sleeper *t, u64 n) { hrtimer_set_expires(&t->timer, ktime_set(0, n)); }
void rust_rw_set_interruptible(void) { set_current_state(TASK_INTERRUPTIBLE); }
void rust_rw_timer_start(struct hrtimer_sleeper *t) { hrtimer_sleeper_start_expires(t, HRTIMER_MODE_REL); }
void rust_rw_set_running(void) { __set_current_state(TASK_RUNNING); }
void rust_rw_timer_destroy(struct hrtimer_sleeper *t) { destroy_hrtimer_on_stack(&t->timer); }
bool rust_rw_rq_list_empty(struct io_comp_batch *b) { return rq_list_empty(&b->req_list); }
void rust_rw_wq_add_tail(struct io_kiocb *r, struct io_ring_ctx *c) { wq_list_add_tail(&r->comp_list, &c->submit_state.compl_reqs); }

void rust_rw_poll_multishot_retry(struct io_kiocb *r) { io_poll_multishot_retry(r); }
