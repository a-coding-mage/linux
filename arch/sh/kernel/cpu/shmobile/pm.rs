// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/shmobile/pm.c
 *
 * Power management support code for SuperH Mobile
 *
 * Copyright (C) 2009 Magnus Damm
 */

// Linux and SuperH declarations used by this translation are supplied by
// external dependencies.

#[cfg(CONFIG_CPU_SUBTYPE_SH7724)]
const RAM_BASE: usize = 0xfd80_0000; // RSMEM
#[cfg(not(CONFIG_CPU_SUBTYPE_SH7724))]
const RAM_BASE: usize = 0xe520_0000; // ILRAM

const SUSP_MODE_SLEEP: ::core::ffi::c_ulong = SUSP_SH_SLEEP;
const SUSP_MODE_SLEEP_SF: ::core::ffi::c_ulong = SUSP_SH_SLEEP | SUSP_SH_SF;
const SUSP_MODE_STANDBY_SF: ::core::ffi::c_ulong = SUSP_SH_STANDBY | SUSP_SH_SF;
const SUSP_MODE_RSTANDBY_SF: ::core::ffi::c_ulong =
    SUSP_SH_RSTANDBY | SUSP_SH_MMU | SUSP_SH_REGS | SUSP_SH_SF;

extern "C" {
    static mut sh_mobile_pre_sleep_notifier_list: atomic_notifier_head;
    static mut sh_mobile_post_sleep_notifier_list: atomic_notifier_head;

    static sh_mobile_sleep_enter_start: u8;
    static sh_mobile_sleep_enter_end: u8;
    static sh_mobile_sleep_resume_start: u8;
    static sh_mobile_sleep_resume_end: u8;

    fn atomic_notifier_call_chain(
        nh: *mut atomic_notifier_head,
        val: ::core::ffi::c_ulong,
        v: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn flush_cache_all();
    fn local_irq_disable();
    fn set_bl_bit();
    fn clear_bl_bit();
    fn sh_mobile_setup_cpuidle() -> ::core::ffi::c_int;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
    fn memcpy(dest: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, n: usize);
    fn roundup(x: usize, y: usize) -> usize;
    fn warn_on(condition: bool) -> bool;
}

// Opaque external kernel types.
#[repr(C)]
pub struct atomic_notifier_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sh_sleep_data {
    pub addr: sh_sleep_addr,
    pub sf_pre: ::core::ffi::c_ulong,
    pub sf_post: ::core::ffi::c_ulong,
    pub resume: ::core::ffi::c_ulong,
}
#[repr(C)]
pub struct sh_sleep_addr {
    pub stbcr: ::core::ffi::c_ulong,
    pub bar: ::core::ffi::c_ulong,
    pub pteh: ::core::ffi::c_ulong,
    pub ptel: ::core::ffi::c_ulong,
    pub ttb: ::core::ffi::c_ulong,
    pub tea: ::core::ffi::c_ulong,
    pub mmucr: ::core::ffi::c_ulong,
    pub ptea: ::core::ffi::c_ulong,
    pub pascr: ::core::ffi::c_ulong,
    pub irmcr: ::core::ffi::c_ulong,
    pub ccr: ::core::ffi::c_ulong,
    pub ramcr: ::core::ffi::c_ulong,
}

pub type suspend_state_t = ::core::ffi::c_int;
#[repr(C)]
pub struct platform_suspend_ops {
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> ::core::ffi::c_int>,
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
}

pub const SUSP_SH_SLEEP: ::core::ffi::c_ulong = 1;
pub const SUSP_SH_SF: ::core::ffi::c_ulong = 2;
pub const SUSP_SH_STANDBY: ::core::ffi::c_ulong = 4;
pub const SUSP_SH_RSTANDBY: ::core::ffi::c_ulong = 8;
pub const SUSP_SH_MMU: ::core::ffi::c_ulong = 16;
pub const SUSP_SH_REGS: ::core::ffi::c_ulong = 32;

#[no_mangle]
pub static mut sh_mobile_sleep_supported: ::core::ffi::c_ulong = SUSP_SH_SLEEP;

#[no_mangle]
pub unsafe extern "C" fn sh_mobile_call_standby(mode: ::core::ffi::c_ulong) {
    let onchip_mem = RAM_BASE as *mut u8;
    let sdp = onchip_mem as *mut sh_sleep_data;
    let standby_onchip_mem = onchip_mem.add(core::mem::size_of::<sh_sleep_data>())
        as *mut unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_ulong);

    atomic_notifier_call_chain(
        &mut sh_mobile_pre_sleep_notifier_list,
        mode,
        core::ptr::null_mut(),
    );
    if mode & SUSP_SH_MMU != 0 {
        flush_cache_all();
    }
    (*standby_onchip_mem)(mode, RAM_BASE as ::core::ffi::c_ulong);
    atomic_notifier_call_chain(
        &mut sh_mobile_post_sleep_notifier_list,
        mode,
        core::ptr::null_mut(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn sh_mobile_register_self_refresh(
    flags: ::core::ffi::c_ulong,
    pre_start: *mut ::core::ffi::c_void,
    pre_end: *mut ::core::ffi::c_void,
    post_start: *mut ::core::ffi::c_void,
    post_end: *mut ::core::ffi::c_void,
) {
    let onchip_mem = RAM_BASE as *mut u8;
    let sdp = onchip_mem as *mut sh_sleep_data;
    (*sdp).addr.stbcr = 0xa415_0020;
    (*sdp).addr.bar = 0xa415_0040;
    (*sdp).addr.pteh = 0xff00_0000;
    (*sdp).addr.ptel = 0xff00_0004;
    (*sdp).addr.ttb = 0xff00_0008;
    (*sdp).addr.tea = 0xff00_000c;
    (*sdp).addr.mmucr = 0xff00_0010;
    (*sdp).addr.ptea = 0xff00_0034;
    (*sdp).addr.pascr = 0xff00_0070;
    (*sdp).addr.irmcr = 0xff00_0078;
    (*sdp).addr.ccr = 0xff00_001c;
    (*sdp).addr.ramcr = 0xff00_0074;
    let mut vp = onchip_mem.add(core::mem::size_of::<sh_sleep_data>());

    let n = (&sh_mobile_sleep_enter_end as *const u8 as usize)
        .wrapping_sub(&sh_mobile_sleep_enter_start as *const u8 as usize);
    memcpy(vp as *mut _, &sh_mobile_sleep_enter_start as *const u8 as *const _, n);
    vp = vp.add(roundup(n, 4));

    let n = (pre_end as usize).wrapping_sub(pre_start as usize);
    memcpy(vp as *mut _, pre_start, n);
    (*sdp).sf_pre = vp as ::core::ffi::c_ulong;
    vp = vp.add(roundup(n, 4));

    let n = (post_end as usize).wrapping_sub(post_start as usize);
    memcpy(vp as *mut _, post_start, n);
    (*sdp).sf_post = vp as ::core::ffi::c_ulong;
    vp = vp.add(roundup(n, 4));

    warn_on(vp as usize > onchip_mem as usize + 0x600);
    vp = onchip_mem.add(0x600);
    let n = (&sh_mobile_sleep_resume_end as *const u8 as usize)
        .wrapping_sub(&sh_mobile_sleep_resume_start as *const u8 as usize);
    memcpy(vp as *mut _, &sh_mobile_sleep_resume_start as *const u8 as *const _, n);
    (*sdp).resume = vp as ::core::ffi::c_ulong;
    sh_mobile_sleep_supported |= flags;
}

unsafe extern "C" fn sh_pm_enter(_state: suspend_state_t) -> ::core::ffi::c_int {
    if sh_mobile_sleep_supported & SUSP_MODE_STANDBY_SF == 0 {
        return -6; // -ENXIO
    }
    local_irq_disable();
    set_bl_bit();
    sh_mobile_call_standby(SUSP_MODE_STANDBY_SF);
    local_irq_disable();
    clear_bl_bit();
    0
}

static sh_pm_ops: platform_suspend_ops = platform_suspend_ops {
    enter: Some(sh_pm_enter),
    valid: Some(suspend_valid_only_mem),
};

unsafe extern "C" fn sh_pm_init() -> ::core::ffi::c_int {
    suspend_set_ops(&sh_pm_ops);
    sh_mobile_setup_cpuidle()
}

// late_initcall(sh_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
