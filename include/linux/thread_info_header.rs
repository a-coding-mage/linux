/* SPDX-License-Identifier: GPL-2.0 */
/* thread_info.h: common low-level thread information accessors */

// C dependencies supplied by other translation units/headers:
// linux/types.h, linux/limits.h, linux/bug.h, linux/restart_block.h,
// linux/errno.h, linux/bitops.h, asm/current.h, and asm/thread_info.h.

#[repr(i32)]
pub enum StackFrameResult {
    BAD_STACK = -1,
    NOT_STACK = 0,
    GOOD_FRAME,
    GOOD_STACK,
}

// These declarations and constants are enabled by CONFIG_GENERIC_ENTRY in C.
#[repr(i32)]
pub enum SyscallWorkBit {
    SYSCALL_WORK_BIT_SECCOMP,
    SYSCALL_WORK_BIT_SYSCALL_TRACEPOINT,
    SYSCALL_WORK_BIT_SYSCALL_TRACE,
    SYSCALL_WORK_BIT_SYSCALL_EMU,
    SYSCALL_WORK_BIT_SYSCALL_AUDIT,
    SYSCALL_WORK_BIT_SYSCALL_USER_DISPATCH,
    SYSCALL_WORK_BIT_SYSCALL_EXIT_TRAP,
    SYSCALL_WORK_BIT_SYSCALL_RSEQ_SLICE,
}

// The BIT macro is supplied by linux/bitops.h.
pub const SYSCALL_WORK_SECCOMP: usize = BIT(SyscallWorkBit::SYSCALL_WORK_BIT_SECCOMP as usize);
pub const SYSCALL_WORK_SYSCALL_TRACEPOINT: usize = BIT(SyscallWorkBit::SYSCALL_WORK_BIT_SYSCALL_TRACEPOINT as usize);
pub const SYSCALL_WORK_SYSCALL_TRACE: usize = BIT(SyscallWorkBit::SYSCALL_WORK_BIT_SYSCALL_TRACE as usize);
pub const SYSCALL_WORK_SYSCALL_EMU: usize = BIT(SyscallWorkBit::SYSCALL_WORK_BIT_SYSCALL_EMU as usize);
pub const SYSCALL_WORK_SYSCALL_AUDIT: usize = BIT(SyscallWorkBit::SYSCALL_WORK_BIT_SYSCALL_AUDIT as usize);
pub const SYSCALL_WORK_SYSCALL_USER_DISPATCH: usize = BIT(SyscallWorkBit::SYSCALL_WORK_BIT_SYSCALL_USER_DISPATCH as usize);
pub const SYSCALL_WORK_SYSCALL_EXIT_TRAP: usize = BIT(SyscallWorkBit::SYSCALL_WORK_BIT_SYSCALL_EXIT_TRAP as usize);
pub const SYSCALL_WORK_SYSCALL_RSEQ_SLICE: usize = BIT(SyscallWorkBit::SYSCALL_WORK_BIT_SYSCALL_RSEQ_SLICE as usize);

// Architecture-provided constants and functions are external dependencies.
// The C fallback aliases TIF_NEED_RESCHED_LAZY to TIF_NEED_RESCHED and
// _TIF_NEED_RESCHED_LAZY to _TIF_NEED_RESCHED when required.
// TIF_RSEQ likewise aliases TIF_NOTIFY_RESUME when required.

#[cfg(feature = "kernel")]
pub unsafe fn set_restart_fn(
    restart: *mut restart_block,
    func: Option<unsafe extern "C" fn(*mut restart_block) -> c_long>,
) -> c_long {
    (*restart).fn_ = func;
    // arch_set_restart_data(restart) is an architecture hook; the C default is no-op.
    -ERESTART_RESTARTBLOCK
}

#[cfg(feature = "kernel")]
pub unsafe fn set_ti_thread_flag(ti: *mut thread_info, flag: c_int) {
    set_bit(flag, &mut (*ti).flags as *mut _ as *mut c_ulong);
}

#[cfg(feature = "kernel")]
pub unsafe fn clear_ti_thread_flag(ti: *mut thread_info, flag: c_int) {
    clear_bit(flag, &mut (*ti).flags as *mut _ as *mut c_ulong);
}

#[cfg(feature = "kernel")]
pub unsafe fn update_ti_thread_flag(ti: *mut thread_info, flag: c_int, value: bool) {
    if value { set_ti_thread_flag(ti, flag); } else { clear_ti_thread_flag(ti, flag); }
}

#[cfg(feature = "kernel")]
pub unsafe fn test_and_set_ti_thread_flag(ti: *mut thread_info, flag: c_int) -> c_int {
    test_and_set_bit(flag, &mut (*ti).flags as *mut _ as *mut c_ulong)
}

#[cfg(feature = "kernel")]
pub unsafe fn test_and_clear_ti_thread_flag(ti: *mut thread_info, flag: c_int) -> c_int {
    test_and_clear_bit(flag, &mut (*ti).flags as *mut _ as *mut c_ulong)
}

#[cfg(feature = "kernel")]
pub unsafe fn test_ti_thread_flag(ti: *mut thread_info, flag: c_int) -> c_int {
    test_bit(flag, &(*ti).flags as *const _ as *const c_ulong)
}

#[cfg(feature = "kernel")]
pub unsafe fn read_ti_thread_flags(ti: *mut thread_info) -> c_ulong {
    READ_ONCE((*ti).flags)
}

#[cfg(feature = "kernel")]
pub unsafe fn tif_test_bit(bit: c_int) -> bool {
    test_bit(bit, &(*current_thread_info()).flags as *const _ as *const c_ulong) != 0
}

#[cfg(feature = "kernel")]
pub unsafe fn tif_need_resched() -> bool { tif_test_bit(TIF_NEED_RESCHED) }

#[cfg(feature = "kernel")]
pub unsafe fn arch_within_stack_frames(
    _stack: *const c_void, _stackend: *const c_void,
    _obj: *const c_void, _len: c_ulong,
) -> c_int { 0 }

#[cfg(feature = "kernel")]
pub unsafe fn arch_setup_new_exec() {}

extern "C" {
    pub fn arch_task_cache_init();
    pub fn arch_release_task_struct(tsk: *mut task_struct);
    pub fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
