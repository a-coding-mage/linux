/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const ARCH_SET_GS: i32 = 0x1001;
pub const ARCH_SET_FS: i32 = 0x1002;
pub const ARCH_GET_FS: i32 = 0x1003;
pub const ARCH_GET_GS: i32 = 0x1004;

pub const ARCH_GET_CPUID: i32 = 0x1011;
pub const ARCH_SET_CPUID: i32 = 0x1012;

pub const ARCH_GET_XCOMP_SUPP: i32 = 0x1021;
pub const ARCH_GET_XCOMP_PERM: i32 = 0x1022;
pub const ARCH_REQ_XCOMP_PERM: i32 = 0x1023;
pub const ARCH_GET_XCOMP_GUEST_PERM: i32 = 0x1024;
pub const ARCH_REQ_XCOMP_GUEST_PERM: i32 = 0x1025;

pub const ARCH_XCOMP_TILECFG: i32 = 17;
pub const ARCH_XCOMP_TILEDATA: i32 = 18;

pub const ARCH_MAP_VDSO_X32: i32 = 0x2001;
pub const ARCH_MAP_VDSO_32: i32 = 0x2002;
pub const ARCH_MAP_VDSO_64: i32 = 0x2003;

/* Don't use 0x3001-0x3004 because of old glibcs */

pub const ARCH_GET_UNTAG_MASK: i32 = 0x4001;
pub const ARCH_ENABLE_TAGGED_ADDR: i32 = 0x4002;
pub const ARCH_GET_MAX_TAG_BITS: i32 = 0x4003;
pub const ARCH_FORCE_TAGGED_SVA: i32 = 0x4004;

pub const ARCH_SHSTK_ENABLE: i32 = 0x5001;
pub const ARCH_SHSTK_DISABLE: i32 = 0x5002;
pub const ARCH_SHSTK_LOCK: i32 = 0x5003;
pub const ARCH_SHSTK_UNLOCK: i32 = 0x5004;
pub const ARCH_SHSTK_STATUS: i32 = 0x5005;

/* ARCH_SHSTK_ features bits */
pub const ARCH_SHSTK_SHSTK: u64 = 1u64 << 0;
pub const ARCH_SHSTK_WRSS: u64 = 1u64 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
