/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/uaccess.h and linux/ptrace.h

#[repr(C)]
pub struct stack_frame {
    pub next_frame: *mut stack_frame,
    pub return_address: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct stacktrace_ops {
    pub address: Option<unsafe extern "C" fn(
        data: *mut ::core::ffi::c_void,
        address: ::core::ffi::c_ulong,
        reliable: ::core::ffi::c_int,
    )>,
}

#[cfg(CONFIG_FRAME_POINTER)]
#[inline]
pub unsafe fn get_frame_pointer(
    task: *mut task_struct,
    segv_regs: *mut pt_regs,
) -> ::core::ffi::c_ulong {
    if task.is_null() || task == current {
        if !segv_regs.is_null() {
            PT_REGS_BP(segv_regs)
        } else {
            current_bp()
        }
    } else {
        KSTK_EBP(task)
    }
}

#[cfg(not(CONFIG_FRAME_POINTER))]
#[inline]
pub unsafe fn get_frame_pointer(
    _task: *mut task_struct,
    _segv_regs: *mut pt_regs,
) -> ::core::ffi::c_ulong {
    0
}

#[inline]
pub unsafe fn get_stack_pointer(
    task: *mut task_struct,
    segv_regs: *mut pt_regs,
) -> *mut ::core::ffi::c_ulong {
    if task.is_null() || task == current {
        if !segv_regs.is_null() {
            PT_REGS_SP(segv_regs) as *mut ::core::ffi::c_ulong
        } else {
            current_sp()
        }
    } else {
        KSTK_ESP(task) as *mut ::core::ffi::c_ulong
    }
}

pub unsafe extern "C" fn dump_trace(
    tsk: *mut task_struct,
    ops: *const stacktrace_ops,
    data: *mut ::core::ffi::c_void,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
