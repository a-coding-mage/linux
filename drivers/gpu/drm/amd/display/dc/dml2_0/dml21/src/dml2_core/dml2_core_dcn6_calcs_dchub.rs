// Faithful source-level Rust translation of dml2_core_dcn6_calcs_dchub.c
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// dependency: "dml2_core_dcn5_calcs_dchub.h"
// dependency: "dml2_core_dcn6_calcs_dchub.h"
// dependency: "dml_top_display_cfg_types.h"
// dependency: "dml2_core_utils.h"
// dependency: "dml_top_types.h"

u32 dcn6_calculate_max_vstartup(
		bool ptoi_supported,
		u32 vblank_nom_default_us,
		*const dml2_timing_cfgtiming,
		enum dml2_uclk_pstate_change_strategy pstate_strategy,
		f64 write_back_delay_us,
		u32 svp_lines)
{
	u32 vblank_size = 0;
	u32 max_vstartup_lines = 0;

	f64 line_time_us = (f64)timing.h_total / ((f64)timing.pixel_clock_khz / 1000);
	u32 vblank_actual = timing.v_total - timing.v_active;
	u32 vblank_nom_default_in_line = (u32)math_floor2((f64)vblank_nom_default_us / line_time_us, 1.0);
	u32 vblank_avail = (timing.vblank_nom == 0) ? vblank_nom_default_in_line : (u32)timing.vblank_nom;

	vblank_size = (u32)math_min2(vblank_actual, vblank_avail);

	if (timing.interlaced && !ptoi_supported)
		max_vstartup_lines = (u32)(math_floor2((vblank_size - 1) / 2.0, 1.0));
	else
		max_vstartup_lines = vblank_size - (u32)math_max2(1.0, math_ceil2(write_back_delay_us / line_time_us, 1.0));

	if (pstate_strategy == dml2_uclk_pstate_change_strategy_force_alternate)
		max_vstartup_lines = (u32)math_min2(max_vstartup_lines, svp_lines);

	max_vstartup_lines = (u32)math_min2(max_vstartup_lines, __DML2_CALCS_MAX_VSTARTUP__);

	dml_log_verbose!("DML::%s: VBlankNom = %lu\n", __func__, timing.vblank_nom);
	dml_log_verbose!("DML::%s: vblank_nom_default_us = %u\n", __func__, vblank_nom_default_us);
	dml_log_verbose!("DML::%s: line_time_us = %f\n", __func__, line_time_us);
	dml_log_verbose!("DML::%s: vblank_actual = %u\n", __func__, vblank_actual);
	dml_log_verbose!("DML::%s: vblank_avail = %u\n", __func__, vblank_avail);
	dml_log_verbose!("DML::%s: max_vstartup_lines = %u\n", __func__, max_vstartup_lines);
	return max_vstartup_lines;
}

