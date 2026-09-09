// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the surrounding kernel translation unit:
// `ELFCORE_ADDR_MAX`, `memparse`, and the `early_param` registration mechanism.

use core::ffi::c_char;

/// Stores the physical address of the ELF header of the crash image.
///
/// Note: `elfcorehdr_addr` is not just limited to vmcore. It is also used by
/// `is_kdump_kernel()` to determine if we are booting after a panic. Hence put
/// it under CONFIG_CRASH_DUMP and not CONFIG_PROC_VMCORE.
#[no_mangle]
pub static mut elfcorehdr_addr: u64 = ELFCORE_ADDR_MAX;

// EXPORT_SYMBOL_GPL(elfcorehdr_addr);

/// Stores the size of the ELF header of the crash image.
#[no_mangle]
pub static mut elfcorehdr_size: u64 = 0;

extern "C" {
    static ELFCORE_ADDR_MAX: u64;
    fn memparse(arg: *const c_char, retptr: *mut *mut c_char) -> u64;
}

/*
 * elfcorehdr= specifies the location of the ELF core header stored by the
 * crashed kernel. This option will be passed by the kexec loader to the
 * capture kernel.
 *
 * Syntax: elfcorehdr=[size[KMG]@]offset[KMG]
 */
#[no_mangle]
pub unsafe extern "C" fn setup_elfcorehdr(arg: *mut c_char) -> i32 {
    let mut end: *mut c_char;
    if arg.is_null() {
        return -EINVAL;
    }
    elfcorehdr_addr = memparse(arg, &mut end);
    if *end == b'@' as c_char {
        elfcorehdr_size = elfcorehdr_addr;
        elfcorehdr_addr = memparse(end.add(1), &mut end);
    }
    if end > arg {
        0
    } else {
        -EINVAL
    }
}

// early_param("elfcorehdr", setup_elfcorehdr);

extern "C" {
    static EINVAL: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
