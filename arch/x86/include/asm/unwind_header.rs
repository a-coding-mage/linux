/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: linux/sched.h, linux/ftrace.h, linux/rethook.h,
// asm/ptrace.h, and asm/stacktrace.h.

pub const IRET_FRAME_OFFSET: usize = core::mem::offset_of!(pt_regs, ip);
pub const IRET_FRAME_SIZE: usize = core::mem::size_of::<pt_regs>() - IRET_FRAME_OFFSET;

#[repr(C)]
pub struct unwind_state {
    pub stack_info: stack_info,
    pub stack_mask: ::core::ffi::c_ulong,
    pub task: *mut task_struct,
    pub graph_idx: ::core::ffi::c_int,
    // Present when CONFIG_RETHOOK is defined.
    #[cfg(feature = "CONFIG_RETHOOK")]
    pub kr_cur: *mut llist_node,
    pub error: bool,
    // CONFIG_UNWINDER_ORC variant.
    #[cfg(feature = "CONFIG_UNWINDER_ORC")]
    pub signal: bool,
    #[cfg(feature = "CONFIG_UNWINDER_ORC")]
    pub full_regs: bool,
    #[cfg(feature = "CONFIG_UNWINDER_ORC")]
    pub sp: ::core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_UNWINDER_ORC")]
    pub bp: ::core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_UNWINDER_ORC")]
    pub ip: ::core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_UNWINDER_ORC")]
    pub regs: *mut pt_regs,
    #[cfg(feature = "CONFIG_UNWINDER_ORC")]
    pub prev_regs: *mut pt_regs,
    // CONFIG_UNWINDER_FRAME_POINTER variant.
    #[cfg(all(not(feature = "CONFIG_UNWINDER_ORC"), feature = "CONFIG_UNWINDER_FRAME_POINTER"))]
    pub got_irq: bool,
    #[cfg(all(not(feature = "CONFIG_UNWINDER_ORC"), feature = "CONFIG_UNWINDER_FRAME_POINTER"))]
    pub bp: *mut ::core::ffi::c_ulong,
    #[cfg(all(not(feature = "CONFIG_UNWINDER_ORC"), feature = "CONFIG_UNWINDER_FRAME_POINTER"))]
    pub orig_sp: *mut ::core::ffi::c_ulong,
    #[cfg(all(not(feature = "CONFIG_UNWINDER_ORC"), feature = "CONFIG_UNWINDER_FRAME_POINTER"))]
    pub ip: ::core::ffi::c_ulong,
    /* If non-NULL, the current frame is incomplete and has no valid BP. */
    #[cfg(all(not(feature = "CONFIG_UNWINDER_ORC"), feature = "CONFIG_UNWINDER_FRAME_POINTER"))]
    pub next_bp: *mut ::core::ffi::c_ulong,
    #[cfg(all(not(feature = "CONFIG_UNWINDER_ORC"), feature = "CONFIG_UNWINDER_FRAME_POINTER"))]
    pub regs: *mut pt_regs,
    // Fallback variant when neither unwinder is configured.
    #[cfg(all(not(feature = "CONFIG_UNWINDER_ORC"), not(feature = "CONFIG_UNWINDER_FRAME_POINTER")))]
    pub sp: *mut ::core::ffi::c_ulong,
}

extern "C" {
    pub fn __unwind_start(state: *mut unwind_state, task: *mut task_struct,
                          regs: *mut pt_regs, first_frame: *mut ::core::ffi::c_ulong);
    pub fn unwind_next_frame(state: *mut unwind_state) -> bool;
    pub fn unwind_get_return_address(state: *mut unwind_state) -> ::core::ffi::c_ulong;
    pub fn unwind_get_return_address_ptr(state: *mut unwind_state) -> *mut ::core::ffi::c_ulong;
}

#[inline]
pub unsafe fn unwind_done(state: *mut unwind_state) -> bool {
    (*state).stack_info.type_ == STACK_TYPE_UNKNOWN
}

#[inline]
pub unsafe fn unwind_error(state: *mut unwind_state) -> bool {
    (*state).error
}

#[inline]
pub unsafe fn unwind_start(state: *mut unwind_state, task: *mut task_struct,
                           regs: *mut pt_regs, first_frame: *mut ::core::ffi::c_ulong) {
    let first_frame = if !first_frame.is_null() { first_frame } else { get_stack_pointer(task, regs) };
    __unwind_start(state, task, regs, first_frame);
}

#[cfg(any(feature = "CONFIG_UNWINDER_ORC", feature = "CONFIG_UNWINDER_FRAME_POINTER"))]
#[inline]
pub unsafe fn unwind_get_entry_regs(state: *mut unwind_state, partial: *mut bool) -> *mut pt_regs {
    if unwind_done(state) { return core::ptr::null_mut(); }
    if !partial.is_null() {
        #[cfg(feature = "CONFIG_UNWINDER_ORC")]
        { *partial = !(*state).full_regs; }
        #[cfg(all(not(feature = "CONFIG_UNWINDER_ORC"), feature = "CONFIG_UNWINDER_FRAME_POINTER"))]
        { *partial = false; }
    }
    (*state).regs
}

#[cfg(not(any(feature = "CONFIG_UNWINDER_ORC", feature = "CONFIG_UNWINDER_FRAME_POINTER")))]
#[inline]
pub unsafe fn unwind_get_entry_regs(_state: *mut unwind_state, _partial: *mut bool) -> *mut pt_regs { core::ptr::null_mut() }

#[cfg(feature = "CONFIG_UNWINDER_ORC")]
extern "C" {
    pub fn unwind_init();
    pub fn unwind_module_init(mod_: *mut module, orc_ip: *mut core::ffi::c_void, orc_ip_size: usize,
                              orc: *mut core::ffi::c_void, orc_size: usize);
}

#[cfg(not(feature = "CONFIG_UNWINDER_ORC"))]
#[inline] pub unsafe fn unwind_init() {}
#[cfg(not(feature = "CONFIG_UNWINDER_ORC"))]
#[inline] pub unsafe fn unwind_module_init(_mod: *mut module, _orc_ip: *mut core::ffi::c_void,
                                           _orc_ip_size: usize, _orc: *mut core::ffi::c_void, _orc_size: usize) {}

#[inline]
pub unsafe fn unwind_recover_rethook(state: *mut unwind_state, addr: ::core::ffi::c_ulong,
                                     addr_p: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    #[cfg(feature = "CONFIG_RETHOOK")]
    if is_rethook_trampoline(addr) {
        return rethook_find_ret_addr((*state).task, addr_p as ::core::ffi::c_ulong, &mut (*state).kr_cur);
    }
    addr
}

#[inline]
pub unsafe fn unwind_recover_ret_addr(state: *mut unwind_state, addr: ::core::ffi::c_ulong,
                                      addr_p: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let ret = ftrace_graph_ret_addr((*state).task, &mut (*state).graph_idx, addr, addr_p);
    unwind_recover_rethook(state, ret, addr_p)
}

// Disables KASAN checking when reading another task's stack.
#[macro_export]
macro_rules! READ_ONCE_TASK_STACK {
    ($task:expr, $x:expr) => {{
        if $task == current { READ_ONCE($x) } else { READ_ONCE_NOCHECK($x) }
    }};
}

#[inline]
pub unsafe fn task_on_another_cpu(task: *mut task_struct) -> bool {
    #[cfg(feature = "CONFIG_SMP")]
    { task != current && (*task).on_cpu }
    #[cfg(not(feature = "CONFIG_SMP"))]
    { false }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
