/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_IO_URING_RW_BINDINGS_H
#define LUPOS_IO_URING_RW_BINDINGS_H
#include <linux/kernel.h>
#include <linux/errno.h>
#include <linux/fs.h>
#include <linux/file.h>
#include <linux/blk-mq.h>
#include <linux/mm.h>
#include <linux/slab.h>
#include <linux/fsnotify.h>
#include <linux/poll.h>
#include <linux/nospec.h>
#include <linux/io_uring/cmd.h>
#include <linux/indirect_call_wrapper.h>
#include <uapi/linux/io_uring.h>
#include "filetable.h"
#include "io_uring.h"
#include "opdef.h"
#include "kbuf.h"
#include "alloc_cache.h"
#include "rsrc.h"
#include "poll.h"
#include "rw.h"
/* Private request overlay, identical to rw.c; generated layout is authoritative. */
struct io_rw {
 struct kiocb kiocb;
 u64 addr;
 u32 len;
 rwf_t flags;
};
static_assert(sizeof(struct io_rw) <= sizeof(struct io_cmd_data));
static_assert(offsetof(struct io_rw, kiocb) == 0);
static_assert(offsetof(struct io_kiocb, cmd) == 0);
bool rust_rw_file_can_poll(struct io_kiocb *req);
__poll_t rust_rw_vfs_poll(struct file *file, struct poll_table_struct *pt);
bool rust_rw_is_compat(struct io_ring_ctx *ctx);
bool rust_rw_vectored(unsigned int opcode);
bool rust_rw_do_buffer_select(struct io_kiocb *req);
void rust_rw_vec_reset(struct iou_vec *vec, struct iovec *iov, unsigned nr);
void rust_rw_vec_kasan(struct iou_vec *vec);
bool rust_rw_cache_put(struct io_alloc_cache *cache, void *entry);
void rust_rw_async_clear(struct io_kiocb *req);
void *rust_rw_async_alloc(struct io_alloc_cache *cache, struct io_kiocb *req);
void rust_rw_iter_save(struct iov_iter *iter, struct iov_iter_state *state);
u16 rust_rw_current_ioprio(void);
int rust_rw_ioprio_check_cap(int ioprio);
unsigned long rust_rw_copy_from_user(void *dest, const void __user *src, unsigned long n);
void rust_rw_assert_locked(struct io_ring_ctx *ctx);
bool rust_rw_worker(void);
bool rust_rw_dying(struct io_ring_ctx *ctx);
void rust_rw_end_write(struct kiocb *iocb);
void rust_rw_start_write(struct kiocb *iocb);
bool rust_rw_sb_start_write_trylock(struct super_block *sb);
void rust_rw_sb_writers_release(struct super_block *sb);
void rust_rw_notify_modify(struct file *file);
void rust_rw_notify_access(struct file *file);
void rust_rw_set_fail(struct io_kiocb *req);
void rust_rw_set_res(struct io_kiocb *req, s32 res, u32 flags);
bool rust_rw_has_async(struct io_kiocb *req);
unsigned int rust_rw_put_kbuf(struct io_kiocb *req, int len, struct io_buffer_list *bl);
bool rust_rw_recycle_kbuf(struct io_kiocb *req, struct io_buffer_list *bl, unsigned flags);
void rust_rw_task_work_add(struct io_kiocb *req, unsigned flags);
void rust_rw_store_complete(struct io_kiocb *req);
bool rust_rw_load_complete(struct io_kiocb *req);
bool rust_rw_iter_ubuf(const struct iov_iter *iter);
bool rust_rw_iter_bvec(const struct iov_iter *iter);
void __user *rust_rw_iter_iov_addr(const struct iov_iter *iter);
size_t rust_rw_iter_iov_len(const struct iov_iter *iter);
bool rust_rw_wake_page_match(struct wait_page_queue *wpq, struct wait_page_key *key);
void rust_rw_list_del_init(struct list_head *entry);
void rust_rw_list_init(struct list_head *entry);
void rust_rw_list_del(struct list_head *entry);
int rust_rw_set_flags(struct kiocb *iocb, rwf_t flags, int type);
u64 rust_rw_ktime_get_ns(void);
void rust_rw_trace_short_write(struct io_ring_ctx *ctx, loff_t pos, size_t wanted, size_t actual);
bool rust_rw_is_uring_cmd(struct io_kiocb *req);
void rust_rw_timer_setup(struct hrtimer_sleeper *timer);
void rust_rw_timer_expires(struct hrtimer_sleeper *timer, u64 ns);
void rust_rw_set_interruptible(void);
void rust_rw_timer_start(struct hrtimer_sleeper *timer);
void rust_rw_set_running(void);
void rust_rw_timer_destroy(struct hrtimer_sleeper *timer);
bool rust_rw_rq_list_empty(struct io_comp_batch *iob);
void rust_rw_wq_add_tail(struct io_kiocb *req, struct io_ring_ctx *ctx);
enum {
 RUST_RW_IOCB_HIPRI = IOCB_HIPRI,
 RUST_RW_IOCB_NOWAIT = IOCB_NOWAIT,
 RUST_RW_IO_URING_F_UNLOCKED = IO_URING_F_UNLOCKED,
 RUST_RW_IO_URING_F_NONBLOCK = IO_URING_F_NONBLOCK,
 RUST_RW_IO_URING_F_MULTISHOT = IO_URING_F_MULTISHOT,
 RUST_RW_FMODE_STREAM = FMODE_STREAM,
 RUST_RW_FMODE_HAS_METADATA = FMODE_HAS_METADATA,
 RUST_RW_FMODE_READ = FMODE_READ,
 RUST_RW_FMODE_WRITE = FMODE_WRITE,
 RUST_RW_FOP_BUFFER_RASYNC = FOP_BUFFER_RASYNC,
 RUST_RW_FOP_BUFFER_WASYNC = FOP_BUFFER_WASYNC,
 RUST_RW_EPOLLIN = EPOLLIN,
 RUST_RW_EPOLLOUT = EPOLLOUT,
};
void rust_rw_poll_multishot_retry(struct io_kiocb *req);
#endif
