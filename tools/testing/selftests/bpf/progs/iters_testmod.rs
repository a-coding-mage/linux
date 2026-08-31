// SPDX-License-Identifier: GPL-2.0

// C dependencies from the original source:
// "vmlinux.h", "bpf_experimental.h", <bpf/bpf_helpers.h>, "bpf_misc.h",
// and "../test_kmods/bpf_testmod_kfunc.h".

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_task_vma {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_task {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_num {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;

    fn bpf_iter_task_vma_new(
        it: *mut bpf_iter_task_vma,
        task: *mut task_struct,
        flags: c_int,
    );
    fn bpf_iter_task_vma_next(it: *mut bpf_iter_task_vma) -> *mut vm_area_struct;
    fn bpf_iter_task_vma_destroy(it: *mut bpf_iter_task_vma);

    fn bpf_iter_task_new(it: *mut bpf_iter_task, task: *mut task_struct, flags: c_int);
    fn bpf_iter_task_next(it: *mut bpf_iter_task) -> *mut task_struct;
    fn bpf_iter_task_destroy(it: *mut bpf_iter_task);

    fn bpf_iter_num_new(it: *mut bpf_iter_num, start: c_int, end: c_int);
    fn bpf_iter_num_next(it: *mut bpf_iter_num) -> *mut c_int;
    fn bpf_iter_num_destroy(it: *mut bpf_iter_num);

    fn bpf_kfunc_trusted_vma_test(vma: *mut vm_area_struct);
    fn bpf_kfunc_rcu_task_test(task: *mut task_struct);
    fn bpf_kfunc_trusted_task_test(task: *mut task_struct);
    fn bpf_kfunc_trusted_num_test(num: *mut c_int);

    fn bpf_kfunc_ret_rcu_test() -> *mut task_struct;
    fn bpf_kfunc_ret_rcu_test_nostruct(arg: c_int) -> *mut c_void;
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn bpf_this_cpu_ptr(ptr: *mut c_void) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
// __success
pub unsafe extern "C" fn iter_next_trusted(ctx: *const c_void) -> c_int {
    let cur_task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let mut vma_it: bpf_iter_task_vma = core::mem::zeroed();
    let mut vma_ptr: *mut vm_area_struct;

    unsafe { bpf_iter_task_vma_new(&mut vma_it, cur_task, 0) };

    vma_ptr = unsafe { bpf_iter_task_vma_next(&mut vma_it) };
    if vma_ptr.is_null() {
        unsafe { bpf_iter_task_vma_destroy(&mut vma_it) };
        return 0;
    }

    unsafe { bpf_kfunc_trusted_vma_test(vma_ptr) };

    unsafe { bpf_iter_task_vma_destroy(&mut vma_it) };
    let _ = ctx;
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
// __failure __msg("Possibly NULL pointer passed to trusted R1")
pub unsafe extern "C" fn iter_next_trusted_or_null(ctx: *const c_void) -> c_int {
    let cur_task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let mut vma_it: bpf_iter_task_vma = core::mem::zeroed();
    let mut vma_ptr: *mut vm_area_struct;

    unsafe { bpf_iter_task_vma_new(&mut vma_it, cur_task, 0) };

    vma_ptr = unsafe { bpf_iter_task_vma_next(&mut vma_it) };

    unsafe { bpf_kfunc_trusted_vma_test(vma_ptr) };

    unsafe { bpf_iter_task_vma_destroy(&mut vma_it) };
    let _ = ctx;
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
// __success
pub unsafe extern "C" fn iter_next_rcu(ctx: *const c_void) -> c_int {
    let cur_task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let mut task_it: bpf_iter_task = core::mem::zeroed();
    let mut task_ptr: *mut task_struct;

    unsafe { bpf_iter_task_new(&mut task_it, cur_task, 0) };

    task_ptr = unsafe { bpf_iter_task_next(&mut task_it) };
    if task_ptr.is_null() {
        unsafe { bpf_iter_task_destroy(&mut task_it) };
        return 0;
    }

    unsafe { bpf_kfunc_rcu_task_test(task_ptr) };

    unsafe { bpf_iter_task_destroy(&mut task_it) };
    let _ = ctx;
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
// __failure __msg("Possibly NULL pointer passed to trusted R1")
pub unsafe extern "C" fn iter_next_rcu_or_null(ctx: *const c_void) -> c_int {
    let cur_task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let mut task_it: bpf_iter_task = core::mem::zeroed();
    let mut task_ptr: *mut task_struct;

    unsafe { bpf_iter_task_new(&mut task_it, cur_task, 0) };

    task_ptr = unsafe { bpf_iter_task_next(&mut task_it) };

    unsafe { bpf_kfunc_rcu_task_test(task_ptr) };

    unsafe { bpf_iter_task_destroy(&mut task_it) };
    let _ = ctx;
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
// __failure __msg("R1 must be referenced or trusted")
pub unsafe extern "C" fn iter_next_rcu_not_trusted(ctx: *const c_void) -> c_int {
    let cur_task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let mut task_it: bpf_iter_task = core::mem::zeroed();
    let mut task_ptr: *mut task_struct;

    unsafe { bpf_iter_task_new(&mut task_it, cur_task, 0) };

    task_ptr = unsafe { bpf_iter_task_next(&mut task_it) };
    if task_ptr.is_null() {
        unsafe { bpf_iter_task_destroy(&mut task_it) };
        return 0;
    }

    unsafe { bpf_kfunc_trusted_task_test(task_ptr) };

    unsafe { bpf_iter_task_destroy(&mut task_it) };
    let _ = ctx;
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
// __failure __msg("R1 cannot write into rdonly_mem")
// Message should not be 'R1 cannot write into rdonly_trusted_mem'
pub unsafe extern "C" fn iter_next_ptr_mem_not_trusted(ctx: *const c_void) -> c_int {
    let mut num_it: bpf_iter_num = core::mem::zeroed();
    let mut num_ptr: *mut c_int;

    unsafe { bpf_iter_num_new(&mut num_it, 0, 10) };

    num_ptr = unsafe { bpf_iter_num_next(&mut num_it) };
    if num_ptr.is_null() {
        unsafe { bpf_iter_num_destroy(&mut num_it) };
        return 0;
    }

    unsafe { bpf_kfunc_trusted_num_test(num_ptr) };

    unsafe { bpf_iter_num_destroy(&mut num_it) };
    let _ = ctx;
    0
}

// Original section: SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
// __failure __msg("kernel func bpf_kfunc_ret_rcu_test requires RCU critical section protection")
pub unsafe extern "C" fn iter_ret_rcu_test_protected(ctx: *const c_void) -> c_int {
    let mut p: *mut task_struct;

    p = unsafe { bpf_kfunc_ret_rcu_test() };
    let _ = ctx;
    unsafe { (*(p as *mut task_struct_pid_view)).pid }
}

#[repr(C)]
struct task_struct_pid_view {
    pid: c_int,
}

// Original section: SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
// __failure __msg("R1 type=rcu_ptr_or_null_ expected=")
pub unsafe extern "C" fn iter_ret_rcu_test_type(ctx: *const c_void) -> c_int {
    let mut p: *mut task_struct;

    unsafe { bpf_rcu_read_lock() };
    p = unsafe { bpf_kfunc_ret_rcu_test() };
    unsafe {
        bpf_this_cpu_ptr(p as *mut c_void);
    }
    unsafe { bpf_rcu_read_unlock() };
    let _ = ctx;
    0
}

// Original section: SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
// __failure __msg("kernel func bpf_kfunc_ret_rcu_test_nostruct requires RCU critical section protection")
pub unsafe extern "C" fn iter_ret_rcu_test_protected_nostruct(ctx: *const c_void) -> c_int {
    let mut p: *mut c_void;

    p = unsafe { bpf_kfunc_ret_rcu_test_nostruct(4) };
    let _ = ctx;
    unsafe { *(p as *mut c_int) }
}

// Original section: SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
// __failure __msg("R1 type=rdonly_rcu_mem_or_null expected=")
pub unsafe extern "C" fn iter_ret_rcu_test_type_nostruct(ctx: *const c_void) -> c_int {
    let mut p: *mut c_void;

    unsafe { bpf_rcu_read_lock() };
    p = unsafe { bpf_kfunc_ret_rcu_test_nostruct(4) };
    unsafe {
        bpf_this_cpu_ptr(p);
    }
    unsafe { bpf_rcu_read_unlock() };
    let _ = ctx;
    0
}
