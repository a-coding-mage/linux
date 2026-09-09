/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/ioctl.h> and <linux/types.h> as used by this header.

#[repr(C)]
pub struct blk_crypto_import_key_arg {
    /* Raw key (input) */
    pub raw_key_ptr: u64,
    pub raw_key_size: u64,
    /* Long-term wrapped key blob (output) */
    pub lt_key_ptr: u64,
    pub lt_key_size: u64,
    pub reserved: [u64; 4],
}

#[repr(C)]
pub struct blk_crypto_generate_key_arg {
    /* Long-term wrapped key blob (output) */
    pub lt_key_ptr: u64,
    pub lt_key_size: u64,
    pub reserved: [u64; 4],
}

#[repr(C)]
pub struct blk_crypto_prepare_key_arg {
    /* Long-term wrapped key blob (input) */
    pub lt_key_ptr: u64,
    pub lt_key_size: u64,
    /* Ephemerally-wrapped key blob (output) */
    pub eph_key_ptr: u64,
    pub eph_key_size: u64,
    pub reserved: [u64; 4],
}

/*
 * These ioctls share the block device ioctl space; see uapi/linux/fs.h.
 * 140-141 are reserved for future blk-crypto ioctls; any more than that would
 * require an additional allocation from the block device ioctl space.
 */

// Equivalent to _IOWR(0x12, nr, type) on Linux ioctl ABIs.
const fn blkcrypto_iowr(nr: u32, size: u32) -> u32 {
    (3u32 << 30) | (size << 16) | (0x12u32 << 8) | nr
}

pub const BLKCRYPTOIMPORTKEY: u32 =
    blkcrypto_iowr(137, core::mem::size_of::<blk_crypto_import_key_arg>() as u32);
pub const BLKCRYPTOGENERATEKEY: u32 =
    blkcrypto_iowr(138, core::mem::size_of::<blk_crypto_generate_key_arg>() as u32);
pub const BLKCRYPTOPREPAREKEY: u32 =
    blkcrypto_iowr(139, core::mem::size_of::<blk_crypto_prepare_key_arg>() as u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
