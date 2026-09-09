/*
 * Copyright (C) 2019  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by the surrounding translation unit: amdgpu_ras.h

/* (addr / 256) * 4096, the higher 26 bits in ErrorAddr is the index of 4KB block */
#[inline]
pub const fn addr_of_4kb_block(addr: u64) -> u64 { (addr & !0xff_u64) << 4 }

/* (addr / 256) * 8192, the higher 26 bits in ErrorAddr is the index of 8KB block */
#[inline]
pub const fn addr_of_8kb_block(addr: u64) -> u64 { (addr & !0xff_u64) << 5 }

/* (addr / 256) * 32768, the higher 26 bits in ErrorAddr is the index of 8KB block */
#[inline]
pub const fn addr_of_32kb_block(addr: u64) -> u64 { (addr & !0xff_u64) << 7 }

/* channel index is the index of 256B block */
#[inline]
pub const fn addr_of_256b_block(channel_index: u64) -> u64 { channel_index << 8 }

/* offset in 256B block */
#[inline]
pub const fn offset_in_256b_block(addr: u64) -> u64 { addr & 0xff_u64 }

// The LOOP_* macros depend on the enclosing amdgpu_device and for_each_set_bit.
macro_rules! loop_umc_inst {
    ($umc_inst:ident, $adev:expr, $body:block) => {
        for $umc_inst in 0..$adev.umc.umc_inst_num { $body }
    };
}
macro_rules! loop_umc_ch_inst {
    ($ch_inst:ident, $adev:expr, $body:block) => {
        for $ch_inst in 0..$adev.umc.channel_inst_num { $body }
    };
}
macro_rules! loop_umc_inst_and_ch {
    ($umc_inst:ident, $ch_inst:ident, $adev:expr, $body:block) => {
        loop_umc_inst!($umc_inst, $adev, {
            loop_umc_ch_inst!($ch_inst, $adev, $body);
        });
    };
}

/* Page retirement tag */
pub const UMC_ECC_NEW_DETECTED_TAG: u64 = 0x1;

/* Channel-index v2 is stored in bit 47 of the EEPROM value. */
pub const UMC_CHANNEL_IDX_V2: u64 = 1_u64 << 47;

/* Save nps value to retired_page[47:40]. */
pub const UMC_NPS_SHIFT: u32 = 40;
pub const UMC_NPS_MASK: u64 = 0xff;

pub type UmcFunc = unsafe extern "C" fn(
    adev: *mut amdgpu_device,
    node_inst: u32,
    umc_inst: u32,
    ch_inst: u32,
    data: *mut core::ffi::c_void,
) -> i32;

#[repr(C)]
pub struct amdgpu_umc_ras {
    pub ras_block: amdgpu_ras_block_object,
    pub err_cnt_init: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
    pub query_ras_poison_mode: Option<unsafe extern "C" fn(*mut amdgpu_device) -> bool>,
    pub ecc_info_query_ras_error_count: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut core::ffi::c_void)>,
    pub ecc_info_query_ras_error_address: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct amdgpu_umc_funcs {
    pub init_registers: Option<unsafe extern "C" fn(*mut amdgpu_device)>,
}

#[repr(C)]
pub struct amdgpu_umc {
    pub max_ras_err_cnt_per_query: u32,
    pub channel_inst_num: u32,
    pub umc_inst_num: u32,
    pub node_inst_num: u32,
    pub channel_offs: u32,
    pub retire_unit: u32,
    pub channel_idx_tbl: *const u32,
    pub ras_if: *mut ras_common_if,
    pub funcs: *const amdgpu_umc_funcs,
    pub ras: *mut amdgpu_umc_ras,
    pub active_mask: core::ffi::c_ulong,
    pub err_addr_cnt: core::ffi::c_ulong,
}

extern "C" {
    pub fn amdgpu_umc_ras_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_umc_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> i32;
    pub fn amdgpu_umc_poison_handler(adev: *mut amdgpu_device, block: amdgpu_ras_block, reset: u32) -> i32;
    pub fn amdgpu_umc_pasid_poison_handler(adev: *mut amdgpu_device, block: amdgpu_ras_block, pasid: u16, pasid_fn: pasid_notify, data: *mut core::ffi::c_void, reset: u32) -> i32;
    pub fn amdgpu_umc_process_ecc_irq(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32;
    pub fn amdgpu_umc_uniras_process_ecc_irq(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32;
    pub fn amdgpu_umc_fill_error_record(err_data: *mut ras_err_data, err_addr: u64, retired_page: u64, channel_index: u32, umc_inst: u32) -> i32;
    pub fn amdgpu_umc_process_ras_data_cb(adev: *mut amdgpu_device, ras_error_status: *mut core::ffi::c_void, entry: *mut amdgpu_iv_entry) -> i32;
    pub fn amdgpu_umc_page_retirement_mca(adev: *mut amdgpu_device, err_addr: u64, ch_inst: u32, umc_inst: u32) -> i32;
    pub fn amdgpu_umc_loop_channels(adev: *mut amdgpu_device, func: UmcFunc, data: *mut core::ffi::c_void) -> i32;
    pub fn amdgpu_umc_handle_bad_pages(adev: *mut amdgpu_device, ras_error_status: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
