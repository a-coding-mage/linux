/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * fscrypt user API
 *
 * These ioctls can be used on filesystems that support fscrypt.  See the
 * "User API" section of Documentation/filesystems/fscrypt.rst.
 */

/* Original C header included <linux/ioctl.h> and <linux/types.h>. */

/* Encryption policy flags */
pub const FSCRYPT_POLICY_FLAGS_PAD_4: u32 = 0x00;
pub const FSCRYPT_POLICY_FLAGS_PAD_8: u32 = 0x01;
pub const FSCRYPT_POLICY_FLAGS_PAD_16: u32 = 0x02;
pub const FSCRYPT_POLICY_FLAGS_PAD_32: u32 = 0x03;
pub const FSCRYPT_POLICY_FLAGS_PAD_MASK: u32 = 0x03;
pub const FSCRYPT_POLICY_FLAG_DIRECT_KEY: u32 = 0x04;
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64: u32 = 0x08;
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32: u32 = 0x10;

/* Encryption algorithms */
pub const FSCRYPT_MODE_AES_256_XTS: u32 = 1;
pub const FSCRYPT_MODE_AES_256_CTS: u32 = 4;
pub const FSCRYPT_MODE_AES_128_CBC: u32 = 5;
pub const FSCRYPT_MODE_AES_128_CTS: u32 = 6;
pub const FSCRYPT_MODE_SM4_XTS: u32 = 7;
pub const FSCRYPT_MODE_SM4_CTS: u32 = 8;
pub const FSCRYPT_MODE_ADIANTUM: u32 = 9;
pub const FSCRYPT_MODE_AES_256_HCTR2: u32 = 10;

/*
 * Legacy policy version; ad-hoc KDF and no key verification.
 * For new encrypted directories, use fscrypt_policy_v2 instead.
 *
 * Careful: the .version field for this is actually 0, not 1.
 */
pub const FSCRYPT_POLICY_V1: u32 = 0;
pub const FSCRYPT_KEY_DESCRIPTOR_SIZE: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_policy_v1 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub master_key_descriptor: [__u8; FSCRYPT_KEY_DESCRIPTOR_SIZE],
}

/*
 * Process-subscribed "logon" key description prefix and payload format.
 * Deprecated; prefer FS_IOC_ADD_ENCRYPTION_KEY instead.
 */
pub const FSCRYPT_KEY_DESC_PREFIX: &[u8; 8] = b"fscrypt:";
pub const FSCRYPT_KEY_DESC_PREFIX_SIZE: usize = 8;
pub const FSCRYPT_MAX_KEY_SIZE: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_key {
    pub mode: __u32,
    pub raw: [__u8; FSCRYPT_MAX_KEY_SIZE],
    pub size: __u32,
}

/*
 * New policy version with HKDF and key verification (recommended).
 */
pub const FSCRYPT_POLICY_V2: u32 = 2;
pub const FSCRYPT_KEY_IDENTIFIER_SIZE: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_policy_v2 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub log2_data_unit_size: __u8,
    pub __reserved: [__u8; 3],
    pub master_key_identifier: [__u8; FSCRYPT_KEY_IDENTIFIER_SIZE],
}

/* Struct passed to FS_IOC_GET_ENCRYPTION_POLICY_EX */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_get_policy_ex_arg {
    pub policy_size: __u64, /* input/output */
    pub policy: fscrypt_get_policy_ex_arg_policy, /* output */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fscrypt_get_policy_ex_arg_policy {
    pub version: __u8,
    pub v1: fscrypt_policy_v1,
    pub v2: fscrypt_policy_v2,
}

/*
 * v1 policy keys are specified by an arbitrary 8-byte key "descriptor",
 * matching fscrypt_policy_v1::master_key_descriptor.
 */
pub const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: u32 = 1;

/*
 * v2 policy keys are specified by a 16-byte key "identifier" which the kernel
 * calculates as a cryptographic hash of the key itself,
 * matching fscrypt_policy_v2::master_key_identifier.
 */
pub const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: u32 = 2;

/*
 * Specifies a key, either for v1 or v2 policies.  This doesn't contain the
 * actual key itself; this is just the "name" of the key.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_key_specifier {
    pub type_: __u32, /* one of FSCRYPT_KEY_SPEC_TYPE_* */
    pub __reserved: __u32,
    pub u: fscrypt_key_specifier_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fscrypt_key_specifier_u {
    pub __reserved: [__u8; 32], /* reserve some extra space */
    pub descriptor: [__u8; FSCRYPT_KEY_DESCRIPTOR_SIZE],
    pub identifier: [__u8; FSCRYPT_KEY_IDENTIFIER_SIZE],
}

/*
 * Payload of Linux keyring key of type "fscrypt-provisioning", referenced by
 * fscrypt_add_key_arg::key_id as an alternative to fscrypt_add_key_arg::raw.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_provisioning_key_payload {
    pub type_: __u32,
    pub flags: __u32,
    pub raw: [__u8; 0],
}

/* Struct passed to FS_IOC_ADD_ENCRYPTION_KEY */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_add_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub raw_size: __u32,
    pub key_id: __u32,
    pub flags: __u32,
    pub __reserved: [__u32; 7],
    pub raw: [__u8; 0],
}

