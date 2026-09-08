// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2020 Google LLC.
 */

// The LSM_HOOK macro expansion is provided by the kernel headers.  Its Rust
// equivalent is retained as an external dependency rather than reimplemented
// here; the final entry is inode_free_security / bpf_inode_storage_free.

#[repr(C)]
pub struct security_hook_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_id {
    pub name: *const u8,
    pub id: u32,
}

#[repr(C)]
pub struct lsm_blob_sizes {
    pub lbs_inode: usize,
}

#[repr(C)]
pub struct bpf_storage_blob {
    _private: [u8; 0],
}

extern "C" {
    pub fn security_add_hooks(
        hooks: *mut security_hook_list,
        count: usize,
        lsmid: *const lsm_id,
    );
    pub fn pr_info(fmt: *const u8, ...);
}

pub static mut bpf_lsm_initialized: bool = false;

// Generated from <linux/lsm_hook_defs.h> by LSM_HOOK/LSM_HOOK_INIT, followed
// by LSM_HOOK_INIT(inode_free_security, bpf_inode_storage_free).
extern "C" {
    static mut bpf_lsm_hooks: [security_hook_list; 0];
}

pub static bpf_lsmid: lsm_id = lsm_id {
    name: b"bpf\0".as_ptr(),
    // LSM_ID_BPF is supplied by <uapi/linux/lsm.h>.
    id: 0,
};

pub unsafe extern "C" fn bpf_lsm_init() -> i32 {
    security_add_hooks(
        bpf_lsm_hooks.as_mut_ptr(),
        bpf_lsm_hooks.len(),
        &bpf_lsmid,
    );
    bpf_lsm_initialized = true;
    pr_info(b"LSM support for eBPF active\n\0".as_ptr());
    0
}

pub static mut bpf_lsm_blob_sizes: lsm_blob_sizes = lsm_blob_sizes {
    lbs_inode: core::mem::size_of::<bpf_storage_blob>(),
};

#[repr(C)]
pub struct lsm_definition {
    pub id: *const lsm_id,
    pub init: unsafe extern "C" fn() -> i32,
    pub blobs: *const lsm_blob_sizes,
}

pub static bpf_lsm: lsm_definition = lsm_definition {
    id: &bpf_lsmid,
    init: bpf_lsm_init,
    blobs: &bpf_lsm_blob_sizes,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
