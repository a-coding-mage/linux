// SPDX-License-Identifier: GPL-2.0-only
/* Native representation and primitive boundaries for pid.rs.
 * No allocator, namespace traversal, task-linkage policy, pidfd policy or
 * sysctl policy from pid.c is implemented here. See NATIVE-BOUNDARY.md.
 */
#include "pid_bindings.h"

/* All PID-owned initial state is defined in pid_initializers.rs. */
static_assert(PID_MAX_LIMIT < PIDNS_ADDING);

struct task_struct *lupos_pid_current(void) { return current; }
void lupos_pid_map_lock(void) { spin_lock(&lupos_pidmap_lock_state); }
void lupos_pid_map_unlock(void) { spin_unlock(&lupos_pidmap_lock_state); }
void lupos_pid_assert_tasklist_write(void) { lockdep_assert_held_write(&tasklist_lock); }
void lupos_pid_assert_tasklist_unheld(void) { lockdep_assert_not_held(&tasklist_lock); }
void lupos_pid_warn_rcu(void) { RCU_LOCKDEP_WARN(!rcu_read_lock_held(), "find_task_by_pid_ns() needs rcu_read_lock() protection"); }
void lupos_pid_warn_reaper(bool value) { WARN_ON(value); }
void lupos_pid_warn_pending(bool value) { WARN_ON(value); }
void lupos_pid_warn_transfer(bool value) { WARN_ON_ONCE(value); }
void lupos_pid_bug(bool value) { BUG_ON(value); }
bool lupos_pid_ref_dec(refcount_t *ref) { return refcount_dec_and_test(ref); }
void lupos_pid_ref_set(refcount_t *ref, int value) { refcount_set(ref, value); }
struct pid *lupos_pid_get(struct pid *pid) { return get_pid(pid); }
struct pid_namespace *lupos_pid_ns_get(struct pid_namespace *ns) { return get_pid_ns(ns); }
void lupos_pid_ns_put(struct pid_namespace *ns) { put_pid_ns(ns); }
void lupos_pid_ns_active_get(struct pid_namespace *ns) { ns_ref_active_get(ns); }
void lupos_pid_ns_active_put(struct pid_namespace *ns) { ns_ref_active_put(ns); }
void lupos_pid_spin_init(spinlock_t *lock) { spin_lock_init(lock); }
void lupos_pid_wait_init(wait_queue_head_t *wait) { init_waitqueue_head(wait); }
void lupos_pid_hlist_init(struct hlist_head *head) { INIT_HLIST_HEAD(head); }
void lupos_pid_hlist_add(struct hlist_node *node, struct hlist_head *head) { hlist_add_head_rcu(node, head); }
void lupos_pid_hlist_del(struct hlist_node *node) { hlist_del_rcu(node); }
void lupos_pid_hlist_swap(struct hlist_head *left, struct hlist_head *right) { hlists_swap_heads_rcu(left, right); }
void lupos_pid_hlist_replace(struct hlist_node *old, struct hlist_node *new) { hlist_replace_rcu(old, new); }
bool lupos_pid_hlist_empty(struct hlist_head *head) { return hlist_empty(head); }
struct hlist_node *lupos_pid_hlist_first(struct hlist_head *head) { return rcu_dereference_check(hlist_first_rcu(head), lockdep_tasklist_lock_is_held()); }
void lupos_pid_rcu_lock(void) { rcu_read_lock(); }
void lupos_pid_rcu_unlock(void) { rcu_read_unlock(); }
struct pid *lupos_pid_rcu_load(struct pid **ptr) { return rcu_dereference(*ptr); }
void lupos_pid_rcu_store(struct pid **ptr, struct pid *value) { rcu_assign_pointer(*ptr, value); }
void lupos_pid_write_nr(pid_t *ptr, pid_t value) { WRITE_ONCE(*ptr, value); }
int lupos_pid_read_int(const int *ptr) { return READ_ONCE(*ptr); }
struct task_struct *lupos_pid_read_task(struct task_struct * const *ptr) { return READ_ONCE(*ptr); }
extern void lupos_pid_delayed_put_pid(struct rcu_head *head);
static void lupos_pid_rcu_callback(struct rcu_head *head) { lupos_pid_delayed_put_pid(head); }
void lupos_pid_call_rcu(struct rcu_head *head) { call_rcu(head, lupos_pid_rcu_callback); }
void lupos_pid_get_task(struct task_struct *task) { get_task_struct(task); }
void lupos_pid_put_task(struct task_struct *task) { put_task_struct(task); }
bool lupos_pid_checkpoint_capable(struct user_namespace *ns) { return checkpoint_restore_ns_capable(ns); }
void *lupos_pid_cache_alloc(struct kmem_cache *cache, gfp_t flags) { return kmem_cache_alloc(cache, flags); }
struct kmem_cache *lupos_pid_cache_create(const char *name, unsigned int size, unsigned int align, slab_flags_t flags) { return kmem_cache_create(name, size, align, flags, NULL); }
void *lupos_pid_memdup(const void *src, size_t len, gfp_t flags) { return kmemdup(src, len, flags); }
unsigned int lupos_pid_idr_cursor(const struct idr *idr) { return idr_get_cursor(idr); }
void lupos_pid_idr_set_cursor(struct idr *idr, unsigned int cursor) { idr_set_cursor(idr, cursor); }
void lupos_pid_idr_preload_end(void) { idr_preload_end(); }
void lupos_pid_idr_init(struct idr *idr) { idr_init(idr); }
unsigned int lupos_pid_possible_cpus(void) { return num_possible_cpus(); }
void lupos_pid_log_limits(unsigned int max, unsigned int min) { pr_info("pid_max: default: %u minimum: %u\n", max, min); }
void *lupos_pid_err_ptr(long err) { return ERR_PTR(err); }
bool lupos_pid_is_err(const void *ptr) { return IS_ERR(ptr); }
long lupos_pid_ptr_err(const void *ptr) { return PTR_ERR(ptr); }
struct file *lupos_pid_fd_file(struct fd fd) { return fd_file(fd); }
bool lupos_pid_fd_empty(struct fd fd) { return fd_empty(fd); }
void lupos_pid_fdput(struct fd fd) { fdput(fd); }

