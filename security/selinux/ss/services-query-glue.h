/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef _SELINUX_SERVICES_QUERY_RUST_GLUE_H
#define _SELINUX_SERVICES_QUERY_RUST_GLUE_H
struct audit_buffer *lupos_services_audit_start(gfp_t gfp, int kind);
void lupos_services_audit_end(struct audit_buffer *ab);
void lupos_services_audit_untrusted(struct audit_buffer *ab, const char *s, size_t len);
void lupos_services_audit_bool_change(const char *name, int new_state, int old_state);
void lupos_services_audit_mls_invalid(struct audit_buffer *ab);
bool lupos_services_warn_bool_count(bool mismatch);
void lupos_services_warn_audit_rule_missing(void);
void lupos_services_warn_audit_sid(u32 sid);
void lupos_services_log_unrecognized_class(const char *name);
unsigned int lupos_services_policydb_reject_unknown(const struct policydb *p);
unsigned int lupos_services_policydb_allow_unknown(const struct policydb *p);
void *lupos_services_vmalloc(unsigned long size);
void *lupos_services_vmalloc_user(unsigned long size);
#ifdef CONFIG_NETLABEL
struct netlbl_lsm_cache *lupos_services_netlbl_cache_alloc(gfp_t flags);
#endif
#endif
