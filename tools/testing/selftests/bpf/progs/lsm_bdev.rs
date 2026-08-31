// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Christian Brauner <brauner@kernel.org> */

/*
 * BPF LSM block device integrity tracker for dm-verity.
 *
 * Tracks block devices in a hashmap keyed by bd_dev.  When dm-verity
 * calls security_bdev_setintegrity() during verity_preresume(), the
 * setintegrity hook records the roothash and signature-validity data.
 * The free hook cleans up when the device goes away.  The alloc hook
 * counts allocations for test validation.
 *
 * The sleepable hooks exercise bpf_copy_from_user() to verify that
 * the sleepable classification actually permits sleepable helpers.
 */

/* Dependencies from C includes:
 *   "vmlinux.h"
 *   <bpf/bpf_helpers.h>
 *   <bpf/bpf_tracing.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub type __u8 = u8;
pub type __u32 = u32;
pub type size_t = usize;

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_NOEXIST: u64 = 1;
pub const LSM_INT_DMVERITY_ROOTHASH: lsm_integrity_type = 0;
pub const LSM_INT_DMVERITY_SIG_VALID: lsm_integrity_type = 1;

pub type lsm_integrity_type = u32;

#[repr(C)]
pub struct block_device {
    pub bd_dev: __u32,
}

#[repr(C)]
pub struct verity_info {
    pub has_roothash: __u8,       /* LSM_INT_DMVERITY_ROOTHASH seen */
    pub sig_valid: __u8,          /* LSM_INT_DMVERITY_SIG_VALID value (non-NULL = valid) */
    pub setintegrity_cnt: __u32,  /* total setintegrity calls for this dev */
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static verity_devices: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 64,
    key_size: core::mem::size_of::<__u32>() as u32, /* dev_t from bdev->bd_dev */
    value_size: core::mem::size_of::<verity_info>() as u32,
};

/* Global counters exposed to userspace via skeleton bss. */
#[no_mangle]
pub static mut alloc_count: i32 = 0;

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, size: size_t, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_map_lookup_elem(map: *const bpf_map_def, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *const bpf_map_def,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_delete_elem(map: *const bpf_map_def, key: *const core::ffi::c_void) -> i64;
}

#[link_section = "lsm.s/bdev_setintegrity"]
#[no_mangle]
pub unsafe extern "C" fn bdev_setintegrity(
    bdev: *mut block_device,
    type_: lsm_integrity_type,
    value: *const core::ffi::c_void,
    size: size_t,
) -> i32 {
    let zero: verity_info = verity_info {
        has_roothash: 0,
        sig_valid: 0,
        setintegrity_cnt: 0,
    };
    let mut info: *mut verity_info;
    let dev: __u32;
    let mut buf: i8 = 0;

    /*
     * Exercise a sleepable helper to confirm the verifier
     * allows it in this sleepable hook.
     */
    let _ = bpf_copy_from_user(
        &mut buf as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&buf),
        core::ptr::null(),
    );

    dev = (*bdev).bd_dev;

    info = bpf_map_lookup_elem(
        &verity_devices as *const _,
        &dev as *const _ as *const core::ffi::c_void,
    ) as *mut verity_info;
    if info.is_null() {
        bpf_map_update_elem(
            &verity_devices as *const _,
            &dev as *const _ as *const core::ffi::c_void,
            &zero as *const _ as *const core::ffi::c_void,
            BPF_NOEXIST,
        );
        info = bpf_map_lookup_elem(
            &verity_devices as *const _,
            &dev as *const _ as *const core::ffi::c_void,
        ) as *mut verity_info;
        if info.is_null() {
            return 0;
        }
    }

    if type_ == LSM_INT_DMVERITY_ROOTHASH {
        (*info).has_roothash = 1;
    } else if type_ == LSM_INT_DMVERITY_SIG_VALID {
        (*info).sig_valid = (value != core::ptr::null()) as __u8;
    }

    core::sync::atomic::AtomicU32::from_ptr(&mut (*info).setintegrity_cnt)
        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);

    let _ = size;
    0
}

#[link_section = "lsm/bdev_free_security"]
#[no_mangle]
pub unsafe extern "C" fn bdev_free_security(bdev: *mut block_device) {
    let dev: __u32 = (*bdev).bd_dev;

    bpf_map_delete_elem(
        &verity_devices as *const _,
        &dev as *const _ as *const core::ffi::c_void,
    );
}

#[link_section = "lsm.s/bdev_alloc_security"]
#[no_mangle]
pub unsafe extern "C" fn bdev_alloc_security(bdev: *mut block_device) -> i32 {
    let mut buf: i8 = 0;

    /*
     * Exercise a sleepable helper to confirm the verifier
     * allows it in this sleepable hook.
     */
    let _ = bpf_copy_from_user(
        &mut buf as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&buf),
        core::ptr::null(),
    );

    core::sync::atomic::AtomicI32::from_ptr(&mut alloc_count)
        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);

    let _ = bdev;
    0
}
