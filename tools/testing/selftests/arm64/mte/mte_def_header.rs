/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020 ARM Limited */

/*
 * Below definitions may be found in kernel headers, However, they are
 * redefined here to decouple the MTE selftests compilations from them.
 */
pub const SEGV_MTEAERR: i32 = 8;
pub const SEGV_MTESERR: i32 = 9;
pub const PROT_MTE: i32 = 0x20;
pub const HWCAP2_MTE: u64 = 1 << 18;

pub const PR_MTE_TCF_SHIFT: u64 = 1;
pub const PR_MTE_TCF_NONE: u64 = 0u64 << PR_MTE_TCF_SHIFT;
pub const PR_MTE_TCF_SYNC: u64 = 1u64 << PR_MTE_TCF_SHIFT;
pub const PR_MTE_TCF_ASYNC: u64 = 2u64 << PR_MTE_TCF_SHIFT;
pub const PR_MTE_TAG_SHIFT: u64 = 3;

/* MTE Hardware feature definitions below. */
pub const MT_TAG_SHIFT: u64 = 56;
pub const MT_TAG_MASK: u64 = 0xF;
pub const MT_FREE_TAG: u64 = 0x0;
pub const MT_GRANULE_SIZE: u64 = 16;
pub const MT_TAG_COUNT: u64 = 16;
pub const MT_INCLUDE_TAG_MASK: u64 = 0xFFFF;
pub const MT_EXCLUDE_TAG_MASK: u64 = 0x0;
pub const MT_ATAG_SHIFT: u64 = 60;
pub const MT_ATAG_MASK: u64 = 0xF;

pub const MT_ALIGN_GRANULE: u64 = MT_GRANULE_SIZE - 1;

pub const fn MT_CLEAR_TAG(x: u64) -> u64 {
    x & !(MT_TAG_MASK << MT_TAG_SHIFT)
}

pub const fn MT_SET_TAG(x: u64, y: u64) -> u64 {
    x | (y << MT_TAG_SHIFT)
}

pub const fn MT_FETCH_TAG(x: u64) -> u64 {
    (x >> MT_TAG_SHIFT) & MT_TAG_MASK
}

pub const fn MT_ALIGN_UP(x: u64) -> u64 {
    (x + MT_ALIGN_GRANULE) & !MT_ALIGN_GRANULE
}

pub const fn MT_CLEAR_ATAG(x: u64) -> u64 {
    x & !(MT_TAG_MASK << MT_ATAG_SHIFT)
}

pub const fn MT_SET_ATAG(x: u64, y: u64) -> u64 {
    x | ((y & MT_ATAG_MASK) << MT_ATAG_SHIFT)
}

pub const fn MT_FETCH_ATAG(x: u64) -> u64 {
    (x >> MT_ATAG_SHIFT) & MT_ATAG_MASK
}

pub const fn MT_CLEAR_TAGS(x: u64) -> u64 {
    MT_CLEAR_ATAG(MT_CLEAR_TAG(x))
}

pub const MT_PSTATE_TCO_SHIFT: u64 = 25;
pub const MT_PSTATE_TCO_MASK: u64 = !(0x1 << MT_PSTATE_TCO_SHIFT);
pub const MT_PSTATE_TCO_EN: i32 = 1;
pub const MT_PSTATE_TCO_DIS: i32 = 0;

pub const fn MT_EXCLUDE_TAG(x: u64) -> u64 {
    1 << x
}

pub const fn MT_INCLUDE_VALID_TAG(x: u64) -> u64 {
    MT_INCLUDE_TAG_MASK ^ MT_EXCLUDE_TAG(x)
}

pub const fn MT_INCLUDE_VALID_TAGS(x: u64) -> u64 {
    MT_INCLUDE_TAG_MASK ^ x
}

pub const MTE_ALLOW_NON_ZERO_TAG: u64 = MT_INCLUDE_VALID_TAG(0);
