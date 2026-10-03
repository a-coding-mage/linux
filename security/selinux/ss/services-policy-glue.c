// SPDX-License-Identifier: GPL-2.0-only
/* Header-inline/macro and fixed-format logging bridges only. */
#include "services-rust.h"
#include "services-policy-glue.h"
#include "policycap_names.h"
#include "netlabel.h"
#include "xfrm.h"
#include "ima.h"
#include "objsec.h"

unsigned int lupos_services_policycap_count(void)
{
	return ARRAY_SIZE(selinux_state.policycap);
}

unsigned int lupos_services_policycap_name_count(void)
{
	return ARRAY_SIZE(selinux_policycap_names);
}

const char *lupos_services_policycap_name(unsigned int index)
{
	return selinux_policycap_names[index];
}

void lupos_services_policycap_write(unsigned int index, bool value)
{
	WRITE_ONCE(selinux_state.policycap[index], value);
}

void lupos_services_policy_assign(struct selinux_policy *policy)
{
	rcu_assign_pointer(selinux_state.policy, policy);
}

void lupos_services_mark_initialized(void)
{
	selinux_mark_initialized();
}

u32 lupos_services_sid_load_acquire(const u32 *sid)
{
	return smp_load_acquire(sid);
}

void lupos_services_sid_store_release(u32 *sid, u32 value)
{
	smp_store_release(sid, value);
}

struct superblock_security_struct *
lupos_services_superblock(const struct super_block *sb)
{
	return selinux_superblock(sb);
}

void lupos_services_netlbl_cache_invalidate(void)
{
	selinux_netlbl_cache_invalidate();
}

void lupos_services_xfrm_notify_policyload(void)
{
	selinux_xfrm_notify_policyload();
}

void lupos_services_ima_measure_state_locked(void)
{
	selinux_ima_measure_state_locked();
}

void lupos_services_log_context_would_invalid(const char *s)
{
	pr_warn("SELinux:  Context %s would be invalid if enforcing\n", s);
}

void lupos_services_log_context_map_error(const char *s, int error)
{
	pr_err("SELinux:   Unable to map context %s, rc = %d.\n", s, error);
}

void lupos_services_log_context_became_valid(const char *s)
{
	pr_info("SELinux:  Context %s became valid (mapped).\n", s);
}

void lupos_services_log_context_became_invalid(const char *s)
{
	pr_info("SELinux:  Context %s became invalid (unmapped).\n", s);
}

void lupos_services_log_initial_sids_lookup_error(void)
{
	pr_err("SELinux:  unable to look up the initial SIDs list\n");
}

void lupos_services_log_policycap(const char *name, int value)
{
	pr_info("SELinux:  policy capability %s=%d\n", name, value);
}

void lupos_services_log_unknown_policycap(unsigned int bit)
{
	pr_info("SELinux:  unknown policy capability %u\n", bit);
}

void lupos_services_log_mls_disable(void)
{
	pr_info("SELinux: Disabling MLS support...\n");
}

void lupos_services_log_mls_enable(void)
{
	pr_info("SELinux: Enabling MLS support...\n");
}

void lupos_services_log_initial_sids_load_error(void)
{
	pr_err("SELinux:  unable to load the initial SIDs\n");
}

void lupos_services_log_preserve_bools_error(void)
{
	pr_err("SELinux:  unable to preserve booleans\n");
}

void lupos_services_log_convert_contexts_error(void)
{
	pr_err("SELinux:  unable to convert the internal representation of contexts in the new SID table\n");
}
