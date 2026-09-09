/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations supplied by other translation units.
#[repr(C)]
pub struct kimage {
    _private: [u8; 0],
}

#[repr(C)]
pub struct boot_params {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn crash_load_segments(image: *mut kimage) -> ::core::ffi::c_int;
    pub fn crash_setup_memmap_entries(
        image: *mut kimage,
        params: *mut boot_params,
    ) -> ::core::ffi::c_int;
    pub fn crash_smp_send_stop();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
