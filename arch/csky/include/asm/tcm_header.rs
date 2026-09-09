/* SPDX-License-Identifier: GPL-2.0 */

// This header is only intended to be included when CONFIG_HAVE_TCM is enabled.

// Tag variables with this: #[link_section = ".tcm.data"]
// Tag constants with this: #[link_section = ".tcm.rodata"]
// Tag functions inside TCM called from outside TCM with this:
// #[link_section = ".tcm.text"] and #[inline(never)]
// Tag functions inside TCM called from inside TCM with this:
// #[link_section = ".tcm.text"]

extern "C" {
    pub fn tcm_alloc(len: usize) -> *mut core::ffi::c_void;
    pub fn tcm_free(addr: *mut core::ffi::c_void, len: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
