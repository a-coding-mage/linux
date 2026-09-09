// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of linux/init/main.c.  Kernel-provided declarations
 * and configuration macros are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

type size_t = usize;
type uint = u32;
type u8_t = u8;
type u32_t = u32;
type ulong = usize;
type ktime_t = i64;
type initcall_t = unsafe extern "C" fn() -> c_int;
type initcall_entry_t = c_void;

extern "C" {
    static mut early_boot_irqs_disabled: bool;
    static mut system_state: c_int;
    static mut late_time_init: Option<unsafe extern "C" fn()>;
    static mut boot_command_line: [c_char; 0];
    static mut saved_command_line: *mut c_char;
    static mut saved_command_line_len: uint;
    static mut static_command_line: *mut c_char;
    static mut extra_command_line: *mut c_char;
    static mut extra_init_args: *mut c_char;
    static mut execute_command: *mut c_char;
    static mut ramdisk_execute_command: *mut c_char;
    static mut reset_devices: uint;
    static mut console_loglevel: c_int;
    static mut loops_per_jiffy: ulong;
    static mut argv_init: [*mut c_char; 0];
    static mut envp_init: [*mut c_char; 0];
    static mut panic_later: *const c_char;
    static mut panic_param: *const c_char;
}

extern "C" {
    fn kernel_init(_: *mut c_void) -> !;
    fn strlen(_: *const c_char) -> size_t;
    fn strcmp(_: *const c_char, _: *const c_char) -> c_int;
    fn strncmp(_: *const c_char, _: *const c_char, _: size_t) -> c_int;
    fn memcmp(_: *const c_void, _: *const c_void, _: size_t) -> c_int;
    fn memcpy(_: *mut c_void, _: *const c_void, _: size_t) -> *mut c_void;
    fn memmove(_: *mut c_void, _: *const c_void, _: size_t) -> *mut c_void;
    fn strcpy(_: *mut c_char, _: *const c_char) -> *mut c_char;
    fn strchr(_: *const c_char, _: c_int) -> *mut c_char;
    fn strstarts(_: *const c_char, _: *const c_char) -> bool;
    fn strnchr(_: *const c_char, _: size_t, _: c_int) -> *mut c_char;
    fn strim(_: *mut c_char) -> *mut c_char;
    fn get_option(_: *mut *mut c_char, _: *mut c_int) -> c_int;
    fn sysctl_is_alias(_: *const c_char) -> bool;
    fn obsolete_checksetup(_: *mut c_char) -> bool;
    fn memblock_alloc_or_panic(_: size_t, _: size_t) -> *mut c_char;
    fn memblock_alloc(_: size_t, _: size_t) -> *mut c_char;
    fn memblock_free(_: *mut c_char, _: size_t);
    fn parse_args(_: *const c_char, _: *mut c_char, _: *mut c_void, _: size_t, _: c_int, _: c_int, _: *mut c_void, _: *mut c_void) -> *mut c_char;
    fn kernel_execve(_: *const c_char, _: *const *mut c_char, _: *const *mut c_char) -> c_int;
    fn pr_err(_: *const c_char, ...);
    fn pr_warn(_: *const c_char, ...);
    fn pr_info(_: *const c_char, ...);
    fn pr_notice(_: *const c_char, ...);
    fn pr_debug(_: *const c_char, ...);
    fn panic(_: *const c_char, ... ) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn set_reset_devices(_: *mut c_char) -> c_int { reset_devices = 1; 1 }

#[no_mangle]
pub unsafe extern "C" fn debug_kernel(_: *mut c_char) -> c_int { console_loglevel = 7; 0 }
#[no_mangle]
pub unsafe extern "C" fn quiet_kernel(_: *mut c_char) -> c_int { console_loglevel = 4; 0 }
#[no_mangle]
pub unsafe extern "C" fn loglevel(mut s: *mut c_char) -> c_int {
    let mut n = 0; if get_option(&mut s, &mut n) != 0 { console_loglevel = n; 0 } else { -22 }
}

#[no_mangle]
pub unsafe extern "C" fn cmdline_has_extra_options() -> bool {
    !extra_command_line.is_null() || !extra_init_args.is_null()
}

unsafe fn repair_env_string(param: *mut c_char, val: *mut c_char) {
    if !val.is_null() {
        let n = strlen(param);
        if val == param.add(n + 1) { *val.sub(1) = b'=' as c_char; }
        else if val == param.add(n + 2) { *val.sub(2) = b'=' as c_char; memmove(val.sub(1) as *mut c_void, val as *const c_void, strlen(val)+1); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_init_arg(param: *mut c_char, val: *mut c_char, _: *const c_char, _: *mut c_void) -> c_int {
    if !panic_later.is_null() { return 0; } repair_env_string(param, val); 0
}

#[no_mangle]
pub unsafe extern "C" fn unknown_bootoption(param: *mut c_char, val: *mut c_char, _: *const c_char, _: *mut c_void) -> c_int {
    let len = strlen(param); if sysctl_is_alias(param) { return 0; } repair_env_string(param, val);
    let bootloader = [b"BOOT_IMAGE=".as_ptr() as *const c_char, b"kexec\0".as_ptr() as *const c_char, core::ptr::null()];
    for p in bootloader { if !p.is_null() && strstarts(param,p) { return 0; } }
    if obsolete_checksetup(param) || !strnchr(param,len,b'.' as c_int).is_null() || !panic_later.is_null() { return 0; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn init_setup(s: *mut c_char) -> c_int { execute_command=s; 1 }
#[no_mangle]
pub unsafe extern "C" fn rdinit_setup(s: *mut c_char) -> c_int { ramdisk_execute_command=s; 1 }

#[no_mangle]
pub unsafe extern "C" fn parse_early_options(cmdline: *mut c_char) { parse_args(b"early options\0".as_ptr() as *const c_char,cmdline,core::ptr::null_mut(),0,0,0,core::ptr::null_mut(),core::ptr::null_mut()); }

#[no_mangle]
pub unsafe extern "C" fn run_init_process(name: *const c_char) -> c_int {
    pr_info(b"Run %s as init process\n\0".as_ptr() as *const c_char,name); kernel_execve(name, argv_init.as_ptr(), envp_init.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn try_to_run_init_process(name: *const c_char) -> c_int {
    let ret=run_init_process(name); if ret != 0 && ret != -2 { pr_err(b"Starting init: %s exists but couldn't execute it (error %d)\n\0".as_ptr() as *const c_char,name,ret); } ret
}

#[no_mangle]
pub unsafe extern "C" fn console_on_rootfs() { }

#[no_mangle]
pub unsafe extern "C" fn do_one_initcall(fn_: initcall_t) -> c_int { fn_() }

#[no_mangle]
pub unsafe extern "C" fn start_kernel() -> ! {
    pr_notice(b"%s\0".as_ptr() as *const c_char, b"Linux\0".as_ptr());
    panic(b"start_kernel requires the Linux kernel runtime\0".as_ptr() as *const c_char)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
