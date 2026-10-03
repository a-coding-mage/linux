/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SELINUX_SERVICES_RUST_H
#define LUPOS_SELINUX_SERVICES_RUST_H
/* services.o is built in; canonical bindgen otherwise adds -DMODULE. */
#ifdef MODULE
#define LUPOS_SERVICES_RESTORE_MODULE
#undef MODULE
#endif
#include <linux/kernel.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <linux/spinlock.h>
#include <linux/rcupdate.h>
#include <linux/errno.h>
#include <linux/in.h>
#include <linux/sched.h>
#include <linux/audit.h>
#include <linux/parser.h>
#include <linux/vmalloc.h>
#include <linux/lsm_hooks.h>
#include <net/netlabel.h>
#include "flask.h"
#include "avc.h"
#include "avc_ss.h"
#include "security.h"
#include "context.h"
#include "policydb.h"
#include "sidtab.h"
#include "services.h"
#include "conditional.h"
#include "mls.h"
#include "objsec.h"
#include "netlabel.h"
#include "xfrm.h"
#include "ebitmap.h"
#include "audit.h"
#include "ima.h"

/* C compiler ABI values, compared with generated Rust types at build time. */
enum {
 LUPOS_SERVICES_SIZE_CONTEXT = sizeof(struct context),
 LUPOS_SERVICES_ALIGN_CONTEXT = __alignof__(struct context),
 LUPOS_SERVICES_OFFSET_CONTEXT_USER = offsetof(struct context, user),
 LUPOS_SERVICES_OFFSET_CONTEXT_ROLE = offsetof(struct context, role),
 LUPOS_SERVICES_OFFSET_CONTEXT_TYPE = offsetof(struct context, type),
 LUPOS_SERVICES_OFFSET_CONTEXT_LEN = offsetof(struct context, len),
 LUPOS_SERVICES_OFFSET_CONTEXT_RANGE = offsetof(struct context, range),
 LUPOS_SERVICES_OFFSET_CONTEXT_STR = offsetof(struct context, str),
 LUPOS_SERVICES_SIZE_POLICYDB = sizeof(struct policydb),
 LUPOS_SERVICES_ALIGN_POLICYDB = __alignof__(struct policydb),
 LUPOS_SERVICES_OFFSET_POLICYDB_MLS_ENABLED = offsetof(struct policydb, mls_enabled),
 LUPOS_SERVICES_OFFSET_POLICYDB_SYMTAB = offsetof(struct policydb, symtab),
 LUPOS_SERVICES_OFFSET_POLICYDB_CLASS_VAL_TO_STRUCT = offsetof(struct policydb, class_val_to_struct),
 LUPOS_SERVICES_OFFSET_POLICYDB_TYPE_VAL_TO_STRUCT = offsetof(struct policydb, type_val_to_struct),
 LUPOS_SERVICES_OFFSET_POLICYDB_TE_AVTAB = offsetof(struct policydb, te_avtab),
 LUPOS_SERVICES_OFFSET_POLICYDB_TE_COND_AVTAB = offsetof(struct policydb, te_cond_avtab),
 LUPOS_SERVICES_OFFSET_POLICYDB_TYPE_ATTR_MAP_ARRAY = offsetof(struct policydb, type_attr_map_array),
 LUPOS_SERVICES_OFFSET_POLICYDB_POLICYCAPS = offsetof(struct policydb, policycaps),
 LUPOS_SERVICES_OFFSET_POLICYDB_LEN = offsetof(struct policydb, len),
 LUPOS_SERVICES_OFFSET_POLICYDB_PROCESS_CLASS = offsetof(struct policydb, process_class),
 LUPOS_SERVICES_SIZE_SELINUX_POLICY = sizeof(struct selinux_policy),
 LUPOS_SERVICES_ALIGN_SELINUX_POLICY = __alignof__(struct selinux_policy),
 LUPOS_SERVICES_OFFSET_SELINUX_POLICY_SIDTAB = offsetof(struct selinux_policy, sidtab),
 LUPOS_SERVICES_OFFSET_SELINUX_POLICY_POLICYDB = offsetof(struct selinux_policy, policydb),
 LUPOS_SERVICES_OFFSET_SELINUX_POLICY_MAP = offsetof(struct selinux_policy, map),
 LUPOS_SERVICES_OFFSET_SELINUX_POLICY_LATEST_GRANTING = offsetof(struct selinux_policy, latest_granting),
 LUPOS_SERVICES_SIZE_SELINUX_MAP = sizeof(struct selinux_map),
 LUPOS_SERVICES_ALIGN_SELINUX_MAP = __alignof__(struct selinux_map),
 LUPOS_SERVICES_SIZE_SIDTAB = sizeof(struct sidtab),
 LUPOS_SERVICES_ALIGN_SIDTAB = __alignof__(struct sidtab),
 LUPOS_SERVICES_SIZE_SIDTAB_ENTRY = sizeof(struct sidtab_entry),
 LUPOS_SERVICES_ALIGN_SIDTAB_ENTRY = __alignof__(struct sidtab_entry),
 LUPOS_SERVICES_OFFSET_SIDTAB_ENTRY_CONTEXT = offsetof(struct sidtab_entry, context),
 LUPOS_SERVICES_SIZE_AV_DECISION = sizeof(struct av_decision),
 LUPOS_SERVICES_ALIGN_AV_DECISION = __alignof__(struct av_decision),
 LUPOS_SERVICES_SIZE_AVTAB_KEY = sizeof(struct avtab_key),
 LUPOS_SERVICES_ALIGN_AVTAB_KEY = __alignof__(struct avtab_key),
 LUPOS_SERVICES_OFFSET_AVTAB_KEY_SOURCE_TYPE = offsetof(struct avtab_key, source_type),
 LUPOS_SERVICES_OFFSET_AVTAB_KEY_TARGET_TYPE = offsetof(struct avtab_key, target_type),
 LUPOS_SERVICES_OFFSET_AVTAB_KEY_TARGET_CLASS = offsetof(struct avtab_key, target_class),
 LUPOS_SERVICES_OFFSET_AVTAB_KEY_SPECIFIED = offsetof(struct avtab_key, specified),
 LUPOS_SERVICES_SIZE_EXTENDED_PERMS = sizeof(struct extended_perms),
 LUPOS_SERVICES_ALIGN_EXTENDED_PERMS = __alignof__(struct extended_perms),
 LUPOS_SERVICES_OFFSET_EXTENDED_PERMS_LEN = offsetof(struct extended_perms, len),
 LUPOS_SERVICES_OFFSET_EXTENDED_PERMS_BASE_PERMS = offsetof(struct extended_perms, base_perms),
 LUPOS_SERVICES_OFFSET_EXTENDED_PERMS_DRIVERS = offsetof(struct extended_perms, drivers),
 LUPOS_SERVICES_SIZE_EXTENDED_PERMS_DECISION = sizeof(struct extended_perms_decision),
 LUPOS_SERVICES_ALIGN_EXTENDED_PERMS_DECISION = __alignof__(struct extended_perms_decision),
 LUPOS_SERVICES_SIZE_SELINUX_LOAD_STATE = sizeof(struct selinux_load_state),
 LUPOS_SERVICES_ALIGN_SELINUX_LOAD_STATE = __alignof__(struct selinux_load_state),
 LUPOS_SERVICES_OFFSET_SELINUX_LOAD_STATE_POLICY = offsetof(struct selinux_load_state, policy),
 LUPOS_SERVICES_OFFSET_SELINUX_LOAD_STATE_CONVERT_DATA = offsetof(struct selinux_load_state, convert_data),
};

