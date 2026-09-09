/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// left external to this translation.

pub const COMMAND_LINE_SIZE: usize = 256;

/*
 * Data structure to map a ID to string
 * Used a lot for bootup reporting of hardware diversity
 */
#[repr(C)]
pub struct id_to_str {
    pub id: core::ffi::c_int,
    pub str: *const core::ffi::c_char,
}

unsafe extern "C" {
    pub static mut root_mountflags: core::ffi::c_int;
    pub static mut end_mem: core::ffi::c_int;

    pub fn setup_processor();
    // C declaration carries the __init section annotation.
    pub fn setup_arch_memory();
    // C declaration carries the __init section annotation.
    pub fn arc_get_mem_sz() -> core::ffi::c_long;

    pub fn arc_mmu_init();
    pub fn arc_mmu_mumbojumbo(
        cpu_id: core::ffi::c_int,
        buf: *mut core::ffi::c_char,
        len: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn arc_cache_init();
    pub fn arc_cache_mumbojumbo(
        cpu_id: core::ffi::c_int,
        buf: *mut core::ffi::c_char,
        len: core::ffi::c_int,
    ) -> core::ffi::c_int;

    // C declaration carries the __init section annotation.
    pub fn handle_uboot_args();
}

/* Helpers used in arc_*_mumbojumbo routines */
#[macro_export]
macro_rules! IS_AVAIL1 {
    ($v:expr, $s:expr) => {
        if $v { $s } else { "" }
    };
}

#[macro_export]
macro_rules! IS_DISABLED_RUN {
    ($v:expr) => {
        if $v { "" } else { "(disabled) " }
    };
}

#[macro_export]
macro_rules! IS_USED_RUN {
    ($v:expr) => {
        if $v { "" } else { "(not used) " }
    };
}

#[macro_export]
macro_rules! IS_USED_CFG {
    ($cfg:expr) => {
        $crate::IS_USED_RUN!(IS_ENABLED!($cfg))
    };
}

#[macro_export]
macro_rules! IS_AVAIL2 {
    ($v:expr, $s:expr, $cfg:expr) => {
        IS_AVAIL1!($v, $s), IS_AVAIL1!($v, IS_USED_CFG!($cfg))
    };
}

#[macro_export]
macro_rules! IS_AVAIL3 {
    ($v:expr, $v2:expr, $s:expr) => {
        IS_AVAIL1!($v, $s), IS_AVAIL1!($v, IS_DISABLED_RUN!($v2))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
