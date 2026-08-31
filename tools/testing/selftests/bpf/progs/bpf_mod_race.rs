// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

pub type pid_t = i32;

#[repr(C)]
pub struct task_struct {
    pub tgid: pid_t,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_copy_from_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i64;
}

#[repr(C)]
pub struct bpf_mod_race_config_t {
    /* thread to activate trace programs for */
    pub tgid: pid_t,
    /* return error from __init function */
    pub inject_error: i32,
    /* uffd monitored range start address */
    pub fault_addr: *mut c_void,
}

#[no_mangle]
pub static mut bpf_mod_race_config: bpf_mod_race_config_t = bpf_mod_race_config_t {
    tgid: -1,
    inject_error: 0,
    fault_addr: core::ptr::null_mut(),
};

#[no_mangle]
pub static mut bpf_blocking: i32 = 0;

#[no_mangle]
pub static mut res_try_get_module: i32 = -1;

#[inline(always)]
unsafe fn check_thread_id() -> bool {
    let task: *mut task_struct = unsafe { bpf_get_current_task_btf() };

    unsafe { (*task).tgid == core::ptr::addr_of!(bpf_mod_race_config.tgid).read_volatile() }
}

/* The trace of execution is something like this:
 *
 * finit_module()
 *   load_module()
 *     prepare_coming_module()
 *       notifier_call(MODULE_STATE_COMING)
 *         btf_parse_module()
 *         btf_alloc_id()		// Visible to userspace at this point
 *         list_add(btf_mod->list, &btf_modules)
 *     do_init_module()
 *       freeinit = kmalloc()
 *       ret = mod->init()
 *         bpf_prog_widen_race()
 *           bpf_copy_from_user()
 *             ...<sleep>...
 *       if (ret < 0)
 *         ...
 *         free_module()
 * return ret
 *
 * At this point, module loading thread is blocked, we now load the program:
 *
 * bpf_check
 *   add_kfunc_call/check_pseudo_btf_id
 *     btf_try_get_module
 *       try_get_module_live == false
 *     return -ENXIO
 *
 * Without the fix (try_get_module_live in btf_try_get_module):
 *
 * bpf_check
 *   add_kfunc_call/check_pseudo_btf_id
 *     btf_try_get_module
 *       try_get_module == true
 *     <store module reference in btf_kfunc_tab or used_btf array>
 *   ...
 * return fd
 *
 * Now, if we inject an error in the blocked program, our module will be freed
 * (going straight from MODULE_STATE_COMING to MODULE_STATE_GOING).
 * Later, when bpf program is freed, it will try to module_put already freed
 * module. This is why try_get_module_live returns false if mod->state is not
 * MODULE_STATE_LIVE.
 */

#[no_mangle]
#[link_section = "fmod_ret.s/bpf_fentry_test1"]
pub unsafe extern "C" fn widen_race(a: i32, ret: i32) -> i32 {
    let mut dst: i8 = 0;

    if !unsafe { check_thread_id() } {
        return 0;
    }
    /* Indicate that we will attempt to block */
    unsafe {
        bpf_blocking = 1;
        bpf_copy_from_user(
            core::ptr::addr_of_mut!(dst).cast::<c_void>(),
            1,
            core::ptr::addr_of!(bpf_mod_race_config.fault_addr)
                .read_volatile()
                .cast_const(),
        );
        core::ptr::addr_of!(bpf_mod_race_config.inject_error).read_volatile()
    }
}

#[no_mangle]
#[link_section = "fexit/do_init_module"]
pub unsafe extern "C" fn fexit_init_module(mod_: *mut module, ret: i32) -> i32 {
    if !unsafe { check_thread_id() } {
        return 0;
    }
    /* Indicate that we finished blocking */
    unsafe {
        bpf_blocking = 2;
    }
    0
}

#[no_mangle]
#[link_section = "fexit/btf_try_get_module"]
pub unsafe extern "C" fn fexit_module_get(btf: *const btf, mod_: *mut module) -> i32 {
    unsafe {
        res_try_get_module = (mod_ != core::ptr::null_mut()) as i32;
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
