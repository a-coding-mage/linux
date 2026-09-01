/* SPDX-License-Identifier: GPL-2.0 */

// Forward declaration from C: struct evsel;
#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn arch_topdown_sample_read(leader: *mut evsel) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
