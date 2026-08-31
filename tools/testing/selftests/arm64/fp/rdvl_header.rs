// SPDX-License-Identifier: GPL-2.0-only

unsafe extern "C" {
    pub fn rdvl_sme() -> ::core::ffi::c_int;
    pub fn rdvl_sve() -> ::core::ffi::c_int;
}
