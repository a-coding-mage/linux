// SPDX-License-Identifier: GPL-2.0-only
/*
 * System call callback functions for SPUs
 */

// C headers and the generated syscall table are supplied by the surrounding
// kernel translation unit.

use core::ffi::c_void;

type SyscallFn = unsafe extern "C" fn(u64, u64, u64, u64, u64, u64) -> c_long;
type c_long = isize;

extern "C" {
    fn pr_debug(format: *const u8, ...);
}

// `struct spu_syscall_block` is declared by asm/spu.h.
#[repr(C)]
pub struct spu_syscall_block {
    pub nr_ret: u64,
    pub parm: [u64; 6],
}

// The entries are generated from asm/syscall_table_spu.h by the C
// __SYSCALL_WITH_COMPAT/__SYSCALL definitions.  Keep this local table's
// generated nature and index layout here for the surrounding translation.
static spu_syscall_table: &[Option<SyscallFn>] = &[];

#[inline]
unsafe fn array_size<T>(array: &[T]) -> usize {
    array.len()
}

pub unsafe extern "C" fn spu_sys_callback(s: *mut spu_syscall_block) -> c_long {
    let syscall: SyscallFn;

    if (*s).nr_ret >= array_size(spu_syscall_table) as u64 {
        pr_debug(b"%s: invalid syscall #%lld\0".as_ptr(),
                 b"spu_sys_callback\0".as_ptr(), (*s).nr_ret);
        return -(38 as c_long); // -ENOSYS
    }

    syscall = match spu_syscall_table[(*s).nr_ret as usize] {
        Some(function) => function,
        None => core::hint::unreachable_unchecked(),
    };

    pr_debug(b"SPU-syscall %pSR:syscall%lld(%llx, %llx, %llx, %llx, %llx, %llx)\n\0".as_ptr(),
             syscall as *const c_void,
             (*s).nr_ret,
             (*s).parm[0], (*s).parm[1], (*s).parm[2],
             (*s).parm[3], (*s).parm[4], (*s).parm[5]);

    syscall((*s).parm[0], (*s).parm[1], (*s).parm[2],
            (*s).parm[3], (*s).parm[4], (*s).parm[5])
}

// EXPORT_SYMBOL_GPL(spu_sys_callback);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
