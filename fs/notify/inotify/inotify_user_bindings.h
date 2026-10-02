/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_INOTIFY_USER_BINDINGS_H
#define LUPOS_INOTIFY_USER_BINDINGS_H
#include <linux/file.h>
#include <linux/fs.h>
#include <linux/fsnotify_backend.h>
#include <linux/idr.h>
#include <linux/init.h>
#include <linux/inotify.h>
#include <linux/kernel.h>
#include <linux/namei.h>
#include <linux/sched/signal.h>
#include <linux/slab.h>
#include <linux/syscalls.h>
#include <linux/anon_inodes.h>
#include <linux/uaccess.h>
#include <linux/poll.h>
#include <linux/wait.h>
#include <linux/memcontrol.h>
#include <linux/security.h>
#include <linux/sysctl.h>
#include "inotify.h"
#include "../fdinfo.h"
#include <asm/ioctls.h>
enum {
 RUST_INOTIFY_WATCH_COST = sizeof(struct inotify_inode_mark) + 2 * sizeof(struct inode),
 RUST_INOTIFY_PAGE_SHIFT = PAGE_SHIFT,
 RUST_INOTIFY_GFP_KERNEL = GFP_KERNEL,
 RUST_INOTIFY_GFP_NOWAIT = GFP_NOWAIT,
 RUST_INOTIFY_SETNEXTWD = INOTIFY_IOC_SETNEXTWD,
 RUST_INOTIFY_EPOLLIN = EPOLLIN,
 RUST_INOTIFY_EPOLLRDNORM = EPOLLRDNORM,
 RUST_INOTIFY_LOOKUP_FOLLOW = LOOKUP_FOLLOW,
 RUST_INOTIFY_LOOKUP_DIRECTORY = LOOKUP_DIRECTORY,
};
struct fsnotify_group *rust_inotify_file_group(struct file *f);
const struct file_operations *rust_inotify_file_ops(struct file *f);
unsigned int rust_inotify_file_flags(struct file *f);
bool rust_inotify_inode_is_dir(struct inode *i);
struct inode *rust_inotify_path_inode(struct path *p);
void rust_inotify_fdput(struct fd f);
struct file *rust_inotify_fd_file(struct fd f);
void rust_inotify_spin_lock(spinlock_t *lock);
void rust_inotify_spin_unlock(spinlock_t *lock);
void rust_inotify_assert_locked(spinlock_t *lock);
void rust_inotify_spin_init(spinlock_t *lock);
unsigned int rust_inotify_refcount(refcount_t *ref);
void rust_inotify_warn_no_wd(struct inotify_inode_mark *m);
void rust_inotify_warn_missing(struct inotify_inode_mark *m);
void rust_inotify_warn_mismatch(struct inotify_inode_mark *m, struct inotify_inode_mark *found);
void rust_inotify_bad_refcount_log(struct inotify_inode_mark *m);
void rust_inotify_poll_wait(struct file *f, wait_queue_head_t *q, poll_table *p);
bool rust_inotify_queue_empty(struct fsnotify_group *group);
void rust_inotify_init_wait(struct wait_queue_entry *wait);
void rust_inotify_init_event(struct fsnotify_event *event);
bool rust_inotify_signal_pending(void);
unsigned long rust_inotify_copy_to_user(void __user *dst, const void *src, size_t len);
unsigned long rust_inotify_clear_user(void __user *dst, size_t len);
int rust_inotify_put_int(int __user *dst, int value);
int rust_inotify_user_path(const char __user *name, unsigned int flags, struct path *path);
int rust_inotify_security_path(struct path *p, u64 mask);
int rust_inotify_path_permission(struct path *p, int mask);
void rust_inotify_idr_preload_end(void);
void rust_inotify_idr_init(struct idr *idr);
void rust_inotify_idr_cursor(struct idr *idr, unsigned int cursor);
void rust_inotify_group_lock(struct fsnotify_group *group);
void rust_inotify_group_unlock(struct fsnotify_group *group);
struct inotify_inode_mark *rust_inotify_alloc_mark(struct kmem_cache *cache);
struct inotify_event_info *rust_inotify_alloc_overflow(void);
struct kmem_cache *rust_inotify_create_cache(void);
struct mem_cgroup *rust_inotify_current_memcg(void);
struct ucounts *rust_inotify_inc_instances(void);
struct ucounts *rust_inotify_inc_watches(struct ucounts *u);
void rust_inotify_dec_watches(struct ucounts *u);
long *rust_inotify_ucount_max(enum ucount_type type);
void rust_inotify_debug_get(struct fsnotify_group *g, struct fsnotify_event *e);
void rust_inotify_debug_copy(struct fsnotify_group *g, struct fsnotify_event *e);
void rust_inotify_debug_read(struct fsnotify_group *g, struct fsnotify_event *e);
void rust_inotify_debug_release(struct fsnotify_group *g);
void rust_inotify_debug_ioctl(struct fsnotify_group *g, unsigned int cmd);
long rust_inotify_init1(int flags);
long rust_inotify_init(void);
long rust_inotify_add_watch(int fd, const char __user *pathname, u32 mask);
long rust_inotify_rm_watch(int fd, s32 wd);
int rust_inotify_user_setup(void);
#endif
