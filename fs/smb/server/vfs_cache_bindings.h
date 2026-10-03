/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Configured authoritative layouts; macro/static-inline boundaries only. */
#ifndef LUPOS_KSMBD_VFS_CACHE_BINDINGS_H
#define LUPOS_KSMBD_VFS_CACHE_BINDINGS_H
#include <linux/fs.h>
#include <linux/filelock.h>
#include <linux/slab.h>
#include <linux/vmalloc.h>
#include <linux/kthread.h>
#include <linux/freezer.h>
#include <linux/dcache.h>
#include <linux/hash.h>
#include <linux/cred.h>
#include <linux/seq_file.h>
#include "glob.h"
#include "vfs_cache.h"
#include "oplock.h"
#include "vfs.h"
#include "connection.h"
#include "misc.h"
#include "mgmt/tree_connect.h"
#include "mgmt/user_session.h"
#include "mgmt/user_config.h"
#include "smb_common.h"
#include "server.h"
#include "smb2pdu.h"

extern rwlock_t rvc_inode_hash_lock;
extern struct mutex rvc_durable_scavenger_lock;
void rvc_read_lock(rwlock_t *l);
void rvc_read_unlock(rwlock_t *l);
void rvc_write_lock(rwlock_t *l);
void rvc_write_unlock(rwlock_t *l);
void rvc_rwlock_init(rwlock_t *l);
void rvc_spin_lock(spinlock_t *l);
void rvc_spin_unlock(spinlock_t *l);
void rvc_spin_lock_init(spinlock_t *l);
void rvc_mutex_init(struct mutex *l);
void rvc_mutex_lock(struct mutex *l);
void rvc_mutex_unlock(struct mutex *l);
void rvc_init_rwsem(struct rw_semaphore *l);
int rvc_atomic_read(const atomic_t *v);
void rvc_atomic_set(atomic_t *v, int i);
void rvc_atomic_inc(atomic_t *v);
void rvc_atomic_dec(atomic_t *v);
bool rvc_atomic_inc_not_zero(atomic_t *v);
bool rvc_atomic_dec_and_test(atomic_t *v);
bool rvc_atomic_sub_and_test(int i, atomic_t *v);
void rvc_atomic_sub(int i, atomic_t *v);
void rvc_atomic_long_set(atomic_long_t *v, long i);
long rvc_atomic_long_dec_return(atomic_long_t *v);
void rvc_atomic_long_inc(atomic_long_t *v);
void rvc_list_init(struct list_head *l);
bool rvc_list_empty(const struct list_head *l);
void rvc_list_del_init(struct list_head *l);
void rvc_list_add_tail(struct list_head *l, struct list_head *h);
void rvc_list_move_tail(struct list_head *l, struct list_head *h);
void rvc_hlist_init(struct hlist_head *h);
void rvc_hlist_add_head(struct hlist_node *n, struct hlist_head *h);
void rvc_hlist_del_init(struct hlist_node *n);
void rvc_rcu_read_lock(void);
void rvc_rcu_read_unlock(void);
struct oplock_info *rvc_opinfo_dereference(struct ksmbd_file *fp);
/* Two separate original list_for_each_entry_rcu diagnostic sites. */
void rvc_session_op_list_check(struct ksmbd_inode *ci);
void rvc_reopen_op_list_check(struct ksmbd_inode *ci);
struct list_head *rvc_rcu_list_next(struct list_head *node);
struct ksmbd_conn *rvc_read_conn(struct ksmbd_file *fp);
struct ksmbd_tree_connect *rvc_read_tcon(struct ksmbd_file *fp);
unsigned int rvc_read_fstate(struct ksmbd_file *fp);
struct inode *rvc_d_inode(struct dentry *d);
struct inode *rvc_file_inode(struct file *f);
struct mnt_idmap *rvc_file_mnt_idmap(struct file *f);
bool rvc_is_err(const void *p);
bool rvc_is_err_or_null(const void *p);
void *rvc_err_ptr(long err);
long rvc_ptr_err(const void *p);
void *rvc_kmalloc(size_t size, gfp_t flags);
void *rvc_kzalloc(size_t size, gfp_t flags);
void *rvc_cache_zalloc(struct kmem_cache *cache, gfp_t flags);
struct kmem_cache *rvc_cache_create(const char *name, unsigned int size,
	unsigned int align, slab_flags_t flags);
