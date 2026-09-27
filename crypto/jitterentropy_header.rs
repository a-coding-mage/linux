// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
pub struct sha3_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn jent_kvzalloc(len: ::kernel::ffi::c_uint) -> *mut ::kernel::ffi::c_void;
    pub fn jent_kvzfree(ptr: *mut ::kernel::ffi::c_void, len: ::kernel::ffi::c_uint);
    pub fn jent_zalloc(len: ::kernel::ffi::c_uint) -> *mut ::kernel::ffi::c_void;
    pub fn jent_zfree(ptr: *mut ::kernel::ffi::c_void);
    pub fn jent_get_nstime(out: *mut u64);
    pub fn jent_hash_time(
        hash_state: *mut sha3_ctx,
        time: u64,
        addtl: *mut u8,
        addtl_len: ::kernel::ffi::c_uint,
        hash_loop_cnt: u64,
        stuck: ::kernel::ffi::c_uint,
    );
    pub fn jent_read_random_block(
        hash_state: *mut sha3_ctx,
        dst: *mut ::kernel::ffi::c_char,
        dst_len: ::kernel::ffi::c_uint,
    );
}

#[repr(C)]
pub struct rand_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn jent_entropy_init(
        osr: ::kernel::ffi::c_uint,
        flags: ::kernel::ffi::c_uint,
        hash_state: *mut sha3_ctx,
        p_ec: *mut rand_data,
    ) -> ::kernel::ffi::c_int;
    pub fn jent_read_entropy(
        ec: *mut rand_data,
        data: *mut u8,
        len: ::kernel::ffi::c_uint,
    ) -> ::kernel::ffi::c_int;
    pub fn jent_entropy_collector_alloc(
        osr: ::kernel::ffi::c_uint,
        flags: ::kernel::ffi::c_uint,
        hash_state: *mut sha3_ctx,
    ) -> *mut rand_data;
    pub fn jent_entropy_collector_free(entropy_collector: *mut rand_data);
}

// CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE selects the external test interface.
#[cfg(CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE)]
extern "C" {
    pub fn jent_raw_hires_entropy_store(value: u64) -> ::kernel::ffi::c_int;
    pub fn jent_testing_init();
    pub fn jent_testing_exit();
}

#[cfg(not(CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE))]
#[inline]
pub fn jent_raw_hires_entropy_store(_value: u64) -> ::kernel::ffi::c_int {
    0
}

#[cfg(not(CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE))]
#[inline]
pub fn jent_testing_init() {}

#[cfg(not(CONFIG_CRYPTO_JITTERENTROPY_TESTINTERFACE))]
#[inline]
pub fn jent_testing_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
