/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// Dependency declarations supplied by the surrounding implementation.
use core::ffi::c_void;

extern "C" {
    fn dc_get_vmid_use_vector(dc: *mut dc) -> i32;
    fn dc_setup_vm_context(dc: *mut dc, config: *const dc_virtual_addr_space_config, vmid: i32);
    fn kzalloc_obj() -> *mut core_vmid;
    fn kfree(ptr: *mut core_vmid);
    fn memset(dest: *mut c_void, value: i32, size: usize) -> *mut c_void;
    fn __assert_fail(expr: *const u8, file: *const u8, line: u32, function: *const u8) -> !;
}

const MAX_VMID: usize = 16;

#[repr(C)]
pub struct mod_vmid {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_virtual_addr_space_config {
    pub page_table_base_addr: u64,
    _private: [u8; 0],
}

#[repr(C)]
struct core_vmid {
    public: mod_vmid,
    dc: *mut dc,
    num_vmid: u32,
    num_vmids_available: u32,
    ptb_assigned_to_vmid: [u64; MAX_VMID],
    base_config: dc_virtual_addr_space_config,
}

unsafe fn add_ptb_to_table(core_vmid: *mut core_vmid, vmid: u32, ptb: u64) {
    if (vmid as usize) < MAX_VMID {
        (*core_vmid).ptb_assigned_to_vmid[vmid as usize] = ptb;
        (*core_vmid).num_vmids_available -= 1;
    }
}

unsafe fn clear_entry_from_vmid_table(core_vmid: *mut core_vmid, vmid: u32) {
    if (vmid as usize) < MAX_VMID {
        (*core_vmid).ptb_assigned_to_vmid[vmid as usize] = 0;
        (*core_vmid).num_vmids_available += 1;
    }
}

unsafe fn evict_vmids(core_vmid: *mut core_vmid) {
    let ord_int = dc_get_vmid_use_vector((*core_vmid).dc);
    if !(ord_int >= 0 && ord_int <= 0xffff) {
        __assert_fail(b"ord_int >= 0 && ord_int <= 0xFFFF\0".as_ptr(), b"vmid.c\0".as_ptr(), 0, b"evict_vmids\0".as_ptr());
    }
    let ord = ord_int as u16;

    // At this point any positions with value 0 are unused vmids, evict them
    let mut i = 1;
    while i < (*core_vmid).num_vmid {
        if (ord & (1u16 << i)) == 0 {
            clear_entry_from_vmid_table(core_vmid, i);
        }
        i += 1;
    }
}

// Return value of -1 indicates vmid table uninitialized or ptb dne in the table
unsafe fn get_existing_vmid_for_ptb(core_vmid: *mut core_vmid, ptb: u64) -> i32 {
    let mut i = 0;
    while i < (*core_vmid).num_vmid {
        if (*core_vmid).ptb_assigned_to_vmid[i as usize] == ptb {
            return i as i32;
        }
        i += 1;
    }
    -1
}

// Expected to be called only when there's an available vmid
unsafe fn get_next_available_vmid(core_vmid: *mut core_vmid) -> i32 {
    let mut i = 1;
    while i < (*core_vmid).num_vmid {
        if (*core_vmid).ptb_assigned_to_vmid[i as usize] == 0 {
            return i as i32;
        }
        i += 1;
    }
    -1
}

pub unsafe fn mod_vmid_get_for_ptb(mod_vmid: *mut mod_vmid, ptb: u64) -> u8 {
    let core_vmid = mod_vmid as *mut core_vmid;
    let mut vmid: i32 = 0;

    // Physical address gets vmid 0
    if ptb == 0 {
        return 0;
    }

    vmid = get_existing_vmid_for_ptb(core_vmid, ptb);

    if vmid == -1 {
        let mut va_config = (*core_vmid).base_config;
        va_config.page_table_base_addr = ptb;

        if (*core_vmid).num_vmids_available == 0 {
            evict_vmids(core_vmid);
        }

        vmid = get_next_available_vmid(core_vmid);
        if vmid != -1 {
            add_ptb_to_table(core_vmid, vmid as u32, ptb);
            dc_setup_vm_context((*core_vmid).dc, &va_config, vmid);
        } else {
            __assert_fail(b"0\0".as_ptr(), b"vmid.c\0".as_ptr(), 0, b"mod_vmid_get_for_ptb\0".as_ptr());
        }
    }

    if !(vmid >= 0 && vmid <= 0xff) {
        __assert_fail(b"vmid >= 0 && vmid <= 0xFF\0".as_ptr(), b"vmid.c\0".as_ptr(), 0, b"mod_vmid_get_for_ptb\0".as_ptr());
    }
    vmid as u8
}

pub unsafe fn mod_vmid_reset(mod_vmid: *mut mod_vmid) {
    let core_vmid = mod_vmid as *mut core_vmid;
    (*core_vmid).num_vmids_available = (*core_vmid).num_vmid - 1;
    memset((*core_vmid).ptb_assigned_to_vmid.as_mut_ptr() as *mut c_void, 0, core::mem::size_of::<u64>() * MAX_VMID);
}

pub unsafe fn mod_vmid_create(
    dc: *mut dc,
    num_vmid: u32,
    va_config: *mut dc_virtual_addr_space_config,
) -> *mut mod_vmid {
    if num_vmid <= 1 {
        return core::ptr::null_mut();
    }
    if dc.is_null() {
        return core::ptr::null_mut();
    }

    let core_vmid = kzalloc_obj();
    if core_vmid.is_null() {
        return core::ptr::null_mut();
    }

    (*core_vmid).dc = dc;
    (*core_vmid).num_vmid = num_vmid;
    (*core_vmid).num_vmids_available = num_vmid - 1;
    (*core_vmid).base_config = *va_config;
    memset((*core_vmid).ptb_assigned_to_vmid.as_mut_ptr() as *mut c_void, 0, core::mem::size_of::<u64>() * MAX_VMID);

    &mut (*core_vmid).public
}

pub unsafe fn mod_vmid_destroy(mod_vmid: *mut mod_vmid) {
    if !mod_vmid.is_null() {
        kfree(mod_vmid as *mut core_vmid);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
