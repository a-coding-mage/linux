/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2009-2010 IBM Corporation
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 */

// pr_fmt(fmt) was defined in C as: KBUILD_MODNAME ": " fmt
// C includes referenced Linux and crypto definitions supplied by other files.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u8 = u8;
pub type uint8_t = u8;
pub type __u8 = u8;
pub type __be16 = u16;
pub type __be32 = u32;
pub type loff_t = i64;
pub type size_t = usize;
pub type key_perm_t = u32;
pub type gfp_t = u32;

pub const EOPNOTSUPP: c_int = 95;

// Supplied by crypto/sha1.h and crypto/hash.h in the original C header.
pub const SHA1_DIGEST_SIZE: usize = 20;
pub const HASH_MAX_DIGESTSIZE: usize = 64;

pub const IMA_MAX_DIGEST_SIZE: usize = HASH_MAX_DIGESTSIZE;

#[repr(C)]
pub struct file {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
	_private: [u8; 0],
}

#[repr(C)]
pub struct modsig {
	_private: [u8; 0],
}

#[repr(C)]
pub struct key {
	_private: [u8; 0],
}

#[repr(C)]
pub struct inode {
	_private: [u8; 0],
}

#[repr(C)]
pub struct audit_context {
	_private: [u8; 0],
}

#[repr(C)]
pub struct audit_buffer {
	_private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum evm_ima_xattr_type {
	IMA_XATTR_DIGEST = 0x01,
	EVM_XATTR_HMAC = 0x02,
	EVM_IMA_XATTR_DIGSIG = 0x03,
	IMA_XATTR_DIGEST_NG = 0x04,
	EVM_XATTR_PORTABLE_DIGSIG = 0x05,
	IMA_VERITY_DIGSIG = 0x06,
	IMA_XATTR_LAST = 0x07,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct evm_ima_xattr_data_hdr {
	pub type_: u8,
}

#[repr(C, packed)]
pub struct evm_ima_xattr_data {
	/* New members must be added within the __struct_group() macro below. */
	pub hdr: evm_ima_xattr_data_hdr,
	pub data: [u8; 0],
}

// C static_assert: offset_of!(evm_ima_xattr_data, data) == sizeof(evm_ima_xattr_data_hdr)

/* Only used in the EVM HMAC code. */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct evm_xattr {
	pub data: evm_ima_xattr_data_hdr,
	pub digest: [u8; SHA1_DIGEST_SIZE],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ima_digest_data_hdr {
	pub algo: u8,
	pub length: u8,
	pub xattr: ima_digest_data_hdr_xattr,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union ima_digest_data_hdr_xattr {
	pub sha1: ima_digest_data_hdr_xattr_sha1,
	pub ng: ima_digest_data_hdr_xattr_ng,
	pub data: [u8; 2],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ima_digest_data_hdr_xattr_sha1 {
	pub unused: u8,
	pub type_: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ima_digest_data_hdr_xattr_ng {
	pub type_: u8,
	pub algo: u8,
}

#[repr(C, packed)]
pub struct ima_digest_data {
	/* New members must be added within the __struct_group() macro below. */
	pub hdr: ima_digest_data_hdr,
	pub digest: [u8; 0],
}

// C static_assert: offset_of!(ima_digest_data, digest) == sizeof(ima_digest_data_hdr)

/*
 * Instead of wrapping the ima_digest_data struct inside a local structure
 * with the maximum hash size, define ima_max_digest_data struct.
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ima_max_digest_data {
	pub hdr: ima_digest_data_hdr,
	pub digest: [u8; HASH_MAX_DIGESTSIZE],
}

/*
 * signature header format v2 - for using with asymmetric keys
 *
 * The signature_v2_hdr struct includes a signature format version
 * to simplify defining new signature formats.
 *
 * signature format:
 * version 2: regular file data hash based signature
 * version 3: struct ima_file_id data based signature
 */
#[repr(C, packed)]
pub struct signature_v2_hdr {
	pub type_: uint8_t,		/* xattr type */
	pub version: uint8_t,	/* signature format version */
	pub hash_algo: uint8_t,	/* Digest algorithm [enum hash_algo] */
	pub keyid: __be32,		/* IMA key identifier - not X509/PGP specific */
	pub sig_size: __be16,	/* signature size */
	pub sig: [uint8_t; 0],		/* signature payload */
}

/*
 * IMA signature version 3 disambiguates the data that is signed, by
 * indirectly signing the hash of the ima_file_id structure data,
 * containing either the fsverity_descriptor struct digest or, in the
 * future, the regular IMA file hash.
 *
 * (The hash of the ima_file_id structure is only of the portion used.)
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ima_file_id {
	pub hash_type: __u8,		/* xattr type [enum evm_ima_xattr_type] */
	pub hash_algorithm: __u8,	/* Digest algorithm [enum hash_algo] */
	pub hash: [__u8; HASH_MAX_DIGESTSIZE],
}

pub const INTEGRITY_KEYRING_EVM: c_uint = 0;
pub const INTEGRITY_KEYRING_IMA: c_uint = 1;
pub const INTEGRITY_KEYRING_PLATFORM: c_uint = 2;
pub const INTEGRITY_KEYRING_MACHINE: c_uint = 3;
pub const INTEGRITY_KEYRING_MAX: c_uint = 4;

unsafe extern "C" {
	pub fn integrity_kernel_read(
		file: *mut file,
		offset: loff_t,
		addr: *mut c_void,
		count: c_ulong,
	) -> c_int;
	pub fn integrity_fs_init() -> c_int;
	pub fn integrity_fs_fini();

	pub static mut integrity_dir: *mut dentry;
}

// CONFIG_INTEGRITY_SIGNATURE selects external declarations; otherwise the C header used inline fallbacks.
#[cfg(CONFIG_INTEGRITY_SIGNATURE)]
unsafe extern "C" {
	pub fn integrity_digsig_verify(
		id: c_uint,
		sig: *const c_char,
		siglen: c_int,
		digest: *const c_char,
		digestlen: c_int,
		algo: u8,
	) -> c_int;
	pub fn integrity_modsig_verify(id: c_uint, modsig: *const modsig) -> c_int;
	pub fn integrity_init_keyring(id: c_uint) -> c_int;
	pub fn integrity_load_x509(id: c_uint, path: *const c_char) -> c_int;
	pub fn integrity_load_cert(
		id: c_uint,
		source: *const c_char,
		data: *const c_void,
		len: size_t,
		perm: key_perm_t,
	) -> c_int;
}

#[cfg(not(CONFIG_INTEGRITY_SIGNATURE))]
pub unsafe fn integrity_digsig_verify(
	_id: c_uint,
	_sig: *const c_char,
	_siglen: c_int,
	_digest: *const c_char,
	_digestlen: c_int,
	_algo: u8,
) -> c_int {
	-EOPNOTSUPP
}

#[cfg(not(CONFIG_INTEGRITY_SIGNATURE))]
pub unsafe fn integrity_modsig_verify(_id: c_uint, _modsig: *const modsig) -> c_int {
	-EOPNOTSUPP
}

#[cfg(not(CONFIG_INTEGRITY_SIGNATURE))]
pub unsafe fn integrity_init_keyring(_id: c_uint) -> c_int {
	0
}

#[cfg(not(CONFIG_INTEGRITY_SIGNATURE))]
pub unsafe fn integrity_load_cert(
	_id: c_uint,
	_source: *const c_char,
	_data: *const c_void,
	_len: size_t,
	_perm: key_perm_t,
) -> c_int {
	0
}

// CONFIG_INTEGRITY_ASYMMETRIC_KEYS selects external declarations; otherwise inline fallbacks return -EOPNOTSUPP.
#[cfg(CONFIG_INTEGRITY_ASYMMETRIC_KEYS)]
unsafe extern "C" {
	pub fn asymmetric_verify(
		keyring: *mut key,
		sig: *const c_char,
		siglen: c_int,
		data: *const c_char,
		datalen: c_int,
	) -> c_int;
	pub fn asymmetric_verify_v3(
		keyring: *mut key,
		sig: *const c_char,
		siglen: c_int,
		data: *const c_char,
		datalen: c_int,
		algo: u8,
	) -> c_int;
}

#[cfg(not(CONFIG_INTEGRITY_ASYMMETRIC_KEYS))]
pub unsafe fn asymmetric_verify(
	_keyring: *mut key,
	_sig: *const c_char,
	_siglen: c_int,
	_data: *const c_char,
	_datalen: c_int,
) -> c_int {
	-EOPNOTSUPP
}

#[cfg(not(CONFIG_INTEGRITY_ASYMMETRIC_KEYS))]
pub unsafe fn asymmetric_verify_v3(
	_keyring: *mut key,
	_sig: *const c_char,
	_siglen: c_int,
	_data: *const c_char,
	_datalen: c_int,
	_algo: u8,
) -> c_int {
	-EOPNOTSUPP
}

// CONFIG_IMA_APPRAISE_MODSIG selects an external declaration; otherwise inline fallback returns -EOPNOTSUPP.
#[cfg(CONFIG_IMA_APPRAISE_MODSIG)]
unsafe extern "C" {
	pub fn ima_modsig_verify(keyring: *mut key, modsig: *const modsig) -> c_int;
}

#[cfg(not(CONFIG_IMA_APPRAISE_MODSIG))]
pub unsafe fn ima_modsig_verify(_keyring: *mut key, _modsig: *const modsig) -> c_int {
	-EOPNOTSUPP
}

// CONFIG_IMA_LOAD_X509 selects an external declaration; otherwise inline fallback does nothing.
#[cfg(CONFIG_IMA_LOAD_X509)]
unsafe extern "C" {
	pub fn ima_load_x509();
}

#[cfg(not(CONFIG_IMA_LOAD_X509))]
pub unsafe fn ima_load_x509() {}

// CONFIG_EVM_LOAD_X509 selects an external declaration; otherwise inline fallback does nothing.
#[cfg(CONFIG_EVM_LOAD_X509)]
unsafe extern "C" {
	pub fn evm_load_x509();
}

#[cfg(not(CONFIG_EVM_LOAD_X509))]
pub unsafe fn evm_load_x509() {}

// CONFIG_INTEGRITY_AUDIT selects external audit declarations and an audit_log_start wrapper.
#[cfg(CONFIG_INTEGRITY_AUDIT)]
unsafe extern "C" {
	pub fn integrity_audit_msg(
		audit_msgno: c_int,
		inode: *mut inode,
		fname: *const u8,
		op: *const c_char,
		cause: *const c_char,
		result: c_int,
		info: c_int,
	);

	pub fn integrity_audit_message(
		audit_msgno: c_int,
		inode: *mut inode,
		fname: *const u8,
		op: *const c_char,
		cause: *const c_char,
		result: c_int,
		info: c_int,
		errno: c_int,
	);

	pub fn audit_log_start(ctx: *mut audit_context, gfp_mask: gfp_t, type_: c_int) -> *mut audit_buffer;
}

#[cfg(CONFIG_INTEGRITY_AUDIT)]
pub unsafe fn integrity_audit_log_start(
	ctx: *mut audit_context,
	gfp_mask: gfp_t,
	type_: c_int,
) -> *mut audit_buffer {
	unsafe { audit_log_start(ctx, gfp_mask, type_) }
}

#[cfg(not(CONFIG_INTEGRITY_AUDIT))]
pub unsafe fn integrity_audit_msg(
	_audit_msgno: c_int,
	_inode: *mut inode,
	_fname: *const u8,
	_op: *const c_char,
	_cause: *const c_char,
	_result: c_int,
	_info: c_int,
) {
}

#[cfg(not(CONFIG_INTEGRITY_AUDIT))]
pub unsafe fn integrity_audit_message(
	_audit_msgno: c_int,
	_inode: *mut inode,
	_fname: *const u8,
	_op: *const c_char,
	_cause: *const c_char,
	_result: c_int,
	_info: c_int,
	_errno: c_int,
) {
}

#[cfg(not(CONFIG_INTEGRITY_AUDIT))]
pub unsafe fn integrity_audit_log_start(
	_ctx: *mut audit_context,
	_gfp_mask: gfp_t,
	_type: c_int,
) -> *mut audit_buffer {
	core::ptr::null_mut()
}

// CONFIG_INTEGRITY_PLATFORM_KEYRING selects an external declaration; otherwise inline fallback does nothing.
#[cfg(CONFIG_INTEGRITY_PLATFORM_KEYRING)]
unsafe extern "C" {
	pub fn add_to_platform_keyring(source: *const c_char, data: *const c_void, len: size_t);
}

#[cfg(not(CONFIG_INTEGRITY_PLATFORM_KEYRING))]
pub unsafe fn add_to_platform_keyring(_source: *const c_char, _data: *const c_void, _len: size_t) {
}

// CONFIG_INTEGRITY_MACHINE_KEYRING selects external declarations; otherwise inline fallbacks do nothing/return false.
#[cfg(CONFIG_INTEGRITY_MACHINE_KEYRING)]
unsafe extern "C" {
	pub fn add_to_machine_keyring(source: *const c_char, data: *const c_void, len: size_t);
	pub fn imputed_trust_enabled() -> bool;
}

#[cfg(not(CONFIG_INTEGRITY_MACHINE_KEYRING))]
pub unsafe fn add_to_machine_keyring(_source: *const c_char, _data: *const c_void, _len: size_t) {
}

#[cfg(not(CONFIG_INTEGRITY_MACHINE_KEYRING))]
pub unsafe fn imputed_trust_enabled() -> bool {
	false
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
