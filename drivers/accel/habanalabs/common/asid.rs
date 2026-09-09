// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2016-2019 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

use core::ffi::c_void;

// Supplied by the surrounding HabanaLabs and Linux compatibility layers.
extern "C" {
    fn bitmap_zalloc(nbits: usize, flags: u32) -> *mut usize;
    fn bitmap_free(bitmap: *mut usize);
    fn mutex_init(lock: *mut hl_mutex);
    fn mutex_destroy(lock: *mut hl_mutex);
    fn mutex_lock(lock: *mut hl_mutex);
    fn mutex_unlock(lock: *mut hl_mutex);
    fn find_first_zero_bit(addr: *const usize, size: usize) -> usize;
    fn set_bit(nr: usize, addr: *mut usize);
    fn clear_bit(nr: usize, addr: *mut usize);
    fn dev_crit(dev: *mut c_void, fmt: *const u8, ...);
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const HL_KERNEL_ASID_ID: usize = 0;

#[repr(C)]
pub struct hl_mutex {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hl_asic_prop {
    pub max_asid: usize,
}

#[repr(C)]
pub struct hl_device {
    pub asid_bitmap: *mut usize,
    pub asic_prop: hl_asic_prop,
    pub asid_mutex: hl_mutex,
    pub dev: *mut c_void,
}

pub unsafe fn hl_asid_init(hdev: *mut hl_device) -> i32 {
    (*hdev).asid_bitmap = bitmap_zalloc((*hdev).asic_prop.max_asid, GFP_KERNEL);
    if (*hdev).asid_bitmap.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*hdev).asid_mutex);

    /* ASID 0 is reserved for the kernel driver and device CPU */
    set_bit(0, (*hdev).asid_bitmap);

    0
}

pub unsafe fn hl_asid_fini(hdev: *mut hl_device) {
    mutex_destroy(&mut (*hdev).asid_mutex);
    bitmap_free((*hdev).asid_bitmap);
}

pub unsafe fn hl_asid_alloc(hdev: *mut hl_device) -> usize {
    let mut found: usize;

    mutex_lock(&mut (*hdev).asid_mutex);

    found = find_first_zero_bit(
        (*hdev).asid_bitmap,
        (*hdev).asic_prop.max_asid,
    );
    if found == (*hdev).asic_prop.max_asid {
        found = 0;
    } else {
        set_bit(found, (*hdev).asid_bitmap);
    }

    mutex_unlock(&mut (*hdev).asid_mutex);

    found
}

pub unsafe fn hl_asid_free(hdev: *mut hl_device, asid: usize) {
    if asid == HL_KERNEL_ASID_ID || asid >= (*hdev).asic_prop.max_asid {
        dev_crit((*hdev).dev, b"Invalid ASID %lu\0".as_ptr(), asid);
        return;
    }

    clear_bit(asid, (*hdev).asid_bitmap);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
