/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1999, 2000  Niibe Yutaka
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/stringify.h, linux/stddef.h, linux/thread_info.h,
// asm/addrspace.h, asm/page.h, and uapi/asm/ptrace.h.

#[inline]
pub unsafe fn user_mode(regs: *const pt_regs) -> bool {
    ((*regs).sr & 0x4000_0000) == 0
}

#[inline]
pub unsafe fn kernel_stack_pointer(regs: *const pt_regs) -> kernel::ffi::c_ulong {
    (*regs).regs[15] as kernel::ffi::c_ulong
}

#[inline]
pub unsafe fn instruction_pointer(regs: *const pt_regs) -> kernel::ffi::c_ulong {
    (*regs).pc
}

#[inline]
pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: kernel::ffi::c_ulong) {
    (*regs).pc = val;
}

#[inline]
pub unsafe fn frame_pointer(regs: *const pt_regs) -> kernel::ffi::c_ulong {
    (*regs).regs[14]
}

#[inline]
pub unsafe fn user_stack_pointer(regs: *const pt_regs) -> kernel::ffi::c_ulong {
    (*regs).regs[15]
}

#[inline]
pub unsafe fn user_stack_pointer_set(regs: *mut pt_regs, val: kernel::ffi::c_ulong) {
    (*regs).regs[15] = val;
}

#[inline]
pub const fn arch_has_single_step() -> kernel::ffi::c_int {
    1
}

#[repr(C)]
pub struct pt_regs_offset {
    pub name: *const kernel::ffi::c_char,
    pub offset: kernel::ffi::c_int,
}

// C macros REG_OFFSET_NAME, REGS_OFFSET_NAME, TREGS_OFFSET_NAME, and
// REG_OFFSET_END, expressed as Rust macros. `offset_of!` denotes the same
// field-offset operation as C offsetof.
#[macro_export]
macro_rules! REG_OFFSET_NAME {
    ($r:ident) => {
        $crate::pt_regs_offset {
            name: concat!(stringify!($r), "\0").as_ptr() as *const kernel::ffi::c_char,
            offset: ::core::mem::offset_of!($crate::pt_regs, $r) as kernel::ffi::c_int,
        }
    };
}

#[macro_export]
macro_rules! REGS_OFFSET_NAME {
    ($num:expr) => {
        $crate::pt_regs_offset {
            name: concat!("r", stringify!($num), "\0").as_ptr() as *const kernel::ffi::c_char,
            offset: ::core::mem::offset_of!($crate::pt_regs, regs[$num]) as kernel::ffi::c_int,
        }
    };
}

#[macro_export]
macro_rules! TREGS_OFFSET_NAME {
    ($num:expr) => {
        $crate::pt_regs_offset {
            name: concat!("tr", stringify!($num), "\0").as_ptr() as *const kernel::ffi::c_char,
            offset: ::core::mem::offset_of!($crate::pt_regs, tregs[$num]) as kernel::ffi::c_int,
        }
    };
}

#[macro_export]
macro_rules! REG_OFFSET_END {
    () => {
        $crate::pt_regs_offset {
            name: ::core::ptr::null(),
            offset: 0,
        }
    };
}

extern "C" {
    pub fn regs_query_register_offset(name: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn regs_query_register_name(offset: kernel::ffi::c_uint) -> *const kernel::ffi::c_char;
    pub static regoffset_table: pt_regs_offset;
}

#[inline]
pub unsafe fn regs_get_register(regs: *mut pt_regs, offset: kernel::ffi::c_uint) -> kernel::ffi::c_ulong {
    if offset > MAX_REG_OFFSET {
        return 0;
    }
    *((regs as *mut kernel::ffi::c_uchar).add(offset as usize) as *mut kernel::ffi::c_ulong)
}

#[inline]
pub unsafe fn regs_within_kernel_stack(regs: *mut pt_regs, addr: kernel::ffi::c_ulong) -> kernel::ffi::c_int {
    ((addr & !(THREAD_SIZE - 1)) == (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1))) as kernel::ffi::c_int
}

#[inline]
pub unsafe fn regs_get_kernel_stack_nth(
    regs: *mut pt_regs,
    n: kernel::ffi::c_uint,
) -> kernel::ffi::c_ulong {
    let addr = (kernel_stack_pointer(regs) as *mut kernel::ffi::c_ulong).add(n as usize);
    if regs_within_kernel_stack(regs, addr as kernel::ffi::c_ulong) != 0 {
        *addr
    } else {
        0
    }
}

pub struct perf_event;
pub struct perf_sample_data;

extern "C" {
    pub fn ptrace_triggered(
        bp: *mut perf_event,
        data: *mut perf_sample_data,
        regs: *mut pt_regs,
    );
}

#[macro_export]
macro_rules! task_pt_regs {
    ($task:expr) => {
        (($crate::task_stack_page($task) + THREAD_SIZE) as *mut $crate::pt_regs).sub(1)
    };
}

#[inline]
pub unsafe fn profile_pc(regs: *mut pt_regs) -> kernel::ffi::c_ulong {
    let pc = (*regs).pc;
    if virt_addr_uncached(pc) {
        CAC_ADDR(pc)
    } else {
        pc
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
