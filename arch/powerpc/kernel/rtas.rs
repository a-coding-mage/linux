// SPDX-License-Identifier: GPL-2.0-or-later
/* Procedures for interfacing to the RTAS on CHRP machines. */

use core::ffi::{c_char, c_int, c_void};

/* Kernel types and constants are supplied by the surrounding PowerPC port. */
#[repr(C)]
pub struct RtasFilter { pub buf_idx1: c_int, pub size_idx1: c_int, pub buf_idx2: c_int, pub size_idx2: c_int, pub fixed_size: c_int }
#[repr(C)]
pub struct RtasFunction { pub token: i32, pub banned_for_syscall_on_le: bool, pub name: *const c_char, pub filter: *const RtasFilter, pub lock: *mut c_void }

#[repr(C)]
pub struct RtasArgs { pub token: u32, pub nargs: u32, pub nret: u32, pub rets: *mut u32, pub args: [u32; 16] }
#[repr(C)]
pub struct Rtas { pub dev: *mut c_void, pub base: u64, pub entry: u64, pub size: u64 }

extern "C" {
    static mut rtas: Rtas;
    static mut rtas_args: RtasArgs;
    static mut rtas_rmo_buf: u64;
    fn enter_rtas(addr: u64);
    fn __pa(p: *const c_void) -> u64;
    fn mfmsr() -> u64;
    fn srr_regs_clobbered();
    fn raw_spin_lock_irqsave(lock: *mut c_void, flags: *mut u64);
    fn raw_spin_unlock_irqrestore(lock: *mut c_void, flags: u64);
    fn rtas_call_unlocked(args: *mut RtasArgs, token: c_int, nargs: c_int, nret: c_int, ...);
    fn rtas_call(token: c_int, nargs: c_int, nret: c_int, outputs: *mut c_int, ... ) -> c_int;
    fn udelay(usecs: u64);
    fn mdelay(msecs: u64);
    fn fsleep(usecs: u64);
    fn cond_resched();
    fn might_sleep();
    fn security_locked_down(reason: c_int) -> c_int;
    fn log_error(buf: *mut c_char, typ: c_int, err: c_int);
    fn kmalloc(size: usize, flags: u32) -> *mut c_char;
    fn kfree(p: *mut c_char);
    fn memmove(dst: *mut c_void, src: *const c_void, n: usize);
    fn warn_once(cond: bool, fmt: *const c_char, ...);
}

pub const RTAS_UNKNOWN_SERVICE: i32 = -1;
pub const RTAS_BUSY: i32 = -2;
pub const RTAS_EXTENDED_DELAY_MIN: i32 = 9900;
pub const RTAS_EXTENDED_DELAY_MAX: i32 = 9905;
pub const RTAS_HARDWARE_ERROR: i32 = -1;
pub const RTAS_INVALID_PARAMETER: i32 = -3;

/* The generated enum values are defined by asm/rtas.h. */
extern "C" {
    static mut rtas_function_table: RtasFunction;
}

#[inline]
pub unsafe fn rtas_function_token(handle: usize) -> i32 {
    if handle >= 64 { return RTAS_UNKNOWN_SERVICE; }
    if rtas.dev.is_null() { return RTAS_UNKNOWN_SERVICE; }
    (&rtas_function_table as *const RtasFunction).add(handle).read().token
}

unsafe fn __do_enter_rtas(args: *mut RtasArgs) {
    enter_rtas(__pa(args.cast()));
    srr_regs_clobbered();
}

unsafe fn do_enter_rtas(args: *mut RtasArgs) {
    let msr = mfmsr();
    /* Tracepoints are intentionally skipped in offline-CPU and real-mode paths. */
    const MSR_IR: u64 = 1 << 5;
    const MSR_DR: u64 = 1 << 4;
    const MSR_RI: u64 = 1 << 1;
    if msr & MSR_RI == 0 { return; }
    let _can_trace = (msr & (MSR_IR | MSR_DR)) == (MSR_IR | MSR_DR);
    __do_enter_rtas(args);
}

pub unsafe fn rtas_busy_delay_time(status: i32) -> u32 {
    if status == RTAS_BUSY { return 1; }
    if status < RTAS_EXTENDED_DELAY_MIN || status > RTAS_EXTENDED_DELAY_MAX { return 0; }
    let mut ms = 1u32;
    let mut order = (status - RTAS_EXTENDED_DELAY_MIN) as u32;
    while order != 0 { ms = ms.wrapping_mul(10); order -= 1; }
    ms
}

pub unsafe fn rtas_busy_delay(status: i32) -> bool {
    match status {
        RTAS_BUSY => { cond_resched(); true }
        RTAS_EXTENDED_DELAY_MIN..=RTAS_EXTENDED_DELAY_MAX => {
            let ms = rtas_busy_delay_time(status).clamp(1, 1000);
            fsleep((ms * 1000) as u64); true
        }
        _ => { might_sleep(); false }
    }
}

pub unsafe fn rtas_error_rc(rtas_rc: i32) -> i32 {
    match rtas_rc { -1 => -5, -3 => -22, -9000 => -14, -9001 => -17, -9002 => -19, _ => -34 }
}

pub unsafe fn rtas_get_power_level(powerdomain: i32, level: *mut i32) -> i32 {
    let token = rtas_function_token(4); if token == -1 { return -2; }
    let mut rc; loop { rc = rtas_call(token, 1, 2, level, powerdomain); if rc != RTAS_BUSY { break; } udelay(1); }
    if rc < 0 { rtas_error_rc(rc) } else { rc }
}

pub unsafe fn rtas_set_power_level(powerdomain: i32, level: i32, setlevel: *mut i32) -> i32 {
    let token = rtas_function_token(42); if token == -1 { return -2; }
    let mut rc; loop { rc = rtas_call(token, 2, 2, setlevel, powerdomain, level); if !rtas_busy_delay(rc) { break; } }
    if rc < 0 { rtas_error_rc(rc) } else { rc }
}

pub unsafe fn rtas_get_sensor(sensor: i32, index: i32, state: *mut i32) -> i32 {
    let token = rtas_function_token(5); if token == -1 { return -2; }
    let mut rc; loop { rc = rtas_call(token, 2, 2, state, sensor, index); if !rtas_busy_delay(rc) { break; } }
    if rc < 0 { rtas_error_rc(rc) } else { rc }
}

pub unsafe fn rtas_set_indicator(indicator: i32, index: i32, new_value: i32) -> i32 {
    let token = rtas_function_token(41); if token == -1 { return -2; }
    let mut rc; loop { rc = rtas_call(token, 3, 1, core::ptr::null_mut(), indicator, index, new_value); if !rtas_busy_delay(rc) { break; } }
    if rc < 0 { rtas_error_rc(rc) } else { rc }
}

pub unsafe fn rtas_ibm_suspend_me(fw_status: *mut i32) -> i32 {
    let fwrc = rtas_call(rtas_function_token(55), 0, 1, core::ptr::null_mut());
    if !fw_status.is_null() { *fw_status = fwrc; }
    match fwrc { 0 => 0, -9003 => -125, -9004 => -11, -9005 | -9006 => -16, _ => -5 }
}

pub unsafe fn rtas_restart(_cmd: *mut c_char) -> ! { loop {} }
pub unsafe fn rtas_power_off() -> ! { loop {} }
pub unsafe fn rtas_halt() -> ! { loop {} }

/* Remaining RTAS entry points retain the same ABI and are supplied by the kernel integration. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