/* Values requiring configured macro evaluation, rather than guessed flags. */
enum {
 LUPOS_SERVICES_GFP_ATOMIC = GFP_ATOMIC,
 LUPOS_SERVICES_GFP_KERNEL = GFP_KERNEL,
};
struct selinux_audit_rule { u32 au_seqno; struct context au_ctxt; };
void lupos_services_rcu_read_lock(void);
void lupos_services_rcu_read_unlock(void);
struct selinux_policy *lupos_services_policy_security_mls_enabled(void);
struct selinux_policy *lupos_services_policy_security_compute_validatetrans(void);
struct selinux_policy *lupos_services_policy_security_bounded_transition(void);
struct selinux_policy *lupos_services_policy_security_compute_xperms_decision(void);
struct selinux_policy *lupos_services_policy_security_compute_av(void);
struct selinux_policy *lupos_services_policy_security_compute_av_user(void);
struct selinux_policy *lupos_services_policy_security_sidtab_hash_stats(void);
struct selinux_policy *lupos_services_policy_security_sid_to_context_core(void);
struct selinux_policy *lupos_services_policy_security_context_to_sid_core(void);
struct selinux_policy *lupos_services_policy_security_compute_sid(void);
struct selinux_policy *lupos_services_policy_security_port_sid(void);
struct selinux_policy *lupos_services_policy_security_ib_pkey_sid(void);
struct selinux_policy *lupos_services_policy_security_ib_endport_sid(void);
struct selinux_policy *lupos_services_policy_security_netif_sid(void);
struct selinux_policy *lupos_services_policy_security_node_sid(void);
struct selinux_policy *lupos_services_policy_security_genfs_sid(void);
struct selinux_policy *lupos_services_policy_security_fs_use(void);
struct selinux_policy *lupos_services_policy_security_get_bool_value(void);
struct selinux_policy *lupos_services_policy_security_sid_mls_copy(void);
struct selinux_policy *lupos_services_policy_security_net_peersid_resolve(void);
struct selinux_policy *lupos_services_policy_security_get_reject_unknown(void);
struct selinux_policy *lupos_services_policy_security_get_allow_unknown(void);
struct selinux_policy *lupos_services_policy_security_policycap_supported(void);
struct selinux_policy *lupos_services_policy_selinux_audit_rule_init(void);
struct selinux_policy *lupos_services_policy_selinux_audit_rule_match(void);
struct selinux_policy *lupos_services_policy_security_netlbl_secattr_to_sid(void);
struct selinux_policy *lupos_services_policy_security_netlbl_sid_to_secattr(void);
struct selinux_policy *lupos_services_policy_locked_selinux_policy_cancel(void);
struct selinux_policy *lupos_services_policy_locked_selinux_policy_commit(void);
struct selinux_policy *lupos_services_policy_locked_security_load_policy(void);
struct selinux_policy *lupos_services_policy_locked_security_set_bools(void);
struct selinux_policy *lupos_services_policy_locked_security_read_policy(void);
struct selinux_policy *lupos_services_policy_locked_security_read_state_kernel(void);

