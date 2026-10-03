// SPDX-License-Identifier: GPL-2.0-only
/* Macro/header-inline/diagnostic bridging only. No services.c algorithm. */
#include "services-rust.h"
#include "initial_sid_to_string.h"
void lupos_services_rcu_read_lock(void) { rcu_read_lock(); }
void lupos_services_rcu_read_unlock(void) { rcu_read_unlock(); }
bool lupos_services_initialized(void) { return selinux_initialized(); }
bool lupos_services_enforcing(void) { return enforcing_enabled(); }
void *lupos_services_kmalloc(size_t n, gfp_t f) { return kmalloc(n, f); }
void *lupos_services_kzalloc(size_t n, gfp_t f) { return kzalloc(n, f); }
void *lupos_services_kcalloc(size_t n, size_t s, gfp_t f) { return kcalloc(n, s, f); }
void *lupos_services_kmemdup(const void *p, size_t n, gfp_t f) { return kmemdup(p, n, f); }
void *lupos_services_kmemdup_nul(const void *p, size_t n, gfp_t f) { return kmemdup_nul(p, n, f); }
char *lupos_services_kstrdup(const char *p, gfp_t f) { return kstrdup(p, f); }
void lupos_services_context_init(struct context *c) { context_init(c); }
void lupos_services_context_destroy(struct context *c) { context_destroy(c); }
int lupos_services_context_copy(struct context *d, const struct context *s) { return context_cpy(d, s); }
bool lupos_services_context_equal(const struct context *a, const struct context *b) { return context_equal(a, b); }
int lupos_services_mls_context_cpy(struct context *d, const struct context *s) { return mls_context_cpy(d, s); }
bool lupos_services_mls_context_equal(const struct context *a, const struct context *b) { return mls_context_equal(a, b); }
bool lupos_services_mls_level_eq(const struct mls_level *a, const struct mls_level *b) { return mls_level_eq(a, b); }
bool lupos_services_mls_level_dom(const struct mls_level *a, const struct mls_level *b) { return mls_level_dom(a, b); }
bool lupos_services_mls_level_incomp(const struct mls_level *a, const struct mls_level *b) { return mls_level_incomp(a, b); }
struct context *lupos_services_sidtab_search(struct sidtab *s, u32 sid) { return sidtab_search(s, sid); }
const char *lupos_services_sym_name(const struct policydb *p, unsigned int s, unsigned int i) { return sym_name(p, s, i); }
const char *lupos_services_initial_sid_string(u32 sid) { return initial_sid_to_string[sid]; }
const struct security_class_mapping *lupos_services_secclass_map(void) { return secclass_map; }
int lupos_services_sid2str_get(struct sidtab *s, struct sidtab_entry *e, char **o, u32 *l) { return sidtab_sid2str_get(s, e, o, l); }
void lupos_services_sid2str_put(struct sidtab *s, struct sidtab_entry *e, const char *t, u32 l) { sidtab_sid2str_put(s, e, t, l); }
u32 lupos_services_ebitmap_start_positive(const struct ebitmap *e, struct ebitmap_node **n) { return ebitmap_start_positive(e, n); }
u32 lupos_services_ebitmap_next_positive(const struct ebitmap *e, struct ebitmap_node **n, u32 b) { return ebitmap_next_positive(e, n, b); }
struct audit_context *lupos_services_audit_context(void) { return audit_context(); }
bool lupos_services_is_socket_class(u16 c) { return security_is_socket_class(c); }
void lupos_services_log_hash_before_load(void) { pr_err("SELinux: %s:  called before initial load_policy\n", "security_sidtab_hash_stats"); }
void lupos_services_log_sid_before_load(u32 sid) { pr_err("SELinux: %s:  called before initial load_policy on unknown SID %d\n", "security_sid_to_context_core", sid); }
void lupos_services_log_unrecognized(const char *f, u32 sid) { pr_err("SELinux: %s:  unrecognized SID %d\n", f, sid); }
void lupos_services_log_bounded_unrecognized(u32 sid) { pr_err("SELinux: %s: unrecognized SID %u\n", "security_bounded_transition", sid); }
void lupos_services_log_mapping_class(const char *n) { pr_info("SELinux:  Class %s not defined in policy.\n", n); }
void lupos_services_log_mapping_permission(const char *p, const char *c) { pr_info("SELinux:  Permission %s in class %s not defined in policy.\n", p, c); }
void lupos_services_log_mapping_unknown(bool allow) { pr_info("SELinux: the above unknown classes and permissions will be %s\n", allow ? "allowed" : "denied"); }
void lupos_services_log_invalid_class_av(u16 c) { pr_warn_ratelimited("SELinux:  Invalid class %u\n", c); }
void lupos_services_log_invalid_class_xperms(u16 c) { pr_warn_ratelimited("SELinux:  Invalid class %hu\n", c); }
void lupos_services_log_unknown_xperm(u8 v) { pr_warn_once("SELinux: unknown extended permission (%u) will be ignored\n", v); }
void lupos_services_log_unknown_key(u16 v) { pr_warn_once("SELinux: unknown specified key (%u)\n", v); }
/* Separate BUG sites preserve the original guard and fatal behavior. */
void lupos_services_bug_constraint_not(int sp) { BUG_ON(sp < 0); }
void lupos_services_bug_constraint_and(int sp) { BUG_ON(sp < 1); }
void lupos_services_bug_constraint_or(int sp) { BUG_ON(sp < 1); }
void lupos_services_bug_constraint_end(int sp) { BUG_ON(sp != 0); }
void lupos_services_bug_bounds_source(const struct type_datum *p) { BUG_ON(!p); }
void lupos_services_bug_bounds_target(const struct type_datum *p) { BUG_ON(!p); }
void lupos_services_bug_bounded_type(const struct type_datum *p) { BUG_ON(!p); }
void __noreturn lupos_services_bug_constraint_mls(void) { BUG(); }
void __noreturn lupos_services_bug_constraint_attr(void) { BUG(); }
void __noreturn lupos_services_bug_constraint_op(void) { BUG(); }
void __noreturn lupos_services_bug_constraint_xcontext(void) { BUG(); }
void __noreturn lupos_services_bug_constraint_names_attr(void) { BUG(); }
void __noreturn lupos_services_bug_constraint_names_op(void) { BUG(); }
void __noreturn lupos_services_bug_constraint_expr(void) { BUG(); }
void lupos_services_audit_sid_invalid_prefix(struct audit_buffer *ab)
{ audit_log_format(ab, "op=security_compute_sid invalid_context="); }
void lupos_services_audit_sid_invalid_suffix(struct audit_buffer *ab, const char *s, const char *t, const char *c)
{ audit_log_format(ab, " scontext=%s tcontext=%s tclass=%s", s, t, c); }
void lupos_services_audit_masked_prefix(struct audit_buffer *ab, const char *r, const char *s, const char *t, const char *c)
{ audit_log_format(ab, "op=security_compute_av reason=%s scontext=%s tcontext=%s tclass=%s perms=", r, s, t, c); }
void lupos_services_audit_masked_permission(struct audit_buffer *ab, const char *sep, const char *p)
{ audit_log_format(ab, "%s%s", sep, p); }
void lupos_services_audit_validtrans(const char *o, const char *n, const char *t, const char *c)
{ audit_log(audit_context(), GFP_ATOMIC, AUDIT_SELINUX_ERR, "op=security_validate_transition seresult=denied oldcontext=%s newcontext=%s taskcontext=%s tclass=%s", o, n, t, c); }
void lupos_services_audit_bounded(const char *o, const char *n)
{ audit_log(audit_context(), GFP_ATOMIC, AUDIT_SELINUX_ERR, "op=security_bounded_transition seresult=denied oldcontext=%s newcontext=%s", o, n); }

