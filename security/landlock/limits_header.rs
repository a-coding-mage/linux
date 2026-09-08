/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock - Limits for different components
 *
 * Copyright (C) 2016-2020 Mickael Salaun <mic@digikod.net>
 * Copyright (C) 2018-2020 ANSSI
 * Copyright (C) 2021-2025 Microsoft Corporation
 */

/* C header dependencies:
 * #include <linux/bitops.h>
 * #include <linux/limits.h>
 * #include <uapi/linux/landlock.h>
 */

const fn __const_hweight64(value: u64) -> u32 {
    value.count_ones()
}

const fn MAX(a: u32, b: u32) -> u32 {
    if a > b { a } else { b }
}

pub const LANDLOCK_MAX_NUM_LAYERS: u32 = 16;
pub const LANDLOCK_MAX_NUM_RULES: u32 = u32::MAX;

pub const LANDLOCK_LAST_ACCESS_FS: u64 = LANDLOCK_ACCESS_FS_RESOLVE_UNIX as u64;
pub const LANDLOCK_MASK_ACCESS_FS: u64 = (LANDLOCK_LAST_ACCESS_FS << 1) - 1;
pub const LANDLOCK_NUM_ACCESS_FS: u32 = __const_hweight64(LANDLOCK_MASK_ACCESS_FS);

pub const LANDLOCK_LAST_ACCESS_NET: u64 = LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP as u64;
pub const LANDLOCK_MASK_ACCESS_NET: u64 = (LANDLOCK_LAST_ACCESS_NET << 1) - 1;
pub const LANDLOCK_NUM_ACCESS_NET: u32 = __const_hweight64(LANDLOCK_MASK_ACCESS_NET);

pub const LANDLOCK_LAST_SCOPE: u64 = LANDLOCK_SCOPE_SIGNAL as u64;
pub const LANDLOCK_MASK_SCOPE: u64 = (LANDLOCK_LAST_SCOPE << 1) - 1;
pub const LANDLOCK_NUM_SCOPE: u32 = __const_hweight64(LANDLOCK_MASK_SCOPE);

pub const LANDLOCK_NUM_ACCESS_MAX: u32 = MAX(
    MAX(LANDLOCK_NUM_ACCESS_FS, LANDLOCK_NUM_ACCESS_NET),
    LANDLOCK_NUM_SCOPE,
);

pub const LANDLOCK_LAST_RESTRICT_SELF: u64 = LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS as u64;
pub const LANDLOCK_MASK_RESTRICT_SELF: u64 = (LANDLOCK_LAST_RESTRICT_SELF << 1) - 1;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
