/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef _SELINUX_SERVICES_POLICY_RUST_GLUE_H
#define _SELINUX_SERVICES_POLICY_RUST_GLUE_H

#include "services.h"
#include "sidtab.h"

struct super_block;
struct superblock_security_struct;

/* This services.c-local type is exposed only to configured bindgen. */
struct selinux_policy_convert_data {
	struct convert_context_args args;
	struct sidtab_convert_params sidtab_params;
};

unsigned int lupos_services_policycap_count(void);
unsigned int lupos_services_policycap_name_count(void);
const char *lupos_services_policycap_name(unsigned int index);
void lupos_services_policycap_write(unsigned int index, bool value);
void lupos_services_policy_assign(struct selinux_policy *policy);
void lupos_services_mark_initialized(void);
u32 lupos_services_sid_load_acquire(const u32 *sid);
void lupos_services_sid_store_release(u32 *sid, u32 value);
struct superblock_security_struct *
lupos_services_superblock(const struct super_block *sb);
void lupos_services_netlbl_cache_invalidate(void);
void lupos_services_xfrm_notify_policyload(void);
void lupos_services_ima_measure_state_locked(void);

void lupos_services_log_context_would_invalid(const char *s);
void lupos_services_log_context_map_error(const char *s, int error);
void lupos_services_log_context_became_valid(const char *s);
void lupos_services_log_context_became_invalid(const char *s);
void lupos_services_log_initial_sids_lookup_error(void);
void lupos_services_log_policycap(const char *name, int value);
void lupos_services_log_unknown_policycap(unsigned int bit);
void lupos_services_log_mls_disable(void);
void lupos_services_log_mls_enable(void);
void lupos_services_log_initial_sids_load_error(void);
void lupos_services_log_preserve_bools_error(void);
void lupos_services_log_convert_contexts_error(void);

#endif
