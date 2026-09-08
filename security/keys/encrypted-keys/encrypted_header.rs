/* SPDX-License-Identifier: GPL-2.0 */

pub const ENCRYPTED_DEBUG: i32 = 0;

pub type u8 = core::ffi::c_uchar;
pub type size_t = usize;

#[repr(C)]
pub struct key {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct encrypted_key_payload {
    _unused: [u8; 0],
}

/*
 * C conditional:
 * defined(CONFIG_TRUSTED_KEYS) ||
 *   (defined(CONFIG_TRUSTED_KEYS_MODULE) && defined(CONFIG_ENCRYPTED_KEYS_MODULE))
 *
 * When true, this header declares the external request_trusted_key().
 * When false, it provides the inline fallback below.
 */
unsafe extern "C" {
    pub fn request_trusted_key(
        trusted_desc: *const core::ffi::c_char,
        master_key: *mut *const u8,
        master_keylen: *mut size_t,
    ) -> *mut key;
}

/*
 * Fallback branch from the C header, preserved for configurations where the
 * external request_trusted_key declaration is not selected:
 *
 * static inline struct key *request_trusted_key(const char *trusted_desc,
 *                                              const u8 **master_key,
 *                                              size_t *master_keylen)
 * {
 *      return ERR_PTR(-EOPNOTSUPP);
 * }
 */

/*
 * ENCRYPTED_DEBUG is 0 in this header, so the compiled C branch provides these
 * empty inline functions. The debug branch would call print_hex_dump()/pr_info()
 * using fields supplied by struct encrypted_key_payload in other kernel code.
 */
#[inline]
pub unsafe fn dump_master_key(_master_key: *const u8, _master_keylen: size_t) {}

#[inline]
pub unsafe fn dump_decrypted_data(_epayload: *mut encrypted_key_payload) {}

#[inline]
pub unsafe fn dump_encrypted_data(
    _epayload: *mut encrypted_key_payload,
    _encrypted_datalen: core::ffi::c_uint,
) {
}

#[inline]
pub unsafe fn dump_hmac(
    _str: *const core::ffi::c_char,
    _digest: *const u8,
    _hmac_size: core::ffi::c_uint,
) {
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
