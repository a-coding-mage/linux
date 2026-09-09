// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of linux/kernel/panic.c. Kernel-provided declarations are
 * intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

const PANIC_TIMER_STEP: c_long = 100;
const PANIC_BLINK_SPD: c_long = 18;
const PANIC_MSG_BUFSZ: usize = 1024;
const PANIC_CPU_INVALID: c_int = -1;

#[repr(C)] pub struct pt_regs { _p: [u8; 0] }
#[repr(C)] pub struct seq_buf { _p: [u8; 0] }
#[repr(C)] pub struct ctl_table { _p: [u8; 0] }
#[repr(C)] pub struct kobject { _p: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { _p: [u8; 0] }
#[repr(C)] pub struct kernel_param { _p: [u8; 0] }
#[repr(C)] pub struct call_single_data_t { pub func: Option<unsafe extern "C" fn(*mut c_void)>, pub info: *mut c_void }
#[repr(C)] pub struct taint_flag { pub c_true: c_char, pub c_false: c_char, pub desc: *const c_char }
#[repr(C)] pub struct warn_args { pub fmt: *const c_char, pub args: *mut c_void }

extern "C" {
    fn raw_smp_processor_id() -> c_int; fn atomic_read(_: *const c_int) -> c_int;
    fn atomic_set(_: *mut c_int, _: c_int); fn atomic_try_cmpxchg(_: *mut c_int, _: *mut c_int, _: c_int) -> bool;
    fn panic_in_progress() -> bool; fn smp_send_stop(); fn cpu_relax(); fn local_irq_disable(); fn local_irq_enable();
    fn preempt_disable_notrace(); fn console_verbose(); fn bust_spinlocks(_: c_int); fn emergency_restart();
    fn mdelay(_: c_long); fn touch_nmi_watchdog(); fn touch_softlockup_watchdog(); fn dump_stack();
    fn kgdb_panic(_: *mut c_char); fn __crash_kexec(_: *mut c_void); fn console_unblank();
    fn debug_locks_off(); fn console_flush_on_panic(_: c_int); fn nbcon_atomic_flush_unsafe();
    fn atomic_notifier_call_chain(_: *mut c_void, _: c_int, _: *mut c_char); fn sys_info(_: c_ulong);
    fn kmsg_dump_desc(_: c_int, _: *mut c_char); fn printk_legacy_allow_panic_sync();
    fn set_cpu_online(_: c_int, _: bool); fn panic_smp_redirect_cpu(_: c_int, _: *mut c_void) -> c_int;
    fn vscnprintf(_: *mut c_char, _: usize, _: *const c_char, _: *mut c_void) -> c_long;
    fn pr_emerg(_: *const c_char, ...); fn pr_warn(_: *const c_char, ...); fn pr_info(_: *const c_char, ...);
    fn pr_info_once(_: *const c_char, ...); fn panic(_: *const c_char, ...);
    fn seq_buf_puts(_: *mut seq_buf, _: *const c_char); fn seq_buf_putc(_: *mut seq_buf, _: c_char);
    fn seq_buf_printf(_: *mut seq_buf, _: *const c_char, ...); fn seq_buf_init(_: *mut seq_buf, _: *mut c_char, _: usize);
    fn seq_buf_str(_: *mut seq_buf) -> *const c_char; fn strlen(_: *const c_char) -> usize;
    fn test_bit(_: c_int, _: *const c_ulong) -> bool; fn set_bit(_: c_int, _: *mut c_ulong);
    fn __debug_locks_off() -> bool; fn sysfs_emit(_: *mut c_char, _: *const c_char, ... ) -> c_long;
    fn disable_trace_on_warning(); fn tracing_off(); fn nbcon_cpu_emergency_enter(); fn nbcon_cpu_emergency_exit();
    fn print_modules(); fn show_regs(_: *mut pt_regs); fn vprintk(_: *const c_char, _: *mut c_void);
    fn print_irqtrace_events(_: *mut c_void); fn generic_bug_clear_once(); fn memset(_: *mut c_void, _: c_int, _: usize);
    fn user_access_save() -> c_ulong; fn user_access_restore(_: c_ulong); fn instrumentation_begin(); fn instrumentation_end();
    fn param_set_ulong(_: *const c_char, _: *const kernel_param) -> c_int; fn param_get_ulong(_: *mut c_char, _: *const kernel_param) -> c_int;
}

pub static mut panic_on_oops: c_int = 0;
static mut tainted_mask: c_ulong = 0;
static mut pause_on_oops: c_int = 0;
static mut pause_on_oops_flag: c_int = 0;
pub static mut crash_kexec_post_notifiers: bool = false;
pub static mut panic_on_warn: c_int = 0;
pub static mut panic_on_taint: c_ulong = 0;
pub static mut panic_on_taint_nousertaint: bool = false;
static mut warn_limit: u32 = 0;
static mut panic_console_replay: bool = false;
pub static mut panic_triggering_all_cpu_backtrace: bool = false;
static mut panic_this_cpu_backtrace_printed: bool = false;
pub static mut panic_timeout: c_int = 0;
pub static mut panic_print: c_ulong = 0;
static mut panic_force_cpu: c_int = -1;
pub static mut panic_cpu: c_int = PANIC_CPU_INVALID;
pub static mut panic_redirect_cpu: c_int = PANIC_CPU_INVALID;
pub static mut panic_blink: Option<unsafe extern "C" fn(c_int) -> c_long> = None;

