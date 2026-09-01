// SPDX-License-Identifier: GPL-2.0-only

unsafe extern "C" {
    pub fn rdvl_sme() -> ::core::ffi::c_int;
    pub fn rdvl_sve() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
