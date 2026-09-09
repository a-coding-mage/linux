/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of fscrypt_private.h. */

pub const FSCRYPT_FILE_NONCE_SIZE: usize = 16;
pub const FSCRYPT_MIN_KEY_SIZE: usize = 16;
pub const FSCRYPT_MAX_RAW_KEY_SIZE: usize = 64;
pub const FSCRYPT_MAX_HW_WRAPPED_KEY_SIZE: usize = BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE;
pub const FSCRYPT_MAX_ANY_KEY_SIZE: usize = if FSCRYPT_MAX_RAW_KEY_SIZE > FSCRYPT_MAX_HW_WRAPPED_KEY_SIZE { FSCRYPT_MAX_RAW_KEY_SIZE } else { FSCRYPT_MAX_HW_WRAPPED_KEY_SIZE };
pub const FSCRYPT_CRYPTOAPI_MASK: u32 = CRYPTO_ALG_ASYNC | CRYPTO_ALG_ALLOCATES_MEMORY | CRYPTO_ALG_KERN_DRIVER_ONLY;
pub const FSCRYPT_CONTEXT_V1: u8 = 1;
pub const FSCRYPT_CONTEXT_V2: u8 = 2;
pub const FSCRYPT_MAX_IV_SIZE: usize = 32;

#[repr(C)]
pub struct fscrypt_context_v1 { pub version: u8, pub contents_encryption_mode: u8, pub filenames_encryption_mode: u8, pub flags: u8, pub master_key_descriptor: [u8; FSCRYPT_KEY_DESCRIPTOR_SIZE], pub nonce: [u8; FSCRYPT_FILE_NONCE_SIZE] }
#[repr(C)]
pub struct fscrypt_context_v2 { pub version: u8, pub contents_encryption_mode: u8, pub filenames_encryption_mode: u8, pub flags: u8, pub log2_data_unit_size: u8, pub __reserved: [u8; 3], pub master_key_identifier: [u8; FSCRYPT_KEY_IDENTIFIER_SIZE], pub nonce: [u8; FSCRYPT_FILE_NONCE_SIZE] }
#[repr(C)]
pub union fscrypt_context { pub version: u8, pub v1: fscrypt_context_v1, pub v2: fscrypt_context_v2 }

pub unsafe fn fscrypt_context_size(ctx: *const fscrypt_context) -> i32 { match (*ctx).version { FSCRYPT_CONTEXT_V1 => 28, FSCRYPT_CONTEXT_V2 => 40, _ => 0 } }
pub unsafe fn fscrypt_context_is_valid(ctx: *const fscrypt_context, ctx_size: i32) -> bool { ctx_size >= 1 && ctx_size == fscrypt_context_size(ctx) }
pub unsafe fn fscrypt_context_nonce(ctx: *const fscrypt_context) -> *const u8 { match (*ctx).version { FSCRYPT_CONTEXT_V1 => (*ctx).v1.nonce.as_ptr(), FSCRYPT_CONTEXT_V2 => (*ctx).v2.nonce.as_ptr(), _ => { WARN_ON_ONCE(1); core::ptr::null() } } }

#[repr(C)] pub union fscrypt_policy { pub version: u8, pub v1: fscrypt_policy_v1, pub v2: fscrypt_policy_v2 }
pub unsafe fn fscrypt_policy_size(policy: *const fscrypt_policy) -> i32 { match (*policy).version { FSCRYPT_POLICY_V1 => core::mem::size_of::<fscrypt_policy_v1>() as i32, FSCRYPT_POLICY_V2 => core::mem::size_of::<fscrypt_policy_v2>() as i32, _ => 0 } }
pub unsafe fn fscrypt_policy_contents_mode(policy: *const fscrypt_policy) -> u8 { match (*policy).version { FSCRYPT_POLICY_V1 => (*policy).v1.contents_encryption_mode, FSCRYPT_POLICY_V2 => (*policy).v2.contents_encryption_mode, _ => { BUG(); 0 } } }
pub unsafe fn fscrypt_policy_fnames_mode(policy: *const fscrypt_policy) -> u8 { match (*policy).version { FSCRYPT_POLICY_V1 => (*policy).v1.filenames_encryption_mode, FSCRYPT_POLICY_V2 => (*policy).v2.filenames_encryption_mode, _ => { BUG(); 0 } } }
pub unsafe fn fscrypt_policy_flags(policy: *const fscrypt_policy) -> u8 { match (*policy).version { FSCRYPT_POLICY_V1 => (*policy).v1.flags, FSCRYPT_POLICY_V2 => (*policy).v2.flags, _ => { BUG(); 0 } } }
pub unsafe fn fscrypt_policy_v2_du_bits(policy: *const fscrypt_policy_v2, inode: *const inode) -> i32 { let v = (*policy).log2_data_unit_size; if v != 0 { v as i32 } else { (*inode).i_blkbits } }
pub unsafe fn fscrypt_policy_du_bits(policy: *const fscrypt_policy, inode: *const inode) -> i32 { match (*policy).version { FSCRYPT_POLICY_V1 => (*inode).i_blkbits, FSCRYPT_POLICY_V2 => fscrypt_policy_v2_du_bits(&(*policy).v2, inode), _ => { BUG(); 0 } } }