unsafe extern "C" fn no_blink(_: c_int) -> c_long { 0 }
#[no_mangle] pub unsafe extern "C" fn panic_smp_self_stop() -> ! { loop { cpu_relax(); } }
#[no_mangle] pub unsafe extern "C" fn nmi_panic_self_stop(_: *mut pt_regs) -> ! { panic_smp_self_stop() }
#[no_mangle] pub unsafe extern "C" fn crash_smp_send_stop() { static mut stopped: bool = false; if stopped { return; } smp_send_stop(); stopped = true; }

#[no_mangle] pub unsafe extern "C" fn panic_try_start() -> bool {
    let mut old = PANIC_CPU_INVALID; atomic_try_cmpxchg(&raw mut panic_cpu, &mut old, raw_smp_processor_id())
}
#[no_mangle] pub unsafe extern "C" fn panic_reset() { atomic_set(&raw mut panic_cpu, PANIC_CPU_INVALID); }
#[no_mangle] pub unsafe extern "C" fn panic_in_progress_rs() -> bool { atomic_read(&raw const panic_cpu) != PANIC_CPU_INVALID }
#[no_mangle] pub unsafe extern "C" fn panic_on_this_cpu() -> bool { atomic_read(&raw const panic_cpu) == raw_smp_processor_id() }
#[no_mangle] pub unsafe extern "C" fn panic_on_other_cpu() -> bool { panic_in_progress_rs() && !panic_on_this_cpu() }

#[no_mangle] pub unsafe extern "C" fn nmi_panic(regs: *mut pt_regs, msg: *const c_char) { if panic_try_start() { panic(b"%s\0".as_ptr() as _, msg); } else if panic_on_other_cpu() { nmi_panic_self_stop(regs); } }

#[no_mangle] pub unsafe extern "C" fn vpanic(fmt: *const c_char, args: *mut c_void) -> ! {
    static mut buf: [c_char; PANIC_MSG_BUFSZ] = [0; PANIC_MSG_BUFSZ];
    local_irq_disable(); preempt_disable_notrace();
    if panic_try_start() { } else if panic_on_other_cpu() { panic_smp_self_stop(); }
    console_verbose(); bust_spinlocks(1); let len = vscnprintf(buf.as_mut_ptr(), PANIC_MSG_BUFSZ, fmt, args);
    if len > 0 && *buf.as_ptr().add((len - 1) as usize) == b'\n' as c_char { *buf.as_mut_ptr().add((len - 1) as usize) = 0; }
    pr_emerg(b"Kernel panic - not syncing: %s\n\0".as_ptr() as _, buf.as_mut_ptr());
    dump_stack(); kgdb_panic(buf.as_mut_ptr()); __crash_kexec(ptr::null_mut()); crash_smp_send_stop();
    atomic_notifier_call_chain(ptr::null_mut(), 0, buf.as_mut_ptr()); sys_info(panic_print); kmsg_dump_desc(0, buf.as_mut_ptr());
    console_unblank(); debug_locks_off(); console_flush_on_panic(0); nbcon_atomic_flush_unsafe();
    if panic_blink.is_none() { panic_blink = Some(no_blink); }
    if panic_timeout != 0 { emergency_restart(); }
    pr_emerg(b"---[ end Kernel panic - not syncing: %s ]---\n\0".as_ptr() as _, buf.as_mut_ptr());
    loop { touch_softlockup_watchdog(); mdelay(PANIC_TIMER_STEP); }
}

#[no_mangle] pub unsafe extern "C" fn panic(fmt: *const c_char, mut args: ...) -> ! { vpanic(fmt, &mut args as *mut _ as *mut c_void) }

#[no_mangle] pub unsafe extern "C" fn test_taint(flag: u32) -> c_int { test_bit(flag as c_int, &raw const tainted_mask) as c_int }
#[no_mangle] pub unsafe extern "C" fn get_taint() -> c_ulong { tainted_mask }
#[no_mangle] pub unsafe extern "C" fn add_taint(flag: u32, _: c_int) { set_bit(flag as c_int, &raw mut tainted_mask); if tainted_mask & panic_on_taint != 0 { panic_on_taint = 0; panic(b"panic_on_taint set ...\0".as_ptr() as _); } }

static mut pause_lock: c_int = 0;
#[no_mangle] pub unsafe extern "C" fn oops_may_print() -> bool { pause_on_oops_flag == 0 }
#[no_mangle] pub unsafe extern "C" fn oops_enter() { nbcon_cpu_emergency_enter(); tracing_off(); debug_locks_off(); if pause_on_oops != 0 { pause_on_oops_flag = 1; } }
#[no_mangle] pub unsafe extern "C" fn oops_exit() { if pause_on_oops != 0 { pause_on_oops_flag = 0; } nbcon_cpu_emergency_exit(); }

#[no_mangle] pub unsafe extern "C" fn check_panic_on_warn(origin: *const c_char) { if panic_on_warn != 0 { panic(b"%s: panic_on_warn set ...\n\0".as_ptr() as _, origin); } }

#[no_mangle] pub unsafe extern "C" fn __warn(file: *const c_char, line: c_int, caller: *mut c_void, taint: u32, regs: *mut pt_regs, args: *mut warn_args) {
    nbcon_cpu_emergency_enter(); disable_trace_on_warning(); if !file.is_null() { pr_warn(b"WARNING: %s:%d\n\0".as_ptr() as _, file, line); } else { pr_warn(b"WARNING: at %pS\n\0".as_ptr() as _, caller); }
    if !args.is_null() { vprintk((*args).fmt, (*args).args); } print_modules(); if !regs.is_null() { show_regs(regs); } check_panic_on_warn(b"kernel\0".as_ptr() as _); if regs.is_null() { dump_stack(); } add_taint(taint, 0); nbcon_cpu_emergency_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
