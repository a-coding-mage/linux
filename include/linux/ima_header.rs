/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008 IBM Corporation
 * Author: Mimi Zohar <zohar@us.ibm.com>
 */

// C dependencies supplied by the surrounding kernel translation.

pub struct linux_binprm;

#[cfg(feature = "CONFIG_IMA")]
extern "C" {
    pub fn ima_get_current_hash_algo() -> hash_algo;
    pub fn ima_file_hash(file: *mut file, buf: *mut core::ffi::c_char, buf_size: usize) -> i32;
    pub fn ima_inode_hash(inode: *mut inode, buf: *mut core::ffi::c_char, buf_size: usize) -> i32;
    pub fn ima_kexec_cmdline(kernel_fd: i32, buf: *const core::ffi::c_void, size: i32);
    pub fn ima_measure_critical_data(
        event_label: *const core::ffi::c_char,
        event_name: *const core::ffi::c_char,
        buf: *const core::ffi::c_void,
        buf_len: usize,
        hash: bool,
        digest: *mut u8,
        digest_len: usize,
    ) -> i32;
}

#[cfg(all(feature = "CONFIG_IMA", feature = "CONFIG_IMA_APPRAISE_BOOTPARAM"))]
extern "C" {
    pub fn ima_appraise_parse_cmdline();
}

#[cfg(any(not(feature = "CONFIG_IMA"), all(feature = "CONFIG_IMA", not(feature = "CONFIG_IMA_APPRAISE_BOOTPARAM"))))]
#[inline]
pub fn ima_appraise_parse_cmdline() {}

#[cfg(all(feature = "CONFIG_IMA", feature = "CONFIG_IMA_KEXEC"))]
extern "C" {
    pub fn ima_add_kexec_buffer(image: *mut kimage);
    pub fn ima_kexec_post_load(image: *mut kimage);
}

#[cfg(any(not(feature = "CONFIG_IMA"), all(feature = "CONFIG_IMA", not(feature = "CONFIG_IMA_KEXEC"))))]
#[inline]
pub fn ima_kexec_post_load(_image: *mut kimage) {}

#[cfg(not(feature = "CONFIG_IMA"))]
#[inline]
pub fn ima_get_current_hash_algo() -> hash_algo { HASH_ALGO__LAST }

#[cfg(not(feature = "CONFIG_IMA"))]
#[inline]
pub fn ima_file_hash(_file: *mut file, _buf: *mut core::ffi::c_char, _buf_size: usize) -> i32 { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_IMA"))]
#[inline]
pub fn ima_inode_hash(_inode: *mut inode, _buf: *mut core::ffi::c_char, _buf_size: usize) -> i32 { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_IMA"))]
#[inline]
pub fn ima_kexec_cmdline(_kernel_fd: i32, _buf: *const core::ffi::c_void, _size: i32) {}

#[cfg(not(feature = "CONFIG_IMA"))]
#[inline]
pub fn ima_measure_critical_data(
    _event_label: *const core::ffi::c_char,
    _event_name: *const core::ffi::c_char,
    _buf: *const core::ffi::c_void,
    _buf_len: usize,
    _hash: bool,
    _digest: *mut u8,
    _digest_len: usize,
) -> i32 { -ENOENT }

#[cfg(feature = "CONFIG_HAVE_IMA_KEXEC")]
extern "C" {
    pub fn ima_free_kexec_buffer() -> i32;
    pub fn ima_get_kexec_buffer(addr: *mut *mut core::ffi::c_void, size: *mut usize) -> i32;
    pub fn ima_validate_range(phys: phys_addr_t, size: usize) -> i32;
}

#[cfg(feature = "CONFIG_IMA_SECURE_AND_OR_TRUSTED_BOOT")]
extern "C" {
    pub fn arch_get_ima_policy() -> *const *const core::ffi::c_char;
}

#[cfg(not(feature = "CONFIG_IMA_SECURE_AND_OR_TRUSTED_BOOT"))]
#[inline]
pub fn arch_get_ima_policy() -> *const *const core::ffi::c_char { core::ptr::null() }

#[cfg(not(feature = "CONFIG_IMA_KEXEC"))]
#[inline]
pub fn ima_add_kexec_buffer(_image: *mut kimage) {}

#[cfg(feature = "CONFIG_IMA_APPRAISE")]
extern "C" {
    pub fn is_ima_appraise_enabled() -> bool;
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn is_ima_appraise_enabled() -> bool { false }

#[cfg(all(feature = "CONFIG_IMA_APPRAISE", feature = "CONFIG_INTEGRITY_TRUSTED_KEYRING"))]
extern "C" {
    pub fn ima_appraise_signature(func: kernel_read_file_id) -> bool;
}

#[cfg(any(not(feature = "CONFIG_IMA_APPRAISE"), not(feature = "CONFIG_INTEGRITY_TRUSTED_KEYRING")))]
#[inline]
pub fn ima_appraise_signature(_func: kernel_read_file_id) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