/* One expansion per original policy-read site preserves RCU lockdep warning state. */
struct selinux_policy *lupos_services_policy_security_mls_enabled(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_compute_validatetrans(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_bounded_transition(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_compute_xperms_decision(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_compute_av(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_compute_av_user(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_sidtab_hash_stats(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_sid_to_context_core(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_context_to_sid_core(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_compute_sid(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_port_sid(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_ib_pkey_sid(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_ib_endport_sid(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_netif_sid(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_node_sid(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_genfs_sid(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_fs_use(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_get_bool_value(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_sid_mls_copy(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_net_peersid_resolve(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_get_reject_unknown(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_get_allow_unknown(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_policycap_supported(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_selinux_audit_rule_init(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_selinux_audit_rule_match(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_netlbl_secattr_to_sid(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_security_netlbl_sid_to_secattr(void)
{ return rcu_dereference(selinux_state.policy); }
struct selinux_policy *lupos_services_policy_locked_selinux_policy_cancel(void)
{ return rcu_dereference_protected(selinux_state.policy, lockdep_is_held(&selinux_state.policy_mutex)); }
struct selinux_policy *lupos_services_policy_locked_selinux_policy_commit(void)
{ return rcu_dereference_protected(selinux_state.policy, lockdep_is_held(&selinux_state.policy_mutex)); }
struct selinux_policy *lupos_services_policy_locked_security_load_policy(void)
{ return rcu_dereference_protected(selinux_state.policy, lockdep_is_held(&selinux_state.policy_mutex)); }
struct selinux_policy *lupos_services_policy_locked_security_set_bools(void)
{ return rcu_dereference_protected(selinux_state.policy, lockdep_is_held(&selinux_state.policy_mutex)); }
struct selinux_policy *lupos_services_policy_locked_security_read_policy(void)
{ return rcu_dereference_protected(selinux_state.policy, lockdep_is_held(&selinux_state.policy_mutex)); }
struct selinux_policy *lupos_services_policy_locked_security_read_state_kernel(void)
{ return rcu_dereference_protected(selinux_state.policy, lockdep_is_held(&selinux_state.policy_mutex)); }
