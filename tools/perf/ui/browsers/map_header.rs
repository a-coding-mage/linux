// SPDX-License-Identifier: GPL-2.0
// C header guard _PERF_UI_MAP_BROWSER_H_ omitted in Rust.

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn map__browse(map: *mut map) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
