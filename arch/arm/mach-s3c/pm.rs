// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2004-2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// S3C common power management (suspend to ram) support.

use core::ffi::{c_int, c_ulong};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut s3c_irqwake_eintallow: c_ulong;
    static mut s3c_irqwake_intallow: c_ulong;
    fn s3c_pm_debug_init_uart();
    fn s3c_pm_save_uarts(is_before: bool);
    fn s3c_pm_save_core();
    fn s3c_pm_configure_extint();
    fn s3c_pm_arch_prepare_irqs();
    fn s3c_pm_check_store();
    fn s3c_pm_arch_stop_clocks();
    fn s3c_pm_restore_core();
    fn s3c_pm_restore_uarts(is_before: bool);
    fn s3c_pm_arch_show_resume_irqs();
    fn s3c_pm_check_restore();
    fn s3c_pm_check_prepare();
    fn s3c_pm_check_cleanup();
    fn samsung_pm_save_gpios();
    fn samsung_pm_saved_gpios();
    fn samsung_pm_restore_gpios();
    fn s3c_pm_restored_gpios();
    fn of_have_populated_dt() -> bool;
    fn flush_cache_all();
    fn cpu_suspend(arg: c_ulong, fn_ptr: unsafe extern "C" fn(c_ulong) -> c_int) -> c_int;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
    fn printk(fmt: *const u8, ...);
}

type suspend_state_t = c_int;

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_suspend_ops {
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn() -> c_int>,
    pub finish: Option<unsafe extern "C" fn()>,
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
}

pub static mut s3c_pm_flags: c_ulong = 0;
pub static mut s3c_irqwake_intmask: c_ulong = 0xffffffff;
pub static mut s3c_irqwake_eintmask: c_ulong = 0xffffffff;

pub static mut pm_cpu_prep: Option<unsafe extern "C" fn()> = None;
pub static mut pm_cpu_sleep: Option<unsafe extern "C" fn(c_ulong) -> c_int> = None;

extern "C" {
    fn IRQ_EINT_BIT(irq: c_uint) -> c_uint;
}

use core::ffi::c_uint;

pub unsafe extern "C" fn s3c_irqext_wake(data: *mut irq_data, state: c_uint) -> c_int {
    let bit: c_ulong = 1u64.wrapping_shl(IRQ_EINT_BIT((*data).irq) as u32) as c_ulong;

    if (s3c_irqwake_eintallow & bit) == 0 {
        return -2;
    }

    printk(b"wake %s for irq %d\n\0".as_ptr(),
           if state != 0 { b"enabled\0".as_ptr() } else { b"disabled\0".as_ptr() },
           (*data).irq);

    if state == 0 {
        s3c_irqwake_eintmask |= bit;
    } else {
        s3c_irqwake_eintmask &= !bit;
    }

    0
}

#[inline]
unsafe fn any_allowed(mask: c_ulong, allow: c_ulong) -> bool {
    (mask & allow) != allow
}

unsafe extern "C" fn s3c_pm_enter(state: suspend_state_t) -> c_int {
    let ret: c_int;
    s3c_pm_debug_init_uart();

    if pm_cpu_prep.is_none() || pm_cpu_sleep.is_none() {
        printk(b"%s: error: no cpu sleep function\n\0".as_ptr(), b"s3c_pm_enter\0".as_ptr());
        return -22;
    }

    if !of_have_populated_dt()
        && !any_allowed(s3c_irqwake_intmask, s3c_irqwake_intallow)
        && !any_allowed(s3c_irqwake_eintmask, s3c_irqwake_eintallow)
    {
        printk(b"%s: No wake-up sources!\n\0".as_ptr(), b"s3c_pm_enter\0".as_ptr());
        printk(b"%s: Aborting sleep\n\0".as_ptr(), b"s3c_pm_enter\0".as_ptr());
        return -22;
    }

    if !of_have_populated_dt() {
        samsung_pm_save_gpios();
        samsung_pm_saved_gpios();
    }

    s3c_pm_save_uarts(false);
    s3c_pm_save_core();
    s3c_pm_configure_extint();
    s3c_pm_arch_prepare_irqs();
    pm_cpu_prep.unwrap_unchecked()();
    flush_cache_all();
    s3c_pm_check_store();
    s3c_pm_arch_stop_clocks();

    ret = cpu_suspend(0, pm_cpu_sleep.unwrap_unchecked());
    if ret != 0 {
        return ret;
    }

    s3c_pm_restore_core();
    s3c_pm_restore_uarts(false);
    if !of_have_populated_dt() {
        samsung_pm_restore_gpios();
        s3c_pm_restored_gpios();
    }
    s3c_pm_debug_init_uart();
    s3c_pm_arch_show_resume_irqs();
    s3c_pm_check_restore();
    0
}

unsafe extern "C" fn s3c_pm_prepare() -> c_int {
    s3c_pm_check_prepare();
    0
}

unsafe extern "C" fn s3c_pm_finish() {
    s3c_pm_check_cleanup();
}

static s3c_pm_ops: platform_suspend_ops = platform_suspend_ops {
    enter: Some(s3c_pm_enter),
    prepare: Some(s3c_pm_prepare),
    finish: Some(s3c_pm_finish),
    valid: Some(suspend_valid_only_mem),
};

pub unsafe extern "C" fn s3c_pm_init() -> c_int {
    printk(b"S3C Power Management, Copyright 2004 Simtec Electronics\n\0".as_ptr());
    suspend_set_ops(&s3c_pm_ops);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
