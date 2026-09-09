// SPDX-License-Identifier: GPL-2.0
/*
 * Stack trace management functions
 *
 *  Copyright IBM Corp. 2006
 */

// C includes provide the external kernel types, functions, constants, and
// build-time configuration symbols referenced below.

#[allow(non_camel_case_types)]
pub type stack_trace_consume_fn = unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> bool;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct unwind_state {
    pub stack_info: stack_info,
    pub regs: *mut pt_regs,
    pub ip: usize,
    _private: [u8; 0],
}
#[repr(C)]
pub struct stack_info {
    pub r#type: i32,
}
#[repr(C)]
pub struct perf_callchain_entry_ctx {
    _private: [u8; 0],
}
#[repr(C)]
pub struct stack_frame_vdso_wrapper {
    pub return_address: usize,
}
#[repr(C)]
pub struct stack_frame_user {
    pub back_chain: usize,
    pub gprs: [usize; 16],
}

extern "C" {
    static mmap_min_addr: usize;
    static current: *mut task_struct;
    static STACK_TYPE_TASK: i32;
    static STACK_FRAME_VDSO_OVERHEAD: usize;
    fn unwind_for_each_frame(state: *mut unwind_state, task: *mut task_struct,
                             regs: *mut pt_regs, flags: usize) -> bool;
    fn unwind_get_return_address(state: *mut unwind_state) -> usize;
    fn unwind_error(state: *mut unwind_state) -> bool;
    fn perf_callchain_store(entry: *mut perf_callchain_entry_ctx, ip: usize) -> i32;
    fn vdso_text_size() -> usize;
    fn in_range(value: usize, start: usize, size: usize) -> bool;
    fn instruction_pointer(regs: *const pt_regs) -> usize;
    fn user_stack_pointer(regs: *const pt_regs) -> usize;
    fn pagefault_disable();
    fn pagefault_enable();
    fn __get_user(value: *mut usize, ptr: *const usize) -> i32;
    #[cfg(CONFIG_RETHOOK)]
    fn arch_rethook_trampoline();
}

pub unsafe fn arch_stack_walk(consume_entry: stack_trace_consume_fn, cookie: *mut core::ffi::c_void,
                              task: *mut task_struct, regs: *mut pt_regs) {
    let mut state: unwind_state = core::mem::zeroed();
    let mut addr: usize;

    // unwind_for_each_frame(&state, task, regs, 0) {
    while unwind_for_each_frame(&mut state, task, regs, 0) {
        addr = unwind_get_return_address(&mut state);
        if addr == 0 || !consume_entry(cookie, addr) {
            break;
        }
    }
}

pub unsafe fn arch_stack_walk_reliable(consume_entry: stack_trace_consume_fn,
                                       cookie: *mut core::ffi::c_void,
                                       task: *mut task_struct) -> i32 {
    let mut state: unwind_state = core::mem::zeroed();
    let mut addr: usize;

    while unwind_for_each_frame(&mut state, task, core::ptr::null_mut(), 0) {
        if state.stack_info.r#type != STACK_TYPE_TASK {
            return -22;
        }
        if !state.regs.is_null() {
            return -22;
        }
        addr = unwind_get_return_address(&mut state);
        if addr == 0 {
            return -22;
        }

        #[cfg(CONFIG_RETHOOK)]
        {
            /*
             * Mark stacktraces with krethook functions on them
             * as unreliable.
             */
            if state.ip == arch_rethook_trampoline as usize {
                return -22;
            }
        }

        if !consume_entry(cookie, addr) {
            return -22;
        }
    }

    /* Check for stack corruption */
    if unwind_error(&mut state) {
        return -22;
    }
    0
}

unsafe fn store_ip(consume_entry: stack_trace_consume_fn, cookie: *mut core::ffi::c_void,
                   entry: *mut perf_callchain_entry_ctx, perf: bool, ip: usize) -> bool {
    #[cfg(CONFIG_PERF_EVENTS)]
    if perf {
        if perf_callchain_store(entry, ip) != 0 {
            return false;
        }
        return true;
    }
    consume_entry(cookie, ip)
}

unsafe fn ip_invalid(ip: usize) -> bool {
    /*
     * Perform some basic checks if an instruction address taken
     * from unreliable source is invalid.
     */
    if ip & 1 != 0 {
        return true;
    }
    if ip < mmap_min_addr {
        return true;
    }
    // current->mm->context.asce_limit
    if ip >= (*current).mm().context().asce_limit {
        return true;
    }
    false
}

unsafe fn ip_within_vdso(ip: usize) -> bool {
    in_range(ip, (*current).mm().context().vdso_base, vdso_text_size())
}

pub unsafe fn arch_stack_walk_user_common(consume_entry: stack_trace_consume_fn,
                                          cookie: *mut core::ffi::c_void,
                                          entry: *mut perf_callchain_entry_ctx,
                                          regs: *const pt_regs, perf: bool) {
    let mut sf_vdso: *mut stack_frame_vdso_wrapper;
    let mut sf: *mut stack_frame_user;
    let mut ip: usize;
    let mut sp: usize;

    if (*current).mm().is_null() {
        return;
    }
    ip = instruction_pointer(regs);
    if !store_ip(consume_entry, cookie, entry, perf, ip) {
        return;
    }
    sf = user_stack_pointer(regs) as *mut stack_frame_user;
    pagefault_disable();
    loop {
        if __get_user(&mut sp, core::ptr::addr_of!((*sf).back_chain)) != 0 {
            break;
        }
        /*
         * VDSO entry code has a non-standard stack frame layout.
         * See VDSO user wrapper code for details.
         */
        if sp == 0 && ip_within_vdso(ip) {
            sf_vdso = sf as *mut stack_frame_vdso_wrapper;
            if __get_user(&mut ip, core::ptr::addr_of!((*sf_vdso).return_address)) != 0 {
                break;
            }
            sp = sf as usize + STACK_FRAME_VDSO_OVERHEAD;
            sf = sp as *mut stack_frame_user;
            if __get_user(&mut sp, core::ptr::addr_of!((*sf).back_chain)) != 0 {
                break;
            }
        } else {
            sf = sp as *mut stack_frame_user;
            if __get_user(&mut ip, core::ptr::addr_of!((*sf).gprs[8])) != 0 {
                break;
            }
        }
        /* Validate SP and RA (ABI requires SP to be 8 byte aligned). */
        if sp & 0x7 != 0 || ip_invalid(ip) {
            break;
        }
        if !store_ip(consume_entry, cookie, entry, perf, ip) {
            break;
        }
    }
    pagefault_enable();
}

pub unsafe fn arch_stack_walk_user(consume_entry: stack_trace_consume_fn,
                                   cookie: *mut core::ffi::c_void,
                                   regs: *const pt_regs) {
    arch_stack_walk_user_common(consume_entry, cookie, core::ptr::null_mut(), regs, false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
