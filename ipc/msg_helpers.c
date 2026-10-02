// SPDX-License-Identifier: GPL-2.0
/* Macro, static-inline, layout and syscall-declaration boundaries only.
 * Queue algorithms, lifecycle, ABI conversion and proc output live in msg.rs.
 */
#include "msg_bindings.h"
struct ipc_namespace *rust_msg_current_ns(void) { return current->nsproxy->ipc_ns; }
struct task_struct *rust_msg_current(void) { return current; }
struct ipc_ids *rust_msg_ids(struct ipc_namespace *ns) { return &ns->ids[IPC_MSG_IDS]; }
int *rust_msg_ctlmax(struct ipc_namespace *ns) { return &ns->msg_ctlmax; }
int *rust_msg_ctlmnb(struct ipc_namespace *ns) { return &ns->msg_ctlmnb; }
int *rust_msg_ctlmni(struct ipc_namespace *ns) { return &ns->msg_ctlmni; }
struct percpu_counter *rust_msg_bytes(struct ipc_namespace *ns) { return &ns->percpu_msg_bytes; }
struct percpu_counter *rust_msg_hdrs(struct ipc_namespace *ns) { return &ns->percpu_msg_hdrs; }
struct rw_semaphore *rust_msg_rwsem(struct ipc_ids *ids) { return &ids->rwsem; }
int rust_msg_in_use(struct ipc_ids *ids) { return ids->in_use; }
int rust_msg_maxidx(struct ipc_ids *ids) { return ipc_get_maxidx(ids); }
void rust_msg_destroy_idr(struct ipc_ids *ids) { idr_destroy(&ids->ipcs_idr); }
void rust_msg_destroy_keys(struct ipc_ids *ids) { rhashtable_destroy(&ids->key_ht); }
struct msg_queue *rust_msg_alloc(void) { return kmalloc_obj(struct msg_queue, GFP_KERNEL_ACCOUNT); }
void rust_msg_rcu_read_lock(void) { rcu_read_lock(); }
void rust_msg_rcu_read_unlock(void) { rcu_read_unlock(); }
void rust_msg_lock(struct kern_ipc_perm *p) { ipc_lock_object(p); }
void rust_msg_unlock(struct kern_ipc_perm *p) { ipc_unlock_object(p); }
bool rust_msg_valid(struct kern_ipc_perm *p) { return ipc_valid_object(p); }
void rust_msg_list_init(struct list_head *p) { INIT_LIST_HEAD(p); }
void rust_msg_list_add_tail(struct list_head *p, struct list_head *h) { list_add_tail(p, h); }
void rust_msg_list_del(struct list_head *p) { list_del(p); }
void rust_msg_list_move_tail(struct list_head *p, struct list_head *h) { list_move_tail(p, h); }
void rust_msg_wake_init(struct wake_q_head *q) { wake_q_init(q); }
struct task_struct *rust_msg_get_task(struct task_struct *t) { return get_task_struct(t); }
struct pid *rust_msg_task_pid(struct task_struct *t) { return task_pid(t); }
struct pid *rust_msg_task_tgid(struct task_struct *t) { return task_tgid(t); }
void rust_msg_update_pid(struct pid **p, struct pid *v) { ipc_update_pid(p, v); }
void rust_msg_set_interruptible(void) { __set_current_state(TASK_INTERRUPTIBLE); }
bool rust_msg_signal_pending(void) { return signal_pending(current); }
void rust_msg_store_release(struct msg_msg **p, struct msg_msg *v) { smp_store_release(p, v); }
void rust_msg_write_once(struct msg_msg **p, struct msg_msg *v) { WRITE_ONCE(*p, v); }
struct msg_msg *rust_msg_read_once(struct msg_msg **p) { return READ_ONCE(*p); }
void rust_msg_acquire_after_ctrl_dep(void) { smp_acquire__after_ctrl_dep(); }
int rust_msg_counter_init(struct percpu_counter *p) { return percpu_counter_init(p, 0, GFP_KERNEL); }
void rust_msg_counter_destroy(struct percpu_counter *p) { percpu_counter_destroy(p); }
void rust_msg_counter_add(struct percpu_counter *p, s64 n) { percpu_counter_add_local(p, n); }
s64 rust_msg_counter_sum(struct percpu_counter *p) { return percpu_counter_sum(p); }
unsigned long rust_msg_copy_to_user(void __user *p, const void *v, size_t n) { return copy_to_user(p, v, n); }
unsigned long rust_msg_copy_from_user(void *p, const void __user *v, size_t n) { return copy_from_user(p, v, n); }
int rust_msg_get_type(const void __user *p, long *v) { return get_user(*v, (const long __user *)p); }
int rust_msg_put_type(void __user *p, long v) { return put_user(v, (long __user *)p); }
#ifdef CONFIG_COMPAT
int rust_msg_get_u32(const u32 __user *p, u32 *v) { return get_user(*v, p); }
int rust_msg_get_u16(const u16 __user *p, u16 *v) { return get_user(*v, p); }
int rust_msg_compat_parse_version(int *cmd) { return compat_ipc_parse_version(cmd); }
int rust_msg_get_compat_type(const void __user *p, compat_long_t *v) { return get_user(*v, (const compat_long_t __user *)p); }
int rust_msg_put_compat_type(void __user *p, long v) { return put_user(v, (compat_long_t __user *)p); }
void __user *rust_msg_compat_ptr(compat_uptr_t p) { return compat_ptr(p); }
#endif
void rust_msg_audit(struct kern_ipc_perm *p) { audit_ipc_obj(p); }
int rust_msg_pid_vnr(struct pid *p) { return pid_vnr(p); }
unsigned int rust_msg_uid(struct seq_file *s, kuid_t id) { return from_kuid_munged(seq_user_ns(s), id); }
unsigned int rust_msg_gid(struct seq_file *s, kgid_t id) { return from_kgid_munged(seq_user_ns(s), id); }
struct ipc_namespace *rust_msg_init_ns_ptr(void) { return &init_ipc_ns; }
void __init rust_msg_proc_init(int (*show)(struct seq_file *, void *))
{
 ipc_init_proc_interface("sysvipc/msg",
  "       key      msqid perms      cbytes       qnum lspid lrpid   uid   gid  cuid  cgid      stime      rtime      ctime\n",
  IPC_MSG_IDS, show);
}
int rust_msg_security_alloc(struct kern_ipc_perm *p) { return security_msg_queue_alloc(p); }
void rust_msg_security_free(struct kern_ipc_perm *p) { security_msg_queue_free(p); }
int rust_msg_security_associate(struct kern_ipc_perm *p, int flg) { return security_msg_queue_associate(p, flg); }
int rust_msg_security_ctl(struct kern_ipc_perm *p, int cmd) { return security_msg_queue_msgctl(p, cmd); }
int rust_msg_security_send(struct kern_ipc_perm *p, struct msg_msg *m, int flg) { return security_msg_queue_msgsnd(p, m, flg); }
int rust_msg_security_recv(struct kern_ipc_perm *p, struct msg_msg *m, struct task_struct *t, long type, int mode) { return security_msg_queue_msgrcv(p, m, t, type, mode); }

