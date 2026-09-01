// SPDX-License-Identifier: GPL-2.0-only
/*
 * Get values of vector registers as soon as the program starts to test if
 * is properly cleaning the values before starting a new program. Vector
 * registers are caller saved, so no function calls may happen before reading
 * the values. To further ensure consistency, this file is compiled without
 * libc and without auto-vectorization.
 *
 * To be "clean" all values must be all zeroes.
 */

use core::arch::asm;
use core::ffi::{c_char, c_int};

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

macro_rules! check_vector_register {
    ($register:ident, $vl:expr, $value:ident) => {{
        for _i in 0..$vl {
            unsafe {
                asm!(
                    ".option push",
                    ".option arch, +v",
                    concat!("vmv.x.s {value}, ", stringify!($register)),
                    concat!(
                        "vsrl.vi ",
                        stringify!($register),
                        ", ",
                        stringify!($register),
                        ", 8"
                    ),
                    ".option pop",
                    value = out(reg) $value,
                    options(nostack, preserves_flags),
                );
            }
            if $value != 0x00 {
                unsafe {
                    printf(
                        concat!(
                            "Register ",
                            stringify!($register),
                            " values not clean! value: %u\n\0"
                        )
                        .as_ptr() as *const c_char,
                        $value as c_int,
                    );
                    exit(-1);
                }
            }
        }
    }};
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut value: c_char = 0;
    let vl: usize;

    if argc > 2 && strcmp(*argv.add(2), c"x".as_ptr()) != 0 {
        unsafe {
            asm!(
                // 0 | zimm[10:0] | rs1 | 1 1 1 | rd |1010111| vsetvli
                // vsetvli	t4, x0, e8, m1, d1
                ".4byte 0b00000000000000000111111011010111",
                "mv {vl}, t4",
                vl = out(reg) vl,
                out("t4") _,
                options(nostack, preserves_flags),
            );
        }
    } else {
        unsafe {
            asm!(
                ".option push",
                ".option arch, +v",
                "vsetvli {vl}, x0, e8, m1, ta, ma",
                ".option pop",
                vl = out(reg) vl,
                options(nostack, preserves_flags),
            );
        }
    }

    check_vector_register!(v0, vl, value);
    check_vector_register!(v1, vl, value);
    check_vector_register!(v2, vl, value);
    check_vector_register!(v3, vl, value);
    check_vector_register!(v4, vl, value);
    check_vector_register!(v5, vl, value);
    check_vector_register!(v6, vl, value);
    check_vector_register!(v7, vl, value);
    check_vector_register!(v8, vl, value);
    check_vector_register!(v9, vl, value);
    check_vector_register!(v10, vl, value);
    check_vector_register!(v11, vl, value);
    check_vector_register!(v12, vl, value);
    check_vector_register!(v13, vl, value);
    check_vector_register!(v14, vl, value);
    check_vector_register!(v15, vl, value);
    check_vector_register!(v16, vl, value);
    check_vector_register!(v17, vl, value);
    check_vector_register!(v18, vl, value);
    check_vector_register!(v19, vl, value);
    check_vector_register!(v20, vl, value);
    check_vector_register!(v21, vl, value);
    check_vector_register!(v22, vl, value);
    check_vector_register!(v23, vl, value);
    check_vector_register!(v24, vl, value);
    check_vector_register!(v25, vl, value);
    check_vector_register!(v26, vl, value);
    check_vector_register!(v27, vl, value);
    check_vector_register!(v28, vl, value);
    check_vector_register!(v29, vl, value);
    check_vector_register!(v30, vl, value);
    check_vector_register!(v31, vl, value);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
