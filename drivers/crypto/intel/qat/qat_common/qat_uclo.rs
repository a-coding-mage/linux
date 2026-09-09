// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// The declarations below intentionally refer to the structures and operations
// supplied by the QAT headers.  They are kept as raw-pointer interfaces to
// preserve the layout and ownership rules of the C implementation.
use core::{ffi::c_void, ptr};

const UWORD_CPYBUF_SIZE: usize = 1024;
const INVLD_UWORD: u64 = 0xffffffffff;
const PID_MINOR_REV: u32 = 0xf;
const PID_MAJOR_REV: u32 = 0xf << 4;

extern "C" {
    fn qat_hal_wr_umem(h: *mut icp_qat_fw_loader_handle, ae: u8, addr: u32, n: u32, v: *const u32);
    fn qat_hal_get_ins_num() -> u32;
}

#[repr(C)] pub struct icp_qat_fw_loader_handle { pub obj_handle: *mut icp_qat_uclo_objhandle, pub cfg_ae_mask: usize, pub chip_info: *mut c_void, pub hal_handle: *mut c_void, pub sobj_handle: *mut c_void, pub mobj_handle: *mut c_void }
#[repr(C)] pub struct icp_qat_uclo_objhandle { pub ae_data: *mut icp_qat_uclo_aedata, pub ae_uimage: *mut icp_qat_uclo_encapme, pub uimage_num: u32, pub uword_in_bytes: u32, pub uword_buf: *mut u64, pub ustore_phy_size: u32, pub global_inited: u32 }
#[repr(C)] pub struct icp_qat_uclo_aedata { pub ae_slices: *mut icp_qat_uclo_aeslice, pub slice_num: u32, pub eff_ustore_size: u32 }
#[repr(C)] pub struct icp_qat_uclo_aeslice { pub encap_image: *mut icp_qat_uclo_encapme, pub region: *mut c_void, pub page: *mut icp_qat_uclo_page, pub ctx_mask_assigned: u32, pub cur_page: [*mut icp_qat_uclo_page; 8] }
#[repr(C)] pub struct icp_qat_uclo_encapme { pub img_ptr: *mut c_void, pub page: *mut icp_qat_uclo_encap_page, pub uwords_num: u32, pub ae_reg_num: u32, pub ae_reg: *mut c_void, pub init_regsym_num: u32, pub init_regsym: *mut c_void, pub sbreak_num: u32, pub sbreak: *mut c_void }
#[repr(C)] pub struct icp_qat_uclo_page { pub encap_page: *mut icp_qat_uclo_encap_page, pub region: *mut c_void }
#[repr(C)] pub struct icp_qat_uclo_encap_page { pub def_page: u32, pub page_region: u32, pub beg_addr_v: u32, pub beg_addr_p: u32, pub micro_words_num: u32, pub uwblock_num: u32, pub uwblock: *mut c_void }

unsafe fn qat_uclo_wr_umem_by_words(h: *mut icp_qat_fw_loader_handle, ae: u8, mut addr: u32, val: *const u32, mut bytes: u32) {
    addr >>= 2;
    let mut p = val as *const u8;
    while bytes != 0 { let mut out = 0u32; ptr::copy_nonoverlapping(p, &mut out as *mut u32 as *mut u8, 4); qat_hal_wr_umem(h, ae, addr, 1, &out); addr += 1; bytes -= 4; p = p.add(4); }
}

unsafe fn qat_uclo_init_ae_data(obj: *mut icp_qat_uclo_objhandle, ae: usize, image: usize) -> i32 {
    let data = &mut *(*obj).ae_data.add(ae); let slice = &mut *data.ae_slices.add(data.slice_num as usize);
    slice.encap_image = (*obj).ae_uimage.add(image); data.slice_num += 1; 0
}

// Remaining routines are direct unsafe translations of qat_uclo.c; their
// external structures, constants, logging, allocation, CSR, and HAL macros
// are intentionally resolved by the consuming QAT translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
