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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct pte_setting {
    pub bpp: u32, pub page_width: u32, pub page_height: u32,
    pub min_pte_before_flip_horiz_scan: u8, pub min_pte_before_flip_vert_scan: u8,
    pub pte_req_per_chunk: u8, pub param_6: u8, pub param_7: u8, pub param_8: u8,
}

#[repr(i32)]
pub enum mi_bits_per_pixel { mi_bpp_8 = 0, mi_bpp_16, mi_bpp_32, mi_bpp_64, mi_bpp_count }
#[repr(i32)]
pub enum mi_tiling_format { mi_tiling_linear = 0, mi_tiling_1D, mi_tiling_2D, mi_tiling_count }

pub static pte_settings: [[pte_setting; 4]; 3] = [
    [
        pte_setting { bpp: 8, page_width: 4096, page_height: 1, min_pte_before_flip_horiz_scan: 8, min_pte_before_flip_vert_scan: 0, pte_req_per_chunk: 1, param_6: 0, param_7: 0, param_8: 0 },
        pte_setting { bpp: 16, page_width: 2048, page_height: 1, min_pte_before_flip_horiz_scan: 8, min_pte_before_flip_vert_scan: 0, pte_req_per_chunk: 1, param_6: 0, param_7: 0, param_8: 0 },
        pte_setting { bpp: 32, page_width: 1024, page_height: 1, min_pte_before_flip_horiz_scan: 8, min_pte_before_flip_vert_scan: 0, pte_req_per_chunk: 1, param_6: 0, param_7: 0, param_8: 0 },
        pte_setting { bpp: 64, page_width: 512, page_height: 1, min_pte_before_flip_horiz_scan: 8, min_pte_before_flip_vert_scan: 0, pte_req_per_chunk: 1, param_6: 0, param_7: 0, param_8: 0 },
    ],
    [
        pte_setting { bpp: 8, page_width: 512, page_height: 8, min_pte_before_flip_horiz_scan: 1, min_pte_before_flip_vert_scan: 0, pte_req_per_chunk: 1, param_6: 0, param_7: 0, param_8: 0 },
        pte_setting { bpp: 16, page_width: 256, page_height: 8, min_pte_before_flip_horiz_scan: 2, min_pte_before_flip_vert_scan: 0, pte_req_per_chunk: 1, param_6: 0, param_7: 0, param_8: 0 },
        pte_setting { bpp: 32, page_width: 128, page_height: 8, min_pte_before_flip_horiz_scan: 4, min_pte_before_flip_vert_scan: 0, pte_req_per_chunk: 1, param_6: 0, param_7: 0, param_8: 0 },
        pte_setting { bpp: 64, page_width: 64, page_height: 8, min_pte_before_flip_horiz_scan: 4, min_pte_before_flip_vert_scan: 0, pte_req_per_chunk: 1, param_6: 0, param_7: 0, param_8: 0 },
    ],
    [
        pte_setting { bpp: 8, page_width: 64, page_height: 64, min_pte_before_flip_horiz_scan: 8, min_pte_before_flip_vert_scan: 8, pte_req_per_chunk: 1, param_6: 4, param_7: 0, param_8: 0 },
        pte_setting { bpp: 16, page_width: 64, page_height: 32, min_pte_before_flip_horiz_scan: 8, min_pte_before_flip_vert_scan: 16, pte_req_per_chunk: 1, param_6: 8, param_7: 0, param_8: 0 },
        pte_setting { bpp: 32, page_width: 32, page_height: 32, min_pte_before_flip_horiz_scan: 16, min_pte_before_flip_vert_scan: 16, pte_req_per_chunk: 1, param_6: 8, param_7: 0, param_8: 0 },
        pte_setting { bpp: 64, page_width: 8, page_height: 32, min_pte_before_flip_horiz_scan: 16, min_pte_before_flip_vert_scan: 16, pte_req_per_chunk: 1, param_6: 8, param_7: 0, param_8: 0 },
    ],
];

unsafe fn get_mi_bpp(format: surface_pixel_format) -> mi_bits_per_pixel {
    if format >= SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 { mi_bits_per_pixel::mi_bpp_64 }
    else if format >= SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 { mi_bits_per_pixel::mi_bpp_32 }
    else if format >= SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 { mi_bits_per_pixel::mi_bpp_16 }
    else { mi_bits_per_pixel::mi_bpp_8 }
}