void rvc_idr_init(struct idr *idr);
void rvc_idr_preload_end(void);
bool rvc_idr_is_empty(struct idr *idr);
void rvc_wait_init(wait_queue_head_t *wq);
bool rvc_wait_active(wait_queue_head_t *wq);
void rvc_wake_up(wait_queue_head_t *wq);
long rvc_wait_timeout(wait_queue_head_t *wq, bool (*alive)(void), long timeout);
unsigned long rvc_jiffies(void);
unsigned long rvc_msecs_to_jiffies(unsigned int m);
unsigned int rvc_jiffies_to_msecs(unsigned long j);
void *rvc_vmalloc(unsigned long size);
const struct cred *rvc_override_creds(const struct cred *cred);
void rvc_revert_creds(const struct cred *cred);
void rvc_module_get(void);
void rvc_module_put(void);
void rvc_set_freezable(void);
bool rvc_try_to_freeze(void);
struct task_struct *rvc_kthread_run(int (*threadfn)(void *), void *data,
	const char *name);
bool rvc_warn_missing_conn(bool missing);
void rvc_err_inode_init(void);
void rvc_err_alloc(void);
void rvc_err_xattr(const char *name);
void rvc_err_thread(long err);
void rvc_warn_proc(void);
#ifdef CONFIG_PROC_FS
void rvc_seq_puts(struct seq_file *m, const char *s);
#endif
void rvc_debug_reconnect(const char *name);
void rvc_err_durable(void *conn, void *tcon);
void rvc_err_in_use(unsigned long long id);
void rvc_err_cache(void);

/* Clang evaluates target endian/GFP/cache macros, bindgen owns the values. */
static const unsigned long RVC_GOLDEN_RATIO_PRIME = GOLDEN_RATIO_PRIME;
static const unsigned long RVC_CACHE_BYTES = L1_CACHE_BYTES;
static const unsigned int RVC_DEFAULT_GFP = KSMBD_DEFAULT_GFP;
static const unsigned int RVC_NO_FID = KSMBD_NO_FID;
static const unsigned int RVC_GFP_KERNEL = GFP_KERNEL;
static const unsigned int RVC_GFP_NOWAIT = GFP_NOWAIT;
static const slab_flags_t RVC_SLAB_HWCACHE_ALIGN = SLAB_HWCACHE_ALIGN;
static const unsigned int RVC_DELETE_ON_CLOSE = FILE_DELETE_ON_CLOSE_LE;
static const unsigned int RVC_SHARE_DELETE = FILE_SHARE_DELETE_LE;
static const unsigned int RVC_LEASE_HANDLE_LE = SMB2_LEASE_HANDLE_CACHING_LE;
static const unsigned int RVC_DURABLE_FLAG = KSMBD_GLOBAL_FLAG_DURABLE_HANDLE;
static const unsigned int RVC_LEASE_NONE = le32_to_cpu(SMB2_LEASE_NONE_LE);
static const unsigned int RVC_LEASE_R = le32_to_cpu(SMB2_LEASE_READ_CACHING_LE);
static const unsigned int RVC_LEASE_H = le32_to_cpu(SMB2_LEASE_HANDLE_CACHING_LE);
static const unsigned int RVC_LEASE_W = le32_to_cpu(SMB2_LEASE_WRITE_CACHING_LE);
static const unsigned int RVC_INODE_STATUS_OK = KSMBD_INODE_STATUS_OK;
static const unsigned int RVC_INODE_STATUS_UNKNOWN = KSMBD_INODE_STATUS_UNKNOWN;
static const unsigned int RVC_INODE_STATUS_PENDING_DELETE = KSMBD_INODE_STATUS_PENDING_DELETE;
#endif
