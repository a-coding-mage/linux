/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012-2013 Cavium Inc., All Rights Reserved.
 *
 * MD5/SHA1/SHA256 instruction definitions added by
 * Aaro Koskinen <aaro.koskinen@iki.fi>.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const OCTEON_CR_OPCODE_PRIORITY: i32 = 300;

extern "C" {
    pub fn octeon_crypto_enable(state: *mut octeon_cop2_state) -> libc::c_ulong;
    pub fn octeon_crypto_disable(state: *mut octeon_cop2_state, flags: libc::c_ulong);
}

#[repr(C)]
pub struct octeon_cop2_state {
    _private: [u8; 0],
}

/* Macros needed to implement MD5/SHA1/SHA256. */

/* The index can be 0-1 (MD5), 0-2 (SHA1), or 0-3 (SHA256). */
#[macro_export]
macro_rules! write_octeon_64bit_hash_dword {
    ($value:expr, $index:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("dmtc2 {rt},0x0048+", stringify!($index)),
                rt = in(reg) ($value).to_be(),
                options(nostack)
            );
        }
    }};
}

#[macro_export]
macro_rules! read_octeon_64bit_hash_dword {
    ($index:expr) => {{
        let mut __value: u64;
        unsafe {
            core::arch::asm!(
                concat!("dmfc2 {rt},0x0048+", stringify!($index)),
                rt = lateout(reg) __value,
                options(nostack)
            );
        }
        u64::from_be(__value)
    }};
}

/* The index can be 0-6. */
#[macro_export]
macro_rules! write_octeon_64bit_block_dword {
    ($value:expr, $index:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("dmtc2 {rt},0x0040+", stringify!($index)),
                rt = in(reg) ($value).to_be(),
                options(nostack)
            );
        }
    }};
}

/* The value is the final block dword (64-bit). */
#[macro_export]
macro_rules! octeon_md5_start {
    ($value:expr) => {{
        unsafe { core::arch::asm!("dmtc2 {rt},0x4047", rt = in(reg) ($value).to_be(), options(nostack)); }
    }};
}

#[macro_export]
macro_rules! octeon_sha1_start {
    ($value:expr) => {{
        unsafe { core::arch::asm!("dmtc2 {rt},0x4057", rt = in(reg) $value, options(nostack)); }
    }};
}

#[macro_export]
macro_rules! octeon_sha256_start {
    ($value:expr) => {{
        unsafe { core::arch::asm!("dmtc2 {rt},0x404f", rt = in(reg) $value, options(nostack)); }
    }};
}

/* Macros needed to implement SHA512. */

/* The index can be 0-7. */
#[macro_export]
macro_rules! write_octeon_64bit_hash_sha512 {
    ($value:expr, $index:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("dmtc2 {rt},0x0250+", stringify!($index)),
                rt = in(reg) $value,
                options(nostack)
            );
        }
    }};
}

#[macro_export]
macro_rules! read_octeon_64bit_hash_sha512 {
    ($index:expr) => {{
        let mut __value: u64;
        unsafe {
            core::arch::asm!(
                concat!("dmfc2 {rt},0x0250+", stringify!($index)),
                rt = lateout(reg) __value,
                options(nostack)
            );
        }
        __value
    }};
}

/* The index can be 0-14. */
#[macro_export]
macro_rules! write_octeon_64bit_block_sha512 {
    ($value:expr, $index:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("dmtc2 {rt},0x0240+", stringify!($index)),
                rt = in(reg) $value,
                options(nostack)
            );
        }
    }};
}

/* The value is the final block word (64-bit). */
#[macro_export]
macro_rules! octeon_sha512_start {
    ($value:expr) => {{
        unsafe { core::arch::asm!("dmtc2 {rt},0x424f", rt = in(reg) $value, options(nostack)); }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
