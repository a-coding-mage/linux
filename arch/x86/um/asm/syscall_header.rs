/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding generic syscall and audit headers.

/// Opaque task structure supplied by the surrounding kernel translation.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

pub type sys_call_ptr_t = unsafe extern "C" fn(
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
) -> isize;

extern "C" {
    pub static sys_call_table: [sys_call_ptr_t; 0];
}

#[inline]
pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> i32 {
    // CONFIG_X86_32 selects AUDIT_ARCH_I386; otherwise AUDIT_ARCH_X86_64.
    #[cfg(CONFIG_X86_32)]
    {
        AUDIT_ARCH_I386
    }
    #[cfg(not(CONFIG_X86_32))]
    {
        AUDIT_ARCH_X86_64
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
