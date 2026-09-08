/* SPDX-License-Identifier: GPL-2.0 */

// Forward declaration: `struct S;`
#[repr(C)]
pub struct S {
    pub foo: core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
