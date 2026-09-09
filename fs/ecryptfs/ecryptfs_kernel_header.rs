/* SPDX-License-Identifier: GPL-2.0-or-later */
/* eCryptfs: Linux filesystem encryption layer. Kernel declarations. */
// C dependencies are supplied by the surrounding kernel translation.

pub const ECRYPTFS_DEFAULT_IV_BYTES: usize = 16;
pub const ECRYPTFS_DEFAULT_EXTENT_SIZE: usize = 4096;
pub const ECRYPTFS_MINIMUM_HEADER_EXTENT_SIZE: usize = 8192;
pub const ECRYPTFS_DEFAULT_MSG_CTX_ELEMS: usize = 32;
pub const ECRYPTFS_DEFAULT_SEND_TIMEOUT: usize = HZ;
pub const ECRYPTFS_MAX_MSG_CTX_TTL: usize = HZ * 3;
pub const ECRYPTFS_DEFAULT_NUM_USERS: usize = 4;
pub const ECRYPTFS_MAX_NUM_USERS: usize = 32768;
pub const ECRYPTFS_XATTR_NAME: &str = "user.ecryptfs";

extern "C" {
    pub fn ecryptfs_dump_auth_tok(auth_tok: *mut ecryptfs_auth_tok);
    pub fn ecryptfs_from_hex(dst: *mut c_char, src: *mut c_char, dst_size: c_int);
}

#[inline]
pub unsafe fn ecryptfs_to_hex(dst: *mut c_char, src: *mut c_char, src_size: usize) {
    let end = bin2hex(dst, src, src_size);
    *end = 0;
}

#[repr(C)] pub struct ecryptfs_key_record { pub type_: u8, pub enc_key_size: usize, pub sig: [u8; ECRYPTFS_SIG_SIZE], pub enc_key: [u8; ECRYPTFS_MAX_ENCRYPTED_KEY_BYTES] }
#[repr(C)] pub struct ecryptfs_auth_tok_list { pub auth_tok: *mut ecryptfs_auth_tok, pub list: list_head }
pub struct ecryptfs_crypt_stat;
pub struct ecryptfs_mount_crypt_stat;
#[repr(C)] pub union ecryptfs_page_crypt_context_param { pub lower_file: *mut file, pub wbc: *mut writeback_control }
#[repr(C)] pub struct ecryptfs_page_crypt_context { pub page: *mut page, pub mode: c_uint, pub param: ecryptfs_page_crypt_context_param }
pub const ECRYPTFS_PREPARE_COMMIT_MODE: c_uint = 0;
pub const ECRYPTFS_WRITEPAGE_MODE: c_uint = 1;

#[cfg(any(CONFIG_ENCRYPTED_KEYS, CONFIG_ENCRYPTED_KEYS_MODULE))]
#[inline] pub unsafe fn ecryptfs_get_encrypted_key_payload_data(key: *mut key) -> *mut ecryptfs_auth_tok {
    if (*key).type_ != &key_type_encrypted { return core::ptr::null_mut(); }
    let payload = (*key).payload.data[0];
    if payload.is_null() { return ERR_PTR(-EKEYREVOKED); }
    (*payload).payload_data as *mut ecryptfs_auth_tok
}
#[cfg(any(CONFIG_ENCRYPTED_KEYS, CONFIG_ENCRYPTED_KEYS_MODULE))]
#[inline] pub unsafe fn ecryptfs_get_encrypted_key(sig: *mut c_char) -> *mut key { request_key(&key_type_encrypted, sig, core::ptr::null()) }
#[cfg(not(any(CONFIG_ENCRYPTED_KEYS, CONFIG_ENCRYPTED_KEYS_MODULE)))]
#[inline] pub unsafe fn ecryptfs_get_encrypted_key_payload_data(_: *mut key) -> *mut ecryptfs_auth_tok { core::ptr::null_mut() }
#[cfg(not(any(CONFIG_ENCRYPTED_KEYS, CONFIG_ENCRYPTED_KEYS_MODULE)))]
#[inline] pub unsafe fn ecryptfs_get_encrypted_key(_: *mut c_char) -> *mut key { ERR_PTR(-ENOKEY) }

