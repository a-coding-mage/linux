// SPDX-License-Identifier: GPL-2.0-or-later
/* Configured C declaration, macro and header-inline boundaries only.
 * All watch, event, group, syscall and initialization policy lives in Rust. */
#include "inotify_user_bindings.h"
#ifdef CONFIG_CFI
__ADDRESSABLE(fsnotify_fasync);
__ADDRESSABLE(noop_llseek);
#ifdef CONFIG_PROC_FS
__ADDRESSABLE(inotify_show_fdinfo);
#endif
#ifdef CONFIG_SYSCTL
__ADDRESSABLE(proc_doulongvec_minmax);
__ADDRESSABLE(proc_dointvec_minmax);
#endif
#endif
struct fsnotify_group *rust_inotify_file_group(struct file *f) { return f->private_data; }
const struct file_operations *rust_inotify_file_ops(struct file *f) { return f->f_op; }
unsigned int rust_inotify_file_flags(struct file *f) { return f->f_flags; }
bool rust_inotify_inode_is_dir(struct inode *i) { return S_ISDIR(i->i_mode); }
struct inode *rust_inotify_path_inode(struct path *p) { return p->dentry->d_inode; }
void rust_inotify_fdput(struct fd f) { fdput(f); }
struct file *rust_inotify_fd_file(struct fd f) { return fd_file(f); }
void rust_inotify_spin_lock(spinlock_t *lock) { spin_lock(lock); }
void rust_inotify_spin_unlock(spinlock_t *lock) { spin_unlock(lock); }
void rust_inotify_assert_locked(spinlock_t *lock) { assert_spin_locked(lock); }
void rust_inotify_spin_init(spinlock_t *lock) { spin_lock_init(lock); }
unsigned int rust_inotify_refcount(refcount_t *ref) { return refcount_read(ref); }
void rust_inotify_warn_no_wd(struct inotify_inode_mark *m)
{ WARN_ONCE(1, "inotify_remove_from_idr: i_mark=%p i_mark->wd=%d i_mark->group=%p\n", m, m->wd, m->fsn_mark.group); }
void rust_inotify_warn_missing(struct inotify_inode_mark *m)
{ WARN_ONCE(1, "inotify_remove_from_idr: i_mark=%p i_mark->wd=%d i_mark->group=%p\n", m, m->wd, m->fsn_mark.group); }
void rust_inotify_warn_mismatch(struct inotify_inode_mark *m, struct inotify_inode_mark *found)
{ WARN_ONCE(1, "inotify_remove_from_idr: i_mark=%p i_mark->wd=%d i_mark->group=%p found_i_mark=%p found_i_mark->wd=%d found_i_mark->group=%p\n", m, m->wd, m->fsn_mark.group, found, found->wd, found->fsn_mark.group); }
void rust_inotify_bad_refcount_log(struct inotify_inode_mark *m)
{ printk(KERN_ERR "inotify_remove_from_idr: i_mark=%p i_mark->wd=%d i_mark->group=%p\n", m, m->wd, m->fsn_mark.group); }
void rust_inotify_poll_wait(struct file *f, wait_queue_head_t *q, poll_table *p) { poll_wait(f, q, p); }
bool rust_inotify_queue_empty(struct fsnotify_group *group) { return fsnotify_notify_queue_is_empty(group); }
void rust_inotify_init_wait(struct wait_queue_entry *wait) { init_wait_func(wait, woken_wake_function); }
void rust_inotify_init_event(struct fsnotify_event *event) { fsnotify_init_event(event); }
bool rust_inotify_signal_pending(void) { return signal_pending(current); }
unsigned long rust_inotify_copy_to_user(void __user *dst, const void *src, size_t len) { return copy_to_user(dst, src, len); }
unsigned long rust_inotify_clear_user(void __user *dst, size_t len) { return clear_user(dst, len); }
int rust_inotify_put_int(int __user *dst, int value) { return put_user(value, dst); }
int rust_inotify_user_path(const char __user *name, unsigned int flags, struct path *path) { return user_path_at(AT_FDCWD, name, flags, path); }
int rust_inotify_security_path(struct path *p, u64 mask) { return security_path_notify(p, mask, FSNOTIFY_OBJ_TYPE_INODE); }
int rust_inotify_path_permission(struct path *p, int mask) { return path_permission(p, mask); }
void rust_inotify_idr_preload_end(void) { idr_preload_end(); }
void rust_inotify_idr_init(struct idr *idr) { idr_init(idr); }
void rust_inotify_idr_cursor(struct idr *idr, unsigned int cursor) { idr_set_cursor(idr, cursor); }
void rust_inotify_group_lock(struct fsnotify_group *group) { fsnotify_group_lock(group); }
void rust_inotify_group_unlock(struct fsnotify_group *group) { fsnotify_group_unlock(group); }
struct inotify_inode_mark *rust_inotify_alloc_mark(struct kmem_cache *cache) { return kmem_cache_alloc(cache, GFP_KERNEL); }
struct inotify_event_info *rust_inotify_alloc_overflow(void) { return kmalloc_obj(struct inotify_event_info, GFP_KERNEL_ACCOUNT); }
struct kmem_cache *rust_inotify_create_cache(void) { return KMEM_CACHE(inotify_inode_mark, SLAB_PANIC | SLAB_ACCOUNT); }
struct mem_cgroup *rust_inotify_current_memcg(void) { return get_mem_cgroup_from_mm(current->mm); }
struct ucounts *rust_inotify_inc_instances(void) { return inc_ucount(current_user_ns(), current_euid(), UCOUNT_INOTIFY_INSTANCES); }
struct ucounts *rust_inotify_inc_watches(struct ucounts *u) { return inc_inotify_watches(u); }
void rust_inotify_dec_watches(struct ucounts *u) { dec_inotify_watches(u); }
long *rust_inotify_ucount_max(enum ucount_type type) { return &init_user_ns.ucount_max[type]; }
void rust_inotify_debug_get(struct fsnotify_group *g, struct fsnotify_event *e) { pr_debug("get_one_event: group=%p event=%p\n", g, e); }
void rust_inotify_debug_copy(struct fsnotify_group *g, struct fsnotify_event *e) { pr_debug("copy_event_to_user: group=%p event=%p\n", g, e); }
void rust_inotify_debug_read(struct fsnotify_group *g, struct fsnotify_event *e) { pr_debug("inotify_read: group=%p kevent=%p\n", g, e); }
void rust_inotify_debug_release(struct fsnotify_group *g) { pr_debug("inotify_release: group=%p\n", g); }
void rust_inotify_debug_ioctl(struct fsnotify_group *g, unsigned int cmd) { pr_debug("inotify_ioctl: group=%p cmd=%u\n", g, cmd); }
/* Architecture-specific syscall ABI, tracing metadata and initcall sections. */
SYSCALL_DEFINE1(inotify_init1, int, flags) { return rust_inotify_init1(flags); }
SYSCALL_DEFINE0(inotify_init) { return rust_inotify_init(); }
SYSCALL_DEFINE3(inotify_add_watch, int, fd, const char __user *, pathname, u32, mask)
{ return rust_inotify_add_watch(fd, pathname, mask); }
SYSCALL_DEFINE2(inotify_rm_watch, int, fd, __s32, wd) { return rust_inotify_rm_watch(fd, wd); }
fs_initcall(rust_inotify_user_setup);
