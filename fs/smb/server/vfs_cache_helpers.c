// SPDX-License-Identifier: GPL-2.0-or-later
// Configured macros and inline primitives only. No vfs_cache.c algorithm.
#include "vfs_cache_bindings.h"
DEFINE_RWLOCK(rvc_inode_hash_lock);
DEFINE_MUTEX(rvc_durable_scavenger_lock);
void rvc_read_lock(rwlock_t *l) { read_lock(l); }
void rvc_read_unlock(rwlock_t *l) { read_unlock(l); }
void rvc_write_lock(rwlock_t *l) { write_lock(l); }
void rvc_write_unlock(rwlock_t *l) { write_unlock(l); }
void rvc_rwlock_init(rwlock_t *l) { rwlock_init(l); }
void rvc_spin_lock(spinlock_t *l) { spin_lock(l); }
void rvc_spin_unlock(spinlock_t *l) { spin_unlock(l); }
void rvc_spin_lock_init(spinlock_t *l) { spin_lock_init(l); }
void rvc_mutex_init(struct mutex *l) { mutex_init(l); }
void rvc_mutex_lock(struct mutex *l) { mutex_lock(l); }
void rvc_mutex_unlock(struct mutex *l) { mutex_unlock(l); }
void rvc_init_rwsem(struct rw_semaphore *l) { init_rwsem(l); }
int rvc_atomic_read(const atomic_t *v) { return atomic_read(v); }
void rvc_atomic_set(atomic_t *v, int i) { atomic_set(v, i); }
void rvc_atomic_inc(atomic_t *v) { atomic_inc(v); }
void rvc_atomic_dec(atomic_t *v) { atomic_dec(v); }
bool rvc_atomic_inc_not_zero(atomic_t *v) { return atomic_inc_not_zero(v); }
bool rvc_atomic_dec_and_test(atomic_t *v) { return atomic_dec_and_test(v); }
bool rvc_atomic_sub_and_test(int i, atomic_t *v) { return atomic_sub_and_test(i, v); }
void rvc_atomic_sub(int i, atomic_t *v) { atomic_sub(i, v); }
void rvc_atomic_long_set(atomic_long_t *v, long i) { atomic_long_set(v, i); }
long rvc_atomic_long_dec_return(atomic_long_t *v) { return atomic_long_dec_return(v); }
void rvc_atomic_long_inc(atomic_long_t *v) { atomic_long_inc(v); }
void rvc_list_init(struct list_head *l) { INIT_LIST_HEAD(l); }
bool rvc_list_empty(const struct list_head *l) { return list_empty(l); }
void rvc_list_del_init(struct list_head *l) { list_del_init(l); }
void rvc_list_add_tail(struct list_head *l, struct list_head *h) { list_add_tail(l, h); }
void rvc_list_move_tail(struct list_head *l, struct list_head *h) { list_move_tail(l, h); }
void rvc_hlist_init(struct hlist_head *h) { INIT_HLIST_HEAD(h); }
void rvc_hlist_add_head(struct hlist_node *n, struct hlist_head *h) { hlist_add_head(n, h); }
void rvc_hlist_del_init(struct hlist_node *n) { hlist_del_init(n); }
void rvc_rcu_read_lock(void) { rcu_read_lock(); }
void rvc_rcu_read_unlock(void) { rcu_read_unlock(); }
struct oplock_info *rvc_opinfo_dereference(struct ksmbd_file *fp)
{ return rcu_dereference(fp->f_opinfo); }
void rvc_session_op_list_check(struct ksmbd_inode *ci)
{ __list_check_rcu(dummy, lockdep_is_held(&ci->m_lock), 0); }
void rvc_reopen_op_list_check(struct ksmbd_inode *ci)
{ __list_check_rcu(dummy, lockdep_is_held(&ci->m_lock), 0); }
struct list_head *rvc_rcu_list_next(struct list_head *node)
{ return READ_ONCE(node->next); }
struct ksmbd_conn *rvc_read_conn(struct ksmbd_file *fp) { return READ_ONCE(fp->conn); }
struct ksmbd_tree_connect *rvc_read_tcon(struct ksmbd_file *fp) { return READ_ONCE(fp->tcon); }
unsigned int rvc_read_fstate(struct ksmbd_file *fp) { return READ_ONCE(fp->f_state); }
struct inode *rvc_d_inode(struct dentry *d) { return d_inode(d); }
struct inode *rvc_file_inode(struct file *f) { return file_inode(f); }
struct mnt_idmap *rvc_file_mnt_idmap(struct file *f) { return file_mnt_idmap(f); }
bool rvc_is_err(const void *p) { return IS_ERR(p); }
bool rvc_is_err_or_null(const void *p) { return IS_ERR_OR_NULL(p); }
void *rvc_err_ptr(long err) { return ERR_PTR(err); }
long rvc_ptr_err(const void *p) { return PTR_ERR(p); }
void *rvc_kmalloc(size_t size, gfp_t flags) { return kmalloc(size, flags); }
void *rvc_kzalloc(size_t size, gfp_t flags) { return kzalloc(size, flags); }
void *rvc_cache_zalloc(struct kmem_cache *cache, gfp_t flags)
{ return kmem_cache_zalloc(cache, flags); }
struct kmem_cache *rvc_cache_create(const char *name, unsigned int size,
	unsigned int align, slab_flags_t flags)
{ return kmem_cache_create(name, size, align, flags, NULL); }
void rvc_idr_init(struct idr *idr) { idr_init(idr); }
void rvc_idr_preload_end(void) { idr_preload_end(); }
bool rvc_idr_is_empty(struct idr *idr) { return idr_is_empty(idr); }
void rvc_wait_init(wait_queue_head_t *wq) { init_waitqueue_head(wq); }
bool rvc_wait_active(wait_queue_head_t *wq) { return waitqueue_active(wq); }
void rvc_wake_up(wait_queue_head_t *wq) { wake_up(wq); }
long rvc_wait_timeout(wait_queue_head_t *wq, bool (*alive)(void), long timeout)
{ return wait_event_interruptible_timeout(*wq, alive() == false, timeout); }
unsigned long rvc_jiffies(void) { return jiffies; }
unsigned long rvc_msecs_to_jiffies(unsigned int m) { return __msecs_to_jiffies(m); }
unsigned int rvc_jiffies_to_msecs(unsigned long j) { return jiffies_to_msecs(j); }
void *rvc_vmalloc(unsigned long size) { return vmalloc(size); }
const struct cred *rvc_override_creds(const struct cred *cred)
{ return override_creds(cred); }
void rvc_revert_creds(const struct cred *cred) { revert_creds(cred); }
void rvc_module_get(void) { __module_get(THIS_MODULE); }
void rvc_module_put(void) { module_put(THIS_MODULE); }
void rvc_set_freezable(void) { set_freezable(); }
bool rvc_try_to_freeze(void) { return try_to_freeze(); }
struct task_struct *rvc_kthread_run(int (*threadfn)(void *), void *data,
	const char *name) { return kthread_run(threadfn, data, "%s", name); }
bool rvc_warn_missing_conn(bool missing) { return WARN_ON_ONCE(missing); }
void rvc_err_inode_init(void) { pr_err("inode initialized failed\n"); }
void rvc_err_alloc(void) { pr_err("Failed to allocate memory\n"); }
void rvc_err_xattr(const char *name) { pr_err("remove xattr failed : %s\n", name); }
void rvc_err_thread(long err) { pr_err("cannot start conn thread, err : %ld\n", err); }
void rvc_warn_proc(void) { pr_warn("Unable to create files procfs entry\n"); }
#ifdef CONFIG_PROC_FS
void rvc_seq_puts(struct seq_file *m, const char *s) { seq_puts(m, s); }
#endif
void rvc_debug_reconnect(const char *name) { ksmbd_debug(SMB, "invalid name reconnect %s\n", name); }
void rvc_err_durable(void *conn, void *tcon) { pr_err("Invalid durable fd [%p:%p]\n", conn, tcon); }
void rvc_err_in_use(unsigned long long id) { pr_err("Still in use durable fd: %llu\n", id); }
void rvc_err_cache(void) { pr_err("failed to allocate file cache\n"); }