#[inline] pub unsafe fn ecryptfs_get_key_payload_data(key: *mut key) -> *mut ecryptfs_auth_tok {
    let auth_tok = ecryptfs_get_encrypted_key_payload_data(key);
    if !auth_tok.is_null() { return auth_tok; }
    let ukp = user_key_payload_locked(key);
    if ukp.is_null() { return ERR_PTR(-EKEYREVOKED); }
    (*ukp).data as *mut ecryptfs_auth_tok
}

pub const ECRYPTFS_MAX_KEYSET_SIZE: usize = 1024;
pub const ECRYPTFS_MAX_CIPHER_NAME_SIZE: usize = 31;
pub const ECRYPTFS_MAX_NUM_ENC_KEYS: usize = 64;
pub const ECRYPTFS_MAX_IV_BYTES: usize = 16;
pub const ECRYPTFS_SALT_BYTES: usize = 2;
pub const MAGIC_ECRYPTFS_MARKER: u32 = 0x3c81b7f5;
pub const MAGIC_ECRYPTFS_MARKER_SIZE_BYTES: usize = 8;
pub const ECRYPTFS_FILE_SIZE_BYTES: usize = core::mem::size_of::<u64>();
pub const ECRYPTFS_SIZE_AND_MARKER_BYTES: usize = ECRYPTFS_FILE_SIZE_BYTES + MAGIC_ECRYPTFS_MARKER_SIZE_BYTES;
pub const ECRYPTFS_DEFAULT_CIPHER: &str = "aes";
pub const ECRYPTFS_DEFAULT_KEY_BYTES: usize = 16;
pub const ECRYPTFS_TAG_1_PACKET_TYPE: u8 = 1;
pub const ECRYPTFS_TAG_3_PACKET_TYPE: u8 = 0x8c;
pub const ECRYPTFS_TAG_11_PACKET_TYPE: u8 = 0xed;
pub const ECRYPTFS_TAG_64_PACKET_TYPE: u8 = 0x40;
pub const ECRYPTFS_TAG_65_PACKET_TYPE: u8 = 0x41;
pub const ECRYPTFS_TAG_66_PACKET_TYPE: u8 = 0x42;
pub const ECRYPTFS_TAG_67_PACKET_TYPE: u8 = 0x43;
pub const ECRYPTFS_TAG_70_PACKET_TYPE: u8 = 0x46;
pub const ECRYPTFS_TAG_71_PACKET_TYPE: u8 = 0x47;
pub const ECRYPTFS_TAG_72_PACKET_TYPE: u8 = 0x48;
pub const ECRYPTFS_TAG_73_PACKET_TYPE: u8 = 0x49;
pub const ECRYPTFS_MIN_PKT_LEN_SIZE: usize = 1;
pub const ECRYPTFS_MAX_PKT_LEN_SIZE: usize = 2;
pub const ECRYPTFS_FILENAME_MIN_RANDOM_PREPEND_BYTES: usize = 16;
pub const ECRYPTFS_NON_NULL: usize = 0x42;
pub const ECRYPTFS_TAG_70_MIN_METADATA_SIZE: usize = 1 + ECRYPTFS_MIN_PKT_LEN_SIZE + ECRYPTFS_SIG_SIZE + 1 + 1;
pub const ECRYPTFS_TAG_70_MAX_METADATA_SIZE: usize = 1 + ECRYPTFS_MAX_PKT_LEN_SIZE + ECRYPTFS_SIG_SIZE + 1 + 1;
pub const ECRYPTFS_FEK_ENCRYPTED_FILENAME_PREFIX: &str = "ECRYPTFS_FEK_ENCRYPTED.";
pub const ECRYPTFS_FEK_ENCRYPTED_FILENAME_PREFIX_SIZE: usize = 23;
pub const ECRYPTFS_FNEK_ENCRYPTED_FILENAME_PREFIX: &str = "ECRYPTFS_FNEK_ENCRYPTED.";
pub const ECRYPTFS_FNEK_ENCRYPTED_FILENAME_PREFIX_SIZE: usize = 24;
pub const ECRYPTFS_ENCRYPTED_DENTRY_NAME_LEN: usize = 18 + 1 + 4 + 1 + 32;

