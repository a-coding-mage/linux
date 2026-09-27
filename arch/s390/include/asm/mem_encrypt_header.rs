/* SPDX-License-Identifier: GPL-2.0 */

// Declarations are excluded from assembler builds in the original header.
// This Rust translation represents the non-assembler interface.

unsafe extern "C" {
    pub fn set_memory_encrypted(vaddr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn set_memory_decrypted(vaddr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
