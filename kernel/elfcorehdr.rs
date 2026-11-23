// SPDX-License-Identifier: GPL-2.0-only

// Dependencies: linux/kernel.h, linux/crash_dump.h, linux/init.h,
// linux/errno.h.
//
// The early_param("elfcorehdr") record stays in
// kernel/elfcorehdr_helpers.c; it calls setup_elfcorehdr() below.

use core::ptr::{addr_of_mut, null_mut};
use kernel::bindings::{memparse, EINVAL};
use kernel::ffi::{c_char, c_int, c_ulonglong};

/// ELFCORE_ADDR_MAX, `(-1ULL)`; kernel/elfcorehdr_helpers.c asserts it.
const ELFCORE_ADDR_MAX: c_ulonglong = c_ulonglong::MAX;

/*
 * stores the physical address of elf header of crash image
 *
 * Note: elfcorehdr_addr is not just limited to vmcore. It is also used by
 * is_kdump_kernel() to determine if we are booting after a panic. Hence put
 * it under CONFIG_CRASH_DUMP and not CONFIG_PROC_VMCORE.
 */
/// The physical address of the crash image's ELF header.
#[no_mangle]
pub static mut elfcorehdr_addr: c_ulonglong = ELFCORE_ADDR_MAX;
// EXPORT_SYMBOL_GPL(elfcorehdr_addr);

/*
 * stores the size of elf header of crash image
 */
/// The size of the crash image's ELF header.
#[no_mangle]
pub static mut elfcorehdr_size: c_ulonglong = 0;

/*
 * elfcorehdr= specifies the location of elf core header stored by the crashed
 * kernel. This option will be passed by kexec loader to the capture kernel.
 *
 * Syntax: elfcorehdr=[size[KMG]@]offset[KMG]
 */
/// # Safety
/// `arg` is NULL or the NUL-terminated parameter value; called during early
/// boot.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn setup_elfcorehdr(arg: *mut c_char) -> c_int {
    let mut end: *mut c_char = null_mut();

    if arg.is_null() {
        return -(EINVAL as c_int);
    }

    // SAFETY: memparse() parses the NUL-terminated argument and sets end.
    unsafe {
        *addr_of_mut!(elfcorehdr_addr) = memparse(arg, &mut end);
        if *end == b'@' {
            *addr_of_mut!(elfcorehdr_size) = *addr_of_mut!(elfcorehdr_addr);
            *addr_of_mut!(elfcorehdr_addr) = memparse(end.add(1), &mut end);
        }
    }

    if end > arg {
        0
    } else {
        -(EINVAL as c_int)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