#[repr(C)] pub struct ecryptfs_key_sig { pub crypt_stat_list: list_head, pub keysig: [c_char; ECRYPTFS_SIG_SIZE_HEX + 1] }
#[repr(C)] pub struct ecryptfs_filename { pub crypt_stat_list: list_head, pub flags: u32, pub seq_no: u32, pub filename: *mut c_char, pub encrypted_filename: *mut c_char, pub filename_size: usize, pub encrypted_filename_size: usize, pub fnek_sig: [c_char; ECRYPTFS_SIG_SIZE_HEX], pub dentry_name: [c_char; ECRYPTFS_ENCRYPTED_DENTRY_NAME_LEN + 1] }
#[repr(C)] pub struct ecryptfs_crypt_stat { pub flags: u32, pub file_version: c_uint, pub iv_bytes: usize, pub metadata_size: usize, pub extent_size: usize, pub key_size: usize, pub extent_shift: usize, pub extent_mask: c_uint, pub mount_crypt_stat: *mut ecryptfs_mount_crypt_stat, pub tfm: *mut crypto_skcipher, pub cipher: [u8; ECRYPTFS_MAX_CIPHER_NAME_SIZE + 1], pub key: [u8; ECRYPTFS_MAX_KEY_BYTES], pub root_iv: [u8; ECRYPTFS_MAX_IV_BYTES], pub keysig_list: list_head, pub keysig_list_mutex: mutex, pub cs_tfm_mutex: mutex, pub cs_mutex: mutex }
#[repr(C)] pub struct ecryptfs_inode_info { pub vfs_inode: inode, pub wii_inode: *mut inode, pub lower_file_mutex: mutex, pub lower_file_count: atomic_t, pub lower_file: *mut file, pub crypt_stat: ecryptfs_crypt_stat }
#[repr(C)] pub struct ecryptfs_global_auth_tok { pub flags: u32, pub mount_crypt_stat_list: list_head, pub global_auth_tok_key: *mut key, pub sig: [u8; ECRYPTFS_SIG_SIZE_HEX + 1] }
#[repr(C)] pub struct ecryptfs_key_tfm { pub key_tfm: *mut crypto_skcipher, pub key_size: usize, pub key_tfm_mutex: mutex, pub key_tfm_list: list_head, pub cipher_name: [u8; ECRYPTFS_MAX_CIPHER_NAME_SIZE + 1] }
extern "C" { pub static mut key_tfm_list_mutex: mutex; }
#[repr(C)] pub struct ecryptfs_mount_crypt_stat { pub flags: u32, pub global_auth_tok_list: list_head, pub global_auth_tok_list_mutex: mutex, pub global_default_cipher_key_size: usize, pub global_default_fn_cipher_key_bytes: usize, pub global_default_cipher_name: [u8; ECRYPTFS_MAX_CIPHER_NAME_SIZE + 1], pub global_default_fn_cipher_name: [u8; ECRYPTFS_MAX_CIPHER_NAME_SIZE + 1], pub global_default_fnek_sig: [c_char; ECRYPTFS_SIG_SIZE_HEX + 1] }
#[repr(C)] pub struct ecryptfs_sb_info { pub wsi_sb: *mut super_block, pub lower_mnt: *mut vfsmount, pub mount_crypt_stat: ecryptfs_mount_crypt_stat }
#[repr(C)] pub struct ecryptfs_file_info { pub wfi_file: *mut file, pub crypt_stat: *mut ecryptfs_crypt_stat }
#[repr(C)] pub struct ecryptfs_auth_tok_list_item { pub encrypted_session_key: [u8; ECRYPTFS_MAX_KEY_BYTES], pub list: list_head, pub auth_tok: ecryptfs_auth_tok }
#[repr(C)] pub struct ecryptfs_message { pub index: u32, pub data_len: u32, pub data: [u8; 0] }
#[repr(C)] pub struct ecryptfs_msg_ctx { pub state: u8, pub type_: u8, pub index: u32, pub counter: u32, pub msg_size: usize, pub msg: *mut ecryptfs_message, pub task: *mut task_struct, pub node: list_head, pub daemon_out_list: list_head, pub mux: mutex }
#[repr(C)] pub struct ecryptfs_daemon { pub flags: u32, pub num_queued_msg_ctx: u32, pub file: *mut file, pub mux: mutex, pub msg_ctx_out_queue: list_head, pub wait: wait_queue_head_t, pub euid_chain: hlist_node }

