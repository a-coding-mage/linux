/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Amit Bhor, Sameer Dhavale: Codito Technologies 2004
 */

// Dependency: <asm-generic/module.h>

#[repr(C)]
pub struct mod_arch_specific {
    #[cfg(feature = "CONFIG_ARC_DW2_UNWIND")]
    pub unw_info: *mut core::ffi::c_void,
    #[cfg(feature = "CONFIG_ARC_DW2_UNWIND")]
    pub unw_sec_idx: core::ffi::c_int,
    pub secstr: *const core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
