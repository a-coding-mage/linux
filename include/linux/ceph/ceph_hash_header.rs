/* SPDX-License-Identifier: GPL-2.0 */

pub const CEPH_STR_HASH_LINUX: u32 = 0x1; // linux dcache hash
pub const CEPH_STR_HASH_RJENKINS: u32 = 0x2; // robert jenkins'

extern "C" {
    pub fn ceph_str_hash_linux(s: *const core::ffi::c_char, len: u32) -> u32;
    pub fn ceph_str_hash_rjenkins(s: *const core::ffi::c_char, len: u32) -> u32;

    pub fn ceph_str_hash(
        type_: core::ffi::c_int,
        s: *const core::ffi::c_char,
        len: u32,
    ) -> u32;
    pub fn ceph_str_hash_name(type_: core::ffi::c_int) -> *const core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
