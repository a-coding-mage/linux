/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */
/*
 * Copyright 1993 by Theodore Ts'o.
 */

pub const LO_NAME_SIZE: usize = 64;
pub const LO_KEY_SIZE: usize = 32;

/*
 * Loop flags
 */
pub const LO_FLAGS_READ_ONLY: i32 = 1;
pub const LO_FLAGS_AUTOCLEAR: i32 = 4;
pub const LO_FLAGS_PARTSCAN: i32 = 8;
pub const LO_FLAGS_DIRECT_IO: i32 = 16;

/* LO_FLAGS that can be set using LOOP_SET_STATUS(64) */
pub const LOOP_SET_STATUS_SETTABLE_FLAGS: i32 =
    LO_FLAGS_AUTOCLEAR | LO_FLAGS_PARTSCAN;

/* LO_FLAGS that can be cleared using LOOP_SET_STATUS(64) */
pub const LOOP_SET_STATUS_CLEARABLE_FLAGS: i32 = LO_FLAGS_AUTOCLEAR;

/* LO_FLAGS that can be set using LOOP_CONFIGURE */
pub const LOOP_CONFIGURE_SETTABLE_FLAGS: i32 = LO_FLAGS_READ_ONLY
    | LO_FLAGS_AUTOCLEAR
    | LO_FLAGS_PARTSCAN
    | LO_FLAGS_DIRECT_IO;

/* __kernel_old_dev_t and __u* are supplied by the corresponding Linux type headers. */

/* Backwards compatibility version */
#[repr(C)]
pub struct loop_info {
    pub lo_number: i32, /* ioctl r/o */
    pub lo_device: __kernel_old_dev_t, /* ioctl r/o */
    pub lo_inode: usize, /* ioctl r/o */
    pub lo_rdevice: __kernel_old_dev_t, /* ioctl r/o */
    pub lo_offset: i32,
    pub lo_encrypt_type: i32, /* obsolete, ignored */
    pub lo_encrypt_key_size: i32, /* ioctl w/o */
    pub lo_flags: i32,
    pub lo_name: [core::ffi::c_char; LO_NAME_SIZE],
    pub lo_encrypt_key: [u8; LO_KEY_SIZE], /* ioctl w/o */
    pub lo_init: [usize; 2],
    pub reserved: [core::ffi::c_char; 4],
}

#[repr(C)]
pub struct loop_info64 {
    pub lo_device: __u64, /* ioctl r/o */
    pub lo_inode: __u64, /* ioctl r/o */
    pub lo_rdevice: __u64, /* ioctl r/o */
    pub lo_offset: __u64,
    pub lo_sizelimit: __u64, /* bytes, 0 == max available */
    pub lo_number: __u32, /* ioctl r/o */
    pub lo_encrypt_type: __u32, /* obsolete, ignored */
    pub lo_encrypt_key_size: __u32, /* ioctl w/o */
    pub lo_flags: __u32,
    pub lo_file_name: [__u8; LO_NAME_SIZE],
    pub lo_crypt_name: [__u8; LO_NAME_SIZE],
    pub lo_encrypt_key: [__u8; LO_KEY_SIZE], /* ioctl w/o */
    pub lo_init: [__u64; 2],
}

/**
 * struct loop_config - Complete configuration for a loop device.
 * @fd: fd of the file to be used as a backing file for the loop device.
 * @block_size: block size to use; ignored if 0.
 * @info: struct loop_info64 to configure the loop device with.
 *
 * This structure is used with the LOOP_CONFIGURE ioctl, and can be used to
 * atomically setup and configure all loop device parameters at once.
 */
#[repr(C)]
pub struct loop_config {
    pub fd: __u32,
    pub block_size: __u32,
    pub info: loop_info64,
    pub __reserved: [__u64; 8],
}

/*
 * Loop filter types
 */
pub const LO_CRYPT_NONE: i32 = 0;
pub const LO_CRYPT_XOR: i32 = 1;
pub const LO_CRYPT_DES: i32 = 2;
pub const LO_CRYPT_FISH2: i32 = 3; /* Twofish encryption */
pub const LO_CRYPT_BLOW: i32 = 4;
pub const LO_CRYPT_CAST128: i32 = 5;
pub const LO_CRYPT_IDEA: i32 = 6;
pub const LO_CRYPT_DUMMY: i32 = 9;
pub const LO_CRYPT_SKIPJACK: i32 = 10;
pub const LO_CRYPT_CRYPTOAPI: i32 = 18;
pub const MAX_LO_CRYPT: i32 = 20;

/*
 * IOCTL commands --- we will commandeer 0x4C ('L')
 */
pub const LOOP_SET_FD: u32 = 0x4C00;
pub const LOOP_CLR_FD: u32 = 0x4C01;
pub const LOOP_SET_STATUS: u32 = 0x4C02;
pub const LOOP_GET_STATUS: u32 = 0x4C03;
pub const LOOP_SET_STATUS64: u32 = 0x4C04;
pub const LOOP_GET_STATUS64: u32 = 0x4C05;
pub const LOOP_CHANGE_FD: u32 = 0x4C06;
pub const LOOP_SET_CAPACITY: u32 = 0x4C07;
pub const LOOP_SET_DIRECT_IO: u32 = 0x4C08;
pub const LOOP_SET_BLOCK_SIZE: u32 = 0x4C09;
pub const LOOP_CONFIGURE: u32 = 0x4C0A;

/* /dev/loop-control interface */
pub const LOOP_CTL_ADD: u32 = 0x4C80;
pub const LOOP_CTL_REMOVE: u32 = 0x4C81;
pub const LOOP_CTL_GET_FREE: u32 = 0x4C82;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
