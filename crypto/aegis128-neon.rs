// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2019 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/cpufeature.h, asm/simd.h, aegis.h, and aegis-neon.h

#[no_mangle]
pub static mut aegis128_have_aes_insn: ::core::ffi::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_have_simd() -> bool {
    if cpu_have_feature(cpu_feature(AES)) {
        aegis128_have_aes_insn = 1;
        return true;
    }
    // Equivalent build-time condition for IS_ENABLED(CONFIG_ARM64).
    cfg!(target_arch = "aarch64")
}

#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_init_simd(
    state: *mut aegis_state,
    key: *const aegis_block,
    iv: *const u8,
) {
    // scoped_ksimd()
    crypto_aegis128_init_neon(state, key, iv);
}

#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_update_simd(
    state: *mut aegis_state,
    msg: *const ::core::ffi::c_void,
) {
    // scoped_ksimd()
    crypto_aegis128_update_neon(state, msg);
}

#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_encrypt_chunk_simd(
    state: *mut aegis_state,
    dst: *mut u8,
    src: *const u8,
    size: ::core::ffi::c_uint,
) {
    // scoped_ksimd()
    crypto_aegis128_encrypt_chunk_neon(state, dst, src, size);
}

#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_decrypt_chunk_simd(
    state: *mut aegis_state,
    dst: *mut u8,
    src: *const u8,
    size: ::core::ffi::c_uint,
) {
    // scoped_ksimd()
    crypto_aegis128_decrypt_chunk_neon(state, dst, src, size);
}

#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_final_simd(
    state: *mut aegis_state,
    tag_xor: *mut aegis_block,
    assoclen: ::core::ffi::c_uint,
    cryptlen: ::core::ffi::c_uint,
    authsize: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    // scoped_ksimd()
    crypto_aegis128_final_neon(state, tag_xor, assoclen, cryptlen, authsize)
}

extern "C" {
    static AES: ::core::ffi::c_int;

    fn cpu_feature(feature: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cpu_have_feature(feature: ::core::ffi::c_int) -> bool;

    fn crypto_aegis128_init_neon(
        state: *mut aegis_state,
        key: *const aegis_block,
        iv: *const u8,
    );
    fn crypto_aegis128_update_neon(state: *mut aegis_state, msg: *const ::core::ffi::c_void);
    fn crypto_aegis128_encrypt_chunk_neon(
        state: *mut aegis_state,
        dst: *mut u8,
        src: *const u8,
        size: ::core::ffi::c_uint,
    );
    fn crypto_aegis128_decrypt_chunk_neon(
        state: *mut aegis_state,
        dst: *mut u8,
        src: *const u8,
        size: ::core::ffi::c_uint,
    );
    fn crypto_aegis128_final_neon(
        state: *mut aegis_state,
        tag_xor: *mut aegis_block,
        assoclen: ::core::ffi::c_uint,
        cryptlen: ::core::ffi::c_uint,
        authsize: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
