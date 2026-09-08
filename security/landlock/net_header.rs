/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock LSM - Network management and hooks
 *
 * Copyright © 2022-2023 Huawei Tech. Co., Ltd.
 */

/* Dependencies in the original header:
 * common.h
 * ruleset.h
 * setup.h
 */

/* Original condition: IS_ENABLED(CONFIG_INET) */
#[cfg(CONFIG_INET)]
unsafe extern "C" {
    /* __init */
    pub fn landlock_add_net_hooks();

    pub fn landlock_append_net_rule(
        ruleset: *const landlock_ruleset,
        port: u16,
        access_rights: access_mask_t,
        flags: u32,
    ) -> core::ffi::c_int;
}

/* Original fallback: !IS_ENABLED(CONFIG_INET) */
#[cfg(not(CONFIG_INET))]
#[inline]
pub fn landlock_add_net_hooks() {}

#[cfg(not(CONFIG_INET))]
#[inline]
pub fn landlock_append_net_rule(
    ruleset: *const landlock_ruleset,
    port: u16,
    access_rights: access_mask_t,
    flags: u32,
) -> core::ffi::c_int {
    let _ = ruleset;
    let _ = port;
    let _ = access_rights;
    let _ = flags;

    -(EAFNOSUPPORT as core::ffi::c_int)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
