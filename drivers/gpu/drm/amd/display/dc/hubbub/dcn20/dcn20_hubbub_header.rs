/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependencies: dcn10/dcn10_hubbub.h and dcn20/dcn20_vmid.h

macro_rules! TO_DCN20_HUBBUB { ($hubbub:expr) => { container_of!($hubbub, dcn20_hubbub, base) }; }

macro_rules! HUBBUB_REG_LIST_DCN20_COMMON { () => {
    HUBBUB_REG_LIST_DCN_COMMON!(), SR!(DCHUBBUB_CRC_CTRL),
    SR!(DCN_VM_FB_LOCATION_BASE), SR!(DCN_VM_FB_LOCATION_TOP), SR!(DCN_VM_FB_OFFSET),
    SR!(DCN_VM_AGP_BOT), SR!(DCN_VM_AGP_TOP), SR!(DCN_VM_AGP_BASE),
    SR!(DCN_VM_FAULT_ADDR_MSB), SR!(DCN_VM_FAULT_ADDR_LSB), SR!(DCN_VM_FAULT_CNTL),
    SR!(DCN_VM_FAULT_STATUS)
}; }

macro_rules! HUBBUB_REG_LIST_DCN20 { ($id:expr) => {
    HUBBUB_REG_LIST_DCN20_COMMON!(), HUBBUB_SR_WATERMARK_REG_LIST!(), HUBBUB_VM_REG_LIST!(),
    SR!(DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_MSB), SR!(DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_LSB)
}; }

macro_rules! HUBBUB_MASK_SH_LIST_DCN20 { ($mask_sh:expr) => {
    HUBBUB_MASK_SH_LIST_DCN_COMMON!($mask_sh), HUBBUB_MASK_SH_LIST_STUTTER!($mask_sh),
    HUBBUB_SF!(DCHUBBUB_GLOBAL_TIMER_CNTL, DCHUBBUB_GLOBAL_TIMER_REFDIV, $mask_sh),
    HUBBUB_SF!(DCN_VM_FB_LOCATION_BASE, FB_BASE, $mask_sh), HUBBUB_SF!(DCN_VM_FB_LOCATION_TOP, FB_TOP, $mask_sh),
    HUBBUB_SF!(DCN_VM_FB_OFFSET, FB_OFFSET, $mask_sh), HUBBUB_SF!(DCN_VM_AGP_BOT, AGP_BOT, $mask_sh),
    HUBBUB_SF!(DCN_VM_AGP_TOP, AGP_TOP, $mask_sh), HUBBUB_SF!(DCN_VM_AGP_BASE, AGP_BASE, $mask_sh),
    HUBBUB_SF!(DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_MSB, DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_MSB, $mask_sh),
    HUBBUB_SF!(DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_LSB, DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_LSB, $mask_sh),
    HUBBUB_SF!(DCN_VM_FAULT_ADDR_MSB, DCN_VM_FAULT_ADDR_MSB, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_ADDR_LSB, DCN_VM_FAULT_ADDR_LSB, $mask_sh),
    HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_CLEAR, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_MODE, $mask_sh),
    HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_INTERRUPT_ENABLE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_RANGE_FAULT_DISABLE, $mask_sh),
    HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_PRQ_FAULT_DISABLE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_STATUS, $mask_sh),
    HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_VMID, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_TABLE_LEVEL, $mask_sh),
    HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_PIPE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_INTERRUPT_STATUS, $mask_sh)
}; }

#[repr(C)]
pub struct dcn20_hubbub {
    pub base: hubbub,
    pub regs: *const dcn_hubbub_registers,
    pub shifts: *const dcn_hubbub_shift,
    pub masks: *const dcn_hubbub_mask,
    pub debug_test_index_pstate: c_uint,
    pub watermarks: dcn_watermark_set,
    pub num_vmid: c_int,
    pub vmid: [dcn20_vmid; 16],
    pub detile_buf_size: c_uint,
    pub crb_size_segs: c_uint,
    pub compbuf_size_segments: c_uint,
    pub pixel_chunk_size: c_uint,
    pub det0_size: c_uint,
    pub det1_size: c_uint,
    pub det2_size: c_uint,
    pub det3_size: c_uint,
    pub allow_sdpif_rate_limit_when_cstate_req: bool,
}

extern "C" {
    pub fn hubbub2_construct(hubbub: *mut dcn20_hubbub, ctx: *mut dc_context, hubbub_regs: *const dcn_hubbub_registers, hubbub_shift: *const dcn_hubbub_shift, hubbub_mask: *const dcn_hubbub_mask);
    pub fn hubbub2_dcc_support_swizzle(swizzle: swizzle_mode_values, bytes_per_element: c_uint, segment_order_horz: *mut segment_order, segment_order_vert: *mut segment_order) -> bool;
    pub fn hubbub2_dcc_support_pixel_format(format: surface_pixel_format, bytes_per_element: *mut c_uint) -> bool;
    pub fn hubbub2_get_dcc_compression_cap(hubbub: *mut hubbub, input: *const dc_dcc_surface_param, output: *mut dc_surface_dcc_cap) -> bool;
    pub fn hubbub2_initialize_vmids(hubbub: *mut hubbub, input: *const dc_dcc_surface_param, output: *mut dc_surface_dcc_cap) -> bool;
    pub fn hubbub2_init_dchub_sys_ctx(hubbub: *mut hubbub, pa_config: *mut dcn_hubbub_phys_addr_config) -> c_int;
    pub fn hubbub2_init_vm_ctx(hubbub: *mut hubbub, va_config: *mut dcn_hubbub_virt_addr_config, vmid: c_int);
    pub fn hubbub2_update_dchub(hubbub: *mut hubbub, dh_data: *mut dchub_init_data);
    pub fn hubbub2_get_dchub_ref_freq(hubbub: *mut hubbub, dccg_ref_freq_inKhz: c_uint, dchub_ref_freq_inKhz: *mut c_uint);
    pub fn hubbub2_wm_read_state(hubbub: *mut hubbub, wm: *mut dcn_hubbub_wm);
    pub fn hubbub2_read_state(hubbub: *mut hubbub, hubbub_state: *mut dcn_hubbub_state);
}

/* Extract bits [47:24] of a physical address for hardware register fields */
macro_rules! ADDR_HI24 { ($a:expr) => { (($a as u64) >> 24) as u32 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
