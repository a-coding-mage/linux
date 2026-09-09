/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <keys/trusted-type.h>

#[repr(C)]
pub struct trusted_key_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tpm_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trusted_key_payload {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trusted_key_options {
    _private: [u8; 0],
}

extern "C" {
    pub static mut trusted_key_tpm_ops: trusted_key_ops;

    pub fn tpm2_seal_trusted(
        chip: *mut tpm_chip,
        payload: *mut trusted_key_payload,
        options: *mut trusted_key_options,
    ) -> ::core::ffi::c_int;

    pub fn tpm2_unseal_trusted(
        chip: *mut tpm_chip,
        payload: *mut trusted_key_payload,
        options: *mut trusted_key_options,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
