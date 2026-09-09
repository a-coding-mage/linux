/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * fs-verity user API
 *
 * These ioctls can be used on filesystems that support fs-verity.  See the
 * "User API" section of Documentation/filesystems/fsverity.rst.
 *
 * Copyright 2019 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const FS_VERITY_HASH_ALG_SHA256: u32 = 1;
pub const FS_VERITY_HASH_ALG_SHA512: u32 = 2;

#[repr(C)]
pub struct fsverity_enable_arg {
    pub version: __u32,
    pub hash_algorithm: __u32,
    pub block_size: __u32,
    pub salt_size: __u32,
    pub salt_ptr: __u64,
    pub sig_size: __u32,
    pub __reserved1: __u32,
    pub sig_ptr: __u64,
    pub __reserved2: [__u64; 11],
}

#[repr(C)]
pub struct fsverity_digest {
    pub digest_algorithm: __u16,
    pub digest_size: __u16, // input/output
    pub digest: [__u8; 0],
}

/*
 * Struct containing a file's Merkle tree properties.  The fs-verity file digest
 * is the hash of this struct.  A userspace program needs this struct only if it
 * needs to compute fs-verity file digests itself, e.g. in order to sign files.
 * It isn't needed just to enable fs-verity on a file.
 *
 * Note: when computing the file digest, 'sig_size' and 'signature' must be left
 * zero and empty, respectively.  These fields are present only because some
 * filesystems reuse this struct as part of their on-disk format.
 */
#[repr(C)]
pub struct fsverity_descriptor {
    pub version: __u8, // must be 1
    pub hash_algorithm: __u8, // Merkle tree hash algorithm
    pub log_blocksize: __u8, // log2 of size of data and tree blocks
    pub salt_size: __u8, // size of salt in bytes; 0 if none
    // __KERNEL__: sig_size; userspace: __reserved_0x04.
    pub __reserved_0x04: __le32, // must be 0
    pub data_size: __le64, // size of file the Merkle tree is built over
    pub root_hash: [__u8; 64], // Merkle tree root hash
    pub salt: [__u8; 32], // salt prepended to each hashed block
    pub __reserved: [__u8; 144], // must be 0's
}

/*
 * Format in which fs-verity file digests are signed in built-in signatures.
 * This is the same as 'struct fsverity_digest', except here some magic bytes
 * are prepended to provide some context about what is being signed in case the
 * same key is used for non-fsverity purposes, and here the fields have fixed
 * endianness.
 *
 * This struct is specific to the built-in signature verification support, which
 * is optional.  fs-verity users may also verify signatures in userspace, in
 * which case userspace is responsible for deciding on what bytes are signed.
 * This struct may still be used, but it doesn't have to be.  For example,
 * userspace could instead use a string like "sha256:$digest_as_hex_string".
 */
#[repr(C)]
pub struct fsverity_formatted_digest {
    pub magic: [::core::ffi::c_char; 8], // must be "FSVerity"
    pub digest_algorithm: __le16,
    pub digest_size: __le16,
    pub digest: [__u8; 0],
}

pub const FS_VERITY_METADATA_TYPE_MERKLE_TREE: u64 = 1;
pub const FS_VERITY_METADATA_TYPE_DESCRIPTOR: u64 = 2;
pub const FS_VERITY_METADATA_TYPE_SIGNATURE: u64 = 3;

#[repr(C)]
pub struct fsverity_read_metadata_arg {
    pub metadata_type: __u64,
    pub offset: __u64,
    pub length: __u64,
    pub buf_ptr: __u64,
    pub __reserved: __u64,
}

pub const FS_IOC_ENABLE_VERITY: _ = _IOW(b'f', 133, fsverity_enable_arg);
pub const FS_IOC_MEASURE_VERITY: _ = _IOWR(b'f', 134, fsverity_digest);
pub const FS_IOC_READ_VERITY_METADATA: _ = _IOWR(b'f', 135, fsverity_read_metadata_arg);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