#[inline] pub unsafe fn ecryptfs_lower_header_size(cs: *mut ecryptfs_crypt_stat) -> usize { if (*cs).flags & ECRYPTFS_METADATA_IN_XATTR != 0 { 0 } else { (*cs).metadata_size } }
#[inline] pub unsafe fn ecryptfs_file_to_private(f: *mut file) -> *mut ecryptfs_file_info { (*f).private_data as *mut ecryptfs_file_info }
#[inline] pub unsafe fn ecryptfs_set_file_private(f: *mut file, i: *mut ecryptfs_file_info) { (*f).private_data = i as *mut c_void; }
#[inline] pub unsafe fn ecryptfs_file_to_lower(f: *mut file) -> *mut file { ecryptfs_file_to_private(f).as_ref().unwrap().wfi_file }
#[inline] pub unsafe fn ecryptfs_set_file_lower(f: *mut file, l: *mut file) { (*ecryptfs_file_to_private(f)).wfi_file = l; }
#[inline] pub unsafe fn ecryptfs_inode_to_private(i: *mut inode) -> *mut ecryptfs_inode_info { container_of!(i, ecryptfs_inode_info, vfs_inode) }
#[inline] pub unsafe fn ecryptfs_inode_to_lower(i: *mut inode) -> *mut inode { (*ecryptfs_inode_to_private(i)).wii_inode }
#[inline] pub unsafe fn ecryptfs_set_inode_lower(i: *mut inode, l: *mut inode) { (*ecryptfs_inode_to_private(i)).wii_inode = l; }
#[inline] pub unsafe fn ecryptfs_superblock_to_private(s: *mut super_block) -> *mut ecryptfs_sb_info { (*s).s_fs_info as *mut ecryptfs_sb_info }
#[inline] pub unsafe fn ecryptfs_set_superblock_private(s: *mut super_block, i: *mut ecryptfs_sb_info) { (*s).s_fs_info = i as *mut c_void; }
#[inline] pub unsafe fn ecryptfs_superblock_to_lower(s: *mut super_block) -> *mut super_block { (*ecryptfs_superblock_to_private(s)).wsi_sb }
#[inline] pub unsafe fn ecryptfs_set_superblock_lower(s: *mut super_block, l: *mut super_block) { (*ecryptfs_superblock_to_private(s)).wsi_sb = l; }
#[inline] pub unsafe fn ecryptfs_set_dentry_lower(d: *mut dentry, l: *mut dentry) { (*d).d_fsdata = l as *mut c_void; }
#[inline] pub unsafe fn ecryptfs_dentry_to_lower(d: *mut dentry) -> *mut dentry { (*d).d_fsdata as *mut dentry }
#[inline] pub unsafe fn ecryptfs_lower_path(d: *mut dentry) -> path { path { mnt: (*ecryptfs_superblock_to_private((*d).d_sb)).lower_mnt, dentry: ecryptfs_dentry_to_lower(d) } }