pub const FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED: u32 = 0x00000001;

/* Struct passed to FS_IOC_REMOVE_ENCRYPTION_KEY */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_remove_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub removal_status_flags: __u32, /* output */
    pub __reserved: [__u32; 5],
}

pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: u32 = 0x00000001;
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: u32 = 0x00000002;

/* Struct passed to FS_IOC_GET_ENCRYPTION_KEY_STATUS */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_get_key_status_arg {
    /* input */
    pub key_spec: fscrypt_key_specifier,
    pub __reserved: [__u32; 6],

    /* output */
    pub status: __u32,
    pub status_flags: __u32,
    pub user_count: __u32,
    pub __out_reserved: [__u32; 13],
}

pub const FSCRYPT_KEY_STATUS_ABSENT: u32 = 1;
pub const FSCRYPT_KEY_STATUS_PRESENT: u32 = 2;
pub const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: u32 = 3;
pub const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: u32 = 0x00000001;

pub const FS_IOC_SET_ENCRYPTION_POLICY: ::core::ffi::c_ulong =
    _IOR!(b'f', 19, fscrypt_policy_v1);
pub const FS_IOC_GET_ENCRYPTION_PWSALT: ::core::ffi::c_ulong =
    _IOW!(b'f', 20, [__u8; 16]);
pub const FS_IOC_GET_ENCRYPTION_POLICY: ::core::ffi::c_ulong =
    _IOW!(b'f', 21, fscrypt_policy_v1);
pub const FS_IOC_GET_ENCRYPTION_POLICY_EX: ::core::ffi::c_ulong =
    _IOWR!(b'f', 22, [__u8; 9]); /* size + version */
pub const FS_IOC_ADD_ENCRYPTION_KEY: ::core::ffi::c_ulong =
    _IOWR!(b'f', 23, fscrypt_add_key_arg);
pub const FS_IOC_REMOVE_ENCRYPTION_KEY: ::core::ffi::c_ulong =
    _IOWR!(b'f', 24, fscrypt_remove_key_arg);
pub const FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS: ::core::ffi::c_ulong =
    _IOWR!(b'f', 25, fscrypt_remove_key_arg);
pub const FS_IOC_GET_ENCRYPTION_KEY_STATUS: ::core::ffi::c_ulong =
    _IOWR!(b'f', 26, fscrypt_get_key_status_arg);
pub const FS_IOC_GET_ENCRYPTION_NONCE: ::core::ffi::c_ulong =
    _IOR!(b'f', 27, [__u8; 16]);

/**********************************************************************/

/* old names; don't add anything new here! */
/* Original C condition: #ifndef __KERNEL__ */
pub type fscrypt_policy = fscrypt_policy_v1;
pub const FS_KEY_DESCRIPTOR_SIZE: usize = FSCRYPT_KEY_DESCRIPTOR_SIZE;
pub const FS_POLICY_FLAGS_PAD_4: u32 = FSCRYPT_POLICY_FLAGS_PAD_4;
pub const FS_POLICY_FLAGS_PAD_8: u32 = FSCRYPT_POLICY_FLAGS_PAD_8;
pub const FS_POLICY_FLAGS_PAD_16: u32 = FSCRYPT_POLICY_FLAGS_PAD_16;
pub const FS_POLICY_FLAGS_PAD_32: u32 = FSCRYPT_POLICY_FLAGS_PAD_32;
pub const FS_POLICY_FLAGS_PAD_MASK: u32 = FSCRYPT_POLICY_FLAGS_PAD_MASK;
pub const FS_POLICY_FLAG_DIRECT_KEY: u32 = FSCRYPT_POLICY_FLAG_DIRECT_KEY;
pub const FS_POLICY_FLAGS_VALID: u32 = 0x07; /* contains old flags only */
pub const FS_ENCRYPTION_MODE_INVALID: u32 = 0; /* never used */
pub const FS_ENCRYPTION_MODE_AES_256_XTS: u32 = FSCRYPT_MODE_AES_256_XTS;
pub const FS_ENCRYPTION_MODE_AES_256_GCM: u32 = 2; /* never used */
pub const FS_ENCRYPTION_MODE_AES_256_CBC: u32 = 3; /* never used */
pub const FS_ENCRYPTION_MODE_AES_256_CTS: u32 = FSCRYPT_MODE_AES_256_CTS;
pub const FS_ENCRYPTION_MODE_AES_128_CBC: u32 = FSCRYPT_MODE_AES_128_CBC;
pub const FS_ENCRYPTION_MODE_AES_128_CTS: u32 = FSCRYPT_MODE_AES_128_CTS;
pub const FS_ENCRYPTION_MODE_ADIANTUM: u32 = FSCRYPT_MODE_ADIANTUM;
pub const FS_KEY_DESC_PREFIX: &[u8; 8] = FSCRYPT_KEY_DESC_PREFIX;
pub const FS_KEY_DESC_PREFIX_SIZE: usize = FSCRYPT_KEY_DESC_PREFIX_SIZE;
pub const FS_MAX_KEY_SIZE: usize = FSCRYPT_MAX_KEY_SIZE;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
