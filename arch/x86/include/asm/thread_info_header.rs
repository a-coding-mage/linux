/* SPDX-License-Identifier: GPL-2.0 */
/* thread_info.h: low-level thread information */

/* TOP_OF_KERNEL_STACK_PADDING depends on the target configuration. */
#[cfg(all(target_pointer_width = "32", feature = "config_vm86"))]
pub const TOP_OF_KERNEL_STACK_PADDING: usize = 16;
#[cfg(all(target_pointer_width = "32", not(feature = "config_vm86")))]
pub const TOP_OF_KERNEL_STACK_PADDING: usize = 8;
#[cfg(all(target_pointer_width = "64", feature = "config_x86_fred"))]
pub const TOP_OF_KERNEL_STACK_PADDING: usize = 2 * 8;
#[cfg(all(target_pointer_width = "64", not(feature = "config_x86_fred")))]
pub const TOP_OF_KERNEL_STACK_PADDING: usize = 0;

#[repr(C)]
pub struct task_struct;

#[repr(C)]
pub struct thread_info {
    pub flags: ::core::ffi::c_ulong,
    pub syscall_work: ::core::ffi::c_ulong,
    pub status: u32,
    #[cfg(feature = "config_smp")]
    pub cpu: u32,
}

#[macro_export]
macro_rules! INIT_THREAD_INFO {
    ($tsk:expr) => {{
        let _ = $tsk;
        $crate::thread_info { flags: 0, syscall_work: 0, status: 0 $(, cpu: 0)? }
    }};
}

/* Generic TIF infrastructure capabilities. */
pub const HAVE_TIF_NEED_RESCHED_LAZY: bool = true;
pub const HAVE_TIF_POLLING_NRFLAG: bool = true;
pub const HAVE_TIF_SINGLESTEP: bool = true;

/* Architecture specific TIF space starts at 16. */
pub const TIF_SSBD: usize = 16;
pub const TIF_SPEC_IB: usize = 17;
pub const TIF_SPEC_L1D_FLUSH: usize = 18;
pub const TIF_NEED_FPU_LOAD: usize = 19;
pub const TIF_NOCPUID: usize = 20;
pub const TIF_NOTSC: usize = 21;
pub const TIF_IO_BITMAP: usize = 22;
pub const TIF_SPEC_FORCE_UPDATE: usize = 23;
pub const TIF_FORCED_TF: usize = 24;
pub const TIF_SINGLESTEP: usize = 25;
pub const TIF_BLOCKSTEP: usize = 26;
pub const TIF_ADDR32: usize = 27;

#[inline]
pub const fn BIT(n: usize) -> ::core::ffi::c_ulong { 1 as ::core::ffi::c_ulong << n }

pub const _TIF_SSBD: ::core::ffi::c_ulong = BIT(TIF_SSBD);
pub const _TIF_SPEC_IB: ::core::ffi::c_ulong = BIT(TIF_SPEC_IB);
pub const _TIF_SPEC_L1D_FLUSH: ::core::ffi::c_ulong = BIT(TIF_SPEC_L1D_FLUSH);
pub const _TIF_NEED_FPU_LOAD: ::core::ffi::c_ulong = BIT(TIF_NEED_FPU_LOAD);
pub const _TIF_NOCPUID: ::core::ffi::c_ulong = BIT(TIF_NOCPUID);
pub const _TIF_NOTSC: ::core::ffi::c_ulong = BIT(TIF_NOTSC);
pub const _TIF_IO_BITMAP: ::core::ffi::c_ulong = BIT(TIF_IO_BITMAP);
pub const _TIF_SPEC_FORCE_UPDATE: ::core::ffi::c_ulong = BIT(TIF_SPEC_FORCE_UPDATE);
pub const _TIF_FORCED_TF: ::core::ffi::c_ulong = BIT(TIF_FORCED_TF);
pub const _TIF_BLOCKSTEP: ::core::ffi::c_ulong = BIT(TIF_BLOCKSTEP);
pub const _TIF_SINGLESTEP: ::core::ffi::c_ulong = BIT(TIF_SINGLESTEP);
pub const _TIF_ADDR32: ::core::ffi::c_ulong = BIT(TIF_ADDR32);

pub const _TIF_WORK_CTXSW_BASE: ::core::ffi::c_ulong =
    _TIF_NOCPUID | _TIF_NOTSC | _TIF_BLOCKSTEP | _TIF_SSBD | _TIF_SPEC_FORCE_UPDATE;
#[cfg(feature = "config_smp")]
pub const _TIF_WORK_CTXSW: ::core::ffi::c_ulong = _TIF_WORK_CTXSW_BASE | _TIF_SPEC_IB;
#[cfg(not(feature = "config_smp"))]
pub const _TIF_WORK_CTXSW: ::core::ffi::c_ulong = _TIF_WORK_CTXSW_BASE;
/* _TIF_USER_RETURN_NOTIFY is supplied by the generic TIF definitions. */
#[cfg(feature = "config_x86_iopl_ioperm")]
pub const _TIF_WORK_CTXSW_PREV: ::core::ffi::c_ulong = _TIF_WORK_CTXSW | _TIF_USER_RETURN_NOTIFY | _TIF_IO_BITMAP;
#[cfg(not(feature = "config_x86_iopl_ioperm"))]
pub const _TIF_WORK_CTXSW_PREV: ::core::ffi::c_ulong = _TIF_WORK_CTXSW | _TIF_USER_RETURN_NOTIFY;
pub const _TIF_WORK_CTXSW_NEXT: ::core::ffi::c_ulong = _TIF_WORK_CTXSW;

/* THREAD_SIZE is supplied by asm/page.h. */
pub const STACK_WARN: usize = THREAD_SIZE / 8;

pub const TS_COMPAT: u32 = 0x0002;
#[cfg(feature = "config_compat")]
pub const TS_I386_REGS_POKED: u32 = 0x0004;

pub unsafe fn arch_within_stack_frames(
    stack: *const ::core::ffi::c_void,
    stackend: *const ::core::ffi::c_void,
    obj: *const ::core::ffi::c_void,
    len: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    #[cfg(feature = "config_frame_pointer")]
    {
        let mut frame: *const ::core::ffi::c_void = core::ptr::null();
        let mut oldframe = __builtin_frame_address(1);
        if !oldframe.is_null() { frame = __builtin_frame_address(2); }
        while (stack as usize) <= (frame as usize) && (frame as usize) < (stackend as usize) {
            if (obj as usize).wrapping_add(len as usize) <= frame as usize {
                return if (obj as usize) >= (oldframe as usize).wrapping_add(2 * core::mem::size_of::<*const ::core::ffi::c_void>()) { GOOD_FRAME } else { BAD_STACK };
            }
            oldframe = frame;
            frame = *(frame as *const *const ::core::ffi::c_void);
        }
        BAD_STACK
    }
    #[cfg(not(feature = "config_frame_pointer"))]
    { NOT_STACK }
}

unsafe extern "C" {
    pub fn arch_setup_new_exec();
    fn __builtin_frame_address(level: ::core::ffi::c_int) -> *const ::core::ffi::c_void;
}

#[cfg(feature = "config_32bit")]
#[inline]
pub const fn in_ia32_syscall() -> bool { true }
#[cfg(not(feature = "config_32bit"))]
#[inline]
pub fn in_ia32_syscall() -> bool {
    cfg!(feature = "config_ia32_emulation") && (current_thread_info().status & TS_COMPAT != 0)
}

/* current_thread_info, THREAD_SIZE, TIF constants, and frame result constants
 * are supplied by the corresponding architecture/generic dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
