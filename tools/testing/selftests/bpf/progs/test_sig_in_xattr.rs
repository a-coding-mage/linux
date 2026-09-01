// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies: vmlinux.h, errno.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h,
// bpf_kfuncs.h, err.h.

type __u32 = u32;
type __s32 = i32;

const SHA256_DIGEST_SIZE: usize = 32;
const MAX_SIG_SIZE: usize = 1024;

const EPERM: i32 = 1;
const ENOENT: i32 = 2;
const EFAULT: i32 = 14;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _data: [u64; 2],
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* By default, "fsverity sign" signs a file with fsverity_formatted_digest
 * of the file. fsverity_formatted_digest on the kernel side is only used
 * with CONFIG_FS_VERITY_BUILTIN_SIGNATURES. However, BPF LSM doesn't not
 * require CONFIG_FS_VERITY_BUILTIN_SIGNATURES, so vmlinux.h may not have
 * fsverity_formatted_digest. In this test, we intentionally avoid using
 * fsverity_formatted_digest.
 *
 * Luckily, fsverity_formatted_digest is simply 8-byte magic followed by
 * fsverity_digest. We use a char array of size fsverity_formatted_digest
 * plus SHA256_DIGEST_SIZE. The magic part of it is filled by user space,
 * and the rest of it is filled by bpf_get_fsverity_digest.
 *
 * Note that, generating signatures based on fsverity_formatted_digest is
 * the design choice of this selftest (and "fsverity sign"). With BPF
 * LSM, we have the flexibility to generate signature based on other data
 * sets, for example, fsverity_digest or only the digest[] part of it.
 */
const MAGIC_SIZE: usize = 8;
const SIZEOF_STRUCT_FSVERITY_DIGEST: usize = 4; /* sizeof(struct fsverity_digest) */
#[no_mangle]
pub static mut digest: [u8; MAGIC_SIZE + SIZEOF_STRUCT_FSVERITY_DIGEST + SHA256_DIGEST_SIZE] =
    [0; MAGIC_SIZE + SIZEOF_STRUCT_FSVERITY_DIGEST + SHA256_DIGEST_SIZE];

#[no_mangle]
pub static mut monitored_pid: __u32 = 0;
#[no_mangle]
pub static mut sig: [u8; MAX_SIG_SIZE] = [0; MAX_SIG_SIZE];
#[no_mangle]
pub static mut sig_size: __u32 = 0;
#[no_mangle]
pub static mut user_keyring_serial: __s32 = 0;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: __u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_get_fsverity_digest(f: *mut file, digest_ptr: *mut bpf_dynptr) -> i32;
    fn bpf_get_file_xattr(f: *mut file, name: *const u8, value_ptr: *mut bpf_dynptr) -> i32;
    fn bpf_lookup_user_key(serial: __s32, flags: u64) -> *mut bpf_key;
    fn bpf_verify_pkcs7_signature(
        data_ptr: *mut bpf_dynptr,
        sig_ptr: *mut bpf_dynptr,
        trusted_keyring: *mut bpf_key,
    ) -> i32;
    fn bpf_key_put(key: *mut bpf_key);
}

#[inline]
unsafe fn set_if_not_errno_or_zero(ret: &mut i32, value: i32) {
    if *ret > 0 {
        *ret = value;
    }
}

#[link_section = "lsm.s/file_open"]
#[no_mangle]
pub unsafe extern "C" fn test_file_open(f: *mut file) -> i32 {
    let mut digest_ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut sig_ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let trusted_keyring: *mut bpf_key;
    let pid: __u32;
    let mut ret: i32;

    pid = (bpf_get_current_pid_tgid() >> 32) as __u32;
    if pid != monitored_pid {
        return 0;
    }

    /* digest_ptr points to fsverity_digest */
    bpf_dynptr_from_mem(
        digest.as_mut_ptr().add(MAGIC_SIZE) as *mut core::ffi::c_void,
        (core::mem::size_of_val(&digest) - MAGIC_SIZE) as __u32,
        0,
        digest_ptr.as_mut_ptr(),
    );

    ret = bpf_get_fsverity_digest(f, digest_ptr.as_mut_ptr());
    /* No verity, allow access */
    if ret < 0 {
        return 0;
    }

    /* Move digest_ptr to fsverity_formatted_digest */
    bpf_dynptr_from_mem(
        digest.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&digest) as __u32,
        0,
        digest_ptr.as_mut_ptr(),
    );

    /* Read signature from xattr */
    bpf_dynptr_from_mem(
        sig.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&sig) as __u32,
        0,
        sig_ptr.as_mut_ptr(),
    );
    ret = bpf_get_file_xattr(f, b"user.sig\0".as_ptr(), sig_ptr.as_mut_ptr());
    /* No signature, reject access */
    if ret < 0 {
        return -EPERM;
    }

    trusted_keyring = bpf_lookup_user_key(user_keyring_serial, 0);
    if trusted_keyring.is_null() {
        return -ENOENT;
    }

    /* Verify signature */
    ret = bpf_verify_pkcs7_signature(
        digest_ptr.as_mut_ptr(),
        sig_ptr.as_mut_ptr(),
        trusted_keyring,
    );

    bpf_key_put(trusted_keyring);

    set_if_not_errno_or_zero(&mut ret, -EFAULT);

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
