// SPDX-License-Identifier: GPL-2.0
/*
 * Ioctl to get a verity file's digest
 *
 * Copyright 2019 Google LLC
 */

// Declarations supplied by fsverity_private.h and the Linux headers are
// intentionally referenced here as external dependencies.

use core::ffi::c_void;

type U8 = u8;
type U32 = u32;
type U64 = u64;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_dynptr_kern {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_prog {
    pub r#type: u32,
}
#[repr(C)]
pub struct fsverity_info {
    pub tree_params: fsverity_tree_params,
    pub file_digest: [u8; FS_VERITY_MAX_DIGEST_SIZE],
}
#[repr(C)]
pub struct fsverity_tree_params {
    pub hash_alg: *const fsverity_hash_alg,
}
#[repr(C)]
pub struct fsverity_hash_alg {
    pub digest_size: usize,
    pub algo_id: hash_algo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_digest {
    pub digest_algorithm: U8,
    pub digest_size: U8,
    pub digest: [U8; FS_VERITY_MAX_DIGEST_SIZE],
}
#[repr(C)]
pub struct btf_kfunc_id_set {
    pub owner: *mut c_void,
    pub set: *const c_void,
    pub filter: Option<unsafe extern "C" fn(*const bpf_prog, U32) -> i32>,
}
#[repr(C)]
pub struct btf_id_set8 {
    _private: [u8; 0],
}

pub type hash_algo = i32;

const FS_VERITY_MAX_DIGEST_SIZE: usize = 64;
const BPF_PROG_TYPE_LSM: u32 = 29;
const ENODATA: i32 = 61;
const EFAULT: i32 = 14;
const EOVERFLOW: i32 = 75;
const EINVAL: i32 = 22;
const EACCES: i32 = 13;
const THIS_MODULE: *mut c_void = core::ptr::null_mut();

extern "C" {
    static fsverity_hash_algs: fsverity_hash_alg;
    static fsverity_set_ids: btf_id_set8;
    fn file_inode(filp: *mut file) -> *const inode;
    fn fsverity_get_info(inode: *const inode) -> *const fsverity_info;
    fn get_user(dst: *mut U8, src: *const U8) -> i32;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, size: usize) -> i32;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: i32, size: usize) -> *mut c_void;
    fn __bpf_dynptr_size(ptr: *const bpf_dynptr_kern) -> U64;
    fn __bpf_dynptr_data_rw(ptr: *const bpf_dynptr_kern, size: U64) -> *mut fsverity_digest;
    fn btf_id_set8_contains(set: *const btf_id_set8, id: U32) -> bool;
    fn register_btf_kfunc_id_set(prog_type: u32, set: *const btf_kfunc_id_set) -> i32;
}

/// fsverity_ioctl_measure() - get a verity file's digest
/// @filp: file to get digest of
/// @_uarg: user pointer to fsverity_digest
pub unsafe extern "C" fn fsverity_ioctl_measure(filp: *mut file, _uarg: *mut c_void) -> i32 {
    let inode = file_inode(filp);
    let uarg = _uarg as *mut fsverity_digest;
    let vi = fsverity_get_info(inode);
    if vi.is_null() {
        return -ENODATA; /* not a verity file */
    }
    let hash_alg = (*vi).tree_params.hash_alg;
    let mut arg = core::mem::MaybeUninit::<fsverity_digest>::uninit();
    let arg_ptr = arg.as_mut_ptr();

    if get_user(&mut (*arg_ptr).digest_size, &(*uarg).digest_size) != 0 {
        return -EFAULT;
    }
    if (*arg_ptr).digest_size as usize < (*hash_alg).digest_size {
        return -EOVERFLOW;
    }

    memset(arg_ptr as *mut c_void, 0, core::mem::size_of::<fsverity_digest>());
    (*arg_ptr).digest_algorithm = hash_alg.offset_from(fsverity_hash_algs as *const fsverity_hash_alg) as U8;
    (*arg_ptr).digest_size = (*hash_alg).digest_size as U8;

    if copy_to_user(uarg as *mut c_void, arg_ptr as *const c_void, core::mem::size_of::<fsverity_digest>()) != 0 {
        return -EFAULT;
    }
    if copy_to_user((*uarg).digest.as_mut_ptr() as *mut c_void,
                    (*vi).file_digest.as_ptr() as *const c_void,
                    (*hash_alg).digest_size) != 0 {
        return -EFAULT;
    }
    0
}