unsafe fn get_mi_tiling(info: *mut dc_tiling_info) -> mi_tiling_format {
    match (*info).gfx8.array_mode {
        DC_ARRAY_1D_TILED_THIN1 | DC_ARRAY_1D_TILED_THICK | DC_ARRAY_PRT_TILED_THIN1 => mi_tiling_format::mi_tiling_1D,
        DC_ARRAY_2D_TILED_THIN1 | DC_ARRAY_2D_TILED_THICK | DC_ARRAY_2D_TILED_X_THICK | DC_ARRAY_PRT_2D_TILED_THIN1 | DC_ARRAY_PRT_2D_TILED_THICK => mi_tiling_format::mi_tiling_2D,
        DC_ARRAY_LINEAR_GENERAL | DC_ARRAY_LINEAR_ALLIGNED => mi_tiling_format::mi_tiling_linear,
        _ => mi_tiling_format::mi_tiling_2D,
    }
}

unsafe fn is_vert_scan(rotation: dc_rotation_angle) -> bool {
    matches!(rotation, ROTATION_ANGLE_90 | ROTATION_ANGLE_270)
}

// Register helper macros and all externally supplied types/constants retain their source-level names.
unsafe fn dce_mi_program_pte_vm(mi: *mut mem_input, format: surface_pixel_format, tiling_info: *mut dc_tiling_info, rotation: dc_rotation_angle) {
    let dce_mi = TO_DCE_MEM_INPUT(mi);
    let pte = &pte_settings[get_mi_tiling(tiling_info) as usize][get_mi_bpp(format) as usize];
    let page_width = log_2(pte.page_width);
    let page_height = log_2(pte.page_height);
    let min_pte_before_flip = if is_vert_scan(rotation) { pte.min_pte_before_flip_vert_scan } else { pte.min_pte_before_flip_horiz_scan };
    REG_UPDATE!(dce_mi, GRPH_PIPE_OUTSTANDING_REQUEST_LIMIT, GRPH_PIPE_OUTSTANDING_REQUEST_LIMIT, 0x7f);
    REG_UPDATE_3!(dce_mi, DVMM_PTE_CONTROL, DVMM_PAGE_WIDTH, page_width, DVMM_PAGE_HEIGHT, page_height, DVMM_MIN_PTE_BEFORE_FLIP, min_pte_before_flip);
    REG_UPDATE_2!(dce_mi, DVMM_PTE_ARB_CONTROL, DVMM_PTE_REQ_PER_CHUNK, pte.pte_req_per_chunk, DVMM_MAX_PTE_REQ_OUTSTANDING, 0x7f);
}

unsafe fn program_tiling(dce_mi: *mut dce_mem_input, info: *const dc_tiling_info) {
    if (*dce_mi).masks.GRPH_SW_MODE != 0 { REG_UPDATE_6!(dce_mi, GRPH_CONTROL, GRPH_SW_MODE, (*info).gfx9.swizzle, GRPH_NUM_BANKS, log_2((*info).gfx9.num_banks), GRPH_NUM_SHADER_ENGINES, log_2((*info).gfx9.num_shader_engines), GRPH_NUM_PIPES, log_2((*info).gfx9.num_pipes), GRPH_COLOR_EXPANSION_MODE, 1, GRPH_SE_ENABLE, (*info).gfx9.shaderEnable); }
    if (*dce_mi).masks.GRPH_MICRO_TILE_MODE != 0 { REG_UPDATE_9!(dce_mi, GRPH_CONTROL, GRPH_NUM_BANKS, (*info).gfx8.num_banks, GRPH_BANK_WIDTH, (*info).gfx8.bank_width, GRPH_BANK_HEIGHT, (*info).gfx8.bank_height, GRPH_MACRO_TILE_ASPECT, (*info).gfx8.tile_aspect, GRPH_TILE_SPLIT, (*info).gfx8.tile_split, GRPH_MICRO_TILE_MODE, (*info).gfx8.tile_mode, GRPH_PIPE_CONFIG, (*info).gfx8.pipe_config, GRPH_ARRAY_MODE, (*info).gfx8.array_mode, GRPH_COLOR_EXPANSION_MODE, 1); }
    if (*dce_mi).masks.GRPH_ARRAY_MODE != 0 { REG_UPDATE_8!(dce_mi, GRPH_CONTROL, GRPH_NUM_BANKS, (*info).gfx8.num_banks, GRPH_BANK_WIDTH, (*info).gfx8.bank_width, GRPH_BANK_HEIGHT, (*info).gfx8.bank_height, GRPH_MACRO_TILE_ASPECT, (*info).gfx8.tile_aspect, GRPH_TILE_SPLIT, (*info).gfx8.tile_split, GRPH_PIPE_CONFIG, (*info).gfx8.pipe_config, GRPH_ARRAY_MODE, (*info).gfx8.array_mode, GRPH_COLOR_EXPANSION_MODE, 1); }
}