bool lupos_services_initialized(void);
bool lupos_services_enforcing(void);
void *lupos_services_kmalloc(size_t size, gfp_t flags);
void *lupos_services_kzalloc(size_t size, gfp_t flags);
void *lupos_services_kcalloc(size_t count, size_t size, gfp_t flags);
void *lupos_services_kmemdup(const void *p, size_t size, gfp_t flags);
void *lupos_services_kmemdup_nul(const void *p, size_t size, gfp_t flags);
char *lupos_services_kstrdup(const char *s, gfp_t flags);
void lupos_services_context_init(struct context *c);
void lupos_services_context_destroy(struct context *c);
int lupos_services_context_copy(struct context *d, const struct context *s);
bool lupos_services_context_equal(const struct context *a, const struct context *b);
int lupos_services_mls_context_cpy(struct context *d, const struct context *s);
bool lupos_services_mls_context_equal(const struct context *a, const struct context *b);
bool lupos_services_mls_level_eq(const struct mls_level *a, const struct mls_level *b);
bool lupos_services_mls_level_dom(const struct mls_level *a, const struct mls_level *b);
bool lupos_services_mls_level_incomp(const struct mls_level *a, const struct mls_level *b);
struct context *lupos_services_sidtab_search(struct sidtab *s, u32 sid);
const char *lupos_services_sym_name(const struct policydb *p, unsigned int sym, unsigned int i);
const char *lupos_services_initial_sid_string(u32 sid);
const struct security_class_mapping *lupos_services_secclass_map(void);
int lupos_services_sid2str_get(struct sidtab *s, struct sidtab_entry *e, char **out, u32 *len);
void lupos_services_sid2str_put(struct sidtab *s, struct sidtab_entry *e, const char *str, u32 len);
u32 lupos_services_ebitmap_start_positive(const struct ebitmap *e, struct ebitmap_node **n);
u32 lupos_services_ebitmap_next_positive(const struct ebitmap *e, struct ebitmap_node **n, u32 bit);
struct audit_context *lupos_services_audit_context(void);
bool lupos_services_is_socket_class(u16 c);
void lupos_services_log_hash_before_load(void);
void lupos_services_log_sid_before_load(u32 sid);
void lupos_services_log_unrecognized(const char *func, u32 sid);
void lupos_services_log_bounded_unrecognized(u32 sid);
void lupos_services_log_mapping_class(const char *name);
void lupos_services_log_mapping_permission(const char *perm, const char *class);
void lupos_services_log_mapping_unknown(bool allow);
void lupos_services_log_invalid_class_av(u16 c);
void lupos_services_log_invalid_class_xperms(u16 c);
void lupos_services_log_unknown_xperm(u8 value);
void lupos_services_log_unknown_key(u16 value);
void lupos_services_bug_constraint_not(int sp);
void lupos_services_bug_constraint_and(int sp);
void lupos_services_bug_constraint_or(int sp);
void lupos_services_bug_constraint_end(int sp);
void lupos_services_bug_bounds_source(const struct type_datum *p);
void lupos_services_bug_bounds_target(const struct type_datum *p);
void lupos_services_bug_bounded_type(const struct type_datum *p);
void lupos_services_bug_constraint_mls(void);
void lupos_services_bug_constraint_attr(void);
void lupos_services_bug_constraint_op(void);
void lupos_services_bug_constraint_xcontext(void);
void lupos_services_bug_constraint_names_attr(void);
void lupos_services_bug_constraint_names_op(void);
void lupos_services_bug_constraint_expr(void);
void lupos_services_audit_sid_invalid_prefix(struct audit_buffer *ab);
void lupos_services_audit_sid_invalid_suffix(struct audit_buffer *ab, const char *s, const char *t, const char *c);
void lupos_services_audit_masked_prefix(struct audit_buffer *ab, const char *r, const char *s, const char *t, const char *c);
void lupos_services_audit_masked_permission(struct audit_buffer *ab, const char *sep, const char *p);
void lupos_services_audit_validtrans(const char *o, const char *n, const char *t, const char *c);
void lupos_services_audit_bounded(const char *o, const char *n);
#include "services-policy-glue.h"
#include "services-query-glue.h"
#ifdef LUPOS_SERVICES_RESTORE_MODULE
#define MODULE
#undef LUPOS_SERVICES_RESTORE_MODULE
#endif
#endif
