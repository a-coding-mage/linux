/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied externally: linux/sched/task_stack.h

pub struct task_struct; /* one of the stranger aspects of C forward declarations */

extern "C" {
    pub fn __switch_to_asm(
        prev: *mut task_struct,
        next: *mut task_struct,
    ) -> *mut task_struct;

    // __visible
    pub fn __switch_to(
        prev: *mut task_struct,
        next: *mut task_struct,
    ) -> *mut task_struct;

    // asmlinkage
    pub fn ret_from_fork_asm();

    // __visible
    pub fn ret_from_fork(
        prev: *mut task_struct,
        regs: *mut pt_regs,
        fun: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
        fn_arg: *mut core::ffi::c_void,
    );
}

#[repr(C)]
pub struct inactive_task_frame {
    #[cfg(feature = "CONFIG_X86_64")]
    pub r15: c_ulong,
    #[cfg(feature = "CONFIG_X86_64")]
    pub r14: c_ulong,
    #[cfg(feature = "CONFIG_X86_64")]
    pub r13: c_ulong,
    #[cfg(feature = "CONFIG_X86_64")]
    pub r12: c_ulong,
    #[cfg(feature = "CONFIG_X86_32")]
    pub flags: c_ulong,
    #[cfg(feature = "CONFIG_X86_32")]
    pub si: c_ulong,
    #[cfg(feature = "CONFIG_X86_32")]
    pub di: c_ulong,
    pub bx: c_ulong,
    /* These two fields form a stack frame header needed by get_frame_pointer(). */
    pub bp: c_ulong,
    pub ret_addr: c_ulong,
}

#[repr(C)]
pub struct fork_frame {
    pub frame: inactive_task_frame,
    pub regs: pt_regs,
}

#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        $last = unsafe { $crate::__switch_to_asm($prev, $next) };
    }};
}

#[cfg(feature = "CONFIG_X86_32")]
pub unsafe fn refresh_sysenter_cs(thread: *mut thread_struct) {
    /* Only happens when SEP is enabled, no need to test "SEP"arately: */
    if unlikely(this_cpu_read(cpu_tss_rw.x86_tss.ss1) == (*thread).sysenter_cs) {
        return;
    }

    this_cpu_write(cpu_tss_rw.x86_tss.ss1, (*thread).sysenter_cs);
    wrmsrq(MSR_IA32_SYSENTER_CS, (*thread).sysenter_cs);
}

/* This is used when switching tasks or entering/exiting vm86 mode. */
pub unsafe fn update_task_stack(task: *mut task_struct) {
    /* sp0 always points to the entry trampoline stack, which is constant: */
    #[cfg(feature = "CONFIG_X86_32")]
    {
        this_cpu_write(cpu_tss_rw.x86_tss.sp1, (*task).thread.sp0);
    }
    #[cfg(not(feature = "CONFIG_X86_32"))]
    {
        if !cpu_feature_enabled(X86_FEATURE_FRED) && cpu_feature_enabled(X86_FEATURE_XENPV) {
            /* Xen PV enters the kernel on the thread stack. */
            load_sp0(task_top_of_stack(task));
        }
    }
}

pub unsafe fn kthread_frame_init(
    frame: *mut inactive_task_frame,
    fun: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
    arg: *mut core::ffi::c_void,
) {
    (*frame).bx = fun.map_or(0, |f| f as usize as c_ulong);
    #[cfg(feature = "CONFIG_X86_32")]
    {
        (*frame).di = arg as usize as c_ulong;
    }
    #[cfg(not(feature = "CONFIG_X86_32"))]
    {
        (*frame).r12 = arg as usize as c_ulong;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