extern "C" { pub fn __ecryptfs_printk(fmt: *const c_char, ...); }
extern "C" { pub static ecryptfs_main_fops: file_operations; pub static ecryptfs_dir_fops: file_operations; pub static ecryptfs_main_iops: inode_operations; pub static ecryptfs_dir_iops: inode_operations; pub static ecryptfs_symlink_iops: inode_operations; pub static ecryptfs_sops: super_operations; pub static ecryptfs_dops: dentry_operations; pub static ecryptfs_aops: address_space_operations; pub static mut ecryptfs_verbosity: c_int; pub static mut ecryptfs_message_buf_len: c_uint; pub static mut ecryptfs_message_wait_timeout: c_long; pub static mut ecryptfs_number_of_users: c_uint; }

extern "C" {
    pub fn ecryptfs_get_inode(lower_inode: *mut inode, sb: *mut super_block) -> *mut inode;
    pub fn ecryptfs_i_size_init(page_virt: *const c_char, inode: *mut inode);
    pub fn ecryptfs_initialize_file(dentry: *mut dentry, inode: *mut inode) -> c_int;
    pub fn ecryptfs_decode_and_decrypt_filename(decrypted_name: *mut *mut c_char, decrypted_name_size: *mut usize, sb: *mut super_block, name: *const c_char, name_size: usize) -> c_int;
    pub fn ecryptfs_encrypt_and_encode_filename(encoded_name: *mut *mut c_char, encoded_name_size: *mut usize, m: *mut ecryptfs_mount_crypt_stat, name: *const c_char, name_size: usize) -> c_int;
    pub fn ecryptfs_dump_hex(data: *mut c_char, bytes: c_int);
    pub fn virt_to_scatterlist(addr: *const c_void, size: c_int, sg: *mut scatterlist, sg_size: c_int) -> c_int;
    pub fn ecryptfs_compute_root_iv(cs: *mut ecryptfs_crypt_stat) -> c_int;
    pub fn ecryptfs_rotate_iv(iv: *mut u8);
    pub fn ecryptfs_init_crypt_stat(cs: *mut ecryptfs_crypt_stat);
    pub fn ecryptfs_destroy_crypt_stat(cs: *mut ecryptfs_crypt_stat);
    pub fn ecryptfs_destroy_mount_crypt_stat(cs: *mut ecryptfs_mount_crypt_stat);
    pub fn ecryptfs_init_crypt_ctx(cs: *mut ecryptfs_crypt_stat) -> c_int;
    pub fn ecryptfs_write_inode_size_to_metadata(i: *mut inode) -> c_int;
    pub fn ecryptfs_encrypt_page(f: *mut folio) -> c_int;
    pub fn ecryptfs_decrypt_page(f: *mut folio) -> c_int;
    pub fn ecryptfs_write_metadata(d: *mut dentry, i: *mut inode) -> c_int;
    pub fn ecryptfs_read_metadata(d: *mut dentry) -> c_int;
    pub fn ecryptfs_new_file_context(i: *mut inode) -> c_int;
    pub fn ecryptfs_write_crypt_stat_flags(p: *mut c_char, cs: *mut ecryptfs_crypt_stat, w: *mut usize);
    pub fn ecryptfs_read_and_validate_header_region(i: *mut inode) -> c_int;
    pub fn ecryptfs_read_and_validate_xattr_region(d: *mut dentry, i: *mut inode) -> c_int;
    pub fn ecryptfs_code_for_cipher_string(n: *mut c_char, k: usize) -> u8;
    pub fn ecryptfs_cipher_code_to_string(s: *mut c_char, z: usize, c: u8) -> c_int;
    pub fn ecryptfs_set_default_sizes(cs: *mut ecryptfs_crypt_stat);
    pub fn ecryptfs_parse_packet_length(data: *mut u8, size: *mut usize, length_size: *mut usize) -> c_int;
    pub fn ecryptfs_write_packet_length(dest: *mut c_char, size: usize, packet_size_length: *mut usize) -> c_int;
    pub fn ecryptfs_init_crypto() -> c_int;
    pub fn ecryptfs_destroy_crypto() -> c_int;
    pub fn ecryptfs_init_kthread() -> c_int;
    pub fn ecryptfs_destroy_kthread();
}

