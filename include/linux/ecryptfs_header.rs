/* SPDX-License-Identifier: GPL-2.0 */

// Version verification for shared data structures w/ userspace
pub const ECRYPTFS_VERSION_MAJOR: u32 = 0x00;
pub const ECRYPTFS_VERSION_MINOR: u32 = 0x04;
pub const ECRYPTFS_SUPPORTED_FILE_VERSION: u32 = 0x03;
// These flags indicate which features are supported by the kernel
// module; userspace tools such as the mount helper read the feature
// bits from a sysfs handle in order to determine how to behave.
pub const ECRYPTFS_VERSIONING_PASSPHRASE: u32 = 0x00000001;
pub const ECRYPTFS_VERSIONING_PUBKEY: u32 = 0x00000002;
pub const ECRYPTFS_VERSIONING_PLAINTEXT_PASSTHROUGH: u32 = 0x00000004;
pub const ECRYPTFS_VERSIONING_POLICY: u32 = 0x00000008;
pub const ECRYPTFS_VERSIONING_XATTR: u32 = 0x00000010;
pub const ECRYPTFS_VERSIONING_MULTKEY: u32 = 0x00000020;
pub const ECRYPTFS_VERSIONING_DEVMISC: u32 = 0x00000040;
pub const ECRYPTFS_VERSIONING_HMAC: u32 = 0x00000080;
pub const ECRYPTFS_VERSIONING_FILENAME_ENCRYPTION: u32 = 0x00000100;
pub const ECRYPTFS_VERSIONING_GCM: u32 = 0x00000200;
pub const ECRYPTFS_MAX_PASSWORD_LENGTH: usize = 64;
pub const ECRYPTFS_MAX_PASSPHRASE_BYTES: usize = ECRYPTFS_MAX_PASSWORD_LENGTH;
pub const ECRYPTFS_SALT_SIZE: usize = 8;
pub const ECRYPTFS_SALT_SIZE_HEX: usize = ECRYPTFS_SALT_SIZE * 2;
// The original signature size is only for what is stored on disk; all
// in-memory representations are expanded hex, so it better adapted to
// be passed around or referenced on the command line
pub const ECRYPTFS_SIG_SIZE: usize = 8;
pub const ECRYPTFS_SIG_SIZE_HEX: usize = ECRYPTFS_SIG_SIZE * 2;
pub const ECRYPTFS_PASSWORD_SIG_SIZE: usize = ECRYPTFS_SIG_SIZE_HEX;
pub const ECRYPTFS_MAX_KEY_BYTES: usize = 64;
pub const ECRYPTFS_MAX_ENCRYPTED_KEY_BYTES: usize = 512;
pub const ECRYPTFS_FILE_VERSION: u32 = 0x03;
pub const ECRYPTFS_MAX_PKI_NAME_BYTES: usize = 16;

pub const RFC2440_CIPHER_DES3_EDE: u32 = 0x02;
pub const RFC2440_CIPHER_CAST_5: u32 = 0x03;
pub const RFC2440_CIPHER_BLOWFISH: u32 = 0x04;
pub const RFC2440_CIPHER_AES_128: u32 = 0x07;
pub const RFC2440_CIPHER_AES_192: u32 = 0x08;
pub const RFC2440_CIPHER_AES_256: u32 = 0x09;
pub const RFC2440_CIPHER_TWOFISH: u32 = 0x0a;
pub const RFC2440_CIPHER_CAST_6: u32 = 0x0b;
pub const RFC2440_CIPHER_RSA: u32 = 0x01;

/**
 * For convenience, we may need to pass around the encrypted session
 * key between kernel and userspace because the authentication token
 * may not be extractable.  For example, the TPM may not release the
 * private key, instead requiring the encrypted data and returning the
 * decrypted data.
 */
#[repr(C)]
pub struct ecryptfs_session_key {
    pub flags: u32,
    pub encrypted_key_size: u32,
    pub decrypted_key_size: u32,
    pub encrypted_key: [u8; ECRYPTFS_MAX_ENCRYPTED_KEY_BYTES],
    pub decrypted_key: [u8; ECRYPTFS_MAX_KEY_BYTES],
}

pub const ECRYPTFS_USERSPACE_SHOULD_TRY_TO_DECRYPT: u32 = 0x00000001;
pub const ECRYPTFS_USERSPACE_SHOULD_TRY_TO_ENCRYPT: u32 = 0x00000002;
pub const ECRYPTFS_CONTAINS_DECRYPTED_KEY: u32 = 0x00000004;
pub const ECRYPTFS_CONTAINS_ENCRYPTED_KEY: u32 = 0x00000008;

#[repr(C)]
pub struct ecryptfs_password {
    pub password_bytes: u32,
    pub hash_algo: i32,
    pub hash_iterations: u32,
    pub session_key_encryption_key_bytes: u32,
    pub flags: u32,
    // Iterated-hash concatenation of salt and passphrase
    pub session_key_encryption_key: [u8; ECRYPTFS_MAX_KEY_BYTES],
    pub signature: [u8; ECRYPTFS_PASSWORD_SIG_SIZE + 1],
    // Always in expanded hex
    pub salt: [u8; ECRYPTFS_SALT_SIZE],
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ecryptfs_token_types {
    ECRYPTFS_PASSWORD = 0,
    ECRYPTFS_PRIVATE_KEY = 1,
}

#[repr(C)]
pub struct ecryptfs_private_key {
    pub key_size: u32,
    pub data_len: u32,
    pub signature: [u8; ECRYPTFS_PASSWORD_SIG_SIZE + 1],
    pub pki_type: [i8; ECRYPTFS_MAX_PKI_NAME_BYTES + 1],
    pub data: [u8; 0],
}

// May be a password or a private key
pub const ECRYPTFS_ENCRYPT_ONLY: u32 = 0x00000001;

#[repr(C, packed)]
pub union ecryptfs_auth_tok_token {
    pub password: ecryptfs_password,
    pub private_key: ecryptfs_private_key,
}

#[repr(C, packed)]
pub struct ecryptfs_auth_tok {
    pub version: u16, // 8-bit major and 8-bit minor
    pub token_type: u16,
    pub flags: u32,
    pub session_key: ecryptfs_session_key,
    pub reserved: [u8; 32],
    pub token: ecryptfs_auth_tok_token,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
