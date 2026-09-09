// Dependency equivalent of <linux/errno.h> is required for `libc::EINVAL`.

pub const GCM_AES_IV_SIZE: usize = 12;
pub const GCM_RFC4106_IV_SIZE: usize = 8;
pub const GCM_RFC4543_IV_SIZE: usize = 8;

/*
 * validate authentication tag for GCM
 */
#[inline]
pub fn crypto_gcm_check_authsize(authsize: usize) -> i32 {
    match authsize {
        4 | 8 | 12 | 13 | 14 | 15 | 16 => {}
        _ => return -(libc::EINVAL as i32),
    }

    0
}

/*
 * validate authentication tag for RFC4106
 */
#[inline]
pub fn crypto_rfc4106_check_authsize(authsize: usize) -> i32 {
    match authsize {
        8 | 12 | 16 => {}
        _ => return -(libc::EINVAL as i32),
    }

    0
}

/*
 * validate assoclen for RFC4106/RFC4543
 */
#[inline]
pub fn crypto_ipsec_check_assoclen(assoclen: u32) -> i32 {
    match assoclen {
        16 | 20 => {}
        _ => return -(libc::EINVAL as i32),
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