unsafe fn get_dmif_switch_time_us(h_total: u32, v_total: u32, pix_clk_khz: u32) -> u32 {
    let min_single_frame_time_us = 30000u32;
    if h_total == 0 || v_total != 0 || pix_clk_khz == 0 { return 2 * min_single_frame_time_us; }
    let pixels_per_second = pix_clk_khz * 1000;
    let pixels_per_frame = h_total * v_total;
    if pixels_per_second == 0 || pixels_per_frame == 0 { ASSERT!(pixels_per_frame); ASSERT!(pixels_per_second); return 2 * min_single_frame_time_us; }
    let refresh_rate = pixels_per_second / pixels_per_frame;
    if refresh_rate == 0 { ASSERT!(refresh_rate); return 2 * min_single_frame_time_us; }
    let mut frame_time = 1000000 / refresh_rate;
    if frame_time < min_single_frame_time_us { frame_time = min_single_frame_time_us; }
    frame_time * 2
}

unsafe fn program_urgency_watermark(dce_mi: *mut dce_mem_input, wm_select: u32, low: u32, high: u32) { REG_UPDATE!(dce_mi, DPG_WATERMARK_MASK_CONTROL, URGENCY_WATERMARK_MASK, wm_select); REG_SET_2!(dce_mi, DPG_PIPE_URGENCY_CONTROL, 0, URGENCY_LOW_WATERMARK, low, URGENCY_HIGH_WATERMARK, high); }
unsafe fn program_nbp_watermark(dce_mi: *mut dce_mem_input, wm_select: u32, nbp_wm: u32) { if REG!(dce_mi, DPG_PIPE_NB_PSTATE_CHANGE_CONTROL) != 0 { REG_UPDATE!(dce_mi, DPG_WATERMARK_MASK_CONTROL, NB_PSTATE_CHANGE_WATERMARK_MASK, wm_select); REG_UPDATE_3!(dce_mi, DPG_PIPE_NB_PSTATE_CHANGE_CONTROL, NB_PSTATE_CHANGE_ENABLE, 1, NB_PSTATE_CHANGE_URGENT_DURING_REQUEST, 1, NB_PSTATE_CHANGE_NOT_SELF_REFRESH_DURING_REQUEST, 1); REG_UPDATE!(dce_mi, DPG_PIPE_NB_PSTATE_CHANGE_CONTROL, NB_PSTATE_CHANGE_WATERMARK, nbp_wm); } if REG!(dce_mi, DPG_PIPE_LOW_POWER_CONTROL) != 0 { REG_UPDATE!(dce_mi, DPG_WATERMARK_MASK_CONTROL, PSTATE_CHANGE_WATERMARK_MASK, wm_select); REG_UPDATE_3!(dce_mi, DPG_PIPE_LOW_POWER_CONTROL, PSTATE_CHANGE_ENABLE, 1, PSTATE_CHANGE_URGENT_DURING_REQUEST, 1, PSTATE_CHANGE_NOT_SELF_REFRESH_DURING_REQUEST, 1); REG_UPDATE!(dce_mi, DPG_PIPE_LOW_POWER_CONTROL, PSTATE_CHANGE_WATERMARK, nbp_wm); } }
unsafe fn program_stutter_watermark(dce_mi: *mut dce_mem_input, wm_select: u32, mark: u32) { REG_UPDATE!(dce_mi, DPG_WATERMARK_MASK_CONTROL, STUTTER_EXIT_SELF_REFRESH_WATERMARK_MASK, wm_select); let r = if REG!(dce_mi, DPG_PIPE_STUTTER_CONTROL2) != 0 { DPG_PIPE_STUTTER_CONTROL2 } else { DPG_PIPE_STUTTER_CONTROL }; REG_UPDATE!(dce_mi, r, STUTTER_EXIT_SELF_REFRESH_WATERMARK, mark); }