#ifdef CONFIG_SYSCTL
kuid_t lupos_pid_current_euid(void) { return current_euid(); }
bool lupos_pid_ns_capable_noaudit(struct user_namespace *ns, int cap) { return ns_capable_noaudit(ns, cap); }
int lupos_pid_in_egroup(kgid_t gid) { return in_egroup_p(gid); }
kuid_t lupos_pid_make_kuid(struct user_namespace *ns, uid_t uid) { return make_kuid(ns, uid); }
kgid_t lupos_pid_make_kgid(struct user_namespace *ns, gid_t gid) { return make_kgid(ns, gid); }
bool lupos_pid_uid_eq(kuid_t left, kuid_t right) { return uid_eq(left, right); }
bool lupos_pid_uid_valid(kuid_t uid) { return uid_valid(uid); }
bool lupos_pid_gid_valid(kgid_t gid) { return gid_valid(gid); }
/* C callback declarations retain native indirect-call type identities. */
extern struct ctl_table_set *lupos_pid_table_root_lookup(struct ctl_table_root *root);
extern int lupos_pid_set_is_seen(struct ctl_table_set *set);
extern int lupos_pid_table_root_permissions(struct ctl_table_header *head, const struct ctl_table *table);
extern void lupos_pid_table_root_set_ownership(struct ctl_table_header *head, kuid_t *uid, kgid_t *gid);
struct ctl_table_set *lupos_pid_sysctl_lookup(struct ctl_table_root *root) { return lupos_pid_table_root_lookup(root); }
int lupos_pid_sysctl_seen(struct ctl_table_set *set) { return lupos_pid_set_is_seen(set); }
int lupos_pid_sysctl_permissions(struct ctl_table_header *head, const struct ctl_table *table) { return lupos_pid_table_root_permissions(head, table); }
void lupos_pid_sysctl_ownership(struct ctl_table_header *head, kuid_t *uid, kgid_t *gid) { lupos_pid_table_root_set_ownership(head, uid, gid); }
#ifdef CONFIG_CFI
__ADDRESSABLE(proc_dointvec_minmax);
#endif
#endif

/* Architecture syscall ABI and tracing metadata are compiler macro products;
 * every syscall check, lookup and action is in the Rust body. */
extern long lupos_pid_sys_pidfd_open(pid_t pid, unsigned int flags);
SYSCALL_DEFINE2(pidfd_open, pid_t, pid, unsigned int, flags)
{ return lupos_pid_sys_pidfd_open(pid, flags); }
extern long lupos_pid_sys_pidfd_getfd(int pidfd, int fd, unsigned int flags);
SYSCALL_DEFINE3(pidfd_getfd, int, pidfd, int, fd, unsigned int, flags)
{ return lupos_pid_sys_pidfd_getfd(pidfd, fd, flags); }
extern int __init lupos_pid_namespace_sysctl_init(void);
subsys_initcall(lupos_pid_namespace_sysctl_init);

EXPORT_SYMBOL_GPL(init_pid_ns);
EXPORT_SYMBOL_GPL(put_pid);
EXPORT_SYMBOL_GPL(find_pid_ns);
EXPORT_SYMBOL_GPL(find_vpid);
EXPORT_SYMBOL(pid_task);
EXPORT_SYMBOL_GPL(get_task_pid);
EXPORT_SYMBOL_GPL(get_pid_task);
EXPORT_SYMBOL_GPL(find_get_pid);
EXPORT_SYMBOL_GPL(pid_nr_ns);
EXPORT_SYMBOL_GPL(pid_vnr);
EXPORT_SYMBOL(__task_pid_nr_ns);
EXPORT_SYMBOL_GPL(task_active_pid_ns);
EXPORT_SYMBOL_GPL(find_ge_pid);
