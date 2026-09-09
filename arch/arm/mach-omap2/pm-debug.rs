// SPDX-License-Identifier: GPL-2.0-only
/* OMAP Power Management debug routines (translated from pm-debug.c). */

// The following declarations are supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

#[cfg(CONFIG_DEBUG_FS)]
mod debug_fs {
    use super::*;

    pub const PWRDM_MAX_PWRSTS: usize = 4;

    #[repr(C)]
    pub struct PowerDomain {
        pub name: *const c_char,
        pub state_timer: [i64; 4],
        pub timer: i64,
        pub state: c_int,
        pub state_counter: [c_int; 4],
        pub ret_logic_off_counter: c_int,
        pub banks: c_int,
        pub ret_mem_off_counter: *mut c_int,
    }
    #[repr(C)]
    pub struct ClockDomain {
        pub name: *const c_char,
        pub pwrdm: PowerDomainPointer,
        pub usecount: c_int,
    }
    #[repr(C)]
    pub struct PowerDomainPointer { pub ptr: *mut PowerDomain }
    #[repr(C)] pub struct SeqFile;
    #[repr(C)] pub struct Dentry;

    extern "C" {
        fn sched_clock() -> i64;
        fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
        fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
        fn printk(fmt: *const c_char, ...);
        fn seq_printf(s: *mut SeqFile, fmt: *const c_char, ...);
        fn seq_putc(s: *mut SeqFile, c: c_int);
        fn pwrdm_read_pwrst(pwrdm: *mut PowerDomain) -> c_int;
        fn pwrdm_state_switch(pwrdm: *mut PowerDomain);
        fn pwrdm_for_each(cb: unsafe extern "C" fn(*mut PowerDomain, *mut c_void) -> c_int, user: *mut c_void) -> c_int;
        fn clkdm_for_each(cb: unsafe extern "C" fn(*mut ClockDomain, *mut c_void) -> c_int, user: *mut c_void) -> c_int;
        fn cpu_is_omap34xx() -> c_int;
        fn omap3_pm_get_suspend_state(pwrdm: *mut PowerDomain) -> c_int;
        fn omap3_pm_set_suspend_state(pwrdm: *mut PowerDomain, state: c_int) -> c_int;
        fn omap3_pm_off_mode_enable(value: u64);
        fn debugfs_create_dir(name: *const c_char, parent: *mut Dentry) -> *mut Dentry;
        fn debugfs_create_file(name: *const c_char, mode: c_int, parent: *mut Dentry, data: *mut c_void, fops: *const c_void) -> *mut c_void;
        static mut enable_off_mode: u32;
    }

    static mut PM_DBG_INIT_DONE: c_int = 0;
    static PWRDM_STATE_NAMES: [&[u8]; 4] = [b"OFF\0", b"RET\0", b"INA\0", b"ON\0"];

    pub unsafe extern "C" fn pm_dbg_update_time(pwrdm: *mut PowerDomain, prev: c_int) {
        if PM_DBG_INIT_DONE == 0 { return; }
        let t = sched_clock();
        (*pwrdm).state_timer[prev as usize] += t - (*pwrdm).timer;
        (*pwrdm).timer = t;
    }

    unsafe extern "C" fn clkdm_dbg_show_counter(clkdm: *mut ClockDomain, user: *mut c_void) -> c_int {
        if strcmp((*clkdm).name, b"emu_clkdm\0".as_ptr() as _) == 0 || strcmp((*clkdm).name, b"wkup_clkdm\0".as_ptr() as _) == 0 || strncmp((*clkdm).name, b"dpll\0".as_ptr() as _, 4) == 0 { return 0; }
        seq_printf(user as _, b"%s->%s (%d)\n\0".as_ptr() as _, (*clkdm).name, (*(*clkdm).pwrdm.ptr).name, (*clkdm).usecount);
        0
    }

    unsafe extern "C" fn pwrdm_dbg_show_counter(pwrdm: *mut PowerDomain, user: *mut c_void) -> c_int {
        if strcmp((*pwrdm).name, b"emu_pwrdm\0".as_ptr() as _) == 0 || strcmp((*pwrdm).name, b"wkup_pwrdm\0".as_ptr() as _) == 0 || strncmp((*pwrdm).name, b"dpll\0".as_ptr() as _, 4) == 0 { return 0; }
        let current = pwrdm_read_pwrst(pwrdm);
        if (*pwrdm).state != current { printk(b"pwrdm state mismatch(%s) %d != %d\n\0".as_ptr() as _, (*pwrdm).name, (*pwrdm).state, current); }
        seq_printf(user as _, b"%s (%s)\0".as_ptr() as _, (*pwrdm).name, PWRDM_STATE_NAMES[(*pwrdm).state as usize].as_ptr());
        for i in 0..PWRDM_MAX_PWRSTS { seq_printf(user as _, b",%s:%d\0".as_ptr() as _, PWRDM_STATE_NAMES[i].as_ptr(), (*pwrdm).state_counter[i]); }
        seq_printf(user as _, b",RET-LOGIC-OFF:%d\0".as_ptr() as _, (*pwrdm).ret_logic_off_counter);
        for i in 0..(*pwrdm).banks { seq_printf(user as _, b",RET-MEMBANK%d-OFF:%d\0".as_ptr() as _, i + 1, *(*pwrdm).ret_mem_off_counter.add(i as usize)); }
        seq_putc(user as _, b'\n' as c_int); 0
    }

