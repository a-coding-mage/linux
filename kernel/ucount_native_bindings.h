/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_UCOUNT_NATIVE_BINDINGS_H
#define LUPOS_UCOUNT_NATIVE_BINDINGS_H

/* Inactive owner-local proposal. Configured native declarations are the only
 * authority for complete types, integer widths, atomics, fields and constants.
 * Rust uses only pointers to aggregate objects except for native kuid_t.
 */
#include <linux/atomic.h>
#include <linux/stat.h>
#include <linux/sysctl.h>
#include <linux/slab.h>
#include <linux/cred.h>
#include <linux/user_namespace.h>

/* These are owner-local storage dimensions from original ucount.c:18-19. */
#define UCOUNTS_HASHTABLE_BITS 10
#define UCOUNTS_HASHTABLE_ENTRIES (1 << UCOUNTS_HASHTABLE_BITS)
static const unsigned int LUPOS_UCOUNT_HASHTABLE_BITS = UCOUNTS_HASHTABLE_BITS;
static const unsigned int LUPOS_UCOUNT_HASHTABLE_ENTRIES = UCOUNTS_HASHTABLE_ENTRIES;
static const unsigned int LUPOS_UCOUNT_COUNTS = UCOUNT_COUNTS;
static const long LUPOS_UCOUNT_LONG_MAX = LONG_MAX;
#ifdef CONFIG_SYSCTL
static const int LUPOS_UCOUNT_S_IRWXU = S_IRWXU;
static const int LUPOS_UCOUNT_S_IROTH = S_IROTH;
#endif

/* All indirect-call/public entries are defined with exact C signatures in the
 * companion. These direct-call Rust bodies do not take callback-valued ABI
 * arguments. No guessed Rust Option<fn> CFI encoding is introduced.
 */
bool lupos_ucount_setup_userns_sysctls(struct user_namespace *ns);
void lupos_ucount_retire_userns_sysctls(struct user_namespace *ns);
struct ucounts *lupos_ucount_alloc_ucounts(struct user_namespace *ns, kuid_t uid);
void lupos_ucount_put_ucounts(struct ucounts *ucounts);
struct ucounts *lupos_ucount_inc_ucount(struct user_namespace *ns, kuid_t uid,
				      enum ucount_type type);
void lupos_ucount_dec_ucount(struct ucounts *ucounts, enum ucount_type type);
long lupos_ucount_inc_rlimit_ucounts(struct ucounts *ucounts,
				   enum rlimit_type type, long v);
bool lupos_ucount_dec_rlimit_ucounts(struct ucounts *ucounts,
				   enum rlimit_type type, long v);
void lupos_ucount_dec_rlimit_put_ucounts(struct ucounts *ucounts,
				      enum rlimit_type type);
long lupos_ucount_inc_rlimit_get_ucounts(struct ucounts *ucounts,
				      enum rlimit_type type, bool override_rlimit);
bool lupos_ucount_is_rlimit_overlimit(struct ucounts *ucounts,
				    enum rlimit_type type, unsigned long rlimit);
int lupos_ucount_user_namespace_sysctl_init(void);
#ifdef CONFIG_SYSCTL
struct ctl_table_set *lupos_ucount_set_lookup(struct ctl_table_root *root);
int lupos_ucount_set_is_seen(struct ctl_table_set *set);
int lupos_ucount_set_permissions(struct ctl_table_header *head,
				const struct ctl_table *table);
#endif

/* One inventory supplies native declarations and primitive definitions. */
#define UCOUNT_LEAF(ret, name, args, body) ret lupos_ucount_##name args;
#include "ucount_native_primitives.inc"
#undef UCOUNT_LEAF

#endif /* LUPOS_UCOUNT_NATIVE_BINDINGS_H */
