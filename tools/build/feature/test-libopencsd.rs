// SPDX-License-Identifier: GPL-2.0
// Depends on OpenCSD C API: <opencsd/c_api/opencsd_c_api.h>

/*
 * Check OpenCSD library version is sufficient to provide required features
 */
const OCSD_MIN_VER: i32 = (1 << 16) | (2 << 8) | 1;

// C preprocessor check preserved from source:
// #if !defined(OCSD_VER_NUM) || (OCSD_VER_NUM < OCSD_MIN_VER)
// #error "OpenCSD >= 1.2.1 is required"
// #endif

unsafe extern "C" {
    fn ocsd_get_version() -> i32;
}

fn main() {
    unsafe {
        let _ = ocsd_get_version();
    }
}
