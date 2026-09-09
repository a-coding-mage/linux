/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * fscrypt user API
 *
 * These ioctls can be used on filesystems that support fscrypt.  See the
 * "User API" section of Documentation/filesystems/fscrypt.rst.
 */

/* Encryption policy flags */
pub const FSCRYPT_POLICY_FLAGS_PAD_4: u8 = 0x00;
pub const FSCRYPT_POLICY_FLAGS_PAD_8: u8 = 0x01;
pub const FSCRYPT_POLICY_FLAGS_PAD_16: u8 = 0x02;
pub const FSCRYPT_POLICY_FLAGS_PAD_32: u8 = 0x03;
pub const FSCRYPT_POLICY_FLAGS_PAD_MASK: u8 = 0x03;
pub const FSCRYPT_POLICY_FLAG_DIRECT_KEY: u8 = 0x04;
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64: u8 = 0x08;
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32: u8 = 0x10;

/* Encryption algorithms */
pub const FSCRYPT_MODE_AES_256_XTS: u8 = 1;
pub const FSCRYPT_MODE_AES_256_CTS: u8 = 4;
pub const FSCRYPT_MODE_AES_128_CBC: u8 = 5;
pub const FSCRYPT_MODE_AES_128_CTS: u8 = 6;
pub const FSCRYPT_MODE_SM4_XTS: u8 = 7;
pub const FSCRYPT_MODE_SM4_CTS: u8 = 8;
pub const FSCRYPT_MODE_ADIANTUM: u8 = 9;
pub const FSCRYPT_MODE_AES_256_HCTR2: u8 = 10;

/* Legacy policy version; ad-hoc KDF and no key verification. */
pub const FSCRYPT_POLICY_V1: u8 = 0;
pub const FSCRYPT_KEY_DESCRIPTOR_SIZE: usize = 8;
#[repr(C)]
pub struct fscrypt_policy_v1 {
    pub version: u8,
    pub contents_encryption_mode: u8,
    pub filenames_encryption_mode: u8,
    pub flags: u8,
    pub master_key_descriptor: [u8; FSCRYPT_KEY_DESCRIPTOR_SIZE],
}

/* Process-subscribed "logon" key description prefix and payload format. */
pub const FSCRYPT_KEY_DESC_PREFIX: &str = "fscrypt:";
pub const FSCRYPT_KEY_DESC_PREFIX_SIZE: usize = 8;
pub const FSCRYPT_MAX_KEY_SIZE: usize = 64;
#[repr(C)]
pub struct fscrypt_key {
    pub mode: u32,
    pub raw: [u8; FSCRYPT_MAX_KEY_SIZE],
    pub size: u32,
}

/* New policy version with HKDF and key verification (recommended). */
pub const FSCRYPT_POLICY_V2: u8 = 2;
pub const FSCRYPT_KEY_IDENTIFIER_SIZE: usize = 16;
#[repr(C)]
pub struct fscrypt_policy_v2 {
    pub version: u8,
    pub contents_encryption_mode: u8,
    pub filenames_encryption_mode: u8,
    pub flags: u8,
    pub log2_data_unit_size: u8,
    pub __reserved: [u8; 3],
    pub master_key_identifier: [u8; FSCRYPT_KEY_IDENTIFIER_SIZE],
}

#[repr(C)]
pub union fscrypt_get_policy_ex_arg_policy {
    pub version: u8,
    pub v1: fscrypt_policy_v1,
    pub v2: fscrypt_policy_v2,
}
#[repr(C)]
pub struct fscrypt_get_policy_ex_arg {
    pub policy_size: u64,
    pub policy: fscrypt_get_policy_ex_arg_policy,
}

pub const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: u32 = 1;
pub const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: u32 = 2;
#[repr(C)]
pub union fscrypt_key_specifier_u {
    pub __reserved: [u8; 32],
    pub descriptor: [u8; FSCRYPT_KEY_DESCRIPTOR_SIZE],
    pub identifier: [u8; FSCRYPT_KEY_IDENTIFIER_SIZE],
}
#[repr(C)]
pub struct fscrypt_key_specifier {
    pub type_: u32,
    pub __reserved: u32,
    pub u: fscrypt_key_specifier_u,
}

#[repr(C)]
pub struct fscrypt_provisioning_key_payload {
    pub type_: u32,
    pub flags: u32,
    pub raw: [u8; 0],
}

pub const FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED: u32 = 0x00000001;
#[repr(C)]
pub struct fscrypt_add_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub raw_size: u32,
    pub key_id: u32,
    pub flags: u32,
    pub __reserved: [u32; 7],
    pub raw: [u8; 0],
}

pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: u32 = 0x00000001;
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: u32 = 0x00000002;
#[repr(C)]
pub struct fscrypt_remove_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub removal_status_flags: u32,
    pub __reserved: [u32; 5],
}

pub const FSCRYPT_KEY_STATUS_ABSENT: u32 = 1;
pub const FSCRYPT_KEY_STATUS_PRESENT: u32 = 2;
pub const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: u32 = 3;
pub const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: u32 = 0x00000001;
#[repr(C)]
pub struct fscrypt_get_key_status_arg {
    pub key_spec: fscrypt_key_specifier,
    pub __reserved: [u32; 6],
    pub status: u32,
    pub status_flags: u32,
    pub user_count: u32,
    pub __out_reserved: [u32; 13],
}

/* ioctl encodings depend on the platform ioctl macros supplied by linux/ioctl.h. */
pub const FS_IOC_SET_ENCRYPTION_POLICY: u32 = _IOR!('f', 19, fscrypt_policy_v1);
pub const FS_IOC_GET_ENCRYPTION_PWSALT: u32 = _IOW!('f', 20, [u8; 16]);
pub const FS_IOC_GET_ENCRYPTION_POLICY: u32 = _IOW!('f', 21, fscrypt_policy_v1);
pub const FS_IOC_GET_ENCRYPTION_POLICY_EX: u32 = _IOWR!('f', 22, [u8; 9]);
pub const FS_IOC_ADD_ENCRYPTION_KEY: u32 = _IOWR!('f', 23, fscrypt_add_key_arg);
pub const FS_IOC_REMOVE_ENCRYPTION_KEY: u32 = _IOWR!('f', 24, fscrypt_remove_key_arg);
pub const FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS: u32 = _IOWR!('f', 25, fscrypt_remove_key_arg);
pub const FS_IOC_GET_ENCRYPTION_KEY_STATUS: u32 = _IOWR!('f', 26, fscrypt_get_key_status_arg);
pub const FS_IOC_GET_ENCRYPTION_NONCE: u32 = _IOR!('f', 27, [u8; 16]);

/* old names; don't add anything new here! */
pub type fscrypt_policy = fscrypt_policy_v1;
pub const FS_KEY_DESCRIPTOR_SIZE: usize = FSCRYPT_KEY_DESCRIPTOR_SIZE;
pub const FS_POLICY_FLAGS_PAD_4: u8 = FSCRYPT_POLICY_FLAGS_PAD_4;
pub const FS_POLICY_FLAGS_PAD_8: u8 = FSCRYPT_POLICY_FLAGS_PAD_8;
pub const FS_POLICY_FLAGS_PAD_16: u8 = FSCRYPT_POLICY_FLAGS_PAD_16;
pub const FS_POLICY_FLAGS_PAD_32: u8 = FSCRYPT_POLICY_FLAGS_PAD_32;
pub const FS_POLICY_FLAGS_PAD_MASK: u8 = FSCRYPT_POLICY_FLAGS_PAD_MASK;
pub const FS_POLICY_FLAG_DIRECT_KEY: u8 = FSCRYPT_POLICY_FLAG_DIRECT_KEY;
pub const FS_POLICY_FLAGS_VALID: u8 = 0x07;
pub const FS_ENCRYPTION_MODE_INVALID: u8 = 0;
pub const FS_ENCRYPTION_MODE_AES_256_XTS: u8 = FSCRYPT_MODE_AES_256_XTS;
pub const FS_ENCRYPTION_MODE_AES_256_GCM: u8 = 2;
pub const FS_ENCRYPTION_MODE_AES_256_CBC: u8 = 3;
pub const FS_ENCRYPTION_MODE_AES_256_CTS: u8 = FSCRYPT_MODE_AES_256_CTS;
pub const FS_ENCRYPTION_MODE_AES_128_CBC: u8 = FSCRYPT_MODE_AES_128_CBC;
pub const FS_ENCRYPTION_MODE_AES_128_CTS: u8 = FSCRYPT_MODE_AES_128_CTS;
pub const FS_ENCRYPTION_MODE_ADIANTUM: u8 = FSCRYPT_MODE_ADIANTUM;
pub const FS_KEY_DESC_PREFIX: &str = FSCRYPT_KEY_DESC_PREFIX;
pub const FS_KEY_DESC_PREFIX_SIZE: usize = FSCRYPT_KEY_DESC_PREFIX_SIZE;
pub const FS_MAX_KEY_SIZE: usize = FSCRYPT_MAX_KEY_SIZE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
