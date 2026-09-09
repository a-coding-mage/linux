// SPDX-License-Identifier: GPL-2.0
/*
 * Provide a default dump_stack() function for architectures
 * which don't implement their own.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Declarations supplied by the kernel headers and other translation units.
extern "C" {
    fn vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: *mut c_void) -> c_int;
    fn printk(fmt: *const c_char, ...);
    fn raw_smp_processor_id() -> c_int;
    fn current_real_cred() -> *const Cred;
    fn __kuid_val(uid: Kuid) -> c_uint;
    fn kexec_crash_loaded() -> bool;
    fn print_tainted() -> *const c_char;
    fn init_utsname() -> *const UtsName;
    fn strcspn(s: *const c_char, reject: *const c_char) -> usize;
    fn preempt_model_str() -> *const c_char;
    fn get_taint() -> c_ulong;
    fn print_tainted_verbose() -> *const c_char;
    fn print_worker_info(log_lvl: *const c_char, task: *mut TaskStruct);
    fn print_stop_info(log_lvl: *const c_char, task: *mut TaskStruct);
    fn print_scx_info(log_lvl: *const c_char, task: *mut TaskStruct);
    fn show_stack(task: *mut c_void, regs: *mut c_void, log_lvl: *const c_char);
    fn panic_on_this_cpu() -> bool;
    fn printk_cpu_sync_get_irqsave(flags: *mut c_ulong);
    fn printk_cpu_sync_put_irqrestore(flags: c_ulong);

    static mut current: *mut TaskStruct;
    static vmlinux_build_id: *const c_char;
}

#[repr(C)]
pub struct Kuid {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Cred {
    pub euid: Kuid,
}

#[repr(C)]
pub struct UtsName {
    pub release: [c_char; 65],
    pub version: [c_char; 65],
}

#[repr(C)]
pub struct TaskStruct {
    pub pid: c_int,
    pub comm: [c_char; 16],
}

static mut DUMP_STACK_ARCH_DESC_STR: [c_char; 128] = [0; 128];

/// dump_stack_set_arch_desc - set arch-specific str to show with task dumps
/// @fmt: printf-style format string
/// @...: arguments for the format string
pub unsafe extern "C" fn dump_stack_set_arch_desc(fmt: *const c_char, mut args: ...) {
    let _ = vsnprintf(
        DUMP_STACK_ARCH_DESC_STR.as_mut_ptr(),
        core::mem::size_of::<[c_char; 128]>(),
        fmt,
        &mut args as *mut _ as *mut c_void,
    );
}

#[cfg(CONFIG_STACKTRACE_BUILD_ID)]
const BUILD_ID_FMT: &[u8] = b" %20phN\0";
#[cfg(CONFIG_STACKTRACE_BUILD_ID)]
const BUILD_ID_VAL: *const c_char = unsafe { vmlinux_build_id };
#[cfg(not(CONFIG_STACKTRACE_BUILD_ID))]
const BUILD_ID_FMT: &[u8] = b"%s\0";
#[cfg(not(CONFIG_STACKTRACE_BUILD_ID))]
const BUILD_ID_VAL: *const c_char = b"\0".as_ptr() as *const c_char;

/// dump_stack_print_info - print generic debug info for dump_stack()
/// @log_lvl: log level
pub unsafe extern "C" fn dump_stack_print_info(log_lvl: *const c_char) {
    let _ = (BUILD_ID_FMT, BUILD_ID_VAL);
    let task = current;
    printk(
        b"%sCPU: %d UID: %u PID: %d Comm: %.20s %s%s %s %.*s %s %20phN\n\0".as_ptr() as *const c_char,
        log_lvl,
        raw_smp_processor_id(),
        __kuid_val((*current_real_cred()).euid),
        (*task).pid,
        (*task).comm.as_ptr(),
        if kexec_crash_loaded() { b"Kdump: loaded \0".as_ptr() } else { b"\0".as_ptr() },
        print_tainted(),
        (*init_utsname()).release.as_ptr(),
        strcspn((*init_utsname()).version.as_ptr(), b" \0".as_ptr()) as c_int,
        (*init_utsname()).version.as_ptr(),
        preempt_model_str(),
        BUILD_ID_VAL,
    );

    if get_taint() != 0 {
        printk(b"%s%s\n\0".as_ptr() as *const c_char, log_lvl, print_tainted_verbose());
    }
    if DUMP_STACK_ARCH_DESC_STR[0] != 0 {
        printk(b"%sHardware name: %s\n\0".as_ptr() as *const c_char, log_lvl, DUMP_STACK_ARCH_DESC_STR.as_ptr());
    }
    print_worker_info(log_lvl, task);
    print_stop_info(log_lvl, task);
    print_scx_info(log_lvl, task);
}

/// show_regs_print_info - print generic debug info for show_regs()
pub unsafe extern "C" fn show_regs_print_info(log_lvl: *const c_char) {
    dump_stack_print_info(log_lvl);
}

unsafe fn __dump_stack(log_lvl: *const c_char) {
    dump_stack_print_info(log_lvl);
    show_stack(core::ptr::null_mut(), core::ptr::null_mut(), log_lvl);
}

/// dump_stack_lvl - dump the current task information and its stack trace
pub unsafe extern "C" fn dump_stack_lvl(log_lvl: *const c_char) {
    let in_panic = panic_on_this_cpu();
    let mut flags: c_ulong = 0;
    if !in_panic {
        printk_cpu_sync_get_irqsave(&mut flags);
    }
    __dump_stack(log_lvl);
    if !in_panic {
        printk_cpu_sync_put_irqrestore(flags);
    }
}

pub unsafe extern "C" fn dump_stack() {
    dump_stack_lvl(b"\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
