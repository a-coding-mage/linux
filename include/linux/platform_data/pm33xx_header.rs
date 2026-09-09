/* SPDX-License-Identifier: GPL-2.0 */
/*
 * TI pm33xx platform data
 *
 * Copyright (C) 2016-2018 Texas Instruments, Inc.
 *	Dave Gerlach <d-gerlach@ti.com>
 */

/* C dependencies: linux/kbuild.h and linux/types.h. */

/*
 * WFI Flags for sleep code control
 *
 * These flags allow PM code to exclude certain operations from happening
 * in the low level ASM code found in sleep33xx.S and sleep43xx.S
 *
 * WFI_FLAG_FLUSH_CACHE: Flush the ARM caches and disable caching. Only
 *                       needed when MPU will lose context.
 * WFI_FLAG_SELF_REFRESH: Let EMIF place DDR memory into self-refresh and
 *                        disable EMIF.
 * WFI_FLAG_SAVE_EMIF: Save context of all EMIF registers and restore in
 *                     resume path. Only needed if PER domain loses context
 *                     and must also have WFI_FLAG_SELF_REFRESH set.
 * WFI_FLAG_WAKE_M3: Disable MPU clock or clockdomain to cause wkup_m3 to
 *                   execute when WFI instruction executes.
 * WFI_FLAG_RTC_ONLY: Configure the RTC to enter RTC+DDR mode.
 */
pub const WFI_FLAG_FLUSH_CACHE: u32 = 1 << 0;
pub const WFI_FLAG_SELF_REFRESH: u32 = 1 << 1;
pub const WFI_FLAG_SAVE_EMIF: u32 = 1 << 2;
pub const WFI_FLAG_WAKE_M3: u32 = 1 << 3;
pub const WFI_FLAG_RTC_ONLY: u32 = 1 << 4;

#[repr(C)]
pub struct am33xx_pm_sram_addr {
    pub do_wfi: Option<unsafe extern "C" fn()>,
    pub do_wfi_sz: *mut usize,
    pub resume_offset: *mut usize,
    pub emif_sram_table: *mut usize,
    pub ro_sram_data: *mut usize,
    pub resume_address: usize,
}

#[repr(C)]
pub struct am33xx_pm_platform_data {
    pub init: Option<unsafe extern "C" fn(Option<unsafe extern "C" fn(u32)>) -> i32>,
    pub deinit: Option<unsafe extern "C" fn() -> i32>,
    pub soc_suspend: Option<unsafe extern "C" fn(u32, Option<unsafe extern "C" fn(usize) -> i32>, usize) -> i32>,
    pub cpu_suspend: Option<unsafe extern "C" fn(Option<unsafe extern "C" fn(usize) -> i32>, usize) -> i32>,
    pub begin_suspend: Option<unsafe extern "C" fn()>,
    pub finish_suspend: Option<unsafe extern "C" fn()>,
    pub get_sram_addrs: Option<unsafe extern "C" fn() -> *mut am33xx_pm_sram_addr>,
    pub save_context: Option<unsafe extern "C" fn()>,
    pub restore_context: Option<unsafe extern "C" fn()>,
    pub check_off_mode_enable: Option<unsafe extern "C" fn() -> i32>,
}

#[repr(C, align(8))]
pub struct am33xx_pm_sram_data {
    pub wfi_flags: u32,
    pub l2_aux_ctrl_val: u32,
    pub l2_prefetch_ctrl_val: u32,
}

#[repr(C, align(8))]
pub struct am33xx_pm_ro_sram_data {
    pub amx3_pm_sram_data_virt: u32,
    pub amx3_pm_sram_data_phys: u32,
    pub rtc_base_virt: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