#[repr(C, packed)] pub struct fscrypt_symlink_data { pub len: __le16, pub encrypted_path: [c_char; 0] }
#[repr(C)] pub struct fscrypt_prepared_key { pub tfm: *mut crypto_sync_skcipher, #[cfg(feature = "CONFIG_FS_ENCRYPTION_INLINE_CRYPT")] pub blk_key: *mut blk_crypto_key }
#[repr(C)] pub struct fscrypt_mode_key { pub key: fscrypt_prepared_key, pub link: list_head, pub hkdf_context: u8, pub mode_num: u8, pub data_unit_bits: u8 }
#[repr(C)] pub struct fscrypt_inode_info { pub ci_enc_key: fscrypt_prepared_key, pub ci_owns_key: u8, pub ci_dirhash_key_initialized: u8, pub ci_data_unit_bits: u8, pub ci_hashed_ino: u32, pub ci_mode: *mut fscrypt_mode, pub ci_inode: *mut inode, pub ci_master_key: *mut fscrypt_master_key, pub ci_master_key_link: list_head, pub ci_direct_key: *mut fscrypt_direct_key, pub ci_dirhash_key: siphash_key_t, pub ci_policy: fscrypt_policy, pub ci_nonce: [u8; FSCRYPT_FILE_NONCE_SIZE] }
pub type fscrypt_direction_t = u32; pub const FS_DECRYPT: u32 = 0; pub const FS_ENCRYPT: u32 = 1;

extern "C" { pub static mut fscrypt_inode_info_cachep: *mut kmem_cache; pub fn fscrypt_initialize(sb: *mut super_block) -> i32; pub fn fscrypt_generate_iv(iv: *mut fscrypt_iv, index: u64, ci: *const fscrypt_inode_info); pub fn __fscrypt_fname_encrypted_size(policy: *const fscrypt_policy, orig_len: u32, max_len: u32, encrypted_len_ret: *mut u32) -> bool; pub fn fscrypt_init_hkdf(hkdf: *mut hmac_sha512_key, master_key: *const u8, master_key_size: u32); pub fn fscrypt_hkdf_expand(hkdf: *const hmac_sha512_key, context: u8, info: *const u8, infolen: u32, okm: *mut u8, okmlen: u32); }

#[repr(C)] pub union fscrypt_iv { pub fields: fscrypt_iv_fields, pub raw: [u8; FSCRYPT_MAX_IV_SIZE], pub dun: [__le64; FSCRYPT_MAX_IV_SIZE / 8] }
#[repr(C)] pub struct fscrypt_iv_fields { pub index: __le64, pub nonce: [u8; FSCRYPT_FILE_NONCE_SIZE] }
pub unsafe fn fscrypt_max_file_dun_bits(sb: *const super_block, du_bits: i32) -> i32 { fls64((*sb).s_maxbytes - 1) - du_bits }

pub const HKDF_CONTEXT_KEY_IDENTIFIER_FOR_RAW_KEY: u8 = 1; pub const HKDF_CONTEXT_PER_FILE_ENC_KEY: u8 = 2; pub const HKDF_CONTEXT_DIRECT_KEY: u8 = 3; pub const HKDF_CONTEXT_IV_INO_LBLK_64_KEY: u8 = 4; pub const HKDF_CONTEXT_DIRHASH_KEY: u8 = 5; pub const HKDF_CONTEXT_IV_INO_LBLK_32_KEY: u8 = 6; pub const HKDF_CONTEXT_INODE_HASH_KEY: u8 = 7; pub const HKDF_CONTEXT_KEY_IDENTIFIER_FOR_HW_WRAPPED_KEY: u8 = 8;

