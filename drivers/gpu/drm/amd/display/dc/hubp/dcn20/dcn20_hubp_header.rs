/*
 * Copyright 2012-2026 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependency: ../dcn10/dcn10_hubp.h

#[macro_export]
macro_rules! TO_DCN20_HUBP { ($hubp:expr) => { container_of!($hubp, dcn20_hubp, base) }; }

#[macro_export]
macro_rules! HUBP_REG_LIST_DCN2_COMMON { ($id:expr) => { HUBP_REG_LIST_DCN!($id); HUBP_REG_LIST_DCN_VM!($id); SRI!(PREFETCH_SETTINGS, HUBPREQ, $id); SRI!(PREFETCH_SETTINGS_C, HUBPREQ, $id); SRI!(DCN_VM_SYSTEM_APERTURE_LOW_ADDR, HUBPREQ, $id); SRI!(DCN_VM_SYSTEM_APERTURE_HIGH_ADDR, HUBPREQ, $id); SRI!(CURSOR_SETTINGS, HUBPREQ, $id); SRI!(CURSOR_SURFACE_ADDRESS_HIGH, CURSOR0_, $id); SRI!(CURSOR_SURFACE_ADDRESS, CURSOR0_, $id); SRI!(CURSOR_SIZE, CURSOR0_, $id); SRI!(CURSOR_CONTROL, CURSOR0_, $id); SRI!(CURSOR_POSITION, CURSOR0_, $id); SRI!(CURSOR_HOT_SPOT, CURSOR0_, $id); SRI!(CURSOR_DST_OFFSET, CURSOR0_, $id); SRI!(DMDATA_ADDRESS_HIGH, CURSOR0_, $id); SRI!(DMDATA_ADDRESS_LOW, CURSOR0_, $id); SRI!(DMDATA_CNTL, CURSOR0_, $id); SRI!(DMDATA_SW_CNTL, CURSOR0_, $id); SRI!(DMDATA_QOS_CNTL, CURSOR0_, $id); SRI!(DMDATA_SW_DATA, CURSOR0_, $id); SRI!(DMDATA_STATUS, CURSOR0_, $id); SRI!(FLIP_PARAMETERS_0, HUBPREQ, $id); SRI!(FLIP_PARAMETERS_1, HUBPREQ, $id); SRI!(FLIP_PARAMETERS_2, HUBPREQ, $id); SRI!(DCN_CUR1_TTU_CNTL0, HUBPREQ, $id); SRI!(DCN_CUR1_TTU_CNTL1, HUBPREQ, $id); SRI!(DCSURF_FLIP_CONTROL2, HUBPREQ, $id); SRI!(VMID_SETTINGS_0, HUBPREQ, $id) }; }
#[macro_export]
macro_rules! HUBP_REG_LIST_DCN20 { ($id:expr) => { HUBP_REG_LIST_DCN2_COMMON!($id); SR!(DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB); SR!(DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB) }; }

// Register and field-list macros are retained as token-level macro expansions;
// their referenced lists and register-field macros are supplied by dependencies.
#[macro_export] macro_rules! HUBP_MASK_SH_LIST_DCN2_SHARE_COMMON { ($m:expr) => { HUBP_MASK_SH_LIST_DCN_SHARE_COMMON!($m); HUBP_MASK_SH_LIST_DCN_VM!($m); HUBP_SF!(HUBP0_DCSURF_SURFACE_CONFIG, ROTATION_ANGLE, $m); HUBP_SF!(HUBP0_DCSURF_SURFACE_CONFIG, H_MIRROR_EN, $m); HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS, DST_Y_PREFETCH, $m); HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS, VRATIO_PREFETCH, $m); HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS_C, VRATIO_PREFETCH_C, $m); HUBP_SF!(HUBPREQ0_DCN_VM_SYSTEM_APERTURE_LOW_ADDR, MC_VM_SYSTEM_APERTURE_LOW_ADDR, $m); HUBP_SF!(HUBPREQ0_DCN_VM_SYSTEM_APERTURE_HIGH_ADDR, MC_VM_SYSTEM_APERTURE_HIGH_ADDR, $m); HUBP_SF!(HUBPREQ0_CURSOR_SETTINGS, CURSOR0_DST_Y_OFFSET, $m); HUBP_SF!(HUBPREQ0_CURSOR_SETTINGS, CURSOR0_CHUNK_HDL_ADJUST, $m); HUBP_SF!(HUBPREQ0_VMID_SETTINGS_0, VMID, $m) }; }
#[macro_export] macro_rules! HUBP_MASK_SH_LIST_DCN2_COMMON { ($m:expr) => { HUBP_MASK_SH_LIST_DCN2_SHARE_COMMON!($m); HUBP_SF!(HUBP0_DCSURF_TILING_CONFIG, RB_ALIGNED, $m); HUBP_SF!(HUBP0_DCHUBP_REQ_SIZE_CONFIG, MPTE_GROUP_SIZE, $m); HUBP_SF!(HUBP0_DCHUBP_REQ_SIZE_CONFIG_C, MPTE_GROUP_SIZE_C, $m) }; }
#[macro_export] macro_rules! HUBP_MASK_SH_LIST_DCN20 { ($m:expr) => { HUBP_MASK_SH_LIST_DCN2_COMMON!($m); HUBP_SF!(DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, DCN_VM_SYSTEM_APERTURE_DEFAULT_SYSTEM, $m); HUBP_SF!(DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, $m); HUBP_SF!(DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, $m) }; }

// The following C field-list macros are represented directly as token lists.
#[macro_export] macro_rules! DCN2_HUBP_REG_COMMON_VARIABLE_LIST { () => { DCN_HUBP_REG_COMMON_VARIABLE_LIST!(); DMDATA_ADDRESS_HIGH: u32, DMDATA_ADDRESS_LOW: u32, DMDATA_CNTL: u32, DMDATA_SW_CNTL: u32, DMDATA_QOS_CNTL: u32, DMDATA_SW_DATA: u32, DMDATA_STATUS: u32, DCSURF_FLIP_CONTROL2: u32, FLIP_PARAMETERS_0: u32, FLIP_PARAMETERS_1: u32, FLIP_PARAMETERS_2: u32, DCN_CUR1_TTU_CNTL0: u32, DCN_CUR1_TTU_CNTL1: u32, VMID_SETTINGS_0: u32, DST_Y_DELTA_DRQ_LIMIT: u32 }; }
#[macro_export] macro_rules! DCN21_HUBP_REG_COMMON_VARIABLE_LIST { () => { DCN2_HUBP_REG_COMMON_VARIABLE_LIST!(); FLIP_PARAMETERS_3: u32, FLIP_PARAMETERS_4: u32, FLIP_PARAMETERS_5: u32, FLIP_PARAMETERS_6: u32, VBLANK_PARAMETERS_5: u32, VBLANK_PARAMETERS_6: u32 }; }
#[macro_export] macro_rules! DCN30_HUBP_REG_COMMON_VARIABLE_LIST { () => { DCN21_HUBP_REG_COMMON_VARIABLE_LIST!(); DCN_DMDATA_VM_CNTL: u32 }; }
#[macro_export] macro_rules! DCN32_HUBP_REG_COMMON_VARIABLE_LIST { () => { DCN30_HUBP_REG_COMMON_VARIABLE_LIST!(); DCHUBP_MALL_CONFIG: u32, DCHUBP_VMPG_CONFIG: u32, UCLK_PSTATE_FORCE: u32 }; }
#[macro_export] macro_rules! DCN401_HUBP_REG_COMMON_VARIABLE_LIST { () => { DCN32_HUBP_REG_COMMON_VARIABLE_LIST!(); _3DLUT_FL_BIAS_SCALE: u32, _3DLUT_FL_CONFIG: u32, HUBP_3DLUT_ADDRESS_HIGH: u32, HUBP_3DLUT_ADDRESS_LOW: u32, HUBP_3DLUT_CONTROL: u32, HUBP_3DLUT_DLG_PARAM: u32, DCSURF_VIEWPORT_MCACHE_SPLIT_COORDINATE: u32, DCHUBP_MCACHEID_CONFIG: u32, DCHUBP_MALL_SUB_VP: u32, DCHUBP_ADDR_CONFIG: u32, HUBP_MALL_STATUS: u32 }; }
#[macro_export] macro_rules! DCN60_HUBP_REG_VARIABLE_LIST { () => { DCSURF_LEGACY_ADDR_CONFIG: u32, DST_Y_ALT_CH_DRQ_LIMIT: u32 }; }

#[repr(C)]
pub struct dcn_hubp2_registers { pub _opaque: [u8; 0] }
#[repr(C)]
pub struct dcn_hubp2_shift { pub _opaque: [u8; 0] }
#[repr(C)]
pub struct dcn_hubp2_mask { pub _opaque: [u8; 0] }

#[repr(C)]
pub struct dcn20_hubp {
    pub base: hubp,
    pub state: dcn_hubp_state,
    pub hubp_regs: *const dcn_hubp2_registers,
    pub hubp_shift: *const dcn_hubp2_shift,
    pub hubp_mask: *const dcn_hubp2_mask,
}

extern "C" {
    pub fn hubp2_construct(hubp2: *mut dcn20_hubp, ctx: *mut dc_context, inst: u32, hubp_regs: *const dcn_hubp2_registers, hubp_shift: *const dcn_hubp2_shift, hubp_mask: *const dcn_hubp2_mask) -> bool;
    pub fn hubp2_setup_interdependent(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st);
    pub fn hubp2_vready_at_or_After_vsync(hubp: *mut hubp, pipe_dest: *mut _vcs_dpi_display_pipe_dest_params_st);
    pub fn hubp2_cursor_set_attributes(hubp: *mut hubp, attr: *const dc_cursor_attributes);
    pub fn hubp2_set_vm_system_aperture_settings(hubp: *mut hubp, apt: *mut vm_system_aperture_param);
    pub fn hubp2_get_lines_per_chunk(cursor_width: c_uint, cursor_mode: dc_cursor_color_format) -> cursor_lines_per_chunk;
    pub fn hubp2_dmdata_set_attributes(hubp: *mut hubp, attr: *const dc_dmdata_attributes);
    pub fn hubp2_dmdata_load(hubp: *mut hubp, dmdata_sw_size: u32, dmdata_sw_data: *const u32);
    pub fn hubp2_dmdata_status_done(hubp: *mut hubp) -> bool;
    pub fn hubp2_enable_triplebuffer(hubp: *mut hubp, enable: bool);
    pub fn hubp2_is_triplebuffer_enabled(hubp: *mut hubp) -> bool;
    pub fn hubp2_set_flip_control_surface_gsl(hubp: *mut hubp, enable: bool);
    pub fn hubp2_program_deadline(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st);
    pub fn hubp2_program_surface_flip_and_addr(hubp: *mut hubp, address: *const dc_plane_address, flip_immediate: bool) -> bool;
    pub fn hubp2_dcc_control(hubp: *mut hubp, enable: bool, independent_64b_blks: hubp_ind_block_size);
    pub fn hubp2_program_size(hubp: *mut hubp, format: surface_pixel_format, plane_size: *const plane_size, dcc: *mut dc_plane_dcc_param);
    pub fn hubp2_program_rotation(hubp: *mut hubp, rotation: dc_rotation_angle, horizontal_mirror: bool);
    pub fn hubp2_program_pixel_format(hubp: *mut hubp, format: surface_pixel_format);
    pub fn hubp2_program_surface_config(hubp: *mut hubp, format: surface_pixel_format, tiling_info: *mut dc_tiling_info, plane_size: *mut plane_size, rotation: dc_rotation_angle, dcc: *mut dc_plane_dcc_param, horizontal_mirror: bool, compat_level: c_uint);
    pub fn hubp2_is_flip_pending(hubp: *mut hubp) -> bool;
    pub fn hubp2_set_blank(hubp: *mut hubp, blank: bool);
    pub fn hubp2_set_blank_regs(hubp: *mut hubp, blank: bool);
    pub fn hubp2_cursor_set_position(hubp: *mut hubp, pos: *const dc_cursor_position, param: *const dc_cursor_mi_param);
    pub fn hubp2_clk_cntl(hubp: *mut hubp, enable: bool);
    pub fn hubp2_vtg_sel(hubp: *mut hubp, otg_inst: u32);
    pub fn hubp2_clear_underflow(hubp: *mut hubp);
    pub fn hubp2_read_state_common(hubp: *mut hubp);
    pub fn hubp2_read_state(hubp: *mut hubp);
    pub fn hubp2_clear_tiling(hubp: *mut hubp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