extern long rust_ksys_msgctl(int msqid, int cmd, void __user *buf, int version);
SYSCALL_DEFINE2(msgget, key_t, key, int, msgflg) { return ksys_msgget(key, msgflg); }
SYSCALL_DEFINE4(msgsnd, int, msqid, struct msgbuf __user *, msgp, size_t, msgsz, int, msgflg) { return ksys_msgsnd(msqid, msgp, msgsz, msgflg); }
SYSCALL_DEFINE5(msgrcv, int, msqid, struct msgbuf __user *, msgp, size_t, msgsz, long, msgtyp, int, msgflg) { return ksys_msgrcv(msqid, msgp, msgsz, msgtyp, msgflg); }
SYSCALL_DEFINE3(msgctl, int, msqid, int, cmd, struct msqid_ds __user *, buf) { return rust_ksys_msgctl(msqid, cmd, buf, IPC_64); }
#ifdef CONFIG_ARCH_WANT_IPC_PARSE_VERSION
SYSCALL_DEFINE3(old_msgctl, int, msqid, int, cmd, struct msqid_ds __user *, buf) { return ksys_old_msgctl(msqid, cmd, buf); }
#endif
#ifdef CONFIG_COMPAT
extern long rust_compat_ksys_msgctl(int msqid, int cmd, void __user *buf, int version);
COMPAT_SYSCALL_DEFINE4(msgsnd, int, msqid, compat_uptr_t, msgp, compat_ssize_t, msgsz, int, msgflg) { return compat_ksys_msgsnd(msqid, msgp, msgsz, msgflg); }
COMPAT_SYSCALL_DEFINE5(msgrcv, int, msqid, compat_uptr_t, msgp, compat_ssize_t, msgsz, compat_long_t, msgtyp, int, msgflg) { return compat_ksys_msgrcv(msqid, msgp, msgsz, msgtyp, msgflg); }
COMPAT_SYSCALL_DEFINE3(msgctl, int, msqid, int, cmd, void __user *, uptr) { return rust_compat_ksys_msgctl(msqid, cmd, uptr, IPC_64); }
#ifdef CONFIG_ARCH_WANT_COMPAT_IPC_PARSE_VERSION
COMPAT_SYSCALL_DEFINE3(old_msgctl, int, msqid, int, cmd, void __user *, uptr) { return compat_ksys_old_msgctl(msqid, cmd, uptr); }
#endif
#endif
