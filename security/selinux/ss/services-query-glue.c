// SPDX-License-Identifier: GPL-2.0-only
/* Original header macros/inlines, bitfields and fixed diagnostics only. */
#include "services-rust.h"
#include "services-query-glue.h"

struct audit_buffer *lupos_services_audit_start(gfp_t gfp, int kind)
{
	return audit_log_start(audit_context(), gfp, kind);
}

void lupos_services_audit_end(struct audit_buffer *ab)
{
	audit_log_end(ab);
}

void lupos_services_audit_untrusted(struct audit_buffer *ab, const char *s, size_t len)
{
	audit_log_n_untrustedstring(ab, s, len);
}

void lupos_services_audit_bool_change(const char *name, int new_state, int old_state)
{
	audit_log(audit_context(), GFP_ATOMIC, AUDIT_MAC_CONFIG_CHANGE,
		  "bool=%s val=%d old_val=%d auid=%u ses=%u", name,
		  new_state, old_state,
		  from_kuid(&init_user_ns, audit_get_loginuid(current)),
		  audit_get_sessionid(current));
}

void lupos_services_audit_mls_invalid(struct audit_buffer *ab)
{
	audit_log_format(ab, "op=security_sid_mls_copy invalid_context=");
}

bool lupos_services_warn_bool_count(bool mismatch)
{
	return WARN_ON(mismatch);
}

void lupos_services_warn_audit_rule_missing(void)
{
	WARN_ONCE(1, "selinux_audit_rule_match: missing rule\n");
}

void lupos_services_warn_audit_sid(u32 sid)
{
	WARN_ONCE(1, "selinux_audit_rule_match: unrecognized SID %d\n", sid);
}

void lupos_services_log_unrecognized_class(const char *name)
{
	pr_err("SELinux: %s:  unrecognized class %s\n", "security_get_permissions", name);
}

unsigned int lupos_services_policydb_reject_unknown(const struct policydb *p)
{
	return p->reject_unknown;
}

unsigned int lupos_services_policydb_allow_unknown(const struct policydb *p)
{
	return p->allow_unknown;
}

void *lupos_services_vmalloc(unsigned long size)
{
	return vmalloc(size);
}

void *lupos_services_vmalloc_user(unsigned long size)
{
	return vmalloc_user(size);
}

#ifdef CONFIG_NETLABEL
struct netlbl_lsm_cache *lupos_services_netlbl_cache_alloc(gfp_t flags)
{
	return netlbl_secattr_cache_alloc(flags);
}
#endif
