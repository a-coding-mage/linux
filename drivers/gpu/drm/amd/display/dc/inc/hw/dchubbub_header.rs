/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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
 */

/// DOC: overview
///
/// There is only one common DCHUBBUB. It contains the common request and return
/// blocks for the Data Fabric Interface that are not clock/power gated.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dcc_control { DccControl256256Xxx, DccControl128128Xxx, DccControl2566464, DccControl256128128, DccControl256256, DccControl256128, DccControl25664 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum segment_order { SegmentOrderNa, SegmentOrderContiguous, SegmentOrderNonContiguous }

#[repr(C)]
pub struct dcn_hubbub_wm_set { pub wm_set: u32, pub data_urgent: u32, pub pte_meta_urgent: u32, pub sr_enter: u32, pub sr_exit: u32, pub dram_clk_change: u32, pub usr_retrain: u32, pub fclk_pstate_change: u32, pub sr_enter_exit_Z8: u32, pub sr_enter_Z8: u32 }
#[repr(C)] pub struct dcn_hubbub_wm { pub sets: [dcn_hubbub_wm_set; 4] }

#[repr(C)] pub enum dcn_hubbub_page_table_depth { DcnPageTableDepth1Level, DcnPageTableDepth2Level, DcnPageTableDepth3Level, DcnPageTableDepth4Level }
#[repr(C)] pub enum dcn_hubbub_page_table_block_size { DcnPageTableBlockSize4KB = 0, DcnPageTableBlockSize8KB = 1, DcnPageTableBlockSize16KB = 2, DcnPageTableBlockSize32KB = 3, DcnPageTableBlockSize64KB = 4, DcnPageTableBlockSize128KB = 5, DcnPageTableBlockSize256KB = 6, DcnPageTableBlockSize512KB = 7, DcnPageTableBlockSize1024KB = 8, DcnPageTableBlockSize2048KB = 9 }

#[repr(C)] pub struct dcn_hubbub_phys_addr_config { pub system_aperture: SystemAperture, pub gart_config: GartConfig, pub page_table_default_page_addr: u64 }
#[repr(C)] pub struct SystemAperture { pub fb_top: u64, pub fb_offset: u64, pub fb_base: u64, pub agp_top: u64, pub agp_bot: u64, pub agp_base: u64 }
#[repr(C)] pub struct GartConfig { pub page_table_start_addr: u64, pub page_table_end_addr: u64, pub page_table_base_addr: u64 }
#[repr(C)] pub struct dcn_hubbub_virt_addr_config { pub page_table_start_addr: u64, pub page_table_end_addr: u64, pub page_table_block_size: dcn_hubbub_page_table_block_size, pub page_table_depth: dcn_hubbub_page_table_depth, pub page_table_base_addr: u64 }
#[repr(C)] pub struct hubbub_addr_config { pub pa_config: dcn_hubbub_phys_addr_config, pub va_config: dcn_hubbub_virt_addr_config, pub default_addrs: DefaultAddrs }
#[repr(C)] pub struct DefaultAddrs { pub aperture_check_fault: u64, pub generic_fault: u64 }
#[repr(C)] pub struct dcn_hubbub_state { pub vm_fault_addr_msb: u32, pub vm_fault_addr_lsb: u32, pub vm_error_status: u32, pub vm_error_vmid: u32, pub vm_error_pipe: u32, pub vm_error_mode: u32, pub test_debug_data: u32, pub watermark_change_cntl: u32, pub dram_state_cntl: u32 }
#[repr(C)] pub struct dcn_hubbub_reg_state { pub det0_ctrl: u32, pub det1_ctrl: u32, pub det2_ctrl: u32, pub det3_ctrl: u32, pub compbuf_ctrl: u32 }
#[repr(C)] pub struct hubbub_urgent_latency_params { pub refclk_mhz: u32, pub t_win_ns: u32, pub bandwidth_mbps: u32, pub bw_factor_x1000: u32 }

// Types declared by other headers; their definitions are external dependencies.
pub enum hubbub {}
pub enum dchub_init_data {}
pub enum dc_dcc_surface_param {}
pub enum dc_surface_dcc_cap {}
pub enum dcn_watermark_set {}
pub enum dml2_display_arb_regs {}
pub enum dc_context {}
pub enum swizzle_mode_values {}
pub enum swizzle_mode_addr3_values {}
pub enum surface_pixel_format {}

#[repr(C)] pub struct hubbub_perfmon_funcs {
    pub reset: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub start_measuring_memory_latencies: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub get_memory_latencies_ns: Option<unsafe extern "C" fn(*mut hubbub, u32, *mut u32, *mut u32, *mut u32) -> u32>,
    pub start_measuring_urgent_assertion_count: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub get_urgent_assertion_count: Option<unsafe extern "C" fn(*mut hubbub, u32, *mut u32, *mut u32, *mut u32) -> bool>,
    pub start_measuring_urgent_ramp_latency: Option<unsafe extern "C" fn(*mut hubbub, *const hubbub_urgent_latency_params)>,
    pub get_urgent_ramp_latency_ns: Option<unsafe extern "C" fn(*mut hubbub, u32) -> u32>,
    pub arm_measuring_out_of_order_bandwidth: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub start_measuring_out_of_order_bandwidth: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub get_out_of_order_bandwidth_mbps: Option<unsafe extern "C" fn(*mut hubbub, u32, *mut u32) -> u32>,
    pub start_measuring_in_order_bandwidth: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub get_in_order_bandwidth_mbps: Option<unsafe extern "C" fn(*mut hubbub, u32, u32, *mut u32) -> u32>,
    pub start_measuring_prefetch_data_size: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub get_prefetch_data_size: Option<unsafe extern "C" fn(*mut hubbub) -> u32>,
}

