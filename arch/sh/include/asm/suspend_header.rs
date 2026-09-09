/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _ASM_SH_SUSPEND_H */

use core::ffi::c_void;
use core::ffi::c_ulong;

/* The following types are supplied by the corresponding external headers. */
extern "C" {
    pub fn sh_mobile_call_standby(mode: c_ulong);
}

#[repr(C)]
pub struct swsusp_arch_regs {
    pub user_regs: pt_regs,
    pub bank1_regs: [c_ulong; 8],
}

#[cfg(feature = "CONFIG_CPU_IDLE")]
extern "C" {
    pub fn sh_mobile_setup_cpuidle() -> i32;
}

#[cfg(not(feature = "CONFIG_CPU_IDLE"))]
#[inline]
pub unsafe fn sh_mobile_setup_cpuidle() -> i32 {
    0
}

/* notifier chains for pre/post sleep hooks */
extern "C" {
    pub static mut sh_mobile_pre_sleep_notifier_list: atomic_notifier_head;
    pub static mut sh_mobile_post_sleep_notifier_list: atomic_notifier_head;
}

/* priority levels for notifiers */
pub const SH_MOBILE_SLEEP_BOARD: i32 = 0;
pub const SH_MOBILE_SLEEP_CPU: i32 = 1;
#[inline]
pub const fn SH_MOBILE_PRE(x: i32) -> i32 {
    x
}
#[inline]
pub const fn SH_MOBILE_POST(x: i32) -> i32 {
    -x
}

/* board code registration function for self-refresh assembly snippets */
extern "C" {
    pub fn sh_mobile_register_self_refresh(
        flags: c_ulong,
        pre_start: *mut c_void,
        pre_end: *mut c_void,
        post_start: *mut c_void,
        post_end: *mut c_void,
    );
}

/* register structure for address/data information */
#[repr(C)]
pub struct sh_sleep_regs {
    pub stbcr: c_ulong,
    pub bar: c_ulong,

    /* MMU */
    pub pteh: c_ulong,
    pub ptel: c_ulong,
    pub ttb: c_ulong,
    pub tea: c_ulong,
    pub mmucr: c_ulong,
    pub ptea: c_ulong,
    pub pascr: c_ulong,
    pub irmcr: c_ulong,

    /* Cache */
    pub ccr: c_ulong,
    pub ramcr: c_ulong,
}

/* data area for low-level sleep code */
#[repr(C)]
pub struct sh_sleep_data {
    /* current sleep mode (SUSP_SH_...) */
    pub mode: c_ulong,

    /* addresses of board specific self-refresh snippets */
    pub sf_pre: c_ulong,
    pub sf_post: c_ulong,

    /* address of resume code */
    pub resume: c_ulong,

    /* register state saved and restored by the assembly code */
    pub vbr: c_ulong,
    pub spc: c_ulong,
    pub sr: c_ulong,
    pub sp: c_ulong,

    /* structure for keeping register addresses */
    pub addr: sh_sleep_regs,

    /* structure for saving/restoring register state */
    pub data: sh_sleep_regs,
}

/* a bitmap of supported sleep modes (SUSP_SH..) */
extern "C" {
    pub static mut sh_mobile_sleep_supported: c_ulong;
}

/* flags passed to assembly suspend code */
pub const SUSP_SH_SLEEP: c_ulong = 1 << 0; /* Regular sleep mode */
pub const SUSP_SH_STANDBY: c_ulong = 1 << 1; /* SH-Mobile Software standby mode */
pub const SUSP_SH_RSTANDBY: c_ulong = 1 << 2; /* SH-Mobile R-standby mode */
pub const SUSP_SH_USTANDBY: c_ulong = 1 << 3; /* SH-Mobile U-standby mode */
pub const SUSP_SH_SF: c_ulong = 1 << 4; /* Enable self-refresh */
pub const SUSP_SH_MMU: c_ulong = 1 << 5; /* Save/restore MMU and cache */
pub const SUSP_SH_REGS: c_ulong = 1 << 6; /* Save/restore registers */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
