// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub fn compat_arm_syscall(regs: *mut pt_regs, scno: ::core::ffi::c_int) -> ::core::ffi::c_long;
    pub fn sys_ni_syscall() -> ::core::ffi::c_long;
}

type syscall_fn_t = unsafe extern "C" fn(*mut pt_regs) -> ::core::ffi::c_long;

#[repr(C)]
pub struct pt_regs {
    pub regs: [u64; 31],
    pub orig_x0: u64,
    pub syscallno: ::core::ffi::c_int,
    pub pstate: u64,
}

extern "C" {
    fn is_compat_task() -> bool;
    fn add_random_kstack_offset();
    fn current() -> *mut ::core::ffi::c_void;
    fn syscall_set_return_value(
        task: *mut ::core::ffi::c_void,
        regs: *mut pt_regs,
        error: ::core::ffi::c_long,
        val: ::core::ffi::c_long,
    );
    fn read_thread_flags() -> ::core::ffi::c_ulong;
    fn syscall_trace_enter(regs: *mut pt_regs) -> ::core::ffi::c_int;
    fn syscall_trace_exit(regs: *mut pt_regs);
}

// External constants and tables supplied by the surrounding kernel translation.
extern "C" {
    static sys_call_table: [syscall_fn_t; __NR_syscalls as usize];
    #[cfg(CONFIG_COMPAT)]
    static compat_sys_call_table: [syscall_fn_t; __NR_compat32_syscalls as usize];
}

const ENOSYS: ::core::ffi::c_long = 38;
const ERESTARTNOINTR: ::core::ffi::c_long = 513;

// These values are supplied by the kernel headers in the original source.
extern "C" {
    static __NR_syscalls: ::core::ffi::c_uint;
    #[cfg(CONFIG_COMPAT)]
    static __NR_compat32_syscalls: ::core::ffi::c_uint;
}

const NO_SYSCALL: ::core::ffi::c_int = -1;

unsafe fn do_ni_syscall(regs: *mut pt_regs, scno: ::core::ffi::c_int) -> ::core::ffi::c_long {
    if is_compat_task() {
        let ret = compat_arm_syscall(regs, scno);
        if ret != -ENOSYS {
            return ret;
        }
    }

    sys_ni_syscall()
}

unsafe fn __invoke_syscall(regs: *mut pt_regs, syscall_fn: syscall_fn_t) -> ::core::ffi::c_long {
    syscall_fn(regs)
}

unsafe fn invoke_syscall(
    regs: *mut pt_regs,
    scno: ::core::ffi::c_uint,
    sc_nr: ::core::ffi::c_uint,
    syscall_table: *const syscall_fn_t,
) {
    let ret: ::core::ffi::c_long;

    add_random_kstack_offset();

    if scno < sc_nr {
        let syscall_fn = *syscall_table.add(scno as usize);
        ret = __invoke_syscall(regs, syscall_fn);
    } else {
        ret = do_ni_syscall(regs, scno as ::core::ffi::c_int);
    }

    syscall_set_return_value(current(), regs, 0, ret);
}

#[inline]
unsafe fn has_syscall_work(flags: ::core::ffi::c_ulong) -> bool {
    (flags & _TIF_SYSCALL_WORK) != 0
}

unsafe fn el0_svc_common(
    regs: *mut pt_regs,
    mut scno: ::core::ffi::c_int,
    sc_nr: ::core::ffi::c_int,
    syscall_table: *const syscall_fn_t,
) {
    let mut flags = read_thread_flags();

    (*regs).orig_x0 = (*regs).regs[0];
    (*regs).syscallno = scno;

    if (flags & _TIF_MTE_ASYNC_FAULT) != 0 {
        syscall_set_return_value(current(), regs, -ERESTARTNOINTR, 0);
        return;
    }

    if has_syscall_work(flags) {
        if scno == NO_SYSCALL {
            syscall_set_return_value(current(), regs, -ENOSYS, 0);
        }
        scno = syscall_trace_enter(regs);
        if scno == NO_SYSCALL {
            goto trace_exit;
        }
    }

    invoke_syscall(regs, scno as ::core::ffi::c_uint, sc_nr as ::core::ffi::c_uint, syscall_table);

    if !has_syscall_work(flags) && !cfg!(CONFIG_DEBUG_RSEQ) {
        flags = read_thread_flags();
        if !has_syscall_work(flags) && (flags & _TIF_SINGLESTEP) == 0 {
            return;
        }
    }

trace_exit:
    syscall_trace_exit(regs);
}

pub unsafe fn do_el0_svc(regs: *mut pt_regs) {
    el0_svc_common(regs, (*regs).regs[8] as ::core::ffi::c_int, __NR_syscalls as ::core::ffi::c_int, sys_call_table.as_ptr());
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn do_el0_svc_compat(regs: *mut pt_regs) {
    el0_svc_common(regs, (*regs).regs[7] as ::core::ffi::c_int, __NR_compat32_syscalls as ::core::ffi::c_int, compat_sys_call_table.as_ptr());
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