#[repr(C)] pub struct hubbub_qos_funcs { pub force_display_nominal_profile: Option<unsafe extern "C" fn(*mut hubbub)>, pub force_display_urgent_profile: Option<unsafe extern "C" fn(*mut hubbub)>, pub reset_display_qos_profile: Option<unsafe extern "C" fn(*mut hubbub)> }

#[repr(C)] pub struct hubbub_funcs {
    pub update_dchub: Option<unsafe extern "C" fn(*mut hubbub, *mut dchub_init_data)>,
    pub init_dchub_sys_ctx: Option<unsafe extern "C" fn(*mut hubbub, *mut dcn_hubbub_phys_addr_config) -> i32>,
    pub init_vm_ctx: Option<unsafe extern "C" fn(*mut hubbub, *mut dcn_hubbub_virt_addr_config, i32)>,
    pub get_dcc_compression_cap: Option<unsafe extern "C" fn(*mut hubbub, *const dc_dcc_surface_param, *mut dc_surface_dcc_cap) -> bool>,
    pub dcc_support_swizzle: Option<unsafe extern "C" fn(swizzle_mode_values, u32, *mut segment_order, *mut segment_order) -> bool>,
    pub dcc_support_swizzle_addr3: Option<unsafe extern "C" fn(swizzle_mode_addr3_values, u32, u32, *mut segment_order, *mut segment_order) -> bool>,
    pub dcc_support_pixel_format_plane0_plane1: Option<unsafe extern "C" fn(surface_pixel_format, *mut u32, *mut u32) -> bool>,
    pub dcc_support_pixel_format: Option<unsafe extern "C" fn(surface_pixel_format, *mut u32) -> bool>,
    pub wm_read_state: Option<unsafe extern "C" fn(*mut hubbub, *mut dcn_hubbub_wm)>,
    pub get_dchub_ref_freq: Option<unsafe extern "C" fn(*mut hubbub, u32, *mut u32)>,
    pub program_watermarks: Option<unsafe extern "C" fn(*mut hubbub, *mut dcn_watermark_set, u32, bool) -> bool>,
    pub is_allow_self_refresh_enabled: Option<unsafe extern "C" fn(*mut hubbub) -> bool>,
    pub allow_self_refresh_control: Option<unsafe extern "C" fn(*mut hubbub, bool)>,
    pub verify_allow_pstate_change_high: Option<unsafe extern "C" fn(*mut hubbub) -> bool>,
    pub apply_DEDCN21_147_wa: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub force_wm_propagate_to_pipes: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub hubbub_read_state: Option<unsafe extern "C" fn(*mut hubbub, *mut dcn_hubbub_state)>,
    pub force_pstate_change_control: Option<unsafe extern "C" fn(*mut hubbub, bool, bool)>,
    pub init_watermarks: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub hubbub_read_reg_state: Option<unsafe extern "C" fn(*mut hubbub, *mut dcn_hubbub_reg_state)>,
    /*
     * @program_det_size:
     *
     * DE-Tile buffers (DET) is a memory that is used to convert the tiled
     * data into linear, which the rest of the display can use to generate
     * the graphics output. One of the main features of this component is
     * that each pipe has a configurable DET buffer which means that when a
     * pipe is not enabled, the device can assign the memory to other
     * enabled pipes to try to be more efficient.
     *
     * DET logic is handled by dchubbub. Some ASICs provide a feature named
     * Configurable Return Buffer (CRB) segments which can be allocated to
     * compressed or detiled buffers.
     */
    pub program_det_size: Option<unsafe extern "C" fn(*mut hubbub, i32, u32)>, pub wait_for_det_apply: Option<unsafe extern "C" fn(*mut hubbub, i32)>, pub program_compbuf_size: Option<unsafe extern "C" fn(*mut hubbub, u32, bool)>, pub init_crb: Option<unsafe extern "C" fn(*mut hubbub)>, pub force_usr_retraining_allow: Option<unsafe extern "C" fn(*mut hubbub, bool)>, pub set_request_limit: Option<unsafe extern "C" fn(*mut hubbub, i32, i32)>, pub dchubbub_init: Option<unsafe extern "C" fn(*mut hubbub)>, pub get_mall_en: Option<unsafe extern "C" fn(*mut hubbub, *mut u32)>, pub program_det_segments: Option<unsafe extern "C" fn(*mut hubbub, i32, u32)>, pub program_compbuf_segments: Option<unsafe extern "C" fn(*mut hubbub, u32, bool)>, pub wait_for_det_update: Option<unsafe extern "C" fn(*mut hubbub, i32)>, pub program_arbiter: Option<unsafe extern "C" fn(*mut hubbub, *mut dml2_display_arb_regs, bool) -> bool>, pub dchvm_init: Option<unsafe extern "C" fn(*mut hubbub)>,
    pub perfmon: hubbub_perfmon_funcs, pub qos: hubbub_qos_funcs,
}

#[repr(C)] pub struct hubbub { pub funcs: *const hubbub_funcs, pub ctx: *mut dc_context, pub riommu_active: bool }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
