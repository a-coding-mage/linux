// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
pub struct sha3_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn jent_kvzalloc(len: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    pub fn jent_kvzfree(ptr: *mut ::core::ffi::c_void, len: ::core::ffi::c_uint);
    pub fn jent_zalloc(len: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    pub fn jent_zfree(ptr: *mut ::core::ffi::c_void);
    pub fn jent_get_nstime(out: *mut u64);
    pub fn jent_hash_time(
        hash_state: *mut sha3_ctx,
        time: u64,
        addtl: *mut u8,
        addtl_len: ::core::ffi::c_uint,
        hash_loop_cnt: u64,
        stuck: ::core::ffi::c_uint,
    );
    pub fn jent_read_random_block(
        hash_state: *mut sha3_ctx,
        dst: *mut ::core::ffi::c_char,
        dst_len: ::core::ffi::c_uint,
    );
}

#[repr(C)]
pub struct rand_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn jent_entropy_init(
        osr: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
        hash_state: *mut sha3_ctx,
        p_ec: *mut rand_data,
    ) -> ::core::ffi::c_int;
    pub fn jent_read_entropy(
        ec: *mut rand_data,
        data: *mut u8,
        len: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn jent_entropy_collector_alloc(
        osr: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
        hash_state: *mut sha3_ctx,
    ) -> *mut rand_data;
    pub fn jent_entropy_collector_free(entropy_collector: *mut rand_data);
}

// CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE selects the external test interface.
#[cfg(feature = "CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE")]
extern "C" {
    pub fn jent_raw_hires_entropy_store(value: u64) -> ::core::ffi::c_int;
    pub fn jent_testing_init();
    pub fn jent_testing_exit();
}

#[cfg(not(feature = "CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE"))]
#[inline]
pub fn jent_raw_hires_entropy_store(_value: u64) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE"))]
#[inline]
pub fn jent_testing_init() {}

#[cfg(not(feature = "CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE"))]
#[inline]
pub fn jent_testing_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
