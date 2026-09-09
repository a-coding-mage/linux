/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const IFE_METAHDRLEN: i32 = 2;

pub const IFE_META_SKBMARK: i32 = 1;
pub const IFE_META_HASHID: i32 = IFE_META_SKBMARK + 1;
pub const IFE_META_PRIO: i32 = IFE_META_HASHID + 1;
pub const IFE_META_QMAP: i32 = IFE_META_PRIO + 1;
pub const IFE_META_TCINDEX: i32 = IFE_META_QMAP + 1;
pub const __IFE_META_MAX: i32 = IFE_META_TCINDEX + 1;

/* Can be overridden at runtime by module option */
pub const IFE_META_MAX: i32 = __IFE_META_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