unsafe fn dcn6_calculate_alternate_svp_lines(dml2_core_calcs_calculate_alternate_svp_lines *p)
{
	u32 i, j;
	f64 line_time_us, max_line_time_us = 0, svp0_time_us, svp1_time_us, vratio, vratio_c, swath_time_us, swath_time_c_us;
	f64 max_swath_time_all_planes_us = 0;
	f64 pad_us = 0;

	for i = 0;  i < p.display_cfg.num_streams;  i++ { {
		line_time_us = ((f64)p.display_cfg.stream_descriptors[i].timing.h_total * 1000 / p.display_cfg.stream_descriptors[i].timing.pixel_clock_khz);
		for j = 0;  j < p.display_cfg.num_planes;  j++ { {
			if (p.display_cfg.plane_descriptors[j].stream_index == i) {
				vratio = p.display_cfg.plane_descriptors[j].composition.scaler_info.plane0.v_ratio;
				vratio_c = p.display_cfg.plane_descriptors[j].composition.scaler_info.plane1.v_ratio;
				/* For now swath_time calculated based only on vratio - can use hdl schedule later once calculated
				 * (though HDL scehdule may produce the same result as just calc from vratio) */
				swath_time_us = ((f64)p.SwathHeightY[j] / vratio) * line_time_us;
				swath_time_c_us = p.BytePerPixelInDETC[j] > 0 ? ((f64)p.SwathHeightC[j] / vratio_c) * line_time_us : 0;
				if (swath_time_us > max_swath_time_all_planes_us || swath_time_c_us > max_swath_time_all_planes_us)
					max_swath_time_all_planes_us = swath_time_us > swath_time_c_us ? swath_time_us : swath_time_c_us;
			}
		}
		if (line_time_us > max_line_time_us)
			max_line_time_us = line_time_us;
	}
	pad_us = max_swath_time_all_planes_us + max_line_time_us;
	svp0_time_us = p.dram_blackout_us + pad_us + max_swath_time_all_planes_us;
	svp1_time_us = p.dram_blackout_us + pad_us;

	for i = 0;  i < p.display_cfg.num_streams;  i++ { {
		line_time_us = ((f64)p.display_cfg.stream_descriptors[i].timing.h_total * 1000 / p.display_cfg.stream_descriptors[i].timing.pixel_clock_khz);
		p.svp0_dst_lines[i] = (u32)math_ceil(svp0_time_us / line_time_us);
		p.svp1_dst_lines[i] = (u32)math_ceil(svp1_time_us / line_time_us);
		p.svp_req_limit[i] = (u32)math_ceil((pad_us + max_swath_time_all_planes_us) / line_time_us);
	}
	dml_log_verbose!("DML::%s: svp0_time_us = %f\n", __func__, svp0_time_us);
	dml_log_verbose!("DML::%s: svp1_time_us = %f\n", __func__, svp1_time_us);
	dml_log_verbose!("DML::%s: max_swath_time_all_planes_us = %f\n", __func__, max_swath_time_all_planes_us);
	dml_log_verbose!("DML::%s: pad_us = %f\n", __func__, pad_us);
}

plane_params {
    u32 viewport_start;
    u32 viewport_size;
    u32 swath_height;
    f64      prefetch_hdl_delta;
    f64      recout_hdl_delta;
    f64      vratio;
    u32 vinit;
};

/**
 * *****************************************************************************************************************************
 * get_plane_params: Get plane related params for chroma vs. luma depending on the chroma flag
 * *****************************************************************************************************************************
 */
static unsafe fn get_plane_params(
    *const dml2_core_calcs_calculate_alternate_paramsp,
    u32 plane_idx,
    bool chroma,
    plane_params *out)
{
	*const dml2_plane_parametersplane = &p.display_cfg.plane_descriptors[plane_idx];
	bool vertical_access = p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_90 || p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_270;

	if (!chroma) {
		out.viewport_start = vertical_access ? plane.composition.viewport.plane0.x_start : plane.composition.viewport.plane0.y_start;
		out.viewport_size = vertical_access ? plane.composition.viewport.plane0.width : plane.composition.viewport.plane0.height;
		out.swath_height = p.SwathHeightY[plane_idx];
		out.prefetch_hdl_delta = p.prefetch_hdl_delta[plane_idx];
		out.recout_hdl_delta   = p.recout_hdl_delta[plane_idx];
		out.vratio = p.display_cfg.plane_descriptors[plane_idx].composition.scaler_info.plane0.v_ratio;
		out.vinit  = p.VInitPrefillY[plane_idx];
	} else {
		out.viewport_start = vertical_access ? plane.composition.viewport.plane1.x_start : plane.composition.viewport.plane1.y_start;
		out.viewport_size = vertical_access ? plane.composition.viewport.plane1.width : plane.composition.viewport.plane1.height;
		out.swath_height = p.SwathHeightC[plane_idx];
		out.prefetch_hdl_delta = p.prefetch_hdl_delta_c[plane_idx];
		out.recout_hdl_delta   = p.recout_hdl_delta_c[plane_idx];
		out.vratio = p.display_cfg.plane_descriptors[plane_idx].composition.scaler_info.plane1.v_ratio;
		out.vinit  = p.VInitPrefillC[plane_idx];
	}
}

/**
 * *****************************************************************************************************************************
 * compute_pre_rec_first_hdl: Computes the first hdl position of pre and rec swath given the input params
 * *****************************************************************************************************************************
 */
static unsafe fn compute_pre_rec_first_hdl(
	*const dml2_core_calcs_calculate_alternate_paramsp,
	bool chroma,
	u32 stream_idx,
	u32 plane_idx,
    f64 *pre_first_hdl_out,
    f64 *rec_first_hdl_out)
{
	u64 vtotal = p.display_cfg.stream_descriptors[stream_idx].timing.v_total;
	u32 vblank_end = p.display_cfg.stream_descriptors[stream_idx].timing.v_blank_end;
    bool access_direction = (p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_90 && !p.display_cfg.plane_descriptors[plane_idx].composition.mirrored) ||
		(p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_270 && p.display_cfg.plane_descriptors[plane_idx].composition.mirrored) ||
		(p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_180);
    f64 dst_y_prefetch = p.dst_y_prefetch[plane_idx];
	f64 dst_y_per_vm_vblank = p.dst_y_per_vm_vblank[plane_idx];
	f64 dst_y_per_row_vblank = p.dst_y_per_row_vblank[plane_idx];
    u32 dst_y_after_scaler = p.DSTYAfterScaler[plane_idx];
    int prefetch_end_line = (vblank_end - dst_y_after_scaler) % vtotal;
    f64 prefetch_start_line = (prefetch_end_line > dst_y_prefetch) ? (prefetch_end_line - dst_y_prefetch) : (prefetch_end_line - dst_y_prefetch + vtotal);
    f64 pre_first_hdl, rec_first_hdl;
    plane_params in = { 0 };

    get_plane_params(p, plane_idx, chroma, &in);

    pre_first_hdl = prefetch_start_line + dst_y_per_vm_vblank + 2.0 * dst_y_per_row_vblank + in.prefetch_hdl_delta;
    rec_first_hdl = access_direction ?
			(in.viewport_start + in.viewport_size - in.vinit - in.swath_height - math_floor2(in.viewport_start + in.viewport_size - in.vinit, in.swath_height)) / in.vratio + (vblank_end - dst_y_after_scaler + in.recout_hdl_delta) :
			(math_floor2(in.viewport_start + in.vinit - 1, in.swath_height) - in.viewport_start - in.vinit) / in.vratio + (vblank_end - dst_y_after_scaler + in.recout_hdl_delta);

    *pre_first_hdl_out = pre_first_hdl;
    *rec_first_hdl_out = rec_first_hdl;
}

/**
 * ************************************************************************************************************************************
 * calculate_copy_swaths: Calculates a tight upper bound for the swaths required for copy given swath params and svp0 + svp1 dst lines
 *
 * Given the first hdl position of pre and rec swaths, and the delta in dst lines between the hdls, this function calculates the
 * number of swaths to be copied in the worst case given svp0 and svp1 dst lines. The upper bound is "tight" because this function
 * assumes svp0 and svp1 cannot both overlap with prefetch (prefetch potentially has the most amount of swaths per dst line).
 *
 * ************************************************************************************************************************************
 */
unsafe fn calculate_copy_swaths(f64 pre_first_hdl,
		f64 rec_first_hdl,
		f64 pre_hdl_delta,
		f64 rec_hdl_delta,
		u32 prefetch_swaths,
		u32 total_swaths,
		u32 svp0_dst_lines,
		u32 svp1_dst_lines,
		u32 vtotal)
{
	u32 svp_dst_lines = svp0_dst_lines + svp1_dst_lines;
	f64 prefetch_dst_lines = (prefetch_swaths - 1) * pre_hdl_delta + 1;
	f64 lines_between_pre_rec_first_hdl = pre_first_hdl < rec_first_hdl ? rec_first_hdl - pre_first_hdl : rec_first_hdl - pre_first_hdl + vtotal;
	f64 lines_for_rec_swaths = svp_dst_lines - lines_between_pre_rec_first_hdl;
	u32 num_swaths;

	if (lines_for_rec_swaths > 0)
		num_swaths = prefetch_swaths + (u32)math_ceil((lines_for_rec_swaths + rec_hdl_delta - 1) / rec_hdl_delta);
	else
		num_swaths = svp_dst_lines > prefetch_dst_lines ? prefetch_swaths + 1 : (u32)math_ceil((svp_dst_lines + pre_hdl_delta - 1) / pre_hdl_delta);

	if (num_swaths > total_swaths)
		num_swaths = total_swaths;

	return num_swaths;
}

 /**
  * *******************************************************************************************************************************************************
  * calculate_max_mem_size_per_plane_per_dpp: Calculate (loose) upper bound for total number of bytes reserved in memory for the copy given number of swaths
  *
  * - The copy width / height is the min of the vp_width/height and maximum number of pixels that can fit across an ODM slice
  *     - This is to account for recout positions that cross the ODM seam but are "mostly" within the same ODM slice
  * - For mem width assume an extra block width and tile width is required (the memory reserved must take into account pitch which must be tiled aligned)
  * - For mem height assumes two extra block heights are required
  *
  * *******************************************************************************************************************************************************
  */
unsafe fn calculate_max_mem_size_per_plane_per_dpp(*const dml2_core_calcs_calculate_alternate_paramsp, u32 plane_idx, u32 copy_swaths, bool chroma)
{
	bool vertical_access = p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_90 || p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_270;
	u32 h_active = p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[plane_idx].stream_index].timing.h_active;
	f64 h_ratio = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.scaler_info.plane1.h_ratio : p.display_cfg.plane_descriptors[plane_idx].composition.scaler_info.plane0.h_ratio;
	u32 copy_src_lines = copy_swaths * (chroma ? p.SwathHeightC[plane_idx] : p.SwathHeightY[plane_idx]);
	u32 vp_width = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane1.width : p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane0.width;
	u32 vp_height = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane1.height : p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane0.height;
	u32 block256_width = chroma ? p.Read256BlockWidthC[plane_idx] : p.Read256BlockWidthY[plane_idx];
	u32 block256_height = chroma ? p.Read256BlockHeightC[plane_idx] : p.Read256BlockHeightY[plane_idx];
	u32 tile_width = chroma ? p.MacroTileWidthC[plane_idx] : p.MacroTileWidthY[plane_idx];
	u32 byte_per_pixel = chroma ? p.BytePerPixelC[plane_idx] : p.BytePerPixelY[plane_idx];
	u32 mem_width;
	u32 mem_height;
	u32 odm_combine_factor;
	f64 odm_slice_pixels;

	if (p.ODMMode[plane_idx] == dml2_odm_mode_combine_4to1)
		odm_combine_factor = 4;
	else if (p.ODMMode[plane_idx] == dml2_odm_mode_combine_3to1)
		odm_combine_factor = 3;
	else if (p.ODMMode[plane_idx] == dml2_odm_mode_combine_2to1)
		odm_combine_factor = 2;
	else
		odm_combine_factor = 1;

	odm_slice_pixels = (f64)h_active / odm_combine_factor * h_ratio + (odm_combine_factor == 3 ? 2 : 0);
	mem_width = (vertical_access ? copy_src_lines : (u32)math_ceil(math_min2(odm_slice_pixels, vp_width))) + block256_width + tile_width;
	mem_height = (vertical_access ? (u32)math_ceil(math_min2(odm_slice_pixels, vp_height)) : copy_src_lines) + 2 * block256_height;

	return (u32)math_ceil2(mem_width * mem_height * byte_per_pixel, 256);
}

 /**
  * ****************************************************************************************************************************************
  * calculate_ub_copy_size_per_plane_per_dpp: Calculate tight upper bound for total number of bytes required for the copy given number of swaths.
  *
  * - This function is intended to be used to calculate the total copy time, since we want to reduce the copy time upper bound as much as possible
  * - For copy width assume two extra block widths (the copy itself does not need to be tiled aligned, only block aligned, so we don't add an extra
  *   tile to the copy width
  * - For copy height assumes two extra block heights are required
  *
  * Note: This function could be optimized further (i.e., an even tighter upper bound) if we take into account
  *       vp_x_start and vp_y_start positions which will tell us if the start and end positions of the copy are already
  *       blocked aligned (then we would not need to add the extra block width/height).
  *
  * ****************************************************************************************************************************************
  */
unsafe fn calculate_ub_copy_size_per_plane_per_dpp(*const dml2_core_calcs_calculate_alternate_paramsp, u32 plane_idx, u32 copy_swaths, bool chroma)
{
	bool vertical_access = p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_90 || p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_270;
	u32 copy_src_lines = copy_swaths * (chroma ? p.SwathHeightC[plane_idx] : p.SwathHeightY[plane_idx]);
	u32 vp_width = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane1.width : p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane0.width;
	u32 vp_height = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane1.height : p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane0.height;
	u32 block256_width = chroma ? p.Read256BlockWidthC[plane_idx] : p.Read256BlockWidthY[plane_idx];
	u32 block256_height = chroma ? p.Read256BlockHeightC[plane_idx] : p.Read256BlockHeightY[plane_idx];
	u32 copy_width = (vertical_access ? copy_src_lines : vp_width / p.NoOfDPP[plane_idx]) + 2 * block256_width;
	u32 copy_height = (vertical_access ? vp_height / p.NoOfDPP[plane_idx] : copy_src_lines) + 2 * block256_height;
	u32 byte_per_pixel = chroma ? p.BytePerPixelC[plane_idx] : p.BytePerPixelY[plane_idx];

	return (u32)math_ceil2(copy_width * copy_height * byte_per_pixel, 256);
}

/**
 * *****************************************************************************************************************************
 * calculate_alt_copy_time_us: Calculates a tight upper bound for the copy time over all planes and streams
 *
 * This function calculates a tight upper bound for the copy time over all planes and streams. This upper bound is "tight"
 * because it does the calculation assuming SVP0 and SVP1 cannot both overlap with prefetch (which potentially has the "most"
 * amount of bytes to copy per dst line).
 *
 * This function uses the exact NoOfDPP as calculated by DML (i.e., it does not use an input num_dpp param). This is because we
 * want the copy time calculation to be precise in order to minimize FW latency / overhead.
 *
 * *****************************************************************************************************************************
 */
unsafe fn calculate_alt_copy_time_us(*const dml2_core_calcs_calculate_alternate_paramsp)
{
    u32 i, j;
    f64 pre_first_hdl = 0.0, rec_first_hdl = 0.0;
    f64 pre_first_hdl_c = 0.0, rec_first_hdl_c = 0.0;
	f64 rec_hdl_delta, rec_hdl_delta_c;
	f64 pre_hdl_delta, pre_hdl_delta_c;
	u32 copy_swaths, copy_swaths_c;
	u32 vtotal;
	u32 copy_size_bytes = 0;

    for i = 0;  i < p.display_cfg.num_streams;  i++ { {
		vtotal = p.display_cfg.stream_descriptors[i].timing.v_total;
		for j = 0;  j < p.display_cfg.num_planes;  j++ { {
			if (p.display_cfg.plane_descriptors[j].stream_index != i)
				continue;
			compute_pre_rec_first_hdl(p, false, i, j, &pre_first_hdl, &rec_first_hdl);
			rec_hdl_delta = p.recout_hdl_delta[j];
			pre_hdl_delta = p.prefetch_hdl_delta[j];
			copy_swaths = calculate_copy_swaths(pre_first_hdl, rec_first_hdl, pre_hdl_delta, rec_hdl_delta, p.prefetch_swaths[j], p.total_swaths[j], p.svp0_dst_lines[i], p.svp1_dst_lines[i], vtotal);
			copy_size_bytes += calculate_ub_copy_size_per_plane_per_dpp(p, j, copy_swaths, false) * p.NoOfDPP[j];
			if (p.BytePerPixelInDETC[j] > 0) {
				compute_pre_rec_first_hdl(p, true, i, j, &pre_first_hdl_c, &rec_first_hdl_c);
				rec_hdl_delta_c = p.recout_hdl_delta_c[j];
				pre_hdl_delta_c = p.prefetch_hdl_delta_c[j];
				copy_swaths_c = calculate_copy_swaths(pre_first_hdl_c, rec_first_hdl_c, pre_hdl_delta_c, rec_hdl_delta_c, p.prefetch_swaths_c[j], p.total_swaths_c[j], p.svp0_dst_lines[i], p.svp1_dst_lines[i], vtotal);
				copy_size_bytes += calculate_ub_copy_size_per_plane_per_dpp(p, j, copy_swaths_c, false) * p.NoOfDPP[j];
			}
		}
    }
    return (u32)math_ceil((f64)copy_size_bytes * 1000 / *p.lsdma_bw_req_for_alt_kbps);
}

/**
 * *****************************************************************************************************************************
 * calculate_ub_copy_size_per_plane_per_dpp_per_svp: Calculates the upper bound copy size in bytes for a given plane and svp_dst_lines
 *
 * @input: p - alternate related params (input only)
 *         svp_dst_lines - number of lines (in dst space) for the svp (one of svp0 or svp1 dst lines)
 *         plane_idx - plane index to calculate for
 *         chroma - flag to indicate if chroma or luma plane
 *
 * TODO: The check for total copy size versus total alt-channel aperture size can be moved directly into mode support (from
 *       optimize / admissibility check layer)
 *
 * Note 2: This function calculates a loose upper bound for the size required for the copy. The reason for this is because each
 *         SVP aperture needs enough space to hold the worst case copy size (i.e., prefetch + some recout swaths), so we calculate
 *         such that any SVP needs to consider the prefetch swaths in the size required. But in actuality only one of SVP0 or SVP1
 *         would have prefetch swaths required to be copied since there cannot be the case where both SVPs overlap with prefetch.
 *         Therefore the result of this function should not be used to calculate the total copy time required (as it would be too long).
 *
 * *****************************************************************************************************************************
 */
unsafe fn calculate_ub_copy_size_per_plane_per_dpp_per_svp(*const dml2_core_calcs_calculate_alternate_paramsp, u32 svp_dst_lines, u32 plane_idx, bool chroma)
{
	u32 svp_lines_for_va;
	u32 copy_swaths;
	u32 copy_size = 0;

	if (chroma && p.BytePerPixelInDETC[plane_idx] > 0) {
		/* chomra plane */
		svp_lines_for_va = svp_dst_lines - (u32)((p.prefetch_swaths_c[plane_idx] - 1) * p.prefetch_hdl_delta_c[plane_idx]);
		copy_swaths = p.prefetch_swaths_c[plane_idx] + (u32)math_ceil(svp_lines_for_va / p.recout_hdl_delta_c[plane_idx]);
		if (copy_swaths > p.total_swaths_c[plane_idx])
			copy_swaths = p.total_swaths_c[plane_idx];
		copy_size = calculate_max_mem_size_per_plane_per_dpp(p, plane_idx, copy_swaths, chroma);
	} else if (!chroma) {
		/* luma plane */
		svp_lines_for_va = svp_dst_lines - (u32)((p.prefetch_swaths[plane_idx] - 1) * p.prefetch_hdl_delta[plane_idx]);
		copy_swaths = p.prefetch_swaths[plane_idx] + (u32)math_ceil(svp_lines_for_va / p.recout_hdl_delta[plane_idx]);
		if (copy_swaths > p.total_swaths[plane_idx])
			copy_swaths = p.total_swaths[plane_idx];
		copy_size = calculate_max_mem_size_per_plane_per_dpp(p, plane_idx, copy_swaths, chroma);
	}

	return copy_size;
}

swath_params {
    u32 prefetch_swaths;
    u32 total_swaths;
    f64       recout_hdl_delta;
    f64       prefetch_hdl_delta;
};

/**
 * ***********************************************************************************************************
 * calculate_swath_params: Function that calculates swath related params for alt-channel and returns the values
 *
 * This function calculates:
 * - number of prefetch swaths
 * - number of total swaths
 * - recout_hdl_delta (number of dst lines between hdls in recout)
 * - prefetch_hdl_delta (number of dst lines between hdls in prefetch)
 *
 * And returns it to the out parameter.
 *
 * ***********************************************************************************************************
 */
static unsafe fn calculate_swath_params(*const dml2_core_calcs_calculate_alternate_paramsp, u32 plane_idx, bool chroma, swath_params *out)
{
	bool vertical_access = p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_90 || p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_270;
	bool access_direction = (p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_90 && !p.display_cfg.plane_descriptors[plane_idx].composition.mirrored) ||
						(p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_270 && p.display_cfg.plane_descriptors[plane_idx].composition.mirrored) ||
						(p.display_cfg.plane_descriptors[plane_idx].composition.rotation_angle == dml2_rotation_180);
	u32 vp_x_start = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane1.x_start : p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane0.x_start;
	u32 vp_y_start = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane1.y_start : p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane0.y_start;
	u32 vp_height = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane1.height : p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane0.height;
	u32 vp_width = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane1.width : p.display_cfg.plane_descriptors[plane_idx].composition.viewport.plane0.width;
	f64 vratio = chroma ? p.display_cfg.plane_descriptors[plane_idx].composition.scaler_info.plane1.v_ratio : p.display_cfg.plane_descriptors[plane_idx].composition.scaler_info.plane0.v_ratio;
	f64 vratio_pre = chroma ? p.VRatioPrefetchC[plane_idx] : p.VRatioPrefetchY[plane_idx];
	u32 swath_height = chroma ? p.SwathHeightC[plane_idx] : p.SwathHeightY[plane_idx];
	u32 vinit = chroma ? p.VInitPrefillC[plane_idx] : p.VInitPrefillY[plane_idx];
	u32 viewport_start = vertical_access ? vp_x_start : vp_y_start;
	u32 viewport_size = vertical_access ? vp_width : vp_height;
	u32 src_y_last_pref_sw_algn, src_y_first_sw_algn, src_y_last_va_sw_algn;

	if (access_direction) {
		src_y_first_sw_algn = (u32)math_floor2(viewport_start + viewport_size - 1, swath_height);
		src_y_last_pref_sw_algn = (u32)math_floor2(viewport_start + viewport_size - vinit, swath_height);
		src_y_last_va_sw_algn = (u32)math_floor2(viewport_start, swath_height);
	} else {
		src_y_first_sw_algn = (u32)math_floor2(viewport_start, swath_height);
		src_y_last_pref_sw_algn = (u32)math_floor2(viewport_start + vinit - 1, swath_height);
		src_y_last_va_sw_algn = (u32)math_floor2(viewport_start + viewport_size - 1, swath_height);
	}
	out.prefetch_swaths = (access_direction ? src_y_first_sw_algn - src_y_last_pref_sw_algn : src_y_last_pref_sw_algn - src_y_first_sw_algn) / swath_height + 1;
	out.total_swaths = (access_direction ? src_y_first_sw_algn - src_y_last_va_sw_algn : src_y_last_va_sw_algn - src_y_first_sw_algn) / swath_height + 1;
	out.recout_hdl_delta = (f64)swath_height / vratio;
	out.prefetch_hdl_delta = (f64)swath_height / vratio_pre;
}

unsafe fn calc_svp_size_64kb_aligned(u32 total_size_bytes)
{
	return ((total_size_bytes + 0xFFFF) >> 16) << 16; // Round up to nearest 64KB boundary
}

unsafe fn dcn6_calculate_alternate_params(dml2_core_calcs_calculate_alternate_params *p)
{
	u32 i, j, k;
	f64 line_time_us = 0, prefetch_time_us, max_prefetch_time_us = 0;
	u32 svp_max_bytes[2];
	u32 svp_max_bytes_per_dpp[2];
	u32 svp_dst_lines[2];
	f64 copy_time_us;
	f64 svp_req_lim_us = 0;
	u32 fw_delay;
	swath_params swath_params;

	*p.svp0_max_bytes = 0;
	*p.svp1_max_bytes = 0;
	svp_max_bytes[0] = 0;
	svp_max_bytes[1] = 0;
	/* This initial loop calculates a few params that are used for calculations / assignments in later parts of the function:
	 * - max_prefetch_time_us
	 */
	for i = 0;  i < p.display_cfg.num_streams;  i++ { {
		line_time_us = ((f64)p.display_cfg.stream_descriptors[i].timing.h_total * 1000 / p.display_cfg.stream_descriptors[i].timing.pixel_clock_khz);
		if (p.svp_req_limit[i] * line_time_us > svp_req_lim_us)
			svp_req_lim_us = p.svp_req_limit[i] * line_time_us;
		for j = 0;  j < p.display_cfg.num_planes;  j++ { {
			prefetch_time_us = line_time_us * p.dst_y_prefetch[j];
			if (prefetch_time_us > max_prefetch_time_us)
				max_prefetch_time_us = prefetch_time_us;
		}
	}

	for i = 0;  i < p.display_cfg.num_streams;  i++ { {
		svp_dst_lines[0] = p.svp0_dst_lines[i];
		svp_dst_lines[1] = p.svp1_dst_lines[i];
		line_time_us = ((f64)p.display_cfg.stream_descriptors[i].timing.h_total * 1000 / p.display_cfg.stream_descriptors[i].timing.pixel_clock_khz);
		p.max_prefetch_in_lines[i] = (u32)math_ceil(max_prefetch_time_us / line_time_us);
		for j = 0;  j < p.display_cfg.num_planes;  j++ { {
			if (p.display_cfg.plane_descriptors[j].stream_index == i) {
				calculate_swath_params(p, j, false, &swath_params);
				p.prefetch_swaths[j] = swath_params.prefetch_swaths;
				p.total_swaths[j] = swath_params.total_swaths;
				p.recout_hdl_delta[j] = swath_params.recout_hdl_delta;
				p.prefetch_hdl_delta[j] = swath_params.prefetch_hdl_delta;
				for k = 0;  k < 2;  k++ { {
					svp_max_bytes_per_dpp[k] = calculate_ub_copy_size_per_plane_per_dpp_per_svp(p, svp_dst_lines[k], j, false);
					svp_max_bytes[k] += calc_svp_size_64kb_aligned(svp_max_bytes_per_dpp[k]) * p.NoOfDPP[j];
				}
				p.svp0_max_bytes_per_dpp[j] = svp_max_bytes_per_dpp[0];
				p.svp1_max_bytes_per_dpp[j] = svp_max_bytes_per_dpp[1];

				if (p.BytePerPixelInDETC[j] > 0) {
					calculate_swath_params(p, j, true, &swath_params);
					p.prefetch_swaths_c[j] = swath_params.prefetch_swaths;
					p.total_swaths_c[j] = swath_params.total_swaths;
					p.recout_hdl_delta_c[j] = swath_params.recout_hdl_delta;
					p.prefetch_hdl_delta_c[j] = swath_params.prefetch_hdl_delta;

					for k = 0;  k < 2;  k++ { {
						svp_max_bytes_per_dpp[k] = calculate_ub_copy_size_per_plane_per_dpp_per_svp(p, svp_dst_lines[k], j, true);
						svp_max_bytes[k] += calc_svp_size_64kb_aligned(svp_max_bytes_per_dpp[k]) * p.NoOfDPP[j];
					}
					p.svp0_max_bytes_per_dpp_c[j] = svp_max_bytes_per_dpp[0];
					p.svp1_max_bytes_per_dpp_c[j] = svp_max_bytes_per_dpp[1];
				} else {
					p.svp0_max_bytes_per_dpp_c[j] = 0;
					p.svp1_max_bytes_per_dpp_c[j] = 0;
				}
			}
		}
	}

	*p.svp0_max_bytes = svp_max_bytes[0];
	*p.svp1_max_bytes = svp_max_bytes[1];
	*p.lsdma_bw_req_for_alt_kbps = p.dcn_non_urgent_bandwidth_kbps;
	copy_time_us = p.display_cfg.overrides.hw.force_alt_chan_copy_time.enable ? p.display_cfg.overrides.hw.force_alt_chan_copy_time.copy_time_us : calculate_alt_copy_time_us(p);
	fw_delay =  p.display_cfg.overrides.hw.force_alt_chan_fw_delay.enable ? p.display_cfg.overrides.hw.force_alt_chan_fw_delay.fw_delay_us : p.alt_chan_fw_delay_us;
	for i = 0;  i < p.display_cfg.num_streams;  i++ { {
		line_time_us = ((f64)p.display_cfg.stream_descriptors[i].timing.h_total * 1000 / p.display_cfg.stream_descriptors[i].timing.pixel_clock_khz);
		/* If copy_time is very short then clamp nom_req_limit to be equal to svp_req_limit. This is to prevent underflow
		 * because a short nom_req_limit prevents DCN from being able to request ahead. */
		p.nom_req_limit_alt[i] = (u32)math_max2(p.svp_req_limit[i], math_ceil((copy_time_us) / line_time_us));
		p.min_lead_dst_lines[i] = (u32)math_ceil((copy_time_us + fw_delay + math_max2(svp_req_lim_us, max_prefetch_time_us)) / line_time_us);
	}
}

unsafe fn dcn6_calculate_flip_schedule(
	dml2_core_internal_scratch *s,
	bool iflip_enable,
	bool ihostvm_enable,
	bool iffbmm_enable,
	f64 HostVMInefficiencyFactor,
	f64 Tvm_trips_flip,
	f64 Tr0_trips_flip,
	f64 Tvm_trips_flip_rounded,
	f64 Tr0_trips_flip_rounded,
	bool GPUVMEnable,
	f64 vm_bytes, // vm_bytes
	f64 DPTEBytesPerRow, // dpte_row_bytes
	enum dml2_source_format_class SourcePixelFormat,
	f64 LineTime,
	f64 VRatio,
	f64 VRatioChroma,
	f64 Tno_bw_flip,
	u32 dpte_row_height,
	u32 dpte_row_height_chroma,
	u32 max_flip_time_us,
	u32 max_flip_time_lines,
	u32 meta_row_height,
	u32 meta_row_height_chroma,

	// Output
	f64 *dst_y_per_vm_flip,
	f64 *dst_y_per_row_flip,
	f64 *final_flip_bw,
	bool *ImmediateFlipSupportedForPipe)
{
	dml2_core_shared_CalculateFlipSchedule_locals *l = &s.CalculateFlipSchedule_locals;

	l.dual_plane = dml2_core_utils_is_420(SourcePixelFormat) || dml2_core_utils_is_422_planar(SourcePixelFormat) || SourcePixelFormat == dml2_rgbe_alpha;
	l.dpte_row_bytes = DPTEBytesPerRow;

	dml_log_verbose!("DML::%s: GPUVMEnable = %u\n", __func__, GPUVMEnable);
	dml_log_verbose!("DML::%s: ip.max_flip_time_us = %d\n", __func__, max_flip_time_us);
	dml_log_verbose!("DML::%s: ip.max_flip_time_lines = %d\n", __func__, max_flip_time_lines);
	dml_log_verbose!("DML::%s: iflip_enable = %u\n", __func__, iflip_enable);
	dml_log_verbose!("DML::%s: HostVMInefficiencyFactor = %f\n", __func__, HostVMInefficiencyFactor);
	dml_log_verbose!("DML::%s: LineTime = %f\n", __func__, LineTime);
	dml_log_verbose!("DML::%s: Tno_bw_flip = %f\n", __func__, Tno_bw_flip);
	dml_log_verbose!("DML::%s: Tvm_trips_flip = %f\n", __func__, Tvm_trips_flip);
	dml_log_verbose!("DML::%s: Tr0_trips_flip = %f\n", __func__, Tr0_trips_flip);
	dml_log_verbose!("DML::%s: Tvm_trips_flip_rounded = %f\n", __func__, Tvm_trips_flip_rounded);
	dml_log_verbose!("DML::%s: Tr0_trips_flip_rounded = %f\n", __func__, Tr0_trips_flip_rounded);
	dml_log_verbose!("DML::%s: vm_bytes = %f\n", __func__, vm_bytes);
	dml_log_verbose!("DML::%s: DPTEBytesPerRow = %f\n", __func__, DPTEBytesPerRow);
	dml_log_verbose!("DML::%s: dpte_row_bytes = %f\n", __func__, l.dpte_row_bytes);
	dml_log_verbose!("DML::%s: dpte_row_height = %d\n", __func__, dpte_row_height);
	dml_log_verbose!("DML::%s: meta_row_height = %d\n", __func__, meta_row_height);
	dml_log_verbose!("DML::%s: VRatio = %f\n", __func__, VRatio);

	bool flip_enable = iflip_enable || (GPUVMEnable && (ihostvm_enable || iffbmm_enable));

	if (GPUVMEnable) {
		if (l.dual_plane) {
			l.min_row_height = dpte_row_height;
			l.min_row_height_chroma = dpte_row_height_chroma;
			l.min_row_time = math_min2(l.min_row_height * LineTime / VRatio, l.min_row_height_chroma * LineTime / VRatioChroma);
		} else {
			l.min_row_height = dpte_row_height;
			l.min_row_time = l.min_row_height * LineTime / VRatio;
		}
		dml_log_verbose!("DML::%s: min_row_time = %f\n", __func__, l.min_row_time);
		debug_assert!(l.min_row_time > 0);

		// For mode check, calculation the flip bw requirement with worst case flip time
		l.max_flip_time = math_min2(math_min2(l.min_row_time, (f64)max_flip_time_lines * LineTime / VRatio),
			math_max2(Tvm_trips_flip_rounded + 2 * Tr0_trips_flip_rounded, (f64)max_flip_time_us));

		//The lower bound on flip bandwidth
		// Note: The get_urgent_bandwidth_required already consider dpte_row_bw and meta_row_bw in bandwidth calculation, so leave final_flip_bw = 0 if iflip not required
		l.lb_flip_bw = 0;

		if (flip_enable) {
			l.hvm_scaled_vm_bytes = vm_bytes * HostVMInefficiencyFactor;
			l.num_rows = 2;
			l.hvm_scaled_row_bytes = l.num_rows * l.dpte_row_bytes * HostVMInefficiencyFactor;
			l.hvm_scaled_vm_row_bytes = l.hvm_scaled_vm_bytes + l.hvm_scaled_row_bytes;
			l.lb_flip_bw = math_max3(
				l.hvm_scaled_vm_row_bytes / (l.max_flip_time - Tno_bw_flip),
				l.hvm_scaled_vm_bytes / (l.max_flip_time - Tno_bw_flip - 2 * Tr0_trips_flip_rounded),
				l.hvm_scaled_row_bytes / (l.max_flip_time - Tvm_trips_flip_rounded));
			dml_log_verbose!("DML::%s: max_flip_time = %f\n", __func__, l.max_flip_time);
			dml_log_verbose!("DML::%s: total vm bytes (hvm ineff scaled) = %f\n", __func__, l.hvm_scaled_vm_bytes);
			dml_log_verbose!("DML::%s: total row bytes (%f row, hvm ineff scaled) = %f\n", __func__, l.num_rows, l.hvm_scaled_row_bytes);
			dml_log_verbose!("DML::%s: total vm+row bytes (hvm ineff scaled) = %f\n", __func__, l.hvm_scaled_vm_row_bytes);
			dml_log_verbose!("DML::%s: lb_flip_bw for vm and row = %f\n", __func__, l.hvm_scaled_vm_row_bytes / (l.max_flip_time - Tno_bw_flip));
			dml_log_verbose!("DML::%s: lb_flip_bw for vm = %f\n", __func__, l.hvm_scaled_vm_bytes / (l.max_flip_time - Tno_bw_flip - 2 * Tr0_trips_flip_rounded));
			dml_log_verbose!("DML::%s: lb_flip_bw for row = %f\n", __func__, l.hvm_scaled_row_bytes / (l.max_flip_time - Tvm_trips_flip_rounded));

			if (l.lb_flip_bw > 0) {
				dml_log_verbose!("DML::%s: mode_support est Tvm_flip = %f (bw-based)\n", __func__, Tno_bw_flip + l.hvm_scaled_vm_bytes / l.lb_flip_bw);
				dml_log_verbose!("DML::%s: mode_support est Tr0_flip = %f (bw-based)\n", __func__, l.hvm_scaled_row_bytes / l.lb_flip_bw / l.num_rows);
				dml_log_verbose!("DML::%s: mode_support est dst_y_per_vm_flip = %f (bw-based)\n", __func__, Tno_bw_flip + l.hvm_scaled_vm_bytes / l.lb_flip_bw / LineTime);
				dml_log_verbose!("DML::%s: mode_support est dst_y_per_row_flip = %f (bw-based)\n", __func__, l.hvm_scaled_row_bytes / l.lb_flip_bw / LineTime / l.num_rows);
				dml_log_verbose!("DML::%s: Tvm_trips_flip_rounded + 2*Tr0_trips_flip_rounded = %f\n", __func__, (Tvm_trips_flip_rounded + 2 * Tr0_trips_flip_rounded));
			}
			l.lb_flip_bw = math_max3(l.lb_flip_bw,
				l.hvm_scaled_vm_bytes / (31 * LineTime) - Tno_bw_flip,
				l.dpte_row_bytes * HostVMInefficiencyFactor / (15 * LineTime));

			dml_log_verbose!("DML::%s: lb_flip_bw for vm reg limit = %f\n", __func__, l.hvm_scaled_vm_bytes / (31 * LineTime) - Tno_bw_flip);
		}

		*final_flip_bw = l.lb_flip_bw;

		if (flip_enable) {
			dml_log_verbose!("DML::%s: final_flip_bw = %f\n", __func__, *final_flip_bw);
			if (*final_flip_bw == 0) {
				l.Tvm_flip = 0;
				l.Tr0_flip = 0;
			} else {
				l.Tvm_flip = math_max3(Tvm_trips_flip,
					Tno_bw_flip + vm_bytes * HostVMInefficiencyFactor / *final_flip_bw,
					LineTime / 4.0);

				l.Tr0_flip = math_max3(Tr0_trips_flip,
					l.dpte_row_bytes * HostVMInefficiencyFactor / *final_flip_bw,
					LineTime / 4.0);
			}
			dml_log_verbose!("DML::%s: total vm bytes (hvm ineff scaled) = %f\n", __func__, vm_bytes * HostVMInefficiencyFactor);
			dml_log_verbose!("DML::%s: Tvm_flip = %f (bw-based), Tvm_trips_flip = %f (latency-based)\n", __func__, Tno_bw_flip + vm_bytes * HostVMInefficiencyFactor / l.ImmediateFlipBW, Tvm_trips_flip);
			*dst_y_per_vm_flip = math_ceil2(4.0 * (l.Tvm_flip / LineTime), 1.0) / 4.0;
			*dst_y_per_row_flip = math_ceil2(4.0 * (l.Tr0_flip / LineTime), 1.0) / 4.0;

			*final_flip_bw = math_max2(vm_bytes * HostVMInefficiencyFactor / (*dst_y_per_vm_flip * LineTime),
				l.dpte_row_bytes * HostVMInefficiencyFactor / (*dst_y_per_row_flip * LineTime));

			if (*dst_y_per_vm_flip >= 32 || *dst_y_per_row_flip >= 16 || l.Tvm_flip + 2 * l.Tr0_flip > l.min_row_time) {
				*ImmediateFlipSupportedForPipe = false;
			} else {
				*ImmediateFlipSupportedForPipe = flip_enable;
			}
		} else {
			l.Tvm_flip = 0;
			l.Tr0_flip = 0;
			*dst_y_per_vm_flip = 0;
			*dst_y_per_row_flip = 0;
			*final_flip_bw = 0;
			*ImmediateFlipSupportedForPipe = flip_enable;
		}
	} else {
		l.Tvm_flip = 0;
		l.Tr0_flip = 0;
		*dst_y_per_vm_flip = 0;
		*dst_y_per_row_flip = 0;
		*final_flip_bw = 0;
		*ImmediateFlipSupportedForPipe = flip_enable;
	}

	dml_log_verbose!("DML::%s: dst_y_per_vm_flip = %f (should be < 32)\n", __func__, *dst_y_per_vm_flip);
	dml_log_verbose!("DML::%s: dst_y_per_row_flip = %f (should be < 16)\n", __func__, *dst_y_per_row_flip);
	dml_log_verbose!("DML::%s: Tvm_flip = %f (final)\n", __func__, l.Tvm_flip);
	dml_log_verbose!("DML::%s: Tr0_flip = %f (final)\n", __func__, l.Tr0_flip);
	dml_log_verbose!("DML::%s: Tvm_flip + 2*Tr0_flip = %f (should be <= min_row_time=%f)\n", __func__, l.Tvm_flip + 2 * l.Tr0_flip, l.min_row_time);
	dml_log_verbose!("DML::%s: final_flip_bw = %f\n", __func__, *final_flip_bw);
	dml_log_verbose!("DML::%s: ImmediateFlipSupportedForPipe = %u\n", __func__, *ImmediateFlipSupportedForPipe);
}

static unsafe fn dcn6_rq_dlg_get_dlg_reg(
		dml2_core_internal_scratch *s,
		dml2_display_dlg_regs *disp_dlg_regs,
		dml2_display_ttu_regs *disp_ttu_regs,
		*const dml2_display_cfgdisplay_cfg,
		*const dml2_core_internal_display_mode_libmode_lib,
		const u32 pipe_idx,
		*const dml2_utm_soc_bbutm_soc_bb)
{
	dml2_core_shared_rq_dlg_get_dlg_reg_locals *l = &s.rq_dlg_get_dlg_reg_locals;

	memset(l, 0, core::mem::size_of::<dml2_core_shared_rq_dlg_get_dlg_reg_locals>());

	dml_log_verbose!("DML_DLG::%s: Calculation for pipe_idx=%d\n", __func__, pipe_idx);

	l.plane_idx = mode_lib.mp.pipe_plane[pipe_idx];
	l.stream_idx = display_cfg.plane_descriptors[l.plane_idx].stream_index;
	debug_assert!(l.plane_idx < DML2_MAX_PLANES);

	l.source_format = dml2_444_8;
	l.odm_mode = dml2_odm_mode_bypass;
	l.dual_plane = false;
	l.htotal = 0;
	l.hactive = 0;
	l.hblank_end = 0;
	l.vblank_end = 0;
	l.interlaced = false;
	l.pclk_freq_in_mhz = 0.0;
	l.refclk_freq_in_mhz = (display_cfg.overrides.hw.dlg_ref_clk_mhz > 0) ? (f64)display_cfg.overrides.hw.dlg_ref_clk_mhz : utm_soc_bb.dchub_refclk_mhz;
	l.ref_freq_to_pix_freq = 0.0;

	if (l.plane_idx < DML2_MAX_PLANES) {

		l.timing = &display_cfg.stream_descriptors[display_cfg.plane_descriptors[l.plane_idx].stream_index].timing;
		l.source_format = display_cfg.plane_descriptors[l.plane_idx].pixel_format;
		l.odm_mode = mode_lib.mp.ODMMode[l.plane_idx];

		l.dual_plane = dml2_core_utils_is_dual_plane(l.source_format);

		l.htotal = l.timing.h_total;
		l.hactive = l.timing.h_active;
		l.hblank_end = l.timing.h_blank_end;
		l.vblank_end = l.timing.v_blank_end;
		l.interlaced = l.timing.interlaced;
		l.pclk_freq_in_mhz = (f64)l.timing.pixel_clock_khz / 1000;
		l.ref_freq_to_pix_freq = l.refclk_freq_in_mhz / l.pclk_freq_in_mhz;

		dml_log_verbose!("DML_DLG::%s: plane_idx = %d\n", __func__, l.plane_idx);
		dml_log_verbose!("DML_DLG: %s: htotal = %d\n", __func__, l.htotal);
		dml_log_verbose!("DML_DLG: %s: refclk_freq_in_mhz = %3.2f\n", __func__, l.refclk_freq_in_mhz);
		dml_log_verbose!("DML_DLG: %s: dlg_ref_clk_mhz = %3.2f\n", __func__, display_cfg.overrides.hw.dlg_ref_clk_mhz);
		dml_log_verbose!("DML_DLG: %s: soc.refclk_mhz = %u\n", __func__, utm_soc_bb.dchub_refclk_mhz);
		dml_log_verbose!("DML_DLG: %s: pclk_freq_in_mhz = %3.2f\n", __func__, l.pclk_freq_in_mhz);
		dml_log_verbose!("DML_DLG: %s: ref_freq_to_pix_freq = %3.2f\n", __func__, l.ref_freq_to_pix_freq);
		dml_log_verbose!("DML_DLG: %s: interlaced = %d\n", __func__, l.interlaced);

		debug_assert!(l.refclk_freq_in_mhz != 0);
		debug_assert!(l.pclk_freq_in_mhz != 0);
		debug_assert!(l.ref_freq_to_pix_freq < 4.0);

		// Need to figure out which side of odm combine we're in
		// Assume the pipe instance under the same plane is in order

		if (l.odm_mode == dml2_odm_mode_bypass) {
			disp_dlg_regs.refcyc_h_blank_end = (u32)((f64)l.hblank_end * l.ref_freq_to_pix_freq);
		} else if (l.odm_mode == dml2_odm_mode_combine_2to1 || l.odm_mode == dml2_odm_mode_combine_3to1 || l.odm_mode == dml2_odm_mode_combine_4to1) {
			// find out how many pipe are in this plane
			l.num_active_pipes = mode_lib.mp.num_active_pipes;
			l.first_pipe_idx_in_plane = DML2_MAX_PLANES;
			l.pipe_idx_in_combine = 0; // pipe index within the plane
			l.odm_combine_factor = 2;

			if (l.odm_mode == dml2_odm_mode_combine_3to1)
				l.odm_combine_factor = 3;
			else if (l.odm_mode == dml2_odm_mode_combine_4to1)
				l.odm_combine_factor = 4;

			for u32 i = 0;  i < l.num_active_pipes;  i++ { {
				if (mode_lib.mp.pipe_plane[i] == l.plane_idx) {
					if (i < l.first_pipe_idx_in_plane) {
						l.first_pipe_idx_in_plane = i;
					}
				}
			}
			l.pipe_idx_in_combine = pipe_idx - l.first_pipe_idx_in_plane; // DML assumes the pipes in the same plane will have continuous indexing (i.e. plane 0 use pipe 0, 1, and plane 1 uses pipe 2, 3, etc.)

			disp_dlg_regs.refcyc_h_blank_end = (u32)(((f64)l.hblank_end + (f64)l.pipe_idx_in_combine * (f64)l.hactive / (f64)l.odm_combine_factor) * l.ref_freq_to_pix_freq);
			dml_log_verbose!("DML_DLG: %s: pipe_idx = %d\n", __func__, pipe_idx);
			dml_log_verbose!("DML_DLG: %s: first_pipe_idx_in_plane = %d\n", __func__, l.first_pipe_idx_in_plane);
			dml_log_verbose!("DML_DLG: %s: pipe_idx_in_combine = %d\n", __func__, l.pipe_idx_in_combine);
			dml_log_verbose!("DML_DLG: %s: odm_combine_factor = %d\n", __func__, l.odm_combine_factor);
		}
		dml_log_verbose!("DML_DLG: %s: refcyc_h_blank_end = %d\n", __func__, disp_dlg_regs.refcyc_h_blank_end);

		debug_assert!(disp_dlg_regs.refcyc_h_blank_end < (u32)math_pow(2, 13));

		disp_dlg_regs.ref_freq_to_pix_freq = (u32)(l.ref_freq_to_pix_freq * math_pow(2, 19));
		disp_dlg_regs.refcyc_per_htotal = (u32)(l.ref_freq_to_pix_freq * (f64)l.htotal * math_pow(2, 8));
		disp_dlg_regs.dlg_vblank_end = l.interlaced ? (l.vblank_end / 2) : l.vblank_end; // 15 bits

		l.min_ttu_vblank = mode_lib.mp.MinTTUVBlank[mode_lib.mp.pipe_plane[pipe_idx]];
		l.min_dst_y_next_start = (u32)(mode_lib.mp.MIN_DST_Y_NEXT_START[mode_lib.mp.pipe_plane[pipe_idx]]);

		dml_log_verbose!("DML_DLG: %s: min_ttu_vblank (us) = %3.2f\n", __func__, l.min_ttu_vblank);
		dml_log_verbose!("DML_DLG: %s: min_dst_y_next_start = %d\n", __func__, l.min_dst_y_next_start);
		dml_log_verbose!("DML_DLG: %s: ref_freq_to_pix_freq = %3.2f\n", __func__, l.ref_freq_to_pix_freq);

		l.vready_after_vcount0 = (u32)(mode_lib.mp.VREADY_AT_OR_AFTER_VSYNC[mode_lib.mp.pipe_plane[pipe_idx]]);
		disp_dlg_regs.vready_after_vcount0 = l.vready_after_vcount0;

		dml_log_verbose!("DML_DLG: %s: vready_after_vcount0 = %d\n", __func__, disp_dlg_regs.vready_after_vcount0);

		l.dst_x_after_scaler = (u32)(mode_lib.mp.DSTXAfterScaler[mode_lib.mp.pipe_plane[pipe_idx]]);
		l.dst_y_after_scaler = (u32)(mode_lib.mp.DSTYAfterScaler[mode_lib.mp.pipe_plane[pipe_idx]]);

		dml_log_verbose!("DML_DLG: %s: dst_x_after_scaler = %d\n", __func__, l.dst_x_after_scaler);
		dml_log_verbose!("DML_DLG: %s: dst_y_after_scaler = %d\n", __func__, l.dst_y_after_scaler);

		l.dst_y_prefetch = mode_lib.mp.dst_y_prefetch[mode_lib.mp.pipe_plane[pipe_idx]];
		l.dst_y_per_vm_vblank = mode_lib.mp.dst_y_per_vm_vblank[mode_lib.mp.pipe_plane[pipe_idx]];
		l.dst_y_per_row_vblank = mode_lib.mp.dst_y_per_row_vblank[mode_lib.mp.pipe_plane[pipe_idx]];
		l.dst_y_per_vm_flip = mode_lib.mp.dst_y_per_vm_flip[mode_lib.mp.pipe_plane[pipe_idx]];
		l.dst_y_per_row_flip = mode_lib.mp.dst_y_per_row_flip[mode_lib.mp.pipe_plane[pipe_idx]];

		dml_log_verbose!("DML_DLG: %s: dst_y_prefetch (after rnd) = %3.2f\n", __func__, l.dst_y_prefetch);
		dml_log_verbose!("DML_DLG: %s: dst_y_per_vm_flip = %3.2f\n", __func__, l.dst_y_per_vm_flip);
		dml_log_verbose!("DML_DLG: %s: dst_y_per_row_flip = %3.2f\n", __func__, l.dst_y_per_row_flip);
		dml_log_verbose!("DML_DLG: %s: dst_y_per_vm_vblank = %3.2f\n", __func__, l.dst_y_per_vm_vblank);
		dml_log_verbose!("DML_DLG: %s: dst_y_per_row_vblank = %3.2f\n", __func__, l.dst_y_per_row_vblank);

		if (l.dst_y_prefetch > 0 && l.dst_y_per_vm_vblank > 0 && l.dst_y_per_row_vblank > 0) {
			debug_assert!(l.dst_y_prefetch > (l.dst_y_per_vm_vblank + l.dst_y_per_row_vblank));
		}

		l.vratio_pre_l = mode_lib.mp.VRatioPrefetchY[mode_lib.mp.pipe_plane[pipe_idx]];
		l.vratio_pre_c = mode_lib.mp.VRatioPrefetchC[mode_lib.mp.pipe_plane[pipe_idx]];

		dml_log_verbose!("DML_DLG: %s: vratio_pre_l = %3.2f\n", __func__, l.vratio_pre_l);
		dml_log_verbose!("DML_DLG: %s: vratio_pre_c = %3.2f\n", __func__, l.vratio_pre_c);

		// Active
		l.refcyc_per_line_delivery_pre_l = mode_lib.mp.DisplayPipeLineDeliveryTimeLumaPrefetch[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_line_delivery_l = mode_lib.mp.DisplayPipeLineDeliveryTimeLuma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;

		dml_log_verbose!("DML_DLG: %s: refcyc_per_line_delivery_pre_l = %3.2f\n", __func__, l.refcyc_per_line_delivery_pre_l);
		dml_log_verbose!("DML_DLG: %s: refcyc_per_line_delivery_l = %3.2f\n", __func__, l.refcyc_per_line_delivery_l);

		l.refcyc_per_line_delivery_pre_c = 0.0;
		l.refcyc_per_line_delivery_c = 0.0;

		if (l.dual_plane) {
			l.refcyc_per_line_delivery_pre_c = mode_lib.mp.DisplayPipeLineDeliveryTimeChromaPrefetch[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
			l.refcyc_per_line_delivery_c = mode_lib.mp.DisplayPipeLineDeliveryTimeChroma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;

			dml_log_verbose!("DML_DLG: %s: refcyc_per_line_delivery_pre_c = %3.2f\n", __func__, l.refcyc_per_line_delivery_pre_c);
			dml_log_verbose!("DML_DLG: %s: refcyc_per_line_delivery_c = %3.2f\n", __func__, l.refcyc_per_line_delivery_c);
		}

		disp_dlg_regs.refcyc_per_vm_dmdata = (u32)(mode_lib.mp.Tdmdl_vm[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz);
		disp_dlg_regs.dmdata_dl_delta = (u32)(mode_lib.mp.Tdmdl[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz);

		l.refcyc_per_req_delivery_pre_l = mode_lib.mp.DisplayPipeRequestDeliveryTimeLumaPrefetch[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_req_delivery_l = mode_lib.mp.DisplayPipeRequestDeliveryTimeLuma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;

		dml_log_verbose!("DML_DLG: %s: refcyc_per_req_delivery_pre_l = %3.2f\n", __func__, l.refcyc_per_req_delivery_pre_l);
		dml_log_verbose!("DML_DLG: %s: refcyc_per_req_delivery_l = %3.2f\n", __func__, l.refcyc_per_req_delivery_l);

		l.refcyc_per_req_delivery_pre_c = 0.0;
		l.refcyc_per_req_delivery_c = 0.0;
		if (l.dual_plane) {
			l.refcyc_per_req_delivery_pre_c = mode_lib.mp.DisplayPipeRequestDeliveryTimeChromaPrefetch[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
			l.refcyc_per_req_delivery_c = mode_lib.mp.DisplayPipeRequestDeliveryTimeChroma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;

			dml_log_verbose!("DML_DLG: %s: refcyc_per_req_delivery_pre_c = %3.2f\n", __func__, l.refcyc_per_req_delivery_pre_c);
			dml_log_verbose!("DML_DLG: %s: refcyc_per_req_delivery_c = %3.2f\n", __func__, l.refcyc_per_req_delivery_c);
		}

		// TTU - Cursor
		debug_assert!(display_cfg.plane_descriptors[l.plane_idx].cursor.num_cursors <= 1);

		// Assign to register structures
		disp_dlg_regs.min_dst_y_next_start = (u32)((f64)l.min_dst_y_next_start * math_pow(2, 2));
		debug_assert!(disp_dlg_regs.min_dst_y_next_start < (u32)math_pow(2, 18));

		disp_dlg_regs.dst_y_after_scaler = l.dst_y_after_scaler; // in terms of line
		disp_dlg_regs.refcyc_x_after_scaler = (u32)((f64)l.dst_x_after_scaler * l.ref_freq_to_pix_freq); // in terms of refclk
		disp_dlg_regs.dst_y_prefetch = (u32)(l.dst_y_prefetch * math_pow(2, 2));
		disp_dlg_regs.dst_y_per_vm_vblank = (u32)(l.dst_y_per_vm_vblank * math_pow(2, 2));
		disp_dlg_regs.dst_y_per_row_vblank = (u32)(l.dst_y_per_row_vblank * math_pow(2, 2));
		disp_dlg_regs.dst_y_per_vm_flip = (u32)(l.dst_y_per_vm_flip * math_pow(2, 2));
		disp_dlg_regs.dst_y_per_row_flip = (u32)(l.dst_y_per_row_flip * math_pow(2, 2));

		disp_dlg_regs.vratio_prefetch = (u32)(l.vratio_pre_l * math_pow(2, 19));
		disp_dlg_regs.vratio_prefetch_c = (u32)(l.vratio_pre_c * math_pow(2, 19));

		dml_log_verbose!("DML_DLG: %s: disp_dlg_regs.dst_y_per_vm_vblank = 0x%x\n", __func__, disp_dlg_regs.dst_y_per_vm_vblank);
		dml_log_verbose!("DML_DLG: %s: disp_dlg_regs.dst_y_per_row_vblank = 0x%x\n", __func__, disp_dlg_regs.dst_y_per_row_vblank);
		dml_log_verbose!("DML_DLG: %s: disp_dlg_regs.dst_y_per_vm_flip = 0x%x\n", __func__, disp_dlg_regs.dst_y_per_vm_flip);
		dml_log_verbose!("DML_DLG: %s: disp_dlg_regs.dst_y_per_row_flip = 0x%x\n", __func__, disp_dlg_regs.dst_y_per_row_flip);

		disp_dlg_regs.refcyc_per_vm_group_vblank = (u32)(mode_lib.mp.TimePerVMGroupVBlank[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz);
		disp_dlg_regs.refcyc_per_vm_group_flip = (u32)(mode_lib.mp.TimePerVMGroupFlip[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz);
		disp_dlg_regs.refcyc_per_vm_req_vblank = (u32)(mode_lib.mp.TimePerVMRequestVBlank[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz * math_pow(2, 10));
		disp_dlg_regs.refcyc_per_vm_req_flip = (u32)(mode_lib.mp.TimePerVMRequestFlip[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz * math_pow(2, 10));

		l.dst_y_per_pte_row_nom_l = mode_lib.mp.DST_Y_PER_PTE_ROW_NOM_L[mode_lib.mp.pipe_plane[pipe_idx]];
		l.dst_y_per_pte_row_nom_c = mode_lib.mp.DST_Y_PER_PTE_ROW_NOM_C[mode_lib.mp.pipe_plane[pipe_idx]];
		l.refcyc_per_pte_group_nom_l = mode_lib.mp.time_per_pte_group_nom_luma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_pte_group_nom_c = mode_lib.mp.time_per_pte_group_nom_chroma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_pte_group_vblank_l = mode_lib.mp.time_per_pte_group_vblank_luma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_pte_group_vblank_c = mode_lib.mp.time_per_pte_group_vblank_chroma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_pte_group_flip_l = mode_lib.mp.time_per_pte_group_flip_luma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_pte_group_flip_c = mode_lib.mp.time_per_pte_group_flip_chroma[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_tdlut_group = mode_lib.mp.time_per_tdlut_group[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;

		disp_dlg_regs.dst_y_per_pte_row_nom_l = (u32)(l.dst_y_per_pte_row_nom_l * math_pow(2, 2));
		disp_dlg_regs.dst_y_per_pte_row_nom_c = (u32)(l.dst_y_per_pte_row_nom_c * math_pow(2, 2));

		disp_dlg_regs.refcyc_per_pte_group_nom_l = (u32)(l.refcyc_per_pte_group_nom_l);
		disp_dlg_regs.refcyc_per_pte_group_nom_c = (u32)(l.refcyc_per_pte_group_nom_c);
		disp_dlg_regs.refcyc_per_pte_group_vblank_l = (u32)(l.refcyc_per_pte_group_vblank_l);
		disp_dlg_regs.refcyc_per_pte_group_vblank_c = (u32)(l.refcyc_per_pte_group_vblank_c);
		disp_dlg_regs.refcyc_per_pte_group_flip_l = (u32)(l.refcyc_per_pte_group_flip_l);
		disp_dlg_regs.refcyc_per_pte_group_flip_c = (u32)(l.refcyc_per_pte_group_flip_c);
		disp_dlg_regs.refcyc_per_line_delivery_pre_l = (u32)math_floor2(l.refcyc_per_line_delivery_pre_l, 1);
		disp_dlg_regs.refcyc_per_line_delivery_l = (u32)math_floor2(l.refcyc_per_line_delivery_l, 1);
		disp_dlg_regs.refcyc_per_line_delivery_pre_c = (u32)math_floor2(l.refcyc_per_line_delivery_pre_c, 1);
		disp_dlg_regs.refcyc_per_line_delivery_c = (u32)math_floor2(l.refcyc_per_line_delivery_c, 1);

		l.dst_y_per_meta_row_nom_l = mode_lib.mp.DST_Y_PER_META_ROW_NOM_L[mode_lib.mp.pipe_plane[pipe_idx]];
		l.dst_y_per_meta_row_nom_c = mode_lib.mp.DST_Y_PER_META_ROW_NOM_C[mode_lib.mp.pipe_plane[pipe_idx]];
		l.refcyc_per_meta_chunk_nom_l = mode_lib.mp.TimePerMetaChunkNominal[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_meta_chunk_nom_c = mode_lib.mp.TimePerChromaMetaChunkNominal[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_meta_chunk_vblank_l = mode_lib.mp.TimePerMetaChunkVBlank[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_meta_chunk_vblank_c = mode_lib.mp.TimePerChromaMetaChunkVBlank[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_meta_chunk_flip_l = mode_lib.mp.TimePerMetaChunkFlip[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;
		l.refcyc_per_meta_chunk_flip_c = mode_lib.mp.TimePerChromaMetaChunkFlip[mode_lib.mp.pipe_plane[pipe_idx]] * l.refclk_freq_in_mhz;

		disp_dlg_regs.dst_y_per_meta_row_nom_l = (u32)(l.dst_y_per_meta_row_nom_l * math_pow(2, 2));
		disp_dlg_regs.dst_y_per_meta_row_nom_c = (u32)(l.dst_y_per_meta_row_nom_c * math_pow(2, 2));
		disp_dlg_regs.refcyc_per_meta_chunk_nom_l = (u32)(l.refcyc_per_meta_chunk_nom_l);
		disp_dlg_regs.refcyc_per_meta_chunk_nom_c = (u32)(l.refcyc_per_meta_chunk_nom_c);
		disp_dlg_regs.refcyc_per_meta_chunk_vblank_l = (u32)(l.refcyc_per_meta_chunk_vblank_l);
		disp_dlg_regs.refcyc_per_meta_chunk_vblank_c = (u32)(l.refcyc_per_meta_chunk_vblank_c);
		disp_dlg_regs.refcyc_per_meta_chunk_flip_l = (u32)(l.refcyc_per_meta_chunk_flip_l);
		disp_dlg_regs.refcyc_per_meta_chunk_flip_c = (u32)(l.refcyc_per_meta_chunk_flip_c);

		disp_dlg_regs.refcyc_per_tdlut_group = (u32)(l.refcyc_per_tdlut_group);

		/* Assign drq limit based on alt-channel enabled or not */
		if (display_cfg.plane_descriptors[l.plane_idx].overrides.uclk_pstate_change_strategy == dml2_uclk_pstate_change_strategy_force_alternate) {
			disp_dlg_regs.dst_y_delta_drq_limit = mode_lib.mp.nom_req_limit_alt[l.stream_idx];
			disp_dlg_regs.dst_y_svp_drq_limit = mode_lib.mp.svp_req_limit[l.stream_idx];
			disp_dlg_regs.force_prefetch_to_vblank = 1; // For alt-channel, always force disp prefetch to vblank
			disp_dlg_regs.force_cursor_to_disp_pref = 1; // For alt-channel, always force cursor to disp prefetch
		} else {
			disp_dlg_regs.dst_y_delta_drq_limit = 0x7fff; // off
			disp_dlg_regs.dst_y_svp_drq_limit = 0x7fff; // off
			disp_dlg_regs.force_prefetch_to_vblank = 0; // off
			disp_dlg_regs.force_cursor_to_disp_pref = 0; // off
		}

		disp_ttu_regs.refcyc_per_req_delivery_pre_l = (u32)(l.refcyc_per_req_delivery_pre_l * math_pow(2, 10));
		disp_ttu_regs.refcyc_per_req_delivery_l = (u32)(l.refcyc_per_req_delivery_l * math_pow(2, 10));
		disp_ttu_regs.refcyc_per_req_delivery_pre_c = (u32)(l.refcyc_per_req_delivery_pre_c * math_pow(2, 10));
		disp_ttu_regs.refcyc_per_req_delivery_c = (u32)(l.refcyc_per_req_delivery_c * math_pow(2, 10));
		disp_ttu_regs.qos_level_low_wm = 0;

		disp_ttu_regs.qos_level_high_wm = (u32)(4.0 * (f64)l.htotal * l.ref_freq_to_pix_freq);

		disp_ttu_regs.qos_level_flip = 14;
		disp_ttu_regs.qos_level_fixed_l = 8;
		disp_ttu_regs.qos_level_fixed_c = 8;
		disp_ttu_regs.qos_ramp_disable_l = 0;
		disp_ttu_regs.qos_ramp_disable_c = 0;
		disp_ttu_regs.min_ttu_vblank = (u32)(l.min_ttu_vblank * l.refclk_freq_in_mhz);

		// CHECK for HW registers' range, DML_ASSERT or clamp
		debug_assert!(l.refcyc_per_req_delivery_pre_l < math_pow(2, 13));
		debug_assert!(l.refcyc_per_req_delivery_l < math_pow(2, 13));
		debug_assert!(l.refcyc_per_req_delivery_pre_c < math_pow(2, 13));
		debug_assert!(l.refcyc_per_req_delivery_c < math_pow(2, 13));
		if (disp_dlg_regs.refcyc_per_vm_group_vblank >= (u32)math_pow(2, 23))
			disp_dlg_regs.refcyc_per_vm_group_vblank = (u32)(math_pow(2, 23) - 1);

		if (disp_dlg_regs.refcyc_per_vm_group_flip >= (u32)math_pow(2, 23))
			disp_dlg_regs.refcyc_per_vm_group_flip = (u32)(math_pow(2, 23) - 1);

		if (disp_dlg_regs.refcyc_per_vm_req_vblank >= (u32)math_pow(2, 23))
			disp_dlg_regs.refcyc_per_vm_req_vblank = (u32)(math_pow(2, 23) - 1);

		if (disp_dlg_regs.refcyc_per_vm_req_flip >= (u32)math_pow(2, 23))
			disp_dlg_regs.refcyc_per_vm_req_flip = (u32)(math_pow(2, 23) - 1);


		debug_assert!(disp_dlg_regs.dst_y_after_scaler < (u32)8);
		debug_assert!(disp_dlg_regs.refcyc_x_after_scaler < (u32)math_pow(2, 13));

		if (disp_dlg_regs.dst_y_per_pte_row_nom_l >= (u32)math_pow(2, 17)) {
			dml_log_verbose!("DML_DLG: %s: Warning DST_Y_PER_PTE_ROW_NOM_L %u > register max U15.2 %u, clamp to max\n", __func__, disp_dlg_regs.dst_y_per_pte_row_nom_l, (u32)math_pow(2, 17) - 1);
			l.dst_y_per_pte_row_nom_l = (u32)math_pow(2, 17) - 1;
		}
		if (l.dual_plane) {
			if (disp_dlg_regs.dst_y_per_pte_row_nom_c >= (u32)math_pow(2, 17)) {
				dml_log_verbose!("DML_DLG: %s: Warning DST_Y_PER_PTE_ROW_NOM_C %u > register max U15.2 %u, clamp to max\n", __func__, disp_dlg_regs.dst_y_per_pte_row_nom_c, (u32)math_pow(2, 17) - 1);
				l.dst_y_per_pte_row_nom_c = (u32)math_pow(2, 17) - 1;
			}
		}

		if (disp_dlg_regs.refcyc_per_pte_group_nom_l >= (u32)math_pow(2, 23))
			disp_dlg_regs.refcyc_per_pte_group_nom_l = (u32)(math_pow(2, 23) - 1);
		if (l.dual_plane) {
			if (disp_dlg_regs.refcyc_per_pte_group_nom_c >= (u32)math_pow(2, 23))
				disp_dlg_regs.refcyc_per_pte_group_nom_c = (u32)(math_pow(2, 23) - 1);
		}
		debug_assert!(disp_dlg_regs.refcyc_per_pte_group_vblank_l < (u32)math_pow(2, 13));
		if (l.dual_plane) {
			debug_assert!(disp_dlg_regs.refcyc_per_pte_group_vblank_c < (u32)math_pow(2, 13));
		}

		debug_assert!(disp_dlg_regs.refcyc_per_line_delivery_pre_l < (u32)math_pow(2, 13));
		debug_assert!(disp_dlg_regs.refcyc_per_line_delivery_l < (u32)math_pow(2, 13));
		debug_assert!(disp_dlg_regs.refcyc_per_line_delivery_pre_c < (u32)math_pow(2, 13));
		debug_assert!(disp_dlg_regs.refcyc_per_line_delivery_c < (u32)math_pow(2, 13));
		debug_assert!(disp_ttu_regs.qos_level_low_wm < (u32)math_pow(2, 14));
		debug_assert!(disp_ttu_regs.qos_level_high_wm < (u32)math_pow(2, 14));
		debug_assert!(disp_ttu_regs.min_ttu_vblank < (u32)math_pow(2, 24));
		dml_log_verbose!("DML_DLG::%s: Calculation for pipe[%d] done\n", __func__, pipe_idx);
	}
}

static unsafe fn dcn6_rq_dlg_get_wm_regs(*const dml2_display_cfgdisplay_cfg, *const dml2_core_internal_display_mode_libmode_lib, *const dml2_utm_soc_bbutm_soc_bb, dml2_dchub_watermark_regs *wm_regs)
{
	f64 refclk_freq_in_mhz = (display_cfg.overrides.hw.dlg_ref_clk_mhz > 0) ? (f64)display_cfg.overrides.hw.dlg_ref_clk_mhz : utm_soc_bb.dchub_refclk_mhz;

	wm_regs.fclk_pstate = (u32)(mode_lib.mp.Watermark.FCLKChangeWatermark * refclk_freq_in_mhz);
	wm_regs.sr_enter = (u32)(mode_lib.mp.Watermark.StutterEnterPlusExitWatermark * refclk_freq_in_mhz);
	wm_regs.sr_exit = (u32)(mode_lib.mp.Watermark.StutterExitWatermark * refclk_freq_in_mhz);
	wm_regs.sr_enter_z8 = (u32)(mode_lib.mp.Watermark.Z8StutterEnterPlusExitWatermark * refclk_freq_in_mhz);
	wm_regs.sr_exit_z8 = (u32)(mode_lib.mp.Watermark.Z8StutterExitWatermark * refclk_freq_in_mhz);
	wm_regs.sr_enter_low_power = (u32)(mode_lib.mp.Watermark.LowPowerStutterEnterPlusExitWatermark * refclk_freq_in_mhz);
	wm_regs.sr_exit_low_power = (u32)(mode_lib.mp.Watermark.LowPowerStutterExitWatermark * refclk_freq_in_mhz);
	wm_regs.temp_read_or_ppt = (u32)(mode_lib.mp.Watermark.temp_read_or_ppt_watermark_us * refclk_freq_in_mhz);
	wm_regs.ppt = (u32)(mode_lib.mp.Watermark.temp_read_or_ppt_watermark_us * refclk_freq_in_mhz);
	wm_regs.uclk_pstate = (u32)(mode_lib.mp.Watermark.DRAMClockChangeWatermark * refclk_freq_in_mhz);
	wm_regs.urgent = (u32)(mode_lib.mp.Watermark.UrgentWatermark * refclk_freq_in_mhz);
	wm_regs.usr = (u32)(mode_lib.mp.Watermark.USRRetrainingWatermark * refclk_freq_in_mhz);
	wm_regs.refcyc_per_trip_to_mem = (u32)(mode_lib.mp.UrgentLatency * refclk_freq_in_mhz);
	wm_regs.refcyc_per_meta_trip_to_mem = (u32)(mode_lib.mp.MetaTripToMemory * refclk_freq_in_mhz);
	wm_regs.frac_urg_bw_flip = (u32)(mode_lib.mp.FractionOfUrgentBandwidthImmediateFlip * 1000);
	wm_regs.frac_urg_bw_nom = (u32)(mode_lib.mp.FractionOfUrgentBandwidth * 1000);
}

unsafe fn dcn6_get_pipe_regs(*const dml2_display_cfgdisplay_cfg,
		*const dml2_core_internal_display_mode_libmode_lib,
		dml2_dchub_per_pipe_register_set *out, int pipe_index,
		*const dml2_utm_soc_bbutm_soc_bb,
		dml2_core_internal_scratch *s)
{
	dcn5_rq_dlg_get_rq_reg(&out.rq_regs, display_cfg, mode_lib, pipe_index);
	dcn6_rq_dlg_get_dlg_reg(s, &out.dlg_regs, &out.ttu_regs, display_cfg, mode_lib, pipe_index, utm_soc_bb);
	out.det_size = mode_lib.mp.DETBufferSizeInKByte[mode_lib.mp.pipe_plane[pipe_index]] / mode_lib.ip.config_return_buffer_segment_size_in_kbytes;
}

unsafe fn dcn6_calculate_pstate_support_method(
		enum dml2_pstate_method method,
		f64 peak_vactive_p_vblank_latency_hiding_margin_us,
		f64 vactive_margin_us,
		f64 reserved_vblank_us,
		f64 blackout_us,
		bool all_streams_blanked,
		/* output */
		enum dml2_pstate_change_support *surface_pstate_change_support)
{
	*surface_pstate_change_support = dml2_pstate_change_unsupported;
	if (method == dml2_pstate_method_na) {
		/* automatic */
		if (all_streams_blanked ||
				(vactive_margin_us > 0 && reserved_vblank_us >= blackout_us))
			*surface_pstate_change_support = dml2_pstate_change_vblank_and_vactive;
		else if (vactive_margin_us > 0)
			*surface_pstate_change_support = dml2_pstate_change_vactive;
		else if (reserved_vblank_us >= blackout_us)
			*surface_pstate_change_support = dml2_pstate_change_vblank;
	} else if (method == dml2_pstate_method_vactive || method == dml2_pstate_method_fw_vactive_drr) {
		/* vactive */
		if (all_streams_blanked ||
				(vactive_margin_us > 0 && reserved_vblank_us >= blackout_us))
			*surface_pstate_change_support = dml2_pstate_change_vblank_and_vactive;
		else if (vactive_margin_us > 0 || peak_vactive_p_vblank_latency_hiding_margin_us > 0)
			*surface_pstate_change_support = dml2_pstate_change_vactive;
	} else if ((method == dml2_pstate_method_vblank || method == dml2_pstate_method_fw_vblank_drr) &&
			reserved_vblank_us >= blackout_us) {
		/* vblank */
		*surface_pstate_change_support = dml2_pstate_change_vblank;
	} else if (method == dml2_pstate_method_fw_drr) {
		/* drr */
		*surface_pstate_change_support = dml2_pstate_change_drr;
	} else if (method == dml2_pstate_method_alternate) {
		/* TODO - alternate */
		*surface_pstate_change_support = dml2_pstate_change_mall_svp;
	}

	return *surface_pstate_change_support != dml2_pstate_change_unsupported;
}

unsafe fn dcn6_calculate_watermarks_and_dram_speed_change_support(
		dml2_core_internal_scratch *scratch,
		dml2_core_calcs_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params *p)
{
	dml2_core_calcs_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals *s = &scratch.CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals;

	f64 reserved_vblank_time_us;
	bool FoundCriticalSurface = false;

	s.TotalActiveWriteback = 0;
	p.Watermark.UrgentWatermark = p.mmSOCParameters.UrgentLatency + p.mmSOCParameters.ExtraLatency;

	dml_log_verbose!("DML::%s: UrgentLatency = %f\n", __func__, p.mmSOCParameters.UrgentLatency);
	dml_log_verbose!("DML::%s: ExtraLatency = %f\n", __func__, p.mmSOCParameters.ExtraLatency);
	dml_log_verbose!("DML::%s: UrgentWatermark = %f\n", __func__, p.Watermark.UrgentWatermark);

	p.Watermark.USRRetrainingWatermark = p.mmSOCParameters.UrgentLatency + p.mmSOCParameters.ExtraLatency + p.mmSOCParameters.USRRetrainingLatency + p.mmSOCParameters.SMNLatency;
	p.Watermark.DRAMClockChangeWatermark = p.mmSOCParameters.DRAMClockChangeLatency + p.Watermark.UrgentWatermark;
	p.Watermark.FCLKChangeWatermark = p.mmSOCParameters.FCLKChangeLatency + p.Watermark.UrgentWatermark;
	p.Watermark.StutterExitWatermark = p.mmSOCParameters.SRExitTime + p.mmSOCParameters.ExtraLatency_sr + 10 / p.DCFClkDeepSleep;
	p.Watermark.StutterEnterPlusExitWatermark = p.mmSOCParameters.SREnterPlusExitTime + p.mmSOCParameters.ExtraLatency_sr + 10 / p.DCFClkDeepSleep;
	p.Watermark.LowPowerStutterExitWatermark = p.mmSOCParameters.SRExitTimeLowPower + p.mmSOCParameters.ExtraLatency_sr + 10 / p.DCFClkDeepSleep;
	p.Watermark.LowPowerStutterEnterPlusExitWatermark = p.mmSOCParameters.SREnterPlusExitTimeLowPower + p.mmSOCParameters.ExtraLatency_sr + 10 / p.DCFClkDeepSleep;
	p.Watermark.Z8StutterExitWatermark = p.mmSOCParameters.SRExitZ8Time + p.mmSOCParameters.ExtraLatency_sr + 10 / p.DCFClkDeepSleep;
	p.Watermark.Z8StutterEnterPlusExitWatermark = p.mmSOCParameters.SREnterPlusExitZ8Time + p.mmSOCParameters.ExtraLatency_sr + 10 / p.DCFClkDeepSleep;
	if (p.mmSOCParameters.qos_type == dml2_qos_param_type_dcn4x) {
		p.Watermark.StutterExitWatermark += p.mmSOCParameters.max_urgent_latency_us + p.mmSOCParameters.df_response_time_us;
		p.Watermark.StutterEnterPlusExitWatermark += p.mmSOCParameters.max_urgent_latency_us + p.mmSOCParameters.df_response_time_us;
		p.Watermark.LowPowerStutterExitWatermark += p.mmSOCParameters.max_urgent_latency_us + p.mmSOCParameters.df_response_time_us;
		p.Watermark.LowPowerStutterEnterPlusExitWatermark += p.mmSOCParameters.max_urgent_latency_us + p.mmSOCParameters.df_response_time_us;
		p.Watermark.Z8StutterExitWatermark += p.mmSOCParameters.max_urgent_latency_us + p.mmSOCParameters.df_response_time_us;
		p.Watermark.Z8StutterEnterPlusExitWatermark += p.mmSOCParameters.max_urgent_latency_us + p.mmSOCParameters.df_response_time_us;
	}
	p.Watermark.temp_read_or_ppt_watermark_us = p.mmSOCParameters.temp_read_or_ppt_blackout_us + p.Watermark.UrgentWatermark;

	dml_log_verbose!("DML::%s: UrgentLatency = %f\n", __func__, p.mmSOCParameters.UrgentLatency);
	dml_log_verbose!("DML::%s: ExtraLatency = %f\n", __func__, p.mmSOCParameters.ExtraLatency);
	dml_log_verbose!("DML::%s: DRAMClockChangeLatency = %f\n", __func__, p.mmSOCParameters.DRAMClockChangeLatency);
	dml_log_verbose!("DML::%s: SREnterPlusExitZ8Time = %f\n", __func__, p.mmSOCParameters.SREnterPlusExitZ8Time);
	dml_log_verbose!("DML::%s: SREnterPlusExitTime = %f\n", __func__, p.mmSOCParameters.SREnterPlusExitTime);
	dml_log_verbose!("DML::%s: UrgentWatermark = %f\n", __func__, p.Watermark.UrgentWatermark);
	dml_log_verbose!("DML::%s: USRRetrainingWatermark = %f\n", __func__, p.Watermark.USRRetrainingWatermark);
	dml_log_verbose!("DML::%s: DRAMClockChangeWatermark = %f\n", __func__, p.Watermark.DRAMClockChangeWatermark);
	dml_log_verbose!("DML::%s: FCLKChangeWatermark = %f\n", __func__, p.Watermark.FCLKChangeWatermark);
	dml_log_verbose!("DML::%s: StutterExitWatermark = %f\n", __func__, p.Watermark.StutterExitWatermark);
	dml_log_verbose!("DML::%s: StutterEnterPlusExitWatermark = %f\n", __func__, p.Watermark.StutterEnterPlusExitWatermark);
	dml_log_verbose!("DML::%s: Z8StutterExitWatermark = %f\n", __func__, p.Watermark.Z8StutterExitWatermark);
	dml_log_verbose!("DML::%s: Z8StutterEnterPlusExitWatermark = %f\n", __func__, p.Watermark.Z8StutterEnterPlusExitWatermark);
	dml_log_verbose!("DML::%s: temp_read_or_ppt_watermark_us = %f\n", __func__, p.Watermark.temp_read_or_ppt_watermark_us);

	s.TotalActiveWriteback = 0;
	for u32 k = 0;  k < p.NumberOfActiveSurfaces;  ++k {
		for u32 j = 0;  j < p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.active_writebacks_per_stream;  ++j {
			s.TotalActiveWriteback = s.TotalActiveWriteback + 1;

	if (s.TotalActiveWriteback <= 1) {
		p.Watermark.WritebackUrgentWatermark = p.mmSOCParameters.WritebackLatency;
	} else {
		p.Watermark.WritebackUrgentWatermark = p.mmSOCParameters.WritebackLatency + p.WritebackChunkSize * 1024.0 / 32.0 / p.SOCCLK;
	}
	if (p.USRRetrainingRequired)
		p.Watermark.WritebackUrgentWatermark = p.Watermark.WritebackUrgentWatermark + p.mmSOCParameters.USRRetrainingLatency;

	if (s.TotalActiveWriteback <= 1) {
		p.Watermark.WritebackDRAMClockChangeWatermark = p.mmSOCParameters.DRAMClockChangeLatency + p.mmSOCParameters.WritebackLatency;
		p.Watermark.WritebackFCLKChangeWatermark = p.mmSOCParameters.FCLKChangeLatency + p.mmSOCParameters.WritebackLatency;
	} else {
		p.Watermark.WritebackDRAMClockChangeWatermark = p.mmSOCParameters.DRAMClockChangeLatency + p.mmSOCParameters.WritebackLatency + p.WritebackChunkSize * 1024.0 / 32.0 / p.SOCCLK;
		p.Watermark.WritebackFCLKChangeWatermark = p.mmSOCParameters.FCLKChangeLatency + p.mmSOCParameters.WritebackLatency + p.WritebackChunkSize * 1024 / 32 / p.SOCCLK;
	}

	if (p.USRRetrainingRequired)
		p.Watermark.WritebackDRAMClockChangeWatermark = p.Watermark.WritebackDRAMClockChangeWatermark + p.mmSOCParameters.USRRetrainingLatency;

	if (p.USRRetrainingRequired)
		p.Watermark.WritebackFCLKChangeWatermark = p.Watermark.WritebackFCLKChangeWatermark + p.mmSOCParameters.USRRetrainingLatency;

	dml_log_verbose!("DML::%s: WritebackDRAMClockChangeWatermark = %f\n", __func__, p.Watermark.WritebackDRAMClockChangeWatermark);
	dml_log_verbose!("DML::%s: WritebackFCLKChangeWatermark = %f\n", __func__, p.Watermark.WritebackFCLKChangeWatermark);
	dml_log_verbose!("DML::%s: WritebackUrgentWatermark = %f\n", __func__, p.Watermark.WritebackUrgentWatermark);
	dml_log_verbose!("DML::%s: USRRetrainingRequired = %u\n", __func__, p.USRRetrainingRequired);
	dml_log_verbose!("DML::%s: USRRetrainingLatency = %f\n", __func__, p.mmSOCParameters.USRRetrainingLatency);

	s.TotalPixelBW = 0.0;
	for u32 k = 0;  k < p.NumberOfActiveSurfaces;  ++k { {
		f64 h_total = (f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.h_total;
		f64 pixel_clock_mhz = p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.pixel_clock_khz / 1000.0;
		f64 v_ratio = p.display_cfg.plane_descriptors[k].composition.scaler_info.plane0.v_ratio;
		f64 v_ratio_c = p.display_cfg.plane_descriptors[k].composition.scaler_info.plane1.v_ratio;
		s.TotalPixelBW = s.TotalPixelBW + p.DPPPerSurface[k]
								     * (p.SwathWidthY[k] * p.BytePerPixelDETY[k] * v_ratio + p.SwathWidthC[k] * p.BytePerPixelDETC[k] * v_ratio_c) / (h_total / pixel_clock_mhz);
	}

	*p.global_fclk_change_supported = true;
	*p.global_dram_clock_change_supported = true;
	*p.global_temp_read_or_ppt_supported = true;

	for u32 k = 0;  k < p.NumberOfActiveSurfaces;  ++k { {
		f64 h_total = (f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.h_total;
		f64 pixel_clock_mhz = p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.pixel_clock_khz / 1000.0;
		f64 v_ratio = p.display_cfg.plane_descriptors[k].composition.scaler_info.plane0.v_ratio;
		f64 v_ratio_c = p.display_cfg.plane_descriptors[k].composition.scaler_info.plane1.v_ratio;
		f64 v_taps = p.display_cfg.plane_descriptors[k].composition.scaler_info.plane0.v_taps;
		f64 v_taps_c = p.display_cfg.plane_descriptors[k].composition.scaler_info.plane1.v_taps;
		f64 h_ratio = p.display_cfg.plane_descriptors[k].composition.scaler_info.plane0.h_ratio;
		f64 h_ratio_c = p.display_cfg.plane_descriptors[k].composition.scaler_info.plane1.h_ratio;
		f64 LBBitPerPixel = 57;

		s.LBLatencyHidingSourceLinesY[k] = (u32)(math_min2((f64)p.MaxLineBufferLines, math_floor2((f64)p.LineBufferSize / LBBitPerPixel / ((f64)p.SwathWidthY[k] / math_max2(h_ratio, 1.0)), 1)) - (v_taps - 1));
		s.LBLatencyHidingSourceLinesC[k] = (u32)(math_min2((f64)p.MaxLineBufferLines, math_floor2((f64)p.LineBufferSize / LBBitPerPixel / ((f64)p.SwathWidthC[k] / math_max2(h_ratio_c, 1.0)), 1)) - (v_taps_c - 1));

		dml_log_verbose!("DML::%s: k=%u, MaxLineBufferLines= %u\n", __func__, k, p.MaxLineBufferLines);
		dml_log_verbose!("DML::%s: k=%u, LineBufferSize = %u\n", __func__, k, p.LineBufferSize);
		dml_log_verbose!("DML::%s: k=%u, LBBitPerPixel = %f\n", __func__, k, LBBitPerPixel);
		dml_log_verbose!("DML::%s: k=%u, HRatio = %f\n", __func__, k, h_ratio);
		dml_log_verbose!("DML::%s: k=%u, VTaps = %f\n", __func__, k, v_taps);

		s.EffectiveLBLatencyHidingY = s.LBLatencyHidingSourceLinesY[k] / v_ratio * (h_total / pixel_clock_mhz);
		s.EffectiveLBLatencyHidingC = s.LBLatencyHidingSourceLinesC[k] / v_ratio_c * (h_total / pixel_clock_mhz);

		s.EffectiveDETBufferSizeY = p.DETBufferSizeY[k];
		if (p.UnboundedRequestEnabled) {
			s.EffectiveDETBufferSizeY = s.EffectiveDETBufferSizeY + p.CompressedBufferSizeInkByte * 1024 * (p.SwathWidthY[k] * p.BytePerPixelDETY[k] * v_ratio) / (h_total / pixel_clock_mhz) / s.TotalPixelBW;
		}

		s.LinesInDETY[k] = (f64)s.EffectiveDETBufferSizeY / p.BytePerPixelDETY[k] / p.SwathWidthY[k];
		s.LinesInDETYRoundedDownToSwath[k] = (u32)(math_floor2(s.LinesInDETY[k], p.SwathHeightY[k]));
		s.FullDETBufferingTimeY = s.LinesInDETYRoundedDownToSwath[k] * (h_total / pixel_clock_mhz) / v_ratio;

		s.ActiveClockChangeLatencyHidingY = s.EffectiveLBLatencyHidingY + s.FullDETBufferingTimeY - ((f64)p.DSTXAfterScaler[k] / h_total + (f64)p.DSTYAfterScaler[k]) * h_total / pixel_clock_mhz;

		if (p.NumberOfActiveSurfaces > 1) {
			s.ActiveClockChangeLatencyHidingY = s.ActiveClockChangeLatencyHidingY - (1.0 - 1.0 / (f64)p.NumberOfActiveSurfaces) * (f64)p.SwathHeightY[k] * (f64)h_total / pixel_clock_mhz / v_ratio;
		}

		if (p.BytePerPixelDETC[k] > 0) {
			s.LinesInDETC[k] = p.DETBufferSizeC[k] / p.BytePerPixelDETC[k] / p.SwathWidthC[k];
			s.LinesInDETCRoundedDownToSwath[k] = (u32)(math_floor2(s.LinesInDETC[k], p.SwathHeightC[k]));
			s.FullDETBufferingTimeC = s.LinesInDETCRoundedDownToSwath[k] * (h_total / pixel_clock_mhz) / v_ratio_c;
			s.ActiveClockChangeLatencyHidingC = s.EffectiveLBLatencyHidingC + s.FullDETBufferingTimeC - ((f64)p.DSTXAfterScaler[k] / (f64)h_total + (f64)p.DSTYAfterScaler[k]) * (f64)h_total / pixel_clock_mhz;
			if (p.NumberOfActiveSurfaces > 1) {
				s.ActiveClockChangeLatencyHidingC = s.ActiveClockChangeLatencyHidingC - (1.0 - 1.0 / (f64)p.NumberOfActiveSurfaces) * (f64)p.SwathHeightC[k] * (f64)h_total / pixel_clock_mhz / v_ratio_c;
			}
			s.ActiveClockChangeLatencyHiding = math_min2(s.ActiveClockChangeLatencyHidingY, s.ActiveClockChangeLatencyHidingC);
		} else {
			s.ActiveClockChangeLatencyHiding = s.ActiveClockChangeLatencyHidingY;
		}

		dml_log_verbose!("DML::%s: ActiveClockChangeLatencyHidingY = %f\n", __func__, s.ActiveClockChangeLatencyHidingY);
		dml_log_verbose!("DML::%s: ActiveClockChangeLatencyHidingC = %f\n", __func__, s.ActiveClockChangeLatencyHidingC);
		dml_log_verbose!("DML::%s: ActiveClockChangeLatencyHiding = %f\n", __func__, s.ActiveClockChangeLatencyHiding);

		reserved_vblank_time_us = (f64)p.display_cfg.plane_descriptors[k].overrides.reserved_vblank_time_ns / 1000;

		s.ActiveDRAMClockChangeLatencyMargin[k] = s.ActiveClockChangeLatencyHiding - p.Watermark.DRAMClockChangeWatermark;
		s.ActiveFCLKChangeLatencyMargin[k] = s.ActiveClockChangeLatencyHiding - p.Watermark.FCLKChangeWatermark;
		s.USRRetrainingLatencyMargin[k] = s.ActiveClockChangeLatencyHiding - p.Watermark.USRRetrainingWatermark;
		s.temp_read_or_ppt_latency_margin[k] = s.ActiveClockChangeLatencyHiding - p.Watermark.temp_read_or_ppt_watermark_us;
		s.peak_vactive_p_vblank_latency_hiding_us = s.ActiveClockChangeLatencyHiding + reserved_vblank_time_us;

		dml_log_verbose!("DML::%s: k=%u, ActiveFCLKChangeLatencyMargin = %f\n", __func__, k, s.ActiveFCLKChangeLatencyMargin[k]);
		dml_log_verbose!("DML::%s: k=%u, ActiveDRAMClockChangeLatencyMargin = %f\n", __func__, k, s.ActiveDRAMClockChangeLatencyMargin[k]);

		if (p.VActiveLatencyHidingMargin) {
			p.VActiveLatencyHidingMargin[k] = s.ActiveDRAMClockChangeLatencyMargin[k];
			dml_log_verbose!("DML::%s: k=%u, VActiveLatencyHidingMargin = %f\n", __func__, k, p.VActiveLatencyHidingMargin[k]);
		}

		if (p.VActiveLatencyHidingUs) {
			p.VActiveLatencyHidingUs[k] = s.ActiveClockChangeLatencyHiding;
			dml_log_verbose!("DML::%s: k=%u, VActiveLatencyHidingUs = %f\n", __func__, k, p.VActiveLatencyHidingUs[k]);
		}

		for u32 j = 0;  j < p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[j].stream_index].writeback.active_writebacks_per_stream;  ++j { {
			f64 byte_per_pixel_luma_in_buffer = 1.0;
			f64 buffer_for_luma = (f64)p.WritebackInterfaceBufferSize * 1024.0 / 2.0;
			if (p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].pixel_format == dml2_444_64) {
				byte_per_pixel_luma_in_buffer = 8.0;
				buffer_for_luma = (f64)p.WritebackInterfaceBufferSize * 1024.0;
			} else if (p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].pixel_format == dml2_444_32) {
				byte_per_pixel_luma_in_buffer = 4.0;
				buffer_for_luma = (f64)p.WritebackInterfaceBufferSize * 1024.0;
			} else if (p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].pixel_format == dml2_422_packed_8
				|| p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].pixel_format == dml2_420_8) {
				byte_per_pixel_luma_in_buffer = 1.0;
				buffer_for_luma = (f64)p.WritebackInterfaceBufferSize * 1024.0 / 2.0;
			} else if (p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].pixel_format == dml2_422_packed_10
				|| p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].pixel_format == dml2_420_10) {
				byte_per_pixel_luma_in_buffer = 10.0 / 8.0;
				buffer_for_luma = (f64)p.WritebackInterfaceBufferSize * 1024.0 / 2.0;
			}

			s.WritebackLatencyHiding = buffer_for_luma
				/ ((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].output_height
					* (f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].output_width
					/ ((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].writeback.writeback_stream[j].input_height
						* (f64)h_total / pixel_clock_mhz)) / byte_per_pixel_luma_in_buffer;

			s.peak_vactive_p_vblank_latency_hiding_us = 0.0; /* not supported with writeback */
			s.WritebackDRAMClockChangeLatencyMargin = s.WritebackLatencyHiding - p.Watermark.WritebackDRAMClockChangeWatermark;
			s.WritebackFCLKChangeLatencyMargin = s.WritebackLatencyHiding - p.Watermark.WritebackFCLKChangeWatermark;
			s.ActiveDRAMClockChangeLatencyMargin[k] = math_min2(s.ActiveDRAMClockChangeLatencyMargin[k], s.WritebackDRAMClockChangeLatencyMargin);
			s.ActiveFCLKChangeLatencyMargin[k] = math_min2(s.ActiveFCLKChangeLatencyMargin[k], s.WritebackFCLKChangeLatencyMargin);
			dml_log_verbose!("DML::%s: k=%u, ActiveFCLKChangeLatencyMargin = %f (WB)\n", __func__, k, s.ActiveFCLKChangeLatencyMargin[k]);
		}

		p.MaxActiveDRAMClockChangeLatencySupported[k] = s.ActiveDRAMClockChangeLatencyMargin[k] + p.mmSOCParameters.DRAMClockChangeLatency;

		*p.global_fclk_change_supported &= dcn6_calculate_pstate_support_method(
				dml2_pstate_method_vactive,
				s.peak_vactive_p_vblank_latency_hiding_us - p.mmSOCParameters.FCLKChangeLatency,
				s.ActiveFCLKChangeLatencyMargin[k],
				reserved_vblank_time_us,
				p.mmSOCParameters.FCLKChangeLatency,
				p.display_cfg.overrides.all_streams_blanked,
				/* output */
				&p.FCLKChangeSupport[k]);

		*p.global_temp_read_or_ppt_supported &= dcn6_calculate_pstate_support_method(
				dml2_pstate_method_vactive,
				s.peak_vactive_p_vblank_latency_hiding_us - p.mmSOCParameters.temp_read_or_ppt_blackout_us,
				s.temp_read_or_ppt_latency_margin[k],
				reserved_vblank_time_us,
				p.mmSOCParameters.temp_read_or_ppt_blackout_us,
				p.display_cfg.overrides.all_streams_blanked,
				/* output */
				&p.temp_read_or_ppt_support[k]);

		*p.global_dram_clock_change_support_required |= p.uclk_pstate_switch_modes[k] != dml2_pstate_method_na;
		*p.global_dram_clock_change_supported &= dcn6_calculate_pstate_support_method(
				p.uclk_pstate_switch_modes[k],
				s.peak_vactive_p_vblank_latency_hiding_us - p.mmSOCParameters.DRAMClockChangeLatency,
				s.ActiveDRAMClockChangeLatencyMargin[k],
				reserved_vblank_time_us,
				p.mmSOCParameters.DRAMClockChangeLatency,
				p.display_cfg.overrides.all_streams_blanked,
				/* output */
				&p.DRAMClockChangeSupport[k]);

		s.dst_y_pstate = (u32)(math_ceil2((p.mmSOCParameters.DRAMClockChangeLatency + p.mmSOCParameters.UrgentLatency) / (h_total / pixel_clock_mhz), 1));
		s.src_y_pstate_l = (u32)(math_ceil2(s.dst_y_pstate * v_ratio, p.SwathHeightY[k]));
		s.src_y_ahead_l = (u32)(math_floor2(p.DETBufferSizeY[k] / p.BytePerPixelDETY[k] / p.SwathWidthY[k], p.SwathHeightY[k]) + s.LBLatencyHidingSourceLinesY[k]);

		dml_log_verbose!("DML::%s: k=%u, DETBufferSizeY = %u\n", __func__, k, p.DETBufferSizeY[k]);
		dml_log_verbose!("DML::%s: k=%u, BytePerPixelDETY = %f\n", __func__, k, p.BytePerPixelDETY[k]);
		dml_log_verbose!("DML::%s: k=%u, SwathWidthY = %u\n", __func__, k, p.SwathWidthY[k]);
		dml_log_verbose!("DML::%s: k=%u, SwathHeightY = %u\n", __func__, k, p.SwathHeightY[k]);
		dml_log_verbose!("DML::%s: k=%u, LBLatencyHidingSourceLinesY = %u\n", __func__, k, s.LBLatencyHidingSourceLinesY[k]);
		dml_log_verbose!("DML::%s: k=%u, dst_y_pstate = %u\n", __func__, k, s.dst_y_pstate);
		dml_log_verbose!("DML::%s: k=%u, src_y_pstate_l = %u\n", __func__, k, s.src_y_pstate_l);
		dml_log_verbose!("DML::%s: k=%u, src_y_ahead_l = %u\n", __func__, k, s.src_y_ahead_l);
		dml_log_verbose!("DML::%s: k=%u, meta_row_height_l = %u\n", __func__, k, p.meta_row_height_l[k]);

		if (p.BytePerPixelDETC[k] > 0) {
			s.src_y_pstate_c = (u32)(math_ceil2(s.dst_y_pstate * v_ratio_c, p.SwathHeightC[k]));
			s.src_y_ahead_c = (u32)(math_floor2(p.DETBufferSizeC[k] / p.BytePerPixelDETC[k] / p.SwathWidthC[k], p.SwathHeightC[k]) + s.LBLatencyHidingSourceLinesC[k]);

			dml_log_verbose!("DML::%s: k=%u, meta_row_height_c = %u\n", __func__, k, p.meta_row_height_c[k]);
			dml_log_verbose!("DML::%s: k=%u, src_y_pstate_c = %u\n", __func__, k, s.src_y_pstate_c);
			dml_log_verbose!("DML::%s: k=%u, src_y_ahead_c = %u\n", __func__, k, s.src_y_ahead_c);
			dml_log_verbose!("DML::%s: k=%u, sub_vp_lines_c = %u\n", __func__, k, s.sub_vp_lines_c);
		}
	}

	for u32 k = 0;  k < p.NumberOfActiveSurfaces;  ++k { {
		dml_log_verbose!("DML::%s: k=%u, ActiveFCLKChangeLatencyMargin=%f\n", __func__, k, s.ActiveFCLKChangeLatencyMargin[k]);
		if (((!FoundCriticalSurface) || ((s.ActiveFCLKChangeLatencyMargin[k] + p.mmSOCParameters.FCLKChangeLatency) < *p.MaxActiveFCLKChangeLatencySupported))) {
			FoundCriticalSurface = true;
			*p.MaxActiveFCLKChangeLatencySupported = s.ActiveFCLKChangeLatencyMargin[k] + p.mmSOCParameters.FCLKChangeLatency;
		}
	}

	dml_log_verbose!("DML::%s: DRAMClockChangeSupport = %u\n", __func__, *p.global_dram_clock_change_supported);
	dml_log_verbose!("DML::%s: FCLKChangeSupport = %u\n", __func__, *p.global_fclk_change_supported);
	dml_log_verbose!("DML::%s: MaxActiveFCLKChangeLatencySupported = %f\n", __func__, *p.MaxActiveFCLKChangeLatencySupported);
	dml_log_verbose!("DML::%s: USRRetrainingSupport = %u\n", __func__, *p.USRRetrainingSupport);
}

unsafe fn dcn6_calculate_stutter_efficiency(dml2_core_internal_scratch *scratch,
		dml2_core_calcs_CalculateStutterEfficiency_params *p)
{
	dml2_core_calcs_CalculateStutterEfficiency_locals *l = &scratch.CalculateStutterEfficiency_locals;

	u32 TotalNumberOfActiveOTG = 0;
	f64 SinglePixelClock = 0;
	u32 SingleHTotal = 0;
	u32 SingleVTotal = 0;
	bool SameTiming = true;
	bool at_least_one_single_pipe_single_plane_surface = false;
	bool FoundCriticalSurface = false;

	memset(l, 0, core::mem::size_of::<dml2_core_calcs_CalculateStutterEfficiency_locals>());

	for u32 k = 0;  k < p.NumberOfActiveSurfaces;  ++k { {
		if (p.display_cfg.plane_descriptors[k].surface.dcc.enable == true) {
			if ((dml2_core_utils_is_vertical_rotation(p.display_cfg.plane_descriptors[k].composition.rotation_angle) && p.BlockWidth256BytesY[k] > p.SwathHeightY[k]) || (!dml2_core_utils_is_vertical_rotation(p.display_cfg.plane_descriptors[k].composition.rotation_angle) && p.BlockHeight256BytesY[k] > p.SwathHeightY[k]) || p.DCCYMaxUncompressedBlock[k] < 256) {
				l.MaximumEffectiveCompressionLuma = 2;
			} else {
				l.MaximumEffectiveCompressionLuma = 4;
			}
			l.TotalCompressedReadBandwidth = l.TotalCompressedReadBandwidth + p.ReadBandwidthSurfaceLuma[k] / math_min2(p.display_cfg.plane_descriptors[k].surface.dcc.informative.dcc_rate_plane0, l.MaximumEffectiveCompressionLuma);
			dml_log_verbose!("DML::%s: k=%u, ReadBandwidthSurfaceLuma = %f\n", __func__, k, p.ReadBandwidthSurfaceLuma[k]);
			dml_log_verbose!("DML::%s: k=%u, NetDCCRateLuma = %f\n", __func__, k, p.display_cfg.plane_descriptors[k].surface.dcc.informative.dcc_rate_plane0);
			dml_log_verbose!("DML::%s: k=%u, MaximumEffectiveCompressionLuma = %f\n", __func__, k, l.MaximumEffectiveCompressionLuma);
			l.TotalZeroSizeRequestReadBandwidth = l.TotalZeroSizeRequestReadBandwidth + p.ReadBandwidthSurfaceLuma[k] * p.display_cfg.plane_descriptors[k].surface.dcc.informative.fraction_of_zero_size_request_plane0;
			l.TotalZeroSizeCompressedReadBandwidth = l.TotalZeroSizeCompressedReadBandwidth + p.ReadBandwidthSurfaceLuma[k] * p.display_cfg.plane_descriptors[k].surface.dcc.informative.fraction_of_zero_size_request_plane0 / l.MaximumEffectiveCompressionLuma;

			if (p.ReadBandwidthSurfaceChroma[k] > 0) {
				if ((dml2_core_utils_is_vertical_rotation(p.display_cfg.plane_descriptors[k].composition.rotation_angle) && p.BlockWidth256BytesC[k] > p.SwathHeightC[k]) || (!dml2_core_utils_is_vertical_rotation(p.display_cfg.plane_descriptors[k].composition.rotation_angle) && p.BlockHeight256BytesC[k] > p.SwathHeightC[k]) || p.DCCCMaxUncompressedBlock[k] < 256) {
					l.MaximumEffectiveCompressionChroma = 2;
				} else {
					l.MaximumEffectiveCompressionChroma = 4;
				}
				l.TotalCompressedReadBandwidth = l.TotalCompressedReadBandwidth + p.ReadBandwidthSurfaceChroma[k] / math_min2(p.display_cfg.plane_descriptors[k].surface.dcc.informative.dcc_rate_plane1, l.MaximumEffectiveCompressionChroma);
				dml_log_verbose!("DML::%s: k=%u, ReadBandwidthSurfaceChroma = %f\n", __func__, k, p.ReadBandwidthSurfaceChroma[k]);
				dml_log_verbose!("DML::%s: k=%u, NetDCCRateChroma = %f\n", __func__, k, p.display_cfg.plane_descriptors[k].surface.dcc.informative.dcc_rate_plane1);
				dml_log_verbose!("DML::%s: k=%u, MaximumEffectiveCompressionChroma = %f\n", __func__, k, l.MaximumEffectiveCompressionChroma);
				l.TotalZeroSizeRequestReadBandwidth = l.TotalZeroSizeRequestReadBandwidth + p.ReadBandwidthSurfaceChroma[k] * p.display_cfg.plane_descriptors[k].surface.dcc.informative.fraction_of_zero_size_request_plane1;
				l.TotalZeroSizeCompressedReadBandwidth = l.TotalZeroSizeCompressedReadBandwidth + p.ReadBandwidthSurfaceChroma[k] * p.display_cfg.plane_descriptors[k].surface.dcc.informative.fraction_of_zero_size_request_plane1 / l.MaximumEffectiveCompressionChroma;
			}
		} else {
			l.TotalCompressedReadBandwidth = l.TotalCompressedReadBandwidth + p.ReadBandwidthSurfaceLuma[k] + p.ReadBandwidthSurfaceChroma[k];
		}
		l.TotalRowReadBandwidth = l.TotalRowReadBandwidth + p.DPPPerSurface[k] * (p.meta_row_bw[k] + p.dpte_row_bw[k]);
	}

	l.AverageDCCCompressionRate = p.TotalDataReadBandwidth / l.TotalCompressedReadBandwidth;
	l.AverageDCCZeroSizeFraction = l.TotalZeroSizeRequestReadBandwidth / p.TotalDataReadBandwidth;

	dml_log_verbose!("DML::%s: UnboundedRequestEnabled = %u\n", __func__, p.UnboundedRequestEnabled);
	dml_log_verbose!("DML::%s: TotalCompressedReadBandwidth = %f\n", __func__, l.TotalCompressedReadBandwidth);
	dml_log_verbose!("DML::%s: TotalZeroSizeRequestReadBandwidth = %f\n", __func__, l.TotalZeroSizeRequestReadBandwidth);
	dml_log_verbose!("DML::%s: TotalZeroSizeCompressedReadBandwidth = %f\n", __func__, l.TotalZeroSizeCompressedReadBandwidth);
	dml_log_verbose!("DML::%s: MaximumEffectiveCompressionLuma = %f\n", __func__, l.MaximumEffectiveCompressionLuma);
	dml_log_verbose!("DML::%s: MaximumEffectiveCompressionChroma = %f\n", __func__, l.MaximumEffectiveCompressionChroma);
	dml_log_verbose!("DML::%s: AverageDCCCompressionRate = %f\n", __func__, l.AverageDCCCompressionRate);
	dml_log_verbose!("DML::%s: AverageDCCZeroSizeFraction = %f\n", __func__, l.AverageDCCZeroSizeFraction);

	dml_log_verbose!("DML::%s: CompbufReservedSpace64B = %u (%f kbytes)\n", __func__, p.CompbufReservedSpace64B, p.CompbufReservedSpace64B * 64 / 1024.0);
	dml_log_verbose!("DML::%s: CompbufReservedSpaceZs = %u\n", __func__, p.CompbufReservedSpaceZs);
	dml_log_verbose!("DML::%s: CompressedBufferSizeInkByte = %u kbytes\n", __func__, p.CompressedBufferSizeInkByte);
	dml_log_verbose!("DML::%s: ROBBufferSizeInKByte = %u kbytes\n", __func__, p.ROBBufferSizeInKByte);
	if (l.AverageDCCZeroSizeFraction == 1) {
		l.AverageZeroSizeCompressionRate = l.TotalZeroSizeRequestReadBandwidth / l.TotalZeroSizeCompressedReadBandwidth;
		l.EffectiveCompressedBufferSize = (f64)p.MetaFIFOSizeInKEntries * 1024 * 64 * l.AverageZeroSizeCompressionRate + ((f64)p.ZeroSizeBufferEntries - p.CompbufReservedSpaceZs) * 64 * l.AverageZeroSizeCompressionRate;


	} else if (l.AverageDCCZeroSizeFraction > 0) {
		l.AverageZeroSizeCompressionRate = l.TotalZeroSizeRequestReadBandwidth / l.TotalZeroSizeCompressedReadBandwidth;
		l.EffectiveCompressedBufferSize = math_min2((f64)p.CompressedBufferSizeInkByte * 1024 * l.AverageDCCCompressionRate,
				(f64)p.MetaFIFOSizeInKEntries * 1024 * 64 / (l.AverageDCCZeroSizeFraction / l.AverageZeroSizeCompressionRate + 1 / l.AverageDCCCompressionRate)) +
						(p.rob_alloc_compressed ? math_min2(((f64)p.ROBBufferSizeInKByte * 1024 - p.CompbufReservedSpace64B * 64) * l.AverageDCCCompressionRate,
								((f64)p.ZeroSizeBufferEntries - p.CompbufReservedSpaceZs) * 64 / (l.AverageDCCZeroSizeFraction / l.AverageZeroSizeCompressionRate))
								: ((f64)p.ROBBufferSizeInKByte * 1024 - p.CompbufReservedSpace64B * 64));


		dml_log_verbose!("DML::%s: min 1 = %f\n", __func__, p.CompressedBufferSizeInkByte * 1024 * l.AverageDCCCompressionRate);
		dml_log_verbose!("DML::%s: min 2 = %f\n", __func__, p.MetaFIFOSizeInKEntries * 1024 * 64 / (l.AverageDCCZeroSizeFraction / l.AverageZeroSizeCompressionRate + 1 / l.AverageDCCCompressionRate));
		dml_log_verbose!("DML::%s: min 3 = %d\n", __func__, (p.ROBBufferSizeInKByte * 1024 - p.CompbufReservedSpace64B * 64));
		dml_log_verbose!("DML::%s: min 4 = %f\n", __func__, (p.ZeroSizeBufferEntries - p.CompbufReservedSpaceZs) * 64 / (l.AverageDCCZeroSizeFraction / l.AverageZeroSizeCompressionRate));
	} else {
		l.EffectiveCompressedBufferSize = math_min2((f64)p.CompressedBufferSizeInkByte * 1024 * l.AverageDCCCompressionRate,
				(f64)p.MetaFIFOSizeInKEntries * 1024 * 64 * l.AverageDCCCompressionRate) +
						((f64)p.ROBBufferSizeInKByte * 1024 - p.CompbufReservedSpace64B * 64) * (p.rob_alloc_compressed ? l.AverageDCCCompressionRate : 1.0);

		dml_log_verbose!("DML::%s: min 1 = %f\n", __func__, p.CompressedBufferSizeInkByte * 1024 * l.AverageDCCCompressionRate);
		dml_log_verbose!("DML::%s: min 2 = %f\n", __func__, p.MetaFIFOSizeInKEntries * 1024 * 64 * l.AverageDCCCompressionRate);
	}

	dml_log_verbose!("DML::%s: MetaFIFOSizeInKEntries = %u\n", __func__, p.MetaFIFOSizeInKEntries);
	dml_log_verbose!("DML::%s: ZeroSizeBufferEntries = %u\n", __func__, p.ZeroSizeBufferEntries);
	dml_log_verbose!("DML::%s: AverageZeroSizeCompressionRate = %f\n", __func__, l.AverageZeroSizeCompressionRate);
	dml_log_verbose!("DML::%s: EffectiveCompressedBufferSize = %f (%f kbytes)\n", __func__, l.EffectiveCompressedBufferSize, l.EffectiveCompressedBufferSize / 1024.0);

	*p.StutterPeriod = 0;

	for u32 k = 0;  k < p.NumberOfActiveSurfaces;  ++k { {
		l.LinesInDETY = ((f64)p.DETBufferSizeY[k] + (p.UnboundedRequestEnabled == true ? l.EffectiveCompressedBufferSize : 0) * p.ReadBandwidthSurfaceLuma[k] / p.TotalDataReadBandwidth) / p.BytePerPixelDETY[k] / p.SwathWidthY[k];
		l.LinesInDETYRoundedDownToSwath = math_floor2(l.LinesInDETY, p.SwathHeightY[k]);
		l.DETBufferingTimeY = l.LinesInDETYRoundedDownToSwath * ((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.h_total / ((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.pixel_clock_khz / 1000)) / p.display_cfg.plane_descriptors[k].composition.scaler_info.plane0.v_ratio;
		at_least_one_single_pipe_single_plane_surface |= (p.DPPPerSurface[k] == 1) && (p.ReadBandwidthSurfaceChroma[k] == 0);
		dml_log_verbose!("DML::%s: k=%u, DETBufferSizeY = %u (%u kbytes)\n", __func__, k, p.DETBufferSizeY[k], p.DETBufferSizeY[k] / 1024);
		dml_log_verbose!("DML::%s: k=%u, BytePerPixelDETY = %f\n", __func__, k, p.BytePerPixelDETY[k]);
		dml_log_verbose!("DML::%s: k=%u, SwathWidthY = %u\n", __func__, k, p.SwathWidthY[k]);
		dml_log_verbose!("DML::%s: k=%u, ReadBandwidthSurfaceLuma = %f\n", __func__, k, p.ReadBandwidthSurfaceLuma[k]);
		dml_log_verbose!("DML::%s: k=%u, TotalDataReadBandwidth = %f\n", __func__, k, p.TotalDataReadBandwidth);
		dml_log_verbose!("DML::%s: k=%u, LinesInDETY = %f\n", __func__, k, l.LinesInDETY);
		dml_log_verbose!("DML::%s: k=%u, LinesInDETYRoundedDownToSwath = %f\n", __func__, k, l.LinesInDETYRoundedDownToSwath);
		dml_log_verbose!("DML::%s: k=%u, VRatio = %f\n", __func__, k, p.display_cfg.plane_descriptors[k].composition.scaler_info.plane0.v_ratio);
		dml_log_verbose!("DML::%s: k=%u, DETBufferingTimeY = %f\n", __func__, k, l.DETBufferingTimeY);

		if (!FoundCriticalSurface || l.DETBufferingTimeY < *p.StutterPeriod) {
			bool isInterlaceTiming = p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.interlaced && !p.ProgressiveToInterlaceUnitInOPP;

			FoundCriticalSurface = true;
			*p.StutterPeriod = l.DETBufferingTimeY;
			l.FrameTimeCriticalSurface = (isInterlaceTiming ? math_floor2((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.v_total / 2.0, 1.0) : p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.v_total) * (f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.h_total / ((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.pixel_clock_khz / 1000);
			l.VActiveTimeCriticalSurface = (isInterlaceTiming ? math_floor2((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.v_active / 2.0, 1.0) : p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.v_active) * (f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.h_total / ((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.pixel_clock_khz / 1000);
			l.BytePerPixelYCriticalSurface = p.BytePerPixelY[k];
			l.SwathWidthYCriticalSurface = p.SwathWidthY[k];
			l.SwathHeightYCriticalSurface = p.SwathHeightY[k];
			l.BlockWidth256BytesYCriticalSurface = p.BlockWidth256BytesY[k];
			l.DETBufferSizeYCriticalSurface = p.DETBufferSizeY[k];
			l.MinTTUVBlankCriticalSurface = p.MinTTUVBlank[k];
			l.SinglePlaneCriticalSurface = (p.ReadBandwidthSurfaceChroma[k] == 0);
			l.SinglePipeCriticalSurface = (p.DPPPerSurface[k] == 1);

			dml_log_verbose!("DML::%s: k=%u, FoundCriticalSurface = %u\n", __func__, k, FoundCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, StutterPeriod = %f\n", __func__, k, *p.StutterPeriod);
			dml_log_verbose!("DML::%s: k=%u, MinTTUVBlankCriticalSurface = %f\n", __func__, k, l.MinTTUVBlankCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, FrameTimeCriticalSurface= %f\n", __func__, k, l.FrameTimeCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, VActiveTimeCriticalSurface = %f\n", __func__, k, l.VActiveTimeCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, BytePerPixelYCriticalSurface = %u\n", __func__, k, l.BytePerPixelYCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, SwathWidthYCriticalSurface = %f\n", __func__, k, l.SwathWidthYCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, SwathHeightYCriticalSurface = %f\n", __func__, k, l.SwathHeightYCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, BlockWidth256BytesYCriticalSurface = %u\n", __func__, k, l.BlockWidth256BytesYCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, SinglePlaneCriticalSurface = %u\n", __func__, k, l.SinglePlaneCriticalSurface);
			dml_log_verbose!("DML::%s: k=%u, SinglePipeCriticalSurface = %u\n", __func__, k, l.SinglePipeCriticalSurface);
		}
	}

	// for bounded req, the stutter period is calculated only based on DET size, but during burst there can be some return inside ROB/compressed buffer
	// stutter period is calculated only on the det sizing
	// if (cdb + rob >= det) the stutter burst will be absorbed by the cdb + rob which is before decompress
	// else
	// the cdb + rob part will be in compressed rate with urg bw (idea bw)
	// the det part will be return at uncompressed rate with 64B/dcfclk
	//
	// for unbounded req, the stutter period should be calculated as total of CDB+ROB+DET, so the term "PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer"
	// should be == EffectiveCompressedBufferSize which will returned a compressed rate, the rest of stutter period is from the DET will be returned at uncompressed rate with 64B/dcfclk

	l.PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer = math_min2(*p.StutterPeriod * p.TotalDataReadBandwidth, l.EffectiveCompressedBufferSize);
	dml_log_verbose!("DML::%s: AverageDCCCompressionRate = %f\n", __func__, l.AverageDCCCompressionRate);
	dml_log_verbose!("DML::%s: StutterPeriod*TotalDataReadBandwidth = %f (%f kbytes)\n", __func__, *p.StutterPeriod * p.TotalDataReadBandwidth, (*p.StutterPeriod * p.TotalDataReadBandwidth) / 1024.0);
	dml_log_verbose!("DML::%s: EffectiveCompressedBufferSize = %f (%f kbytes)\n", __func__, l.EffectiveCompressedBufferSize, l.EffectiveCompressedBufferSize / 1024.0);
	dml_log_verbose!("DML::%s: PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer = %f (%f kbytes)\n", __func__, l.PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer, l.PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer / 1024);
	dml_log_verbose!("DML::%s: ReturnBW = %f\n", __func__, p.ReturnBW);
	dml_log_verbose!("DML::%s: TotalDataReadBandwidth = %f\n", __func__, p.TotalDataReadBandwidth);
	dml_log_verbose!("DML::%s: TotalRowReadBandwidth = %f\n", __func__, l.TotalRowReadBandwidth);
	dml_log_verbose!("DML::%s: DCFCLK = %f\n", __func__, p.DCFCLK);

	l.StutterBurstTime = l.PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer
			/ (p.ReturnBW * (p.hw_debug5 ? 1 : l.AverageDCCCompressionRate)) +
			(*p.StutterPeriod * p.TotalDataReadBandwidth - l.PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer)
			/ math_min2(p.DCFCLK * 64, p.ReturnBW * (p.hw_debug5 ? 1 : l.AverageDCCCompressionRate)) +
			*p.StutterPeriod * l.TotalRowReadBandwidth / p.ReturnBW;
	dml_log_verbose!("DML::%s: Part 1 = %f\n", __func__, l.PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer / p.ReturnBW / (p.hw_debug5 ? 1 : l.AverageDCCCompressionRate));
	dml_log_verbose!("DML::%s: Part 2 = %f\n", __func__, (*p.StutterPeriod * p.TotalDataReadBandwidth - l.PartOfUncompressedPixelBurstThatFitsInROBAndCompressedBuffer) / (p.DCFCLK * 64));
	dml_log_verbose!("DML::%s: Part 3 = %f\n", __func__, *p.StutterPeriod * l.TotalRowReadBandwidth / p.ReturnBW);
	dml_log_verbose!("DML::%s: StutterBurstTime = %f\n", __func__, l.StutterBurstTime);
	l.TotalActiveWriteback = 0;
	memset(l.stream_visited, 0, DML2_MAX_PLANES * core::mem::size_of::<bool>());

	for u32 k = 0;  k < p.NumberOfActiveSurfaces;  ++k { {
		if (!l.stream_visited[p.display_cfg.plane_descriptors[k].stream_index]) {

			for u32 j = 0;  j < p.display_cfg.stream_descriptors[k].writeback.active_writebacks_per_stream;  j++ {
				l.TotalActiveWriteback = l.TotalActiveWriteback + 1;

			if (TotalNumberOfActiveOTG == 0) { // first otg
				SinglePixelClock = ((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.pixel_clock_khz / 1000);
				SingleHTotal = p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.h_total;
				SingleVTotal = p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.v_total;
			} else if (SinglePixelClock != ((f64)p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.pixel_clock_khz / 1000) ||
					SingleHTotal != p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.h_total ||
					SingleVTotal != p.display_cfg.stream_descriptors[p.display_cfg.plane_descriptors[k].stream_index].timing.v_total) {
				SameTiming = false;
			}
			TotalNumberOfActiveOTG = TotalNumberOfActiveOTG + 1;
			l.stream_visited[p.display_cfg.plane_descriptors[k].stream_index] = 1;
		}
	}

	if (l.TotalActiveWriteback == 0) {
		dml_log_verbose!("DML::%s: SRExitTime = %f\n", __func__, p.SRExitTime);
		dml_log_verbose!("DML::%s: SRExitZ8Time = %f\n", __func__, p.SRExitZ8Time);
		dml_log_verbose!("DML::%s: SRExitTimeLowPower = %f\n", __func__, p.SRExitTime);
		dml_log_verbose!("DML::%s: StutterPeriod = %f\n", __func__, *p.StutterPeriod);
		*p.StutterEfficiencyNotIncludingVBlank = math_max2(0., 1 - (p.SRExitTime + l.StutterBurstTime) / *p.StutterPeriod) * 100;
		*p.Z8StutterEfficiencyNotIncludingVBlank = math_max2(0., 1 - (p.SRExitZ8Time + l.StutterBurstTime) / *p.StutterPeriod) * 100;
		*p.LowPowerStutterEfficiencyNotIncludingVBlank = math_max2(0., 1 - (p.SRExitTimeLowPower + l.StutterBurstTime) / *p.StutterPeriod) * 100;
		*p.NumberOfStutterBurstsPerFrame = (*p.StutterEfficiencyNotIncludingVBlank > 0 ? (u32)(math_ceil2(l.VActiveTimeCriticalSurface / *p.StutterPeriod, 1)) : 0);
		*p.Z8NumberOfStutterBurstsPerFrame = (*p.Z8StutterEfficiencyNotIncludingVBlank > 0 ? (u32)(math_ceil2(l.VActiveTimeCriticalSurface / *p.StutterPeriod, 1)) : 0);
		*p.LowPowerNumberOfStutterBurstsPerFrame = (*p.LowPowerStutterEfficiencyNotIncludingVBlank > 0 ? (u32)(math_ceil2(l.VActiveTimeCriticalSurface / *p.StutterPeriod, 1)) : 0);

	} else {
		*p.StutterEfficiencyNotIncludingVBlank = 0.0;
		*p.Z8StutterEfficiencyNotIncludingVBlank = 0.0;
		*p.LowPowerStutterEfficiencyNotIncludingVBlank = 0.0;
		*p.NumberOfStutterBurstsPerFrame = 0;
		*p.Z8NumberOfStutterBurstsPerFrame = 0;
		*p.LowPowerNumberOfStutterBurstsPerFrame = 0;
	}
	dml_log_verbose!("DML::%s: VActiveTimeCriticalSurface = %f\n", __func__, l.VActiveTimeCriticalSurface);
	dml_log_verbose!("DML::%s: StutterEfficiencyNotIncludingVBlank = %f\n", __func__, *p.StutterEfficiencyNotIncludingVBlank);
	dml_log_verbose!("DML::%s: Z8StutterEfficiencyNotIncludingVBlank = %f\n", __func__, *p.Z8StutterEfficiencyNotIncludingVBlank);
	dml_log_verbose!("DML::%s: LowPowerStutterEfficiencyNotIncludingVBlank = %f\n", __func__, *p.LowPowerStutterEfficiencyNotIncludingVBlank);
	dml_log_verbose!("DML::%s: NumberOfStutterBurstsPerFrame = %u\n", __func__, *p.NumberOfStutterBurstsPerFrame);
	dml_log_verbose!("DML::%s: Z8NumberOfStutterBurstsPerFrame = %u\n", __func__, *p.Z8NumberOfStutterBurstsPerFrame);
	dml_log_verbose!("DML::%s: LowPowerNumberOfStutterBurstsPerFrame = %u\n", __func__, *p.LowPowerNumberOfStutterBurstsPerFrame);

	if (*p.StutterEfficiencyNotIncludingVBlank > 0) {
		if (!((p.SynchronizeTimings || TotalNumberOfActiveOTG == 1) && SameTiming)) {
			*p.StutterEfficiency = *p.StutterEfficiencyNotIncludingVBlank;
		} else {
			*p.StutterEfficiency = (1 - (*p.NumberOfStutterBurstsPerFrame * p.SRExitTime + l.StutterBurstTime * l.VActiveTimeCriticalSurface / *p.StutterPeriod) / l.FrameTimeCriticalSurface) * 100;
		}
	} else {
		*p.StutterEfficiency = 0;
		*p.NumberOfStutterBurstsPerFrame = 0;
	}

	if (*p.Z8StutterEfficiencyNotIncludingVBlank > 0) {
		if (!((p.SynchronizeTimings || TotalNumberOfActiveOTG == 1) && SameTiming)) {
			*p.Z8StutterEfficiency = *p.Z8StutterEfficiencyNotIncludingVBlank;
		} else {
			*p.Z8StutterEfficiency = (1 - (*p.Z8NumberOfStutterBurstsPerFrame * p.SRExitZ8Time + l.StutterBurstTime * l.VActiveTimeCriticalSurface / *p.StutterPeriod) / l.FrameTimeCriticalSurface) * 100;
		}
	} else {
		*p.Z8StutterEfficiency = 0.;
		*p.Z8NumberOfStutterBurstsPerFrame = 0;
	}

	if (*p.LowPowerStutterEfficiencyNotIncludingVBlank > 0) {
		if (!((p.SynchronizeTimings || TotalNumberOfActiveOTG == 1) && SameTiming)) {
			*p.LowPowerStutterEfficiency = *p.LowPowerStutterEfficiencyNotIncludingVBlank;
		} else {
			*p.LowPowerStutterEfficiency = (1 - (*p.LowPowerNumberOfStutterBurstsPerFrame * p.SRExitTimeLowPower + l.StutterBurstTime * l.VActiveTimeCriticalSurface / *p.StutterPeriod) / l.FrameTimeCriticalSurface) * 100;
		}
	} else {
		*p.LowPowerStutterEfficiency = 0;
		*p.LowPowerNumberOfStutterBurstsPerFrame = 0;
	}

	dml_log_verbose!("DML::%s: TotalNumberOfActiveOTG = %u\n", __func__, TotalNumberOfActiveOTG);
	dml_log_verbose!("DML::%s: SameTiming = %u\n", __func__, SameTiming);
	dml_log_verbose!("DML::%s: SynchronizeTimings = %u\n", __func__, p.SynchronizeTimings);
	dml_log_verbose!("DML::%s: LastZ8StutterPeriod = %f\n", __func__, *p.Z8StutterEfficiencyNotIncludingVBlank > 0 ? l.VActiveTimeCriticalSurface - (*p.Z8NumberOfStutterBurstsPerFrame - 1) * *p.StutterPeriod : 0);
	dml_log_verbose!("DML::%s: Z8StutterEnterPlusExitWatermark = %f\n", __func__, p.Z8StutterEnterPlusExitWatermark);
	dml_log_verbose!("DML::%s: StutterBurstTime = %f\n", __func__, l.StutterBurstTime);
	dml_log_verbose!("DML::%s: StutterPeriod = %f\n", __func__, *p.StutterPeriod);
	dml_log_verbose!("DML::%s: StutterEfficiency = %f\n", __func__, *p.StutterEfficiency);
	dml_log_verbose!("DML::%s: Z8StutterEfficiency = %f\n", __func__, *p.Z8StutterEfficiency);
	dml_log_verbose!("DML::%s: StutterEfficiencyNotIncludingVBlank = %f\n", __func__, *p.StutterEfficiencyNotIncludingVBlank);
	dml_log_verbose!("DML::%s: LowPowerStutterEfficiency = %f\n", __func__, *p.LowPowerStutterEfficiency);
	dml_log_verbose!("DML::%s: Z8NumberOfStutterBurstsPerFrame = %u\n", __func__, *p.Z8NumberOfStutterBurstsPerFrame);

	*p.DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE = !(!p.UnboundedRequestEnabled && (TotalNumberOfActiveOTG == 1) && at_least_one_single_pipe_single_plane_surface);

	dml_log_verbose!("DML::%s: DETBufferSizeYCriticalSurface = %u\n", __func__, l.DETBufferSizeYCriticalSurface);
	dml_log_verbose!("DML::%s: PixelChunkSizeInKByte = %u\n", __func__, p.PixelChunkSizeInKByte);
	dml_log_verbose!("DML::%s: DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE = %u\n", __func__, *p.DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE);
}

unsafe fn dcn6_get_watermarks(*const dml2_display_cfgdisplay_cfg, *const dml2_core_internal_display_mode_libmode_lib, *const dml2_utm_soc_bbutm_soc_bb, dml2_dchub_watermark_regs *out)
{
	dcn6_rq_dlg_get_wm_regs(display_cfg, mode_lib, utm_soc_bb, out);
}

unsafe fn dcn6_calculate_excess_vactive_bandwidth_required(
	*const dml2_display_cfgdisplay_cfg,
	u32 bytes_required_l[dml2_pstate_type_count][DML2_MAX_PLANES],
	u32 bytes_required_c[dml2_pstate_type_count][DML2_MAX_PLANES],
	/* outputs */
	f64 excess_vactive_fill_bw_l[],
	f64 excess_vactive_fill_bw_c[])
{
	u32 plane_index;
	enum dml2_pstate_type pstate_type;

	for plane_index = 0;  plane_index < display_cfg.num_planes;  plane_index++ { {
		excess_vactive_fill_bw_l[plane_index] = 0.0;
		excess_vactive_fill_bw_c[plane_index] = 0.0;

		for pstate_type = 0;  pstate_type < dml2_pstate_type_count;  pstate_type++ { {
			if (display_cfg.plane_descriptors[plane_index].overrides.max_vactive_det_fill_delay_us[pstate_type] > 0) {
				excess_vactive_fill_bw_l[plane_index] = math_max2(
					(f64)bytes_required_l[pstate_type][plane_index] /
					(f64)display_cfg.plane_descriptors[plane_index].overrides.max_vactive_det_fill_delay_us[pstate_type],
					excess_vactive_fill_bw_l[plane_index]);
				excess_vactive_fill_bw_c[plane_index] = math_max2(
					(f64)bytes_required_c[pstate_type][plane_index] /
					(f64)display_cfg.plane_descriptors[plane_index].overrides.max_vactive_det_fill_delay_us[pstate_type],
					excess_vactive_fill_bw_c[plane_index]);
			}
		}
	}
}

unsafe fn dcn6_calculate_pstate_schedule_windows(
		int num_active_planes,
		const u32 v_blank_start[DML2_MAX_PLANES],
		const u32 v_blank_end[DML2_MAX_PLANES],
		const f64 otg_vline_time_us[DML2_MAX_PLANES],
		const f64 det_fill_delay_us[DML2_MAX_PLANES],
		const f64 reserved_vblank_us[DML2_MAX_PLANES],
		const f64 blackout_us,
		// Outputs
		f64 allow_start_us[DML2_MAX_PLANES],
		f64 allow_end_us[DML2_MAX_PLANES])
{
	int k;
	int allow_start_otg_vlines, allow_end_otg_vlines;
	int det_fill_delay_otg_vlines;
	int blackout_otg_vlines;
	int reserved_vblank_otg_vlines;

	/**
	 * Calculate allow start and end for pstate in vactive:
	 *
	 * |vblank end                                         vblank start|
	 * |<------------------------ vactive ---------------------------.|
	 * |<-- det fill delay .|<----- allow window ----.|<----------- blackout -----------.|
	 * |                     |allow start      allow end|              |<- reserved blank .|
	 */
	for k = 0;  k < num_active_planes;  k++ { {
		/* Calculate the allow window in units of vlines */
		blackout_otg_vlines = (int)(math_ceil(blackout_us / otg_vline_time_us[k]));
		det_fill_delay_otg_vlines = (int)(math_ceil(det_fill_delay_us[k] / otg_vline_time_us[k]));
		reserved_vblank_otg_vlines = (int)(math_ceil(reserved_vblank_us[k] / otg_vline_time_us[k]));

		allow_start_otg_vlines = v_blank_end[k] + det_fill_delay_otg_vlines;
		allow_end_otg_vlines = v_blank_start[k] - (blackout_otg_vlines - reserved_vblank_otg_vlines);

		/* Convert them back to time since the start of a frame */
		allow_start_us[k] = allow_start_otg_vlines * otg_vline_time_us[k];
		allow_end_us[k] = allow_end_otg_vlines * otg_vline_time_us[k];
	}
}

unsafe fn dcn6_calculate_pstate_schedule_admissibility(
		uint32_t num_active_planes,
		f64 max_allow_delay_us,
		f64 min_allow_width_us,
		const uint32_t timing_group_id[DML2_MAX_PLANES],
		uint32_t timing_group_count,
		const f64 frame_time_us[DML2_MAX_PLANES],
		const f64 allow_start_us[DML2_MAX_PLANES],
		const f64 allow_end_us[DML2_MAX_PLANES],
		const enum dml2_pstate_method pstate_method[DML2_MAX_PLANES],
		const bool is_drr[DML2_MAX_DCN_PIPES],
		// Output
		f64 allow_window_us[DML2_MAX_DCN_PIPES],
		f64 disallow_window_us[DML2_MAX_DCN_PIPES],
		bool *pstate_admissible)
{
	u32 cur_id = 0, other_id = 0;
	u32 k = 0, i = 0;
	u32 sorted[DML2_MAX_DCN_PIPES]; // group IDs sorted by disallow window size, from highest to lowest
	f64 sum_of_disallow_windows_us = 0.0;
	f64 sum_of_allow_windows_us = 0.0;

	/* Initialize as not admissible first */
	*pstate_admissible = false;

	/**
	 * Calculate the allow and disallow window for each timing group.
	 * A timing group may contain multiple planes rendered under a synchronized and identical timing framework.
	 * Each plane has its own allow window slack described by allow_start_us and allow_end_us.
	 * The goal is to find the intersection of allow windows across all planes in a timing group. This is used as
	 * a key data point to determine the p-state admissibility result.
	 * Here is an example of 3 planes in the same timing group and how the allow window is calculated:
	 *             |----------------------------frame_time_us-----------------------------|
	 * plane0      |XXXXXXXXXXXXXX|----------- allow window 0 ------------------|XXXXXXXXX|
	 * plane1      |XXXX|--------------------- allow window 1 -------------|XXXXXXXXXXXXXX|
	 * plane2      |XXXXXXXX|----------------- allow window 2 -----------|XXXXXXXXXXXXXXXX|
	 * group       |XXXXXXXXXXXXXX|----------- allow window (group)------|XXXXXXXXXXXXXXXX|
	 */
	memset(allow_window_us, 0, core::mem::size_of::<f64>() * DML2_MAX_DCN_PIPES);
	memset(disallow_window_us, 0, core::mem::size_of::<f64>() * DML2_MAX_DCN_PIPES);
	for cur_id = 0;  cur_id < timing_group_count;  cur_id++ { {
		f64 group_allow_start_us = 0.0;
		f64 group_allow_end_us = 0.0;
		f64 group_frame_time_us = 0.0;
		bool first = true;

		for k = 0;  k < num_active_planes;  k++ { {
			/* Skip planes that are not part of the current timing group */
			if (timing_group_id[k] != cur_id)
				continue;
			if (first) {
				group_frame_time_us = frame_time_us[k];
				group_allow_start_us = allow_start_us[k];
				group_allow_end_us = allow_end_us[k];
				first = false;
			} else {
				group_allow_start_us = math_max2(group_allow_start_us, allow_start_us[k]);
				group_allow_end_us = math_min2(group_allow_end_us, allow_end_us[k]);
			}
		}

		allow_window_us[cur_id] = group_allow_end_us - group_allow_start_us;
		if (allow_window_us[cur_id] > group_frame_time_us)
			/* Clamp allow window to frame time */
			allow_window_us[cur_id] = group_frame_time_us;
		disallow_window_us[cur_id] = group_frame_time_us - allow_window_us[cur_id];
	}

	/* Calculate the total sum of all allow and disallow windows */
	for cur_id = 0;  cur_id < timing_group_count;  cur_id++ { {
		sum_of_disallow_windows_us += disallow_window_us[cur_id];
		sum_of_allow_windows_us += allow_window_us[cur_id];
	}

	/**
	 * Check 1 - Every group has a positive allow window greater than or equal to the minimum allow width.
	 */
	for cur_id = 0;  cur_id < timing_group_count;  cur_id++ {
		if (allow_window_us[cur_id] <= 0
				|| allow_window_us[cur_id] < min_allow_width_us)
			return;

	/**
	 * Check 2 - Every group has a disallow window within the FAMS maximum scheduling latency budget.
	 */
	for cur_id = 0;  cur_id < timing_group_count;  cur_id++ {
		if (disallow_window_us[cur_id] > max_allow_delay_us)
			return;

	/**
	 * Schedulable Case 1 - Single group
	 */
	if (timing_group_count == 1) {
		*pstate_admissible = true;
		return;
	}

	/**
	 * Schedulable Case 2 - Positive allow fragment after recursive slice halving.
	 * Passing Conditions:
	 * 1. Total sum of disallow windows across all groups is less than the FAMS maximum scheduling latency budget.
	 * 2. Every group's remaining allow fragment is still positive after recursive slicing and halving.
	 */

	if (sum_of_disallow_windows_us < max_allow_delay_us) {
		*pstate_admissible = true;
		/* Bubble sort group IDs by disallow window size, from highest to lowest */
		for cur_id = 0;  cur_id < timing_group_count;  cur_id++ {
			sorted[cur_id] = cur_id;
		for cur_id = 0;  cur_id < timing_group_count;  cur_id++ {
			for other_id = cur_id + 1;  other_id < timing_group_count;  other_id++ {
				if (disallow_window_us[sorted[cur_id]] < disallow_window_us[sorted[other_id]])
					swap(sorted[cur_id], sorted[other_id]);
		/* Continuously slice each group's allow fragment */
		for cur_id = 0;  cur_id < timing_group_count;  cur_id++ { {
			f64 allow_fragment_us = allow_window_us[cur_id];

			for i = 0;  i < timing_group_count;  i++ { {
				other_id = sorted[i];

				if (cur_id == other_id || disallow_window_us[other_id] <= 0.0)
					continue;

				// slicing
				allow_fragment_us -= disallow_window_us[other_id];
				// halving
				allow_fragment_us /= 2;
				if (allow_window_us[other_id] < allow_fragment_us)
					allow_fragment_us = allow_window_us[other_id];
			}

			if (allow_fragment_us <= 0.0) {
				*pstate_admissible = false;
				break;
			}
		}

		if (*pstate_admissible)
			return;
	}

	/**
	 * Schedulable Case 3 - Nesting frame times.
	 * Passing Conditions:
	 * 1. Total sum of disallow windows across all groups is less than the FAMS maximum scheduling latency budget.
	 * 2. Every group's frame time can be fully contained by another group's allow window recursively, like nesting dolls.
	 * 3. The nested groups must not have DRR enabled and active.
	 */
	if (sum_of_disallow_windows_us < max_allow_delay_us) {
		*pstate_admissible = true;
		/* Bubble sort group IDs by frame time, from highest to lowest */
		for cur_id = 0;  cur_id < timing_group_count;  cur_id++ {
			sorted[cur_id] = cur_id;
		for cur_id = 0;  cur_id < timing_group_count;  cur_id++ { {
			f64 cur_frame_time_us = allow_window_us[cur_id] + disallow_window_us[cur_id];
			for other_id = cur_id + 1;  other_id < timing_group_count;  other_id++ { {
				f64 other_frame_time_us = allow_window_us[other_id] + disallow_window_us[other_id];
				if (cur_frame_time_us < other_frame_time_us)
					swap(sorted[cur_id], sorted[other_id]);
			}
		}
		/* Check nesting */
		for cur_id = 0;  cur_id < timing_group_count - 1;  cur_id++ { {
			other_id = cur_id + 1;
			if (allow_window_us[sorted[cur_id]] < allow_window_us[sorted[other_id]] + disallow_window_us[sorted[other_id]]) {
				*pstate_admissible = false;
				break;
			}
		}

		/* Starting from the first nested group, check DRR support */
		for cur_id = 1;  cur_id < timing_group_count;  cur_id++ {
			if (is_drr[sorted[cur_id]]) {
				*pstate_admissible = false;
				break;
			}

		if (*pstate_admissible)
			return;
	}

	/**
	 * Schedulable Case 4 - Non-harmonic phase drifting.
	 * Passing Conditions:
	 * 1. Applicable only for 2 timing groups.
	 * 2. The small frame time does not perfectly align with the large frame time, so the allow and disallow
	 *    windows drift in and out of phase across frames, providing opportunities for p-state changes.
	 * 3. The delay to recover from a worst-case phase shift to a common allow window is less than the FAMS
	 *    maximum scheduling latency budget.
	 * 4. The drift per frame is smaller than the combined allow window (p-state can complete within the window
	 *    as it drifts through).
	 */
	if (timing_group_count == 2
		&& !((1 << pstate_method[0] | 1 << pstate_method[1]) & PMO_FW_STRATEGY_MASK)) { // neither is FW strategy
		int small_group_id = 0;
		int large_group_id = 1;
		f64 shift_per_frame = 0.0;
		f64 max_shift_us = 0.0;
		f64 max_disallow_window_us = 0.0;

		*pstate_admissible = true;
		if (allow_window_us[small_group_id] + disallow_window_us[small_group_id]
				> allow_window_us[large_group_id] + disallow_window_us[large_group_id])
			swap(small_group_id, large_group_id);

		shift_per_frame = math_mod((allow_window_us[large_group_id] + disallow_window_us[large_group_id]),
				(allow_window_us[small_group_id] + disallow_window_us[small_group_id]));

		max_shift_us = disallow_window_us[large_group_id] - allow_window_us[small_group_id];
		max_disallow_window_us = max_shift_us / shift_per_frame * (allow_window_us[large_group_id] + disallow_window_us[large_group_id]);

		if (shift_per_frame == 0.0)
			/* Perfectly aligned, no drifting */
			*pstate_admissible = false;
		if (shift_per_frame >= sum_of_allow_windows_us)
			/* Drifting, but the shift per frame exceeds the total allow window; no opportunity for p-state change */
			*pstate_admissible = false;

		if (max_disallow_window_us >= max_allow_delay_us)
			/**
			 * The delay to recover from a worst-case phase shift to a common allow window exceeds the FAMS
			 * maximum scheduling latency budget.
			 */
			*pstate_admissible = false;

		if (*pstate_admissible)
			return;
	}

	return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
