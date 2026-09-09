/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * omap-secure.h: OMAP Secure infrastructure header.
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 *	Santosh Shilimkar <santosh.shilimkar@ti.com>
 * Copyright (C) 2012 Ivaylo Dimitrov <freemangordon@abv.bg>
 * Copyright (C) 2013 Pali Rohár <pali@kernel.org>
 */

// #include <linux/types.h>

/* Monitor error code */
pub const API_HAL_RET_VALUE_NS2S_CONVERSION_ERROR: u32 = 0xFFFFFFFE;
pub const API_HAL_RET_VALUE_SERVICE_UNKNWON: u32 = 0xFFFFFFFF;

/* HAL API error codes */
pub const API_HAL_RET_VALUE_OK: u32 = 0x00;
pub const API_HAL_RET_VALUE_FAIL: u32 = 0x01;

/* Secure HAL API flags */
pub const FLAG_START_CRITICAL: u32 = 0x4;
pub const FLAG_IRQFIQ_MASK: u32 = 0x3;
pub const FLAG_IRQ_ENABLE: u32 = 0x2;
pub const FLAG_FIQ_ENABLE: u32 = 0x1;
pub const NO_FLAG: u32 = 0x0;

/* Maximum Secure memory storage size */
pub const OMAP_SECURE_RAM_STORAGE: u32 = 88 * (1u32 << 10);

pub const OMAP3_SAVE_SECURE_RAM_SZ: u32 = 0x803F;

/* Secure low power HAL API index */
pub const OMAP4_HAL_SAVESECURERAM_INDEX: u32 = 0x1a;
pub const OMAP4_HAL_SAVEHW_INDEX: u32 = 0x1b;
pub const OMAP4_HAL_SAVEALL_INDEX: u32 = 0x1c;
pub const OMAP4_HAL_SAVEGIC_INDEX: u32 = 0x1d;

/* Secure Monitor mode APIs */
pub const OMAP4_MON_SCU_PWR_INDEX: u32 = 0x108;
pub const OMAP4_MON_L2X0_DBG_CTRL_INDEX: u32 = 0x100;
pub const OMAP4_MON_L2X0_CTRL_INDEX: u32 = 0x102;
pub const OMAP4_MON_L2X0_AUXCTRL_INDEX: u32 = 0x109;
pub const OMAP4_MON_L2X0_PREFETCH_INDEX: u32 = 0x113;

pub const OMAP5_DRA7_MON_SET_CNTFRQ_INDEX: u32 = 0x109;
pub const OMAP5_MON_AMBA_IF_INDEX: u32 = 0x108;
pub const OMAP5_DRA7_MON_SET_ACR_INDEX: u32 = 0x107;

/* Secure PPA(Primary Protected Application) APIs */
pub const OMAP4_PPA_SERVICE_0: u32 = 0x21;
pub const OMAP4_PPA_L2_POR_INDEX: u32 = 0x23;
pub const OMAP4_PPA_CPU_ACTRL_SMP_INDEX: u32 = 0x25;

pub const AM43xx_PPA_SVC_PM_SUSPEND: u32 = 0x71;
pub const AM43xx_PPA_SVC_PM_RESUME: u32 = 0x72;

/* Secure RX-51 PPA (Primary Protected Application) APIs */
pub const RX51_PPA_HWRNG: u32 = 29;
pub const RX51_PPA_L2_INVAL: u32 = 40;
pub const RX51_PPA_WRITE_ACR: u32 = 42;

extern "C" {
    pub fn omap_secure_dispatcher(
        idx: u32,
        flag: u32,
        nargs: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
        arg4: u32,
    ) -> u32;
    pub fn omap_smccc_smc(fn_: u32, arg: u32);
    pub fn omap_smc1(fn_: u32, arg: u32);
    pub fn omap_smc2(id: u32, flag: u32, pargs: u32) -> u32;
    pub fn omap_smc3(id: u32, process: u32, flag: u32, pargs: u32) -> u32;
    pub fn omap_secure_ram_reserve_memblock() -> i32;
    pub fn save_secure_ram_context(args_pa: u32) -> u32;
    pub fn omap3_save_secure_ram(save_regs: *mut core::ffi::c_void, size: i32) -> u32;

    pub fn rx51_secure_update_aux_cr(set_bits: u32, clear_bits: u32) -> u32;
    pub fn rx51_secure_rng_call(ptr: u32, count: u32, flag: u32) -> u32;

    pub static mut optee_available: bool;
    pub fn omap_secure_init();
}

#[cfg(CONFIG_SOC_HAS_REALTIME_COUNTER)]
extern "C" {
    pub fn set_cntfreq();
}

#[cfg(not(CONFIG_SOC_HAS_REALTIME_COUNTER))]
#[inline]
pub fn set_cntfreq() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
