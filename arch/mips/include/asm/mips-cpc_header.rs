/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2013 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// This header is intended to be included through asm/mips-cps.h.

use core::ffi::c_void;

// Supplied by the platform and by the surrounding MIPS CPS bindings.
extern "C" {
    pub static mut mips_cpc_base: *mut c_void;
    pub fn mips_cpc_default_phys_base() -> usize;
}

#[cfg(feature = "CONFIG_MIPS_CPC")]
extern "C" {
    pub fn mips_cpc_probe() -> i32;
}

#[cfg(not(feature = "CONFIG_MIPS_CPC"))]
#[inline]
pub fn mips_cpc_probe() -> i32 {
    // ENODEV
    -19
}

#[inline]
pub unsafe fn mips_cpc_present() -> bool {
    #[cfg(feature = "CONFIG_MIPS_CPC")]
    {
        mips_cpc_base != core::ptr::null_mut()
    }
    #[cfg(not(feature = "CONFIG_MIPS_CPC"))]
    {
        false
    }
}

pub const MIPS_CPC_GCB_OFS: usize = 0x0000;
pub const MIPS_CPC_CLCB_OFS: usize = 0x2000;
pub const MIPS_CPC_COCB_OFS: usize = 0x4000;

// These macros expand to the corresponding CPS register accessors supplied by
// the surrounding MIPS CPS implementation.
macro_rules! CPC_ACCESSOR_RO {
    ($sz:tt, $off:expr, $name:ident, $redir:ident) => {
        CPS_ACCESSOR_RO!(cpc, $sz, MIPS_CPC_GCB_OFS + $off, $name);
        CPS_ACCESSOR_RO!(cpc, $sz, MIPS_CPC_COCB_OFS + $off, $redir);
    };
}
macro_rules! CPC_ACCESSOR_RW {
    ($sz:tt, $off:expr, $name:ident, $redir:ident) => {
        CPS_ACCESSOR_RW!(cpc, $sz, MIPS_CPC_GCB_OFS + $off, $name);
        CPS_ACCESSOR_RW!(cpc, $sz, MIPS_CPC_COCB_OFS + $off, $redir);
    };
}
macro_rules! CPC_CX_ACCESSOR_RO {
    ($sz:tt, $off:expr, $cl:ident, $co:ident) => {
        CPS_ACCESSOR_RO!(cpc, $sz, MIPS_CPC_CLCB_OFS + $off, $cl);
        CPS_ACCESSOR_RO!(cpc, $sz, MIPS_CPC_COCB_OFS + $off, $co);
    };
}
macro_rules! CPC_CX_ACCESSOR_RW {
    ($sz:tt, $off:expr, $cl:ident, $co:ident) => {
        CPS_ACCESSOR_RW!(cpc, $sz, MIPS_CPC_CLCB_OFS + $off, $cl);
        CPS_ACCESSOR_RW!(cpc, $sz, MIPS_CPC_COCB_OFS + $off, $co);
    };
}

CPC_ACCESSOR_RW!(32, 0x000, access, redir_access);
CPC_ACCESSOR_RW!(32, 0x008, seqdel, redir_seqdel);
CPC_ACCESSOR_RW!(32, 0x010, rail, redir_rail);
CPC_ACCESSOR_RW!(32, 0x018, resetlen, redir_resetlen);
CPC_ACCESSOR_RO!(32, 0x020, revision, redir_revision);
CPC_ACCESSOR_RW!(32, 0x030, pwrup_ctl, redir_pwrup_ctl);
CPC_ACCESSOR_RW!(64, 0x138, config, redir_config);
CPC_ACCESSOR_RW!(32, 0x140, sys_config, redir_sys_config);
CPC_CX_ACCESSOR_RW!(32, 0x000, cl_cmd, co_cmd);
CPC_CX_ACCESSOR_RW!(32, 0x008, cl_stat_conf, co_stat_conf);
CPC_CX_ACCESSOR_RW!(32, 0x010, cl_other, co_other);
CPC_CX_ACCESSOR_RW!(32, 0x020, cl_vp_stop, co_vp_stop);
CPC_CX_ACCESSOR_RW!(32, 0x028, cl_vp_run, co_vp_run);
CPC_CX_ACCESSOR_RW!(32, 0x030, cl_vp_running, co_vp_running);
CPC_CX_ACCESSOR_RW!(32, 0x090, cl_config, co_config);

pub const CPC_PWRUP_CTL_CM_PWRUP: u32 = 1 << 0;
pub const CPC_SYS_CONFIG_BE_IMMEDIATE: u32 = 1 << 2;
pub const CPC_SYS_CONFIG_BE_STATUS: u32 = 1 << 1;
pub const CPC_SYS_CONFIG_BE: u32 = 1 << 0;

pub const CPC_Cx_CMD: u32 = (1 << 4) - 1;
pub const CPC_Cx_CMD_CLOCKOFF: u32 = 0x1;
pub const CPC_Cx_CMD_PWRDOWN: u32 = 0x2;
pub const CPC_Cx_CMD_PWRUP: u32 = 0x3;
pub const CPC_Cx_CMD_RESET: u32 = 0x4;

pub const CPC_Cx_STAT_CONF_PWRUPE: u32 = 1 << 23;
pub const CPC_Cx_STAT_CONF_SEQSTATE: u32 = ((1 << 4) - 1) << 19;
pub const CPC_Cx_STAT_CONF_SEQSTATE_D0: u32 = 0x0;
pub const CPC_Cx_STAT_CONF_SEQSTATE_U0: u32 = 0x1;
pub const CPC_Cx_STAT_CONF_SEQSTATE_U1: u32 = 0x2;
pub const CPC_Cx_STAT_CONF_SEQSTATE_U2: u32 = 0x3;
pub const CPC_Cx_STAT_CONF_SEQSTATE_U3: u32 = 0x4;
pub const CPC_Cx_STAT_CONF_SEQSTATE_U4: u32 = 0x5;
pub const CPC_Cx_STAT_CONF_SEQSTATE_U5: u32 = 0x6;
pub const CPC_Cx_STAT_CONF_SEQSTATE_U6: u32 = 0x7;
pub const CPC_Cx_STAT_CONF_SEQSTATE_D1: u32 = 0x8;
pub const CPC_Cx_STAT_CONF_SEQSTATE_D3: u32 = 0x9;
pub const CPC_Cx_STAT_CONF_SEQSTATE_D2: u32 = 0xa;
pub const CPC_Cx_STAT_CONF_CLKGAT_IMPL: u32 = 1 << 17;
pub const CPC_Cx_STAT_CONF_PWRDN_IMPL: u32 = 1 << 16;
pub const CPC_Cx_STAT_CONF_EJTAG_PROBE: u32 = 1 << 15;
pub const CPC_Cx_OTHER_CORENUM: u32 = ((1 << 8) - 1) << 16;

#[cfg(feature = "CONFIG_MIPS_CPC")]
extern "C" {
    pub fn mips_cpc_lock_other(core: u32);
    pub fn mips_cpc_unlock_other();
}

#[cfg(not(feature = "CONFIG_MIPS_CPC"))]
#[inline]
pub fn mips_cpc_lock_other(_core: u32) {}

#[cfg(not(feature = "CONFIG_MIPS_CPC"))]
#[inline]
pub fn mips_cpc_unlock_other() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