/// fsverity_get_digest() - get a verity file's digest
pub unsafe extern "C" fn fsverity_get_digest(
    inode: *mut inode,
    raw_digest: *mut U8,
    alg: *mut U8,
    halg: *mut hash_algo,
) -> i32 {
    let vi = fsverity_get_info(inode);
    if vi.is_null() {
        return 0; /* not a verity file */
    }
    let hash_alg = (*vi).tree_params.hash_alg;
    memcpy(raw_digest as *mut c_void, (*vi).file_digest.as_ptr() as *const c_void, (*hash_alg).digest_size);
    if !alg.is_null() {
        *alg = hash_alg.offset_from(fsverity_hash_algs as *const fsverity_hash_alg) as U8;
    }
    if !halg.is_null() {
        *halg = (*hash_alg).algo_id;
    }
    (*hash_alg).digest_size as i32
}

/// bpf_get_fsverity_digest: read fsverity digest of file
pub unsafe extern "C" fn bpf_get_fsverity_digest(file: *mut file, digest_p: *const bpf_dynptr) -> i32 {
    let digest_ptr = digest_p as *const bpf_dynptr_kern;
    let inode = file_inode(file);
    let dynptr_sz = __bpf_dynptr_size(digest_ptr);
    if dynptr_sz < core::mem::size_of::<fsverity_digest>() as U64 {
        return -EINVAL;
    }
    let arg = __bpf_dynptr_data_rw(digest_ptr, dynptr_sz);
    if arg.is_null() || (arg as usize) % core::mem::align_of::<fsverity_digest>() != 0 {
        return -EINVAL;
    }
    let vi = fsverity_get_info(inode);
    if vi.is_null() {
        return -ENODATA; /* not a verity file */
    }
    let hash_alg = (*vi).tree_params.hash_alg;
    let out_digest_sz = dynptr_sz - core::mem::size_of::<fsverity_digest>() as U64;
    if out_digest_sz < (*hash_alg).digest_size as U64 {
        return -EOVERFLOW;
    }
    (*arg).digest_algorithm = hash_alg.offset_from(fsverity_hash_algs as *const fsverity_hash_alg) as U8;
    (*arg).digest_size = (*hash_alg).digest_size as U8;
    memcpy((*arg).digest.as_mut_ptr() as *mut c_void, (*vi).file_digest.as_ptr() as *const c_void, (*hash_alg).digest_size);
    if out_digest_sz > (*hash_alg).digest_size as U64 {
        memset((*arg).digest.as_mut_ptr().add((*hash_alg).digest_size) as *mut c_void, 0,
               (out_digest_sz - (*hash_alg).digest_size as U64) as usize);
    }
    0
}

unsafe extern "C" fn bpf_get_fsverity_digest_filter(_prog: *const bpf_prog, kfunc_id: U32) -> i32 {
    if !btf_id_set8_contains(&fsverity_set_ids, kfunc_id) {
        return 0;
    }
    /* Only allow to attach from LSM hooks, to avoid recursion */
    if (*_prog).r#type != BPF_PROG_TYPE_LSM {
        -EACCES
    } else {
        0
    }
}

static BPF_FSVERITY_SET: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: THIS_MODULE,
    set: &fsverity_set_ids as *const _ as *const c_void,
    filter: Some(bpf_get_fsverity_digest_filter),
};

pub unsafe extern "C" fn fsverity_init_bpf() {
    register_btf_kfunc_id_set(BPF_PROG_TYPE_LSM, &BPF_FSVERITY_SET);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
