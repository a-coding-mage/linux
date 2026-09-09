/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <asm/extable.h>
// Architecture-specific dependency:
// #if defined(__sparc__) && defined(__arch64__)
// #include <asm/uaccess_64.h>
// #else
// #include <asm/uaccess_32.h>
// #endif

use core::ffi::{c_char, c_long};

unsafe extern "C" {
    pub fn strncpy_from_user(
        dest: *mut c_char,
        src: *const c_char,
        count: c_long,
    ) -> c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
