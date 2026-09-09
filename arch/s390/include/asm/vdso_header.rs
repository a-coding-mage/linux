/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the vDSO datapage definitions.

// This declaration is excluded when compiling as assembler in the C source.
unsafe extern "C" {
    pub fn vdso_getcpu_init() -> ::core::ffi::c_int;
}

pub const __VDSO_PAGES: ::core::ffi::c_int = 4;

// The C source uses the build/link-time token `LINUX_2.6.29` as the version
// string macro value; retain that token as a Rust string literal.
pub const VDSO_VERSION_STRING: &str = "LINUX_2.6.29";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