    unsafe extern "C" fn pwrdm_dbg_show_timer(pwrdm: *mut PowerDomain, user: *mut c_void) -> c_int {
        if strcmp((*pwrdm).name, b"emu_pwrdm\0".as_ptr() as _) == 0 || strcmp((*pwrdm).name, b"wkup_pwrdm\0".as_ptr() as _) == 0 || strncmp((*pwrdm).name, b"dpll\0".as_ptr() as _, 4) == 0 { return 0; }
        pwrdm_state_switch(pwrdm);
        seq_printf(user as _, b"%s (%s)\0".as_ptr() as _, (*pwrdm).name, PWRDM_STATE_NAMES[(*pwrdm).state as usize].as_ptr());
        for i in 0..4 { seq_printf(user as _, b",%s:%lld\0".as_ptr() as _, PWRDM_STATE_NAMES[i].as_ptr(), (*pwrdm).state_timer[i]); }
        seq_putc(user as _, b'\n' as c_int); 0
    }

    unsafe extern "C" fn pm_dbg_counters_show(s: *mut SeqFile, _unused: *mut c_void) -> c_int {
        pwrdm_for_each(pwrdm_dbg_show_counter, s as _); clkdm_for_each(clkdm_dbg_show_counter, s as _); 0
    }
    unsafe extern "C" fn pm_dbg_timers_show(s: *mut SeqFile, _unused: *mut c_void) -> c_int { pwrdm_for_each(pwrdm_dbg_show_timer, s as _); 0 }

    unsafe extern "C" fn pwrdm_suspend_get(data: *mut c_void, val: *mut u64) -> c_int {
        let mut ret = -22; if cpu_is_omap34xx() != 0 { ret = omap3_pm_get_suspend_state(data as _); } *val = ret as u64; if ret >= 0 { 0 } else { *val as c_int }
    }
    unsafe extern "C" fn pwrdm_suspend_set(data: *mut c_void, val: u64) -> c_int { if cpu_is_omap34xx() != 0 { omap3_pm_set_suspend_state(data as _, val as c_int) } else { -22 } }

    unsafe extern "C" fn option_get(data: *mut c_void, val: *mut u64) -> c_int { *val = *(data as *mut u32) as u64; 0 }
    unsafe extern "C" fn option_set(data: *mut c_void, val: u64) -> c_int {
        *(data as *mut u32) = val as u32;
        if data == core::ptr::addr_of_mut!(enable_off_mode) as _ && cpu_is_omap34xx() != 0 { omap3_pm_off_mode_enable(val); }
        0
    }

    unsafe extern "C" fn pwrdms_setup(pwrdm: *mut PowerDomain, dir: *mut c_void) -> c_int {
        let t = sched_clock();
        for i in 0..4 { (*pwrdm).state_timer[i] = 0; }
        (*pwrdm).timer = t;
        if strncmp((*pwrdm).name, b"dpll\0".as_ptr() as _, 4) == 0 { return 0; }
        let d = debugfs_create_dir((*pwrdm).name, dir as _);
        debugfs_create_file(b"suspend\0".as_ptr() as _, 0o444 | 0o200, d, pwrdm as _, core::ptr::null());
        0
    }

    unsafe extern "C" fn pm_dbg_init() -> c_int {
        if PM_DBG_INIT_DONE != 0 { return 0; }
        let d = debugfs_create_dir(b"pm_debug\0".as_ptr() as _, core::ptr::null_mut());
        debugfs_create_file(b"count\0".as_ptr() as _, 0o444, d, core::ptr::null_mut(), core::ptr::null());
        debugfs_create_file(b"time\0".as_ptr() as _, 0o444, d, core::ptr::null_mut(), core::ptr::null());
        pwrdm_for_each(pwrdms_setup, d as _);
        debugfs_create_file(b"enable_off_mode\0".as_ptr() as _, 0o444 | 0o200, d, core::ptr::addr_of_mut!(enable_off_mode) as _, core::ptr::null());
        PM_DBG_INIT_DONE = 1;
        0
    }

    // DEFINE_SHOW_ATTRIBUTE, DEFINE_DEBUGFS_ATTRIBUTE, DEFINE_SIMPLE_ATTRIBUTE,
    // and omap_arch_initcall expand to kernel-specific file-operation objects and
    // registration boilerplate supplied by the surrounding translation.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
