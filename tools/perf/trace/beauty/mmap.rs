// SPDX-License-Identifier: LGPL-2.1

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct strarray {
    pub prefix: *const c_char,
    pub nr_entries: usize,
    pub entries: *const *const c_char,
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub mask: c_ulong,
    pub show_string_prefix: bool,
}

unsafe extern "C" {
    // From trace/beauty/beauty.h and generated strarray sources.
    static strarray__mmap_prot: strarray;
    static strarray__mmap_flags: strarray;
    static strarray__mremap_flags: strarray;

    // Local static created in C inside madvise__scnprintf_behavior after
    // including trace/beauty/generated/madvise_behavior_array.c.
    static strarray__madvise_advices: strarray;

    // Macro constants supplied by Linux headers in the original C translation unit.
    static MAP_ANONYMOUS: c_ulong;
    static MREMAP_FIXED: c_ulong;

    fn strarray__scnprintf_flags(
        sa: *const strarray,
        bf: *mut c_char,
        size: usize,
        show_prefix: bool,
        flags: c_ulong,
    ) -> usize;

    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
}

unsafe fn mmap__scnprintf_prot(
    prot: c_ulong,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    unsafe { strarray__scnprintf_flags(&raw const strarray__mmap_prot, bf, size, show_prefix, prot) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_mmap_prot(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let prot: c_ulong = unsafe { (*arg).val };

    if prot == 0 {
        return unsafe {
            scnprintf(
                bf,
                size,
                c"%sNONE".as_ptr(),
                if (*arg).show_string_prefix {
                    strarray__mmap_prot.prefix
                } else {
                    c"".as_ptr()
                },
            )
        };
    }

    unsafe { mmap__scnprintf_prot(prot, bf, size, (*arg).show_string_prefix) }
}

unsafe fn mmap__scnprintf_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    unsafe { strarray__scnprintf_flags(&raw const strarray__mmap_flags, bf, size, show_prefix, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_mmap_flags(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let flags: c_ulong = unsafe { (*arg).val };

    if unsafe { flags & MAP_ANONYMOUS } != 0 {
        unsafe {
            (*arg).mask |= (1 << 4) | (1 << 5);
        } /* Mask 4th ('fd') and 5th ('offset') args, ignored */
    }

    unsafe { mmap__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix) }
}

unsafe fn mremap__scnprintf_flags(
    flags: c_ulong,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    unsafe { strarray__scnprintf_flags(&raw const strarray__mremap_flags, bf, size, show_prefix, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_mremap_flags(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let flags: c_ulong = unsafe { (*arg).val };

    if unsafe { flags & MREMAP_FIXED } == 0 {
        unsafe {
            (*arg).mask |= 1 << 5;
        } /* Mask 5th ('new_address') args, ignored */
    }

    unsafe { mremap__scnprintf_flags(flags, bf, size, (*arg).show_string_prefix) }
}

unsafe fn madvise__scnprintf_behavior(behavior: c_int, bf: *mut c_char, size: usize) -> usize {
    if (behavior as usize) < unsafe { strarray__madvise_advices.nr_entries }
        && unsafe { *strarray__madvise_advices.entries.add(behavior as usize) }.is_null() == false
    {
        return unsafe {
            scnprintf(
                bf,
                size,
                c"MADV_%s".as_ptr(),
                *strarray__madvise_advices.entries.add(behavior as usize),
            )
        };
    }

    unsafe { scnprintf(bf, size, c"%#".as_ptr(), behavior) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_madvise_behavior(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    unsafe { madvise__scnprintf_behavior((*arg).val as c_int, bf, size) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
