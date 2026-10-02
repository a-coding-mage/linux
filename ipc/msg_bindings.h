/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_IPC_MSG_BINDINGS_H
#define LUPOS_IPC_MSG_BINDINGS_H
#include <linux/capability.h>
#include <linux/msg.h>
#include <linux/spinlock.h>
#include <linux/init.h>
#include <linux/mm.h>
#include <linux/proc_fs.h>
#include <linux/list.h>
#include <linux/security.h>
#include <linux/sched/wake_q.h>
#include <linux/syscalls.h>
#include <linux/audit.h>
#include <linux/seq_file.h>
#include <linux/rwsem.h>
#include <linux/nsproxy.h>
#include <linux/ipc_namespace.h>
#include <linux/rhashtable.h>
#include <linux/percpu_counter.h>
#include <asm/current.h>
#include <linux/uaccess.h>
#include "util.h"
/* The private ABI from msg.c, shared with bindgen; no handwritten Rust layouts. */
struct msg_queue {
 struct kern_ipc_perm q_perm;
 time64_t q_stime, q_rtime, q_ctime;
 unsigned long q_cbytes, q_qnum, q_qbytes;
 struct pid *q_lspid, *q_lrpid;
 struct list_head q_messages, q_receivers, q_senders;
} __randomize_layout;
struct msg_receiver {
 struct list_head r_list;
 struct task_struct *r_tsk;
 int r_mode;
 long r_msgtype, r_maxsize;
 struct msg_msg *r_msg;
};
struct msg_sender {
 struct list_head list;
 struct task_struct *tsk;
 size_t msgsz;
};
#ifdef CONFIG_COMPAT
struct compat_msqid_ds {
 struct compat_ipc_perm msg_perm;
 compat_uptr_t msg_first, msg_last;
 old_time32_t msg_stime, msg_rtime, msg_ctime;
 compat_ulong_t msg_lcbytes, msg_lqbytes;
 unsigned short msg_cbytes, msg_qnum, msg_qbytes;
 compat_ipc_pid_t msg_lspid, msg_lrpid;
};
#endif
struct ipc_namespace *rust_msg_current_ns(void);
struct task_struct *rust_msg_current(void);
struct ipc_ids *rust_msg_ids(struct ipc_namespace *ns);
int *rust_msg_ctlmax(struct ipc_namespace *ns);
int *rust_msg_ctlmnb(struct ipc_namespace *ns);
int *rust_msg_ctlmni(struct ipc_namespace *ns);
struct percpu_counter *rust_msg_bytes(struct ipc_namespace *ns);
struct percpu_counter *rust_msg_hdrs(struct ipc_namespace *ns);
struct rw_semaphore *rust_msg_rwsem(struct ipc_ids *ids);
int rust_msg_in_use(struct ipc_ids *ids);
int rust_msg_maxidx(struct ipc_ids *ids);
void rust_msg_destroy_idr(struct ipc_ids *ids);
void rust_msg_destroy_keys(struct ipc_ids *ids);
struct msg_queue *rust_msg_alloc(void);
void rust_msg_rcu_read_lock(void);
void rust_msg_rcu_read_unlock(void);
void rust_msg_lock(struct kern_ipc_perm *p);
void rust_msg_unlock(struct kern_ipc_perm *p);
bool rust_msg_valid(struct kern_ipc_perm *p);
void rust_msg_list_init(struct list_head *p);
void rust_msg_list_add_tail(struct list_head *p, struct list_head *h);
void rust_msg_list_del(struct list_head *p);
void rust_msg_list_move_tail(struct list_head *p, struct list_head *h);
void rust_msg_wake_init(struct wake_q_head *q);
struct task_struct *rust_msg_get_task(struct task_struct *t);
struct pid *rust_msg_task_pid(struct task_struct *t);
struct pid *rust_msg_task_tgid(struct task_struct *t);
void rust_msg_update_pid(struct pid **p, struct pid *v);
void rust_msg_set_interruptible(void);
bool rust_msg_signal_pending(void);
void rust_msg_store_release(struct msg_msg **p, struct msg_msg *v);
void rust_msg_write_once(struct msg_msg **p, struct msg_msg *v);
struct msg_msg *rust_msg_read_once(struct msg_msg **p);
void rust_msg_acquire_after_ctrl_dep(void);
int rust_msg_counter_init(struct percpu_counter *p);
void rust_msg_counter_destroy(struct percpu_counter *p);
enum { RUST_MSGSEG = MSGSEG };
void rust_msg_counter_add(struct percpu_counter *p, s64 n);
s64 rust_msg_counter_sum(struct percpu_counter *p);
unsigned long rust_msg_copy_to_user(void __user *p, const void *v, size_t n);
unsigned long rust_msg_copy_from_user(void *p, const void __user *v, size_t n);
int rust_msg_get_type(const void __user *p, long *v);
int rust_msg_put_type(void __user *p, long v);
#ifdef CONFIG_COMPAT
int rust_msg_get_u32(const u32 __user *p, u32 *v);
int rust_msg_get_u16(const u16 __user *p, u16 *v);
int rust_msg_compat_parse_version(int *cmd);
int rust_msg_get_compat_type(const void __user *p, compat_long_t *v);
int rust_msg_put_compat_type(void __user *p, long v);
void __user *rust_msg_compat_ptr(compat_uptr_t p);
#endif
void rust_msg_audit(struct kern_ipc_perm *p);
int rust_msg_pid_vnr(struct pid *p);
unsigned int rust_msg_uid(struct seq_file *s, kuid_t id);
unsigned int rust_msg_gid(struct seq_file *s, kgid_t id);
struct ipc_namespace *rust_msg_init_ns_ptr(void);
void rust_msg_proc_init(int (*show)(struct seq_file *, void *));
/* LSM functions may be static inlines when CONFIG_SECURITY is disabled. */
int rust_msg_security_alloc(struct kern_ipc_perm *p);
void rust_msg_security_free(struct kern_ipc_perm *p);
int rust_msg_security_associate(struct kern_ipc_perm *p, int flg);
int rust_msg_security_ctl(struct kern_ipc_perm *p, int cmd);
int rust_msg_security_send(struct kern_ipc_perm *p, struct msg_msg *m, int flg);
int rust_msg_security_recv(struct kern_ipc_perm *p, struct msg_msg *m, struct task_struct *t, long type, int mode);
#endif
