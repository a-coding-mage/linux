/* SPDX-License-Identifier: GPL-2.0-only */
/* adi_64.h: ADI related data structures
 *
 * Copyright (c) 2016 Oracle and/or its affiliates. All rights reserved.
 * Author: Khalid Aziz (khalid.aziz@oracle.com)
 */

// Dependency intent: __u64 and bool originate from the Linux types header.

#[repr(C)]
pub struct adi_caps {
    pub blksz: u64,
    pub nbits: u64,
    pub ue_on_adi: u64,
}

#[repr(C)]
pub struct adi_config {
    pub enabled: bool,
    pub caps: adi_caps,
}

extern "C" {
    pub static mut adi_state: adi_config;

    pub fn mdesc_adi_init();
}

#[inline]
pub unsafe fn adi_capable() -> bool
{
    adi_state.enabled
}

#[inline]
pub unsafe fn adi_blksize() -> usize
{
    adi_state.caps.blksz as usize
}

#[inline]
pub unsafe fn adi_nbits() -> usize
{
    adi_state.caps.nbits as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