#[repr(C)] pub struct fscrypt_master_key_user { pub link: list_head, pub uid: kuid_t, pub quota_key: *mut key }
#[repr(C)] pub struct fscrypt_master_key_secret { pub hkdf: hmac_sha512_key, pub is_hw_wrapped: bool, pub size: u32, pub bytes: [u8; FSCRYPT_MAX_ANY_KEY_SIZE] }
#[repr(C)] pub struct fscrypt_master_key { pub mk_node: hlist_node, pub mk_sem: rw_semaphore, pub mk_active_refs: refcount_t, pub mk_struct_refs: refcount_t, pub mk_rcu_head: rcu_head, pub mk_secret: fscrypt_master_key_secret, pub mk_spec: fscrypt_key_specifier, pub mk_users: list_head, pub mk_decrypted_inodes: list_head, pub mk_decrypted_inodes_lock: spinlock_t, pub mk_mode_keys: list_head, pub mk_ino_hash_key: siphash_key_t, pub mk_ino_hash_key_initialized: bool, pub mk_present: bool }
pub unsafe fn master_key_spec_type(spec: *const fscrypt_key_specifier) -> *const c_char { match (*spec).type_ { FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR => b"descriptor\0".as_ptr() as _, FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER => b"identifier\0".as_ptr() as _, _ => b"[unknown]\0".as_ptr() as _ } }
pub unsafe fn master_key_spec_len(spec: *const fscrypt_key_specifier) -> i32 { match (*spec).type_ { FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR => FSCRYPT_KEY_DESCRIPTOR_SIZE as i32, FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER => FSCRYPT_KEY_IDENTIFIER_SIZE as i32, _ => 0 } }

#[repr(C)] pub struct fscrypt_mode { pub friendly_name: *const c_char, pub cipher_str: *const c_char, pub keysize: i32, pub security_strength: i32, pub ivsize: i32, pub logged_cryptoapi_impl: i32, pub logged_blk_crypto_native: i32, pub logged_blk_crypto_fallback: i32, pub blk_crypto_mode: blk_crypto_mode_num }
extern "C" { pub static mut fscrypt_modes: [fscrypt_mode; 0]; pub fn fscrypt_get_encryption_info(inode: *mut inode, allow_unsupported: bool) -> i32; pub fn fscrypt_put_direct_key(dk: *mut fscrypt_direct_key); pub fn fscrypt_prepare_key(prep_key: *mut fscrypt_prepared_key, raw_key: *const u8, ci: *const fscrypt_inode_info) -> i32; pub fn fscrypt_destroy_prepared_key(sb: *mut super_block, prep_key: *mut fscrypt_prepared_key); }

extern "C" {
    pub fn fscrypt_msg(inode: *const inode, level: *const c_char, fmt: *const c_char, ...);
    pub fn fscrypt_put_master_key(mk: *mut fscrypt_master_key);
    pub fn fscrypt_put_master_key_activeref(sb: *mut super_block, mk: *mut fscrypt_master_key);
    pub fn fscrypt_find_master_key(sb: *mut super_block, spec: *const fscrypt_key_specifier) -> *mut fscrypt_master_key;
    pub fn fscrypt_get_test_dummy_key_identifier(id: *mut u8);
    pub fn fscrypt_add_test_dummy_key(sb: *mut super_block, spec: *mut fscrypt_key_specifier) -> i32;
    pub fn fscrypt_verify_key_added(sb: *mut super_block, id: *const u8) -> i32;
    pub fn fscrypt_init_keyring();
    pub fn fscrypt_set_per_file_enc_key(ci: *mut fscrypt_inode_info, raw_key: *const u8) -> i32;
    pub fn fscrypt_derive_dirhash_key(ci: *mut fscrypt_inode_info, mk: *const fscrypt_master_key);
    pub fn fscrypt_hash_inode_number(ci: *mut fscrypt_inode_info, mk: *const fscrypt_master_key);
    pub fn fscrypt_setup_v1_file_key(ci: *mut fscrypt_inode_info, raw_master_key: *const u8) -> i32;
    pub fn fscrypt_setup_v1_file_key_via_subscribed_keyrings(ci: *mut fscrypt_inode_info) -> i32;
    pub fn fscrypt_policies_equal(policy1: *const fscrypt_policy, policy2: *const fscrypt_policy) -> bool;
    pub fn fscrypt_policy_to_key_spec(policy: *const fscrypt_policy, spec: *mut fscrypt_key_specifier) -> i32;
    pub fn fscrypt_get_dummy_policy(sb: *mut super_block) -> *const fscrypt_policy;
    pub fn fscrypt_supported_policy(policy: *const fscrypt_policy, inode: *const inode) -> bool;
    pub fn fscrypt_policy_from_context(policy: *mut fscrypt_policy, ctx: *const fscrypt_context, ctx_size: i32) -> i32;
    pub fn fscrypt_policy_to_inherit(dir: *mut inode) -> *const fscrypt_policy;
}

pub unsafe fn fscrypt_require_key(inode: *mut inode) -> i32 {
    if IS_ENCRYPTED(inode) {
        let err = fscrypt_get_encryption_info(inode, false);
        if err != 0 { return err; }
        if !fscrypt_has_encryption_key(inode) { return -ENOKEY; }
    }
    0
}

/* Types and constants supplied by the kernel headers are intentionally external dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
