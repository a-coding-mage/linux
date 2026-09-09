// SPDX-License-Identifier: GPL-2.0
/*
 * System call table for UML/x86-64, copied from arch/x86/kernel/syscall_*.c
 * with some changes for UML.
 */

// Dependencies supplied by the Linux/UML build environment:
// linux/linkage.h, linux/sys.h, linux/cache.h, asm/syscall.h

pub type SysCallPtrT = unsafe extern "C" fn(
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
) -> isize;

pub type sys_call_ptr_t = SysCallPtrT;

pub const sys_iopl: sys_call_ptr_t = sys_ni_syscall;
pub const sys_ioperm: sys_call_ptr_t = sys_ni_syscall;

unsafe extern "C" {
    pub fn sys_ni_syscall(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> isize;
}

/*
 * Below you can see, in terms of #define's, the differences between the x86-64
 * and the UML syscall table.
 */

/* Not going to be implemented by UML, since we have no hardware. */

// __SYSCALL_NORETURN __SYSCALL

// The asm/syscalls_64.h include expands these declarations and table entries
// through the C __SYSCALL macro. Its contents are an external build dependency
// and are intentionally not reproduced here.

#[no_mangle]
pub static sys_call_table: [sys_call_ptr_t; 0] = [];

#[no_mangle]
pub static mut syscall_table_size: usize = core::mem::size_of::<[sys_call_ptr_t; 0]>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