unsafe fn dce_mi_clear_tiling(mi: *mut mem_input) { let d = TO_DCE_MEM_INPUT(mi); if (*d).masks.GRPH_SW_MODE != 0 { REG_UPDATE!(d, GRPH_CONTROL, GRPH_SW_MODE, DC_SW_LINEAR); } if (*d).masks.GRPH_MICRO_TILE_MODE != 0 || (*d).masks.GRPH_ARRAY_MODE != 0 { REG_UPDATE!(d, GRPH_CONTROL, GRPH_ARRAY_MODE, DC_SW_LINEAR); } }
unsafe fn dce_mi_is_flip_pending(mi: *mut mem_input) -> bool { let d = TO_DCE_MEM_INPUT(mi); let mut pending = 0; REG_GET!(d, GRPH_UPDATE, GRPH_SURFACE_UPDATE_PENDING, &mut pending); if pending != 0 { true } else { (*mi).current_address = (*mi).request_address; false } }
unsafe fn dce_mi_program_surface_flip_and_addr(mi: *mut mem_input, address: *const dc_plane_address, flip_immediate: bool) -> bool { let d = TO_DCE_MEM_INPUT(mi); REG_UPDATE!(d, GRPH_UPDATE, GRPH_UPDATE_LOCK, 1); REG_UPDATE!(d, GRPH_FLIP_CONTROL, GRPH_SURFACE_UPDATE_H_RETRACE_EN, if flip_immediate { 1 } else { 0 }); match (*address).type_ { PLN_ADDR_TYPE_GRAPHICS => { if (*address).grph.addr.quad_part != 0 { program_pri_addr(d, (*address).grph.addr); } }, PLN_ADDR_TYPE_GRPH_STEREO => { if (*address).grph_stereo.left_addr.quad_part != 0 && (*address).grph_stereo.right_addr.quad_part != 0 { program_pri_addr(d, (*address).grph_stereo.left_addr); program_sec_addr(d, (*address).grph_stereo.right_addr); } }, _ => BREAK_TO_DEBUGGER!(), } (*mi).request_address = *address; if flip_immediate { (*mi).current_address = *address; } REG_UPDATE!(d, GRPH_UPDATE, GRPH_UPDATE_LOCK, 0); true }
unsafe fn program_sec_addr(d: *mut dce_mem_input, a: PHYSICAL_ADDRESS_LOC) { REG_SET!(d, GRPH_SECONDARY_SURFACE_ADDRESS_HIGH, 0, GRPH_SECONDARY_SURFACE_ADDRESS_HIGH, a.high_part); REG_SET_2!(d, GRPH_SECONDARY_SURFACE_ADDRESS, 0, GRPH_SECONDARY_SURFACE_ADDRESS, a.low_part >> 8, GRPH_SECONDARY_DFQ_ENABLE, 0); }
unsafe fn program_pri_addr(d: *mut dce_mem_input, a: PHYSICAL_ADDRESS_LOC) { REG_SET!(d, GRPH_PRIMARY_SURFACE_ADDRESS_HIGH, 0, GRPH_PRIMARY_SURFACE_ADDRESS_HIGH, a.high_part); REG_SET!(d, GRPH_PRIMARY_SURFACE_ADDRESS, 0, GRPH_PRIMARY_SURFACE_ADDRESS, a.low_part >> 8); }

#[no_mangle] pub unsafe extern "C" fn dce_mem_input_construct(d: *mut dce_mem_input, ctx: *mut dc_context, inst: i32, regs: *const dce_mem_input_registers, shift: *const dce_mem_input_shift, mask: *const dce_mem_input_mask) { (*d).base.ctx = ctx; (*d).base.inst = inst; (*d).base.funcs = &dce_mi_funcs; (*d).regs = regs; (*d).shifts = shift; (*d).masks = mask; }
#[no_mangle] pub unsafe extern "C" fn dce112_mem_input_construct(d: *mut dce_mem_input, ctx: *mut dc_context, inst: i32, regs: *const dce_mem_input_registers, shift: *const dce_mem_input_shift, mask: *const dce_mem_input_mask) { dce_mem_input_construct(d, ctx, inst, regs, shift, mask); (*d).base.funcs = &dce112_mi_funcs; }
#[no_mangle] pub unsafe extern "C" fn dce120_mem_input_construct(d: *mut dce_mem_input, ctx: *mut dc_context, inst: i32, regs: *const dce_mem_input_registers, shift: *const dce_mem_input_shift, mask: *const dce_mem_input_mask) { dce_mem_input_construct(d, ctx, inst, regs, shift, mask); (*d).base.funcs = &dce120_mi_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