pub const ECRYPTFS_STRUCT_INITIALIZED: u32 = 1;
pub const ECRYPTFS_POLICY_APPLIED: u32 = 2;
pub const ECRYPTFS_ENCRYPTED: u32 = 4;
pub const ECRYPTFS_METADATA_IN_XATTR: u32 = 0x80;
pub const ECRYPTFS_MSG_CTX_STATE_FREE: u8 = 1;
pub const ECRYPTFS_MSG_CTX_STATE_PENDING: u8 = 2;
pub const ECRYPTFS_MSG_CTX_STATE_DONE: u8 = 3;
pub const ECRYPTFS_MSG_CTX_STATE_NO_REPLY: u8 = 4;
pub const ECRYPTFS_MSG_HELO: u8 = 100;
pub const ECRYPTFS_MSG_QUIT: u8 = 101;
pub const ECRYPTFS_MSG_REQUEST: u8 = 102;
pub const ECRYPTFS_MSG_RESPONSE: u8 = 103;

extern "C" {
    pub fn ecryptfs_truncate(d: *mut dentry, n: loff_t) -> c_int;
    pub fn ecryptfs_getxattr_lower(d: *mut dentry, i: *mut inode, n: *const c_char, v: *mut c_void, z: usize) -> ssize_t;
    pub fn ecryptfs_setxattr(d: *mut dentry, i: *mut inode, n: *const c_char, v: *const c_void, z: usize, f: c_int) -> c_int;
    pub fn ecryptfs_read_xattr_region(p: *mut c_char, i: *mut inode) -> c_int;
    pub fn ecryptfs_write_header_metadata(v: *mut c_char, cs: *mut ecryptfs_crypt_stat, w: *mut usize);
    pub fn ecryptfs_add_keysig(cs: *mut ecryptfs_crypt_stat, s: *mut c_char) -> c_int;
    pub fn ecryptfs_add_global_auth_tok(m: *mut ecryptfs_mount_crypt_stat, s: *mut c_char, f: u32) -> c_int;
    pub fn ecryptfs_get_global_auth_tok_for_sig(a: *mut *mut ecryptfs_global_auth_tok, m: *mut ecryptfs_mount_crypt_stat, s: *mut c_char) -> c_int;
    pub fn ecryptfs_add_new_key_tfm(t: *mut *mut ecryptfs_key_tfm, n: *mut c_char, z: usize) -> c_int;
    pub fn ecryptfs_tfm_exists(n: *mut c_char, t: *mut *mut ecryptfs_key_tfm) -> c_int;
    pub fn ecryptfs_get_tfm_and_mutex_for_cipher_name(t: *mut *mut crypto_skcipher, m: *mut *mut mutex, n: *mut c_char) -> c_int;
    pub fn ecryptfs_keyring_auth_tok_for_sig(k: *mut *mut key, a: *mut *mut ecryptfs_auth_tok, s: *mut c_char) -> c_int;
    pub fn ecryptfs_write_lower(i: *mut inode, d: *mut c_char, o: loff_t, z: usize) -> c_int;
    pub fn ecryptfs_write_lower_page_segment(i: *mut inode, f: *mut folio, o: usize, z: usize) -> c_int;
    pub fn ecryptfs_write(i: *mut inode, d: *mut c_char, o: loff_t, z: usize) -> c_int;
    pub fn ecryptfs_read_lower(d: *mut c_char, o: loff_t, z: usize, i: *mut inode) -> c_int;
    pub fn ecryptfs_read_lower_page_segment(f: *mut folio, p: pgoff_t, o: usize, z: usize, i: *mut inode) -> c_int;
    pub fn ecryptfs_privileged_open(f: *mut *mut file, d: *mut dentry, m: *mut vfsmount, c: *const cred) -> c_int;
    pub fn ecryptfs_get_lower_file(d: *mut dentry, i: *mut inode) -> c_int;
    pub fn ecryptfs_put_lower_file(i: *mut inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
