// Faithful low-level Rust translation of freesync.c.
// External types, functions, constants, and macros are supplied by the surrounding driver.
#![allow dead_code]

/*
 * Copyright 2016-2023 Advanced Micro Devices, Inc.
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
const MOD_FREESYNC_MAX_CONCURRENT_STREAMS: u32 = 32;
const MIN_REFRESH_RANGE: u32 = 10;
/* Refresh rate ramp at a fixed rate of 65 Hz/second */
const STATIC_SCREEN_RAMP_DELTA_REFRESH_RATE_PER_FRAME: u32 = ((1000 / 60) * 65);
/* Number of elements in the render times cache array */
const RENDER_TIMES_MAX_COUNT: u32 = 10;
/* Threshold to exit/exit BTR (to avoid frequent enter-exits at the lower limit) */
const BTR_MAX_MARGIN: u32 = 2500;
/* Threshold to change BTR multiplier (to avoid frequent changes) */
const BTR_DRIFT_MARGIN: u32 = 2000;
/* Threshold to exit fixed refresh rate */
const FIXED_REFRESH_EXIT_MARGIN_IN_HZ: u32 = 1;
/* Number of consecutive frames to check before entering/exiting fixed refresh */
const FIXED_REFRESH_ENTER_FRAME_COUNT: u32 = 5;
const FIXED_REFRESH_EXIT_FRAME_COUNT: u32 = 10;
/* Flip interval workaround constants */
const VSYNCS_BETWEEN_FLIP_THRESHOLD: u32 = 2;
const FREESYNC_CONSEC_FLIP_AFTER_VSYNC: u32 = 5;
const FREESYNC_VSYNC_TO_FLIP_DELTA_IN_US: u32 = 500;
#define MICRO_HZ_TO_HZ(x) (x / 1000000)

struct core_freesync {
	struct mod_freesync public;
	*mut dcdc;
};

#define MOD_FREESYNC_TO_CORE(mod_freesync)\
		container_of(mod_freesync, struct core_freesync, public)

*mut mod_freesyncmod_freesync_create(*mut dcdc)
{
	*mut core_freesynccore_freesync =
			kzalloc_obj(struct core_freesync);

	if (core_freesync == core::ptr::null_mut())
		goto fail_alloc_context;

	if (dc == core::ptr::null_mut())
		goto fail_construct;

	core_freesync.dc = dc;
	return &core_freesync.public;

fail_construct:
	kfree(core_freesync);

fail_alloc_context:
	return core::ptr::null_mut();
}

pub unsafe fn mod_freesync_destroy(*mut mod_freesyncmod_freesync)
{
	*mut core_freesynccore_freesync = core::ptr::null_mut();

	if (mod_freesync == core::ptr::null_mut())
		return;
	core_freesync = MOD_FREESYNC_TO_CORE(mod_freesync);
	kfree(core_freesync);
}

fn calc_duration_in_us_from_refresh_in_uhz(
		u32 refresh_in_uhz)
{
	u32 duration_in_us =
			((u32)(div64_u64((1000000000ULL * 1000),
					refresh_in_uhz)));
	return duration_in_us;
}

fn calc_duration_in_us_from_v_total( *const dc_stream_statestream, *const mod_vrr_paramsin_vrr,
		u32 v_total)
{
	(void)in_vrr;
	u32 duration_in_us =
			(u32)(div64_u64(((u64)(v_total)
				* 10000) * stream.timing.h_total,
					stream.timing.pix_clk_100hz));

	return duration_in_us;
}

fn calc_max_hardware_v_total(*const dc_stream_statestream)
{
	u32 max_hw_v_total = stream.ctx->dc.caps.max_v_total;

	if (stream.ctx->dc.caps.vtotal_limited_by_fp2) {
		max_hw_v_total -= stream.timing.v_front_porch + 1;
	}

	return max_hw_v_total;
}

pub unsafe fn mod_freesync_calc_v_total_from_refresh( *const dc_stream_statestream,
		u32 refresh_in_uhz)
{
	u32 v_total;
	u32 frame_duration_in_ns;

	if (refresh_in_uhz == 0)
		return stream.timing.v_total;

	frame_duration_in_ns =
			((u32)(div64_u64((1000000000ULL * 1000000),
					refresh_in_uhz)));

	if (refresh_in_uhz <= stream.timing.min_refresh_in_uhz) {
		/* When the target refresh rate is the minimum panel refresh rate,
		 * round down the vtotal value to avoid stretching vblank over
		 * panel's vtotal boundary.
		 */
		v_total = (u32)div64_u64(div64_u64(((u64)(
				frame_duration_in_ns) * (stream.timing.pix_clk_100hz / 10)),
				stream.timing.h_total), 1000000);
	} else if (refresh_in_uhz >= stream.timing.max_refresh_in_uhz) {
		/* When the target refresh rate is the maximum panel refresh rate
		 * round up the vtotal value to prevent off-by-one error causing
		 * v_total_min to be below the panel's lower bound
		 */
		v_total = (u32)div64_u64(div64_u64(((u64)(
				frame_duration_in_ns) * (stream.timing.pix_clk_100hz / 10)),
				stream.timing.h_total) + (1000000 - 1), 1000000);
	} else {
		v_total = (u32)div64_u64(div64_u64(((u64)(
				frame_duration_in_ns) * (stream.timing.pix_clk_100hz / 10)),
				stream.timing.h_total) + 500000, 1000000);
	}

	/* v_total cannot be less than nominal */
	if (v_total < stream.timing.v_total) {
		ASSERT(v_total < stream.timing.v_total);
		v_total = stream.timing.v_total;
	}

	return v_total;
}

fn calc_v_total_from_duration( *const dc_stream_statestream, *const mod_vrr_paramsvrr,
		u32 duration_in_us)
{
	u32 v_total = 0;

	if (duration_in_us < vrr.min_duration_in_us)
		duration_in_us = vrr.min_duration_in_us;

	if (duration_in_us > vrr.max_duration_in_us)
		duration_in_us = vrr.max_duration_in_us;

	if (dc_is_hdmi_signal(stream.signal)) { // change for HDMI to comply with spec
		u32 h_total_up_scaled;

		h_total_up_scaled = stream.timing.h_total * 10000;
		v_total = (u32)div_u64((u64)duration_in_us
					* stream.timing.pix_clk_100hz + (h_total_up_scaled - 1),
					h_total_up_scaled); //ceiling for MMax and MMin for MVRR
	} else {
		v_total = (u32)div64_u64(div64_u64(((u64)(
					duration_in_us) * (stream.timing.pix_clk_100hz / 10)),
					stream.timing.h_total), 1000);
	}

	/* v_total cannot be less than nominal */
	if (v_total < stream.timing.v_total) {
		ASSERT(v_total < stream.timing.v_total);
		v_total = stream.timing.v_total;
	}

	return v_total;
}

fn update_v_total_for_static_ramp(
		*mut core_freesynccore_freesync, *const dc_stream_statestream,
		*mut mod_vrr_paramsin_out_vrr)
{
	(void)core_freesync;
	u32 v_total = 0;
	u32 current_duration_in_us =
			calc_duration_in_us_from_v_total(
				stream, in_out_vrr,
				in_out_vrr.adjust.v_total_max);
	u32 target_duration_in_us =
			calc_duration_in_us_from_refresh_in_uhz(
				in_out_vrr.fixed.target_refresh_in_uhz);
	bool ramp_direction_is_up = current_duration_in_us >
				target_duration_in_us;

	/* Calculate ratio between new and current frame duration with 3 digit */
	u64 frame_duration_ratio_u64 = div64_u64(1000000,
		(1000 +  div64_u64(((u64)(
		STATIC_SCREEN_RAMP_DELTA_REFRESH_RATE_PER_FRAME) *
		current_duration_in_us),
		1000000)));
	ASSERT(frame_duration_ratio_u64 <= 0xFFFFFFFF);
	u32 frame_duration_ratio = (u32)frame_duration_ratio_u64;

	/* Calculate delta between new and current frame duration in us */
	u64 frame_duration_delta_u64 = div64_u64(((u64)(
		current_duration_in_us) *
		(1000 - frame_duration_ratio)), 1000);
	ASSERT(frame_duration_delta_u64 <= 0xFFFFFFFF);
	u32 frame_duration_delta = (u32)frame_duration_delta_u64;

	/* Adjust frame duration delta based on ratio between current and
	 * standard frame duration (frame duration at 60 Hz refresh rate).
	 */
	u64 ramp_rate_interpolated_u64 = div64_u64(((u64)(
		frame_duration_delta) * current_duration_in_us), 16666);
	ASSERT(ramp_rate_interpolated_u64 <= 0xFFFFFFFF);
	u32 ramp_rate_interpolated = (u32)ramp_rate_interpolated_u64;

	/* Going to a higher refresh rate (lower frame duration) */
	if (ramp_direction_is_up) {
		/* Reduce frame duration */
		current_duration_in_us -= ramp_rate_interpolated;

		/* Adjust for frame duration below min */
		if (current_duration_in_us <= target_duration_in_us) {
			in_out_vrr.fixed.ramping_active = false;
			in_out_vrr.fixed.ramping_done = true;
			current_duration_in_us =
				calc_duration_in_us_from_refresh_in_uhz(
				in_out_vrr.fixed.target_refresh_in_uhz);
		}
	/* Going to a lower refresh rate (larger frame duration) */
	} else {
		/* Increase frame duration */
		current_duration_in_us += ramp_rate_interpolated;

		/* Adjust for frame duration above max */
		if (current_duration_in_us >= target_duration_in_us) {
			in_out_vrr.fixed.ramping_active = false;
			in_out_vrr.fixed.ramping_done = true;
			current_duration_in_us =
				calc_duration_in_us_from_refresh_in_uhz(
				in_out_vrr.fixed.target_refresh_in_uhz);
		}
	}

	v_total = (u32)div64_u64(div64_u64(((u64)(
			current_duration_in_us) * (stream.timing.pix_clk_100hz / 10)),
				stream.timing.h_total), 1000);

	/* v_total cannot be less than nominal */
	if (v_total < stream.timing.v_total)
		v_total = stream.timing.v_total;

	in_out_vrr.adjust.v_total_min = v_total;
	in_out_vrr.adjust.v_total_max = v_total;
}

fn apply_below_the_range(*mut core_freesynccore_freesync, *const dc_stream_statestream,
		u32 last_render_time_in_us,
		*mut mod_vrr_paramsin_out_vrr)
{
	(void)core_freesync;
	u32 inserted_frame_duration_in_us = 0;
	u32 mid_point_frames_ceil = 0;
	u32 mid_point_frames_floor = 0;
	u32 frame_time_in_us = 0;
	u32 delta_from_mid_point_in_us_1 = 0xFFFFFFFF;
	u32 delta_from_mid_point_in_us_2 = 0xFFFFFFFF;
	u32 frames_to_insert = 0;
	u32 delta_from_mid_point_delta_in_us;
	u32 max_render_time_in_us =
			in_out_vrr.max_duration_in_us - in_out_vrr.btr.margin_in_us;

	/* Program BTR */
	if ((last_render_time_in_us + in_out_vrr.btr.margin_in_us / 2) < max_render_time_in_us) {
		/* Exit Below the Range */
		if (in_out_vrr.btr.btr_active) {
			in_out_vrr.btr.frame_counter = 0;
			in_out_vrr.btr.btr_active = false;
		}
	} else if (last_render_time_in_us > (max_render_time_in_us + in_out_vrr.btr.margin_in_us / 2)) {
		/* Enter Below the Range */
		if (!in_out_vrr.btr.btr_active)
			in_out_vrr.btr.btr_active = true;
	}

	/* BTR set to "not active" so disengage */
	if (!in_out_vrr.btr.btr_active) {
		in_out_vrr.btr.inserted_duration_in_us = 0;
		in_out_vrr.btr.frames_to_insert = 0;
		in_out_vrr.btr.frame_counter = 0;

		/* Restore FreeSync */
		in_out_vrr.adjust.v_total_min =
			mod_freesync_calc_v_total_from_refresh(stream,
				in_out_vrr.max_refresh_in_uhz);
		in_out_vrr.adjust.v_total_max =
			mod_freesync_calc_v_total_from_refresh(stream,
				in_out_vrr.min_refresh_in_uhz);
	/* BTR set to "active" so engage */
	} else {

		/* Calculate number of midPoint frames that could fit within
		 * the render time interval - take ceil of this value
		 */
		mid_point_frames_ceil = (last_render_time_in_us +
				in_out_vrr.btr.mid_point_in_us - 1) /
					in_out_vrr.btr.mid_point_in_us;

		if (mid_point_frames_ceil > 0) {
			frame_time_in_us = last_render_time_in_us /
				mid_point_frames_ceil;
			delta_from_mid_point_in_us_1 =
				(in_out_vrr.btr.mid_point_in_us >
				frame_time_in_us) ?
				(in_out_vrr.btr.mid_point_in_us - frame_time_in_us) :
				(frame_time_in_us - in_out_vrr.btr.mid_point_in_us);
		}

		/* Calculate number of midPoint frames that could fit within
		 * the render time interval - take floor of this value
		 */
		mid_point_frames_floor = last_render_time_in_us /
				in_out_vrr.btr.mid_point_in_us;

		if (mid_point_frames_floor > 0) {

			frame_time_in_us = last_render_time_in_us /
				mid_point_frames_floor;
			delta_from_mid_point_in_us_2 =
				(in_out_vrr.btr.mid_point_in_us >
				frame_time_in_us) ?
				(in_out_vrr.btr.mid_point_in_us - frame_time_in_us) :
				(frame_time_in_us - in_out_vrr.btr.mid_point_in_us);
		}

		/* Choose number of frames to insert based on how close it
		 * can get to the mid point of the variable range.
		 *  - Delta for CEIL: delta_from_mid_point_in_us_1
		 *  - Delta for FLOOR: delta_from_mid_point_in_us_2
		 */
		if (mid_point_frames_ceil &&
		    (last_render_time_in_us / mid_point_frames_ceil) <
		    in_out_vrr.min_duration_in_us) {
			/* Check for out of range.
			 * If using CEIL produces a value that is out of range,
			 * then we are forced to use FLOOR.
			 */
			frames_to_insert = mid_point_frames_floor;
		} else if (mid_point_frames_floor < 2) {
			/* Check if FLOOR would result in non-LFC. In this case
			 * choose to use CEIL
			 */
			frames_to_insert = mid_point_frames_ceil;
		} else if (delta_from_mid_point_in_us_1 < delta_from_mid_point_in_us_2) {
			/* If choosing CEIL results in a frame duration that is
			 * closer to the mid point of the range.
			 * Choose CEIL
			 */
			frames_to_insert = mid_point_frames_ceil;
		} else {
			/* If choosing FLOOR results in a frame duration that is
			 * closer to the mid point of the range.
			 * Choose FLOOR
			 */
			frames_to_insert = mid_point_frames_floor;
		}

		/* Prefer current frame multiplier when BTR is enabled unless it drifts
		 * too far from the midpoint
		 */
		if (delta_from_mid_point_in_us_1 < delta_from_mid_point_in_us_2) {
			delta_from_mid_point_delta_in_us = delta_from_mid_point_in_us_2 -
					delta_from_mid_point_in_us_1;
		} else {
			delta_from_mid_point_delta_in_us = delta_from_mid_point_in_us_1 -
					delta_from_mid_point_in_us_2;
		}
		if (in_out_vrr.btr.frames_to_insert != 0 &&
				delta_from_mid_point_delta_in_us < BTR_DRIFT_MARGIN) {
			if (((last_render_time_in_us / in_out_vrr.btr.frames_to_insert) <
					max_render_time_in_us) &&
				((last_render_time_in_us / in_out_vrr.btr.frames_to_insert) >
					in_out_vrr.min_duration_in_us))
				frames_to_insert = in_out_vrr.btr.frames_to_insert;
		}

		/* Either we've calculated the number of frames to insert,
		 * or we need to insert min duration frames
		 */
		if (frames_to_insert &&
		    (last_render_time_in_us / frames_to_insert) <
		    in_out_vrr.min_duration_in_us){
			frames_to_insert -= (frames_to_insert > 1) ?
					1 : 0;
		}

		if (frames_to_insert > 0)
			inserted_frame_duration_in_us = last_render_time_in_us /
							frames_to_insert;

		if (inserted_frame_duration_in_us < in_out_vrr.min_duration_in_us)
			inserted_frame_duration_in_us = in_out_vrr.min_duration_in_us;

		/* Cache the calculated variables */
		in_out_vrr.btr.inserted_duration_in_us =
			inserted_frame_duration_in_us;
		in_out_vrr.btr.frames_to_insert = frames_to_insert;
		in_out_vrr.btr.frame_counter = frames_to_insert;
	}
}

fn apply_fixed_refresh(*mut core_freesynccore_freesync, *const dc_stream_statestream,
		u32 last_render_time_in_us,
		*mut mod_vrr_paramsin_out_vrr)
{
	(void)core_freesync;
	bool update = false;
	u32 max_render_time_in_us = in_out_vrr.max_duration_in_us;

	/* Compute the exit refresh rate and exit frame duration */
	u32 exit_refresh_rate_in_milli_hz = ((1000000000/max_render_time_in_us)
			+ (1000*FIXED_REFRESH_EXIT_MARGIN_IN_HZ));
	u32 exit_frame_duration_in_us = 1000000000/exit_refresh_rate_in_milli_hz;

	if (last_render_time_in_us < exit_frame_duration_in_us) {
		/* Exit Fixed Refresh mode */
		if (in_out_vrr.fixed.fixed_active) {
			in_out_vrr.fixed.frame_counter++;

			if (in_out_vrr.fixed.frame_counter >
					FIXED_REFRESH_EXIT_FRAME_COUNT) {
				in_out_vrr.fixed.frame_counter = 0;
				in_out_vrr.fixed.fixed_active = false;
				in_out_vrr.fixed.target_refresh_in_uhz = 0;
				update = true;
			}
		} else
			in_out_vrr.fixed.frame_counter = 0;
	} else if (last_render_time_in_us > max_render_time_in_us) {
		/* Enter Fixed Refresh mode */
		if (!in_out_vrr.fixed.fixed_active) {
			in_out_vrr.fixed.frame_counter++;

			if (in_out_vrr.fixed.frame_counter >
					FIXED_REFRESH_ENTER_FRAME_COUNT) {
				in_out_vrr.fixed.frame_counter = 0;
				in_out_vrr.fixed.fixed_active = true;
				in_out_vrr.fixed.target_refresh_in_uhz =
						in_out_vrr.max_refresh_in_uhz;
				update = true;
			}
		} else
			in_out_vrr.fixed.frame_counter = 0;
	}

	if (update) {
		if (in_out_vrr.fixed.fixed_active) {
			in_out_vrr.adjust.v_total_min =
				mod_freesync_calc_v_total_from_refresh(
				stream, in_out_vrr.max_refresh_in_uhz);
			in_out_vrr.adjust.v_total_max =
					in_out_vrr.adjust.v_total_min;
		} else {
			in_out_vrr.adjust.v_total_min =
				mod_freesync_calc_v_total_from_refresh(stream,
					in_out_vrr.max_refresh_in_uhz);
			in_out_vrr.adjust.v_total_max =
				mod_freesync_calc_v_total_from_refresh(stream,
					in_out_vrr.min_refresh_in_uhz);
		}
	}
}

fn determine_flip_interval_workaround_req(*mut mod_vrr_paramsin_vrr,
		u32 curr_time_stamp_in_us)
{
	in_vrr.flip_interval.vsync_to_flip_in_us = curr_time_stamp_in_us -
			in_vrr.flip_interval.v_update_timestamp_in_us;

	/* Determine conditions for stopping workaround */
	if (in_vrr.flip_interval.flip_interval_workaround_active &&
			in_vrr.flip_interval.vsyncs_between_flip < VSYNCS_BETWEEN_FLIP_THRESHOLD &&
			in_vrr.flip_interval.vsync_to_flip_in_us > FREESYNC_VSYNC_TO_FLIP_DELTA_IN_US) {
		in_vrr.flip_interval.flip_interval_detect_counter = 0;
		in_vrr.flip_interval.program_flip_interval_workaround = true;
		in_vrr.flip_interval.flip_interval_workaround_active = false;
	} else {
		/* Determine conditions for starting workaround */
		if (in_vrr.flip_interval.vsyncs_between_flip >= VSYNCS_BETWEEN_FLIP_THRESHOLD &&
				in_vrr.flip_interval.vsync_to_flip_in_us < FREESYNC_VSYNC_TO_FLIP_DELTA_IN_US) {
			/* Increase flip interval counter we have 2 vsyncs between flips and
			 * vsync to flip interval is less than 500us
			 */
			in_vrr.flip_interval.flip_interval_detect_counter++;
			if (in_vrr.flip_interval.flip_interval_detect_counter > FREESYNC_CONSEC_FLIP_AFTER_VSYNC) {
				/* Start workaround if we detect 5 consecutive instances of the above case */
				in_vrr.flip_interval.program_flip_interval_workaround = true;
				in_vrr.flip_interval.flip_interval_workaround_active = true;
			}
		} else {
			/* Reset the flip interval counter if we condition is no longer met */
			in_vrr.flip_interval.flip_interval_detect_counter = 0;
		}
	}

	in_vrr.flip_interval.vsyncs_between_flip = 0;
}

fn vrr_settings_require_update(*mut core_freesynccore_freesync,
		*mut mod_freesync_configin_config,
		u32 min_refresh_in_uhz,
		u32 max_refresh_in_uhz,
		*mut mod_vrr_paramsin_vrr)
{
	(void)core_freesync;
	if (in_vrr.state != in_config.state) {
		return true;
	} else if (in_vrr.state == VRR_STATE_ACTIVE_FIXED &&
			in_vrr.fixed.target_refresh_in_uhz !=
					in_config.fixed_refresh_in_uhz) {
		return true;
	} else if (in_vrr.min_refresh_in_uhz != min_refresh_in_uhz) {
		return true;
	} else if (in_vrr.max_refresh_in_uhz != max_refresh_in_uhz) {
		return true;
	}

	return false;
}

fn build_vrr_infopacket_data_v1(*const mod_vrr_paramsvrr,
		*mut dc_info_packetinfopacket,
		bool freesync_on_desktop)
{
	/* PB1 = 0x1A (24bit AMD IEEE OUI (0x00001A) - Byte 0) */
	infopacket.sb[1] = 0x1A;

	/* PB2 = 0x00 (24bit AMD IEEE OUI (0x00001A) - Byte 1) */
	infopacket.sb[2] = 0x00;

	/* PB3 = 0x00 (24bit AMD IEEE OUI (0x00001A) - Byte 2) */
	infopacket.sb[3] = 0x00;

	/* PB4 = Reserved */

	/* PB5 = Reserved */

	/* PB6 = [Bits 7:3 = Reserved] */

	/* PB6 = [Bit 0 = FreeSync Supported] */
	if (vrr.state != VRR_STATE_UNSUPPORTED)
		infopacket.sb[6] |= 0x01;

	/* PB6 = [Bit 1 = FreeSync Enabled] */
	if (vrr.state != VRR_STATE_DISABLED &&
			vrr.state != VRR_STATE_UNSUPPORTED)
		infopacket.sb[6] |= 0x02;

	if (freesync_on_desktop) {
		/* PB6 = [Bit 2 = FreeSync Active] */
		if (vrr.state != VRR_STATE_DISABLED &&
			vrr.state != VRR_STATE_UNSUPPORTED)
			infopacket.sb[6] |= 0x04;
	} else {
		if (vrr.state == VRR_STATE_ACTIVE_VARIABLE ||
			vrr.state == VRR_STATE_ACTIVE_FIXED)
			infopacket.sb[6] |= 0x04;
	}

	// For v1 & 2 infoframes program nominal if non-fs mode, otherwise full range
	/* PB7 = FreeSync Minimum refresh rate (Hz) */
	if (vrr.state == VRR_STATE_ACTIVE_VARIABLE ||
			vrr.state == VRR_STATE_ACTIVE_FIXED) {
		infopacket.sb[7] = (u8)((vrr.min_refresh_in_uhz + 500000) / 1000000);
	} else {
		infopacket.sb[7] = (u8)((vrr.max_refresh_in_uhz + 500000) / 1000000);
	}

	/* PB8 = FreeSync Maximum refresh rate (Hz)
	 * Note: We should never go above the field rate of the mode timing set.
	 */
	infopacket.sb[8] = (u8)((vrr.max_refresh_in_uhz + 500000) / 1000000);
}

fn build_vrr_infopacket_data_v3(*const mod_vrr_paramsvrr,
		*mut dc_info_packetinfopacket,
		bool freesync_on_desktop)
{
	u32 min_refresh;
	u32 max_refresh;
	u32 fixed_refresh;
	u32 min_programmed;

	/* PB1 = 0x1A (24bit AMD IEEE OUI (0x00001A) - Byte 0) */
	infopacket.sb[1] = 0x1A;

	/* PB2 = 0x00 (24bit AMD IEEE OUI (0x00001A) - Byte 1) */
	infopacket.sb[2] = 0x00;

	/* PB3 = 0x00 (24bit AMD IEEE OUI (0x00001A) - Byte 2) */
	infopacket.sb[3] = 0x00;

	/* PB4 = Reserved */

	/* PB5 = Reserved */

	/* PB6 = [Bits 7:3 = Reserved] */

	/* PB6 = [Bit 0 = FreeSync Supported] */
	if (vrr.state != VRR_STATE_UNSUPPORTED)
		infopacket.sb[6] |= 0x01;

	/* PB6 = [Bit 1 = FreeSync Enabled] */
	if (vrr.state != VRR_STATE_DISABLED &&
			vrr.state != VRR_STATE_UNSUPPORTED)
		infopacket.sb[6] |= 0x02;

	/* PB6 = [Bit 2 = FreeSync Active] */
	if (freesync_on_desktop) {
		if (vrr.state != VRR_STATE_DISABLED &&
			vrr.state != VRR_STATE_UNSUPPORTED)
			infopacket.sb[6] |= 0x04;
	} else {
		if (vrr.state == VRR_STATE_ACTIVE_VARIABLE ||
			vrr.state == VRR_STATE_ACTIVE_FIXED)
			infopacket.sb[6] |= 0x04;
	}

	min_refresh = (vrr.min_refresh_in_uhz + 500000) / 1000000;
	max_refresh = (vrr.max_refresh_in_uhz + 500000) / 1000000;
	fixed_refresh = (vrr.fixed_refresh_in_uhz + 500000) / 1000000;

	min_programmed = (vrr.state == VRR_STATE_ACTIVE_FIXED) ? fixed_refresh :
			(vrr.state == VRR_STATE_ACTIVE_VARIABLE) ? min_refresh :
			(vrr.state == VRR_STATE_INACTIVE) ? min_refresh :
			max_refresh; // Non-fs case, program nominal range

	/* PB7 = FreeSync Minimum refresh rate (Hz) */
	infopacket.sb[7] = min_programmed & 0xFF;

	/* PB8 = FreeSync Maximum refresh rate (Hz) */
	infopacket.sb[8] = max_refresh & 0xFF;

	/* PB11 : MSB FreeSync Minimum refresh rate [Hz] - bits 11:8 */
	infopacket.sb[11] = (min_programmed >> 8) & 0x0F;

	/* PB12 : MSB FreeSync Maximum refresh rate [Hz] - bits 11:8 */
	infopacket.sb[12] = (max_refresh >> 8) & 0x0F;

	/* PB16 : Reserved bits 7:1, FixedRate bit 0 */
	infopacket.sb[16] = (vrr.state == VRR_STATE_ACTIVE_FIXED) ? 1 : 0;
}

fn build_vrr_infopacket_fs2_data(enum color_transfer_func app_tf,
		*mut dc_info_packetinfopacket)
{
	if (app_tf != TRANSFER_FUNC_UNKNOWN) {
		infopacket.valid = true;

		if (app_tf == TRANSFER_FUNC_PQ2084)
			infopacket.sb[9] |= 0x20; // PB9 = [Bit 5 = PQ EOTF Active]
		else {
			infopacket.sb[6] |= 0x08;  // PB6 = [Bit 3 = Native Color Active]
			if (app_tf == TRANSFER_FUNC_GAMMA_22)
				infopacket.sb[9] |= 0x04;  // PB9 = [Bit 2 = Gamma 2.2 EOTF Active]
		}
	}
}

fn build_vrr_infopacket_header_v1(enum signal_type signal,
		*mut dc_info_packetinfopacket,
		u32 *payload_size)
{
	if (dc_is_hdmi_signal(signal)) {

		/* HEADER */

		/* HB0  = Packet Type = 0x83 (Source Product
		 *	  Descriptor InfoFrame)
		 */
		infopacket.hb0 = DC_HDMI_INFOFRAME_TYPE_SPD;

		/* HB1  = Version = 0x01 */
		infopacket.hb1 = 0x01;

		/* HB2  = [Bits 7:5 = 0] [Bits 4:0 = Length = 0x08] */
		infopacket.hb2 = 0x08;

		*payload_size = 0x08;

	} else if (dc_is_dp_signal(signal)) {

		/* HEADER */

		/* HB0  = Secondary-data Packet ID = 0 - Only non-zero
		 *	  when used to associate audio related info packets
		 */
		infopacket.hb0 = 0x00;

		/* HB1  = Packet Type = 0x83 (Source Product
		 *	  Descriptor InfoFrame)
		 */
		infopacket.hb1 = DC_HDMI_INFOFRAME_TYPE_SPD;

		/* HB2  = [Bits 7:0 = Least significant eight bits -
		 *	  For INFOFRAME, the value must be 1Bh]
		 */
		infopacket.hb2 = 0x1B;

		/* HB3  = [Bits 7:2 = INFOFRAME SDP Version Number = 0x1]
		 *	  [Bits 1:0 = Most significant two bits = 0x00]
		 */
		infopacket.hb3 = 0x04;

		*payload_size = 0x1B;
	}
}

fn build_vrr_infopacket_header_v2(enum signal_type signal,
		*mut dc_info_packetinfopacket,
		u32 *payload_size)
{
	if (dc_is_hdmi_signal(signal)) {

		/* HEADER */

		/* HB0  = Packet Type = 0x83 (Source Product
		 *	  Descriptor InfoFrame)
		 */
		infopacket.hb0 = DC_HDMI_INFOFRAME_TYPE_SPD;

		/* HB1  = Version = 0x02 */
		infopacket.hb1 = 0x02;

		/* HB2  = [Bits 7:5 = 0] [Bits 4:0 = Length = 0x09] */
		infopacket.hb2 = 0x09;

		*payload_size = 0x09;
	} else if (dc_is_dp_signal(signal)) {

		/* HEADER */

		/* HB0  = Secondary-data Packet ID = 0 - Only non-zero
		 *	  when used to associate audio related info packets
		 */
		infopacket.hb0 = 0x00;

		/* HB1  = Packet Type = 0x83 (Source Product
		 *	  Descriptor InfoFrame)
		 */
		infopacket.hb1 = DC_HDMI_INFOFRAME_TYPE_SPD;

		/* HB2  = [Bits 7:0 = Least significant eight bits -
		 *	  For INFOFRAME, the value must be 1Bh]
		 */
		infopacket.hb2 = 0x1B;

		/* HB3  = [Bits 7:2 = INFOFRAME SDP Version Number = 0x2]
		 *	  [Bits 1:0 = Most significant two bits = 0x00]
		 */
		infopacket.hb3 = 0x08;

		*payload_size = 0x1B;
	}
}

fn build_vrr_infopacket_header_v3(enum signal_type signal,
		*mut dc_info_packetinfopacket,
		u32 *payload_size)
{
	u8 version;

	version = 3;
	if (dc_is_hdmi_signal(signal)) {

		/* HEADER */

		/* HB0  = Packet Type = 0x83 (Source Product
		 *	  Descriptor InfoFrame)
		 */
		infopacket.hb0 = DC_HDMI_INFOFRAME_TYPE_SPD;

		/* HB1  = Version = 0x03 */
		infopacket.hb1 = version;

		/* HB2  = [Bits 7:5 = 0] [Bits 4:0 = Length] */
		infopacket.hb2 = 0x10;

		*payload_size = 0x10;
	} else if (dc_is_dp_signal(signal)) {

		/* HEADER */

		/* HB0  = Secondary-data Packet ID = 0 - Only non-zero
		 *	  when used to associate audio related info packets
		 */
		infopacket.hb0 = 0x00;

		/* HB1  = Packet Type = 0x83 (Source Product
		 *	  Descriptor InfoFrame)
		 */
		infopacket.hb1 = DC_HDMI_INFOFRAME_TYPE_SPD;

		/* HB2  = [Bits 7:0 = Least significant eight bits -
		 *	  For INFOFRAME, the value must be 1Bh]
		 */
		infopacket.hb2 = 0x1B;

		/* HB3  = [Bits 7:2 = INFOFRAME SDP Version Number = 0x2]
		 *	  [Bits 1:0 = Most significant two bits = 0x00]
		 */

		infopacket.hb3 = (version & 0x3F) << 2;

		*payload_size = 0x1B;
	}
}

fn build_vrr_infopacket_checksum(u32 *payload_size,
		*mut dc_info_packetinfopacket)
{
	/* Calculate checksum */
	u32 idx = 0;
	u8 checksum = 0;

	checksum += infopacket.hb0;
	checksum += infopacket.hb1;
	checksum += infopacket.hb2;
	checksum += infopacket.hb3;

	for (idx = 1; idx <= *payload_size; idx++)
		checksum += infopacket.sb[idx];

	/* PB0 = Checksum (one byte complement) */
	infopacket.sb[0] = (u8)(0x100 - checksum);

	infopacket.valid = true;
}

fn build_vrr_infopacket_v1(enum signal_type signal, *const mod_vrr_paramsvrr,
		*mut dc_info_packetinfopacket,
		bool freesync_on_desktop)
{
	/* SPD info packet for FreeSync */
	u32 payload_size = 0;

	build_vrr_infopacket_header_v1(signal, infopacket, &payload_size);
	build_vrr_infopacket_data_v1(vrr, infopacket, freesync_on_desktop);
	build_vrr_infopacket_checksum(&payload_size, infopacket);

	infopacket.valid = true;
}

fn build_vrr_infopacket_v2(enum signal_type signal, *const mod_vrr_paramsvrr,
		enum color_transfer_func app_tf,
		*mut dc_info_packetinfopacket,
		bool freesync_on_desktop)
{
	u32 payload_size = 0;

	build_vrr_infopacket_header_v2(signal, infopacket, &payload_size);
	build_vrr_infopacket_data_v1(vrr, infopacket, freesync_on_desktop);

	build_vrr_infopacket_fs2_data(app_tf, infopacket);

	build_vrr_infopacket_checksum(&payload_size, infopacket);

	infopacket.valid = true;
}

fn build_vrr_infopacket_v3(enum signal_type signal, *const mod_vrr_paramsvrr,
		enum color_transfer_func app_tf,
		*mut dc_info_packetinfopacket,
		bool freesync_on_desktop)
{
	u32 payload_size = 0;

	build_vrr_infopacket_header_v3(signal, infopacket, &payload_size);
	build_vrr_infopacket_data_v3(vrr, infopacket, freesync_on_desktop);

	build_vrr_infopacket_fs2_data(app_tf, infopacket);

	build_vrr_infopacket_checksum(&payload_size, infopacket);

	infopacket.valid = true;
}

fn build_vrr_infopacket_sdp_v1_3(enum vrr_packet_type packet_type,
										*mut dc_info_packetinfopacket)
{
	u8 idx = 0, size = 0;

	size = ((packet_type == PACKET_TYPE_FS_V1) ? 0x08 :
			(packet_type == PACKET_TYPE_FS_V3) ? 0x10 :
												0x09);

	for (idx = infopacket.hb2; idx > 1; idx--) // Data Byte Count: 0x1B
		infopacket.sb[idx] = infopacket.sb[idx-1];

	infopacket.sb[1] = size;                         // Length
	infopacket.sb[0] = (infopacket.hb3 >> 2) & 0x3F;//Version
	infopacket.hb3   = (0x13 << 2);                  // Header,SDP 1.3
	infopacket.hb2   = 0x1D;
}

pub unsafe fn mod_freesync_build_vrr_infopacket(*mut mod_freesyncmod_freesync, *const dc_stream_statestream, *const mod_vrr_paramsvrr,
		enum vrr_packet_type packet_type,
		enum color_transfer_func app_tf,
		*mut dc_info_packetinfopacket,
		bool pack_sdp_v1_3)
{
	(void)mod_freesync;
	/* SPD info packet for FreeSync
	 * VTEM info packet for HdmiVRR
	 * Check if Freesync is supported. Return if false. If true,
	 * set the corresponding bit in the info packet
	 */
	if (!vrr.send_info_frame)
		return;

	switch (packet_type) {
	case PACKET_TYPE_FS_V3:
		build_vrr_infopacket_v3(stream.signal, vrr, app_tf, infopacket, stream.freesync_on_desktop);
		break;
	case PACKET_TYPE_FS_V2:
		build_vrr_infopacket_v2(stream.signal, vrr, app_tf, infopacket, stream.freesync_on_desktop);
		break;
	case PACKET_TYPE_VRR:
	case PACKET_TYPE_FS_V1:
	default:
		build_vrr_infopacket_v1(stream.signal, vrr, infopacket, stream.freesync_on_desktop);
	}

	if (true == pack_sdp_v1_3 &&
		true == dc_is_dp_signal(stream.signal) &&
		packet_type != PACKET_TYPE_VRR &&
		packet_type != PACKET_TYPE_VTEM)
		build_vrr_infopacket_sdp_v1_3(packet_type, infopacket);
}

pub unsafe fn mod_freesync_build_vrr_params(*mut mod_freesyncmod_freesync, *const dc_stream_statestream,
		*mut mod_freesync_configin_config,
		*mut mod_vrr_paramsin_out_vrr)
{
	*mut core_freesynccore_freesync = core::ptr::null_mut();
	u64 nominal_field_rate_in_uhz = 0;
	u64 rounded_nominal_in_uhz = 0;
	u32 refresh_range = 0;
	u64 min_refresh_in_uhz = 0;
	u64 max_refresh_in_uhz = 0;
	u64 min_hardware_refresh_in_uhz = 0;

	if (mod_freesync == core::ptr::null_mut())
		return;

	core_freesync = MOD_FREESYNC_TO_CORE(mod_freesync);

	/* Calculate nominal field rate for stream */
	nominal_field_rate_in_uhz =
			mod_freesync_calc_nominal_field_rate(stream);

	if (stream.ctx->dc.caps.max_v_total != 0 && stream.timing.h_total != 0) {
		min_hardware_refresh_in_uhz = div64_u64((stream.timing.pix_clk_100hz * 100000000ULL),
			(stream.timing.h_total * (long long)calc_max_hardware_v_total(stream)));
	}
	/* Limit minimum refresh rate to what can be supported by hardware */
	min_refresh_in_uhz = min_hardware_refresh_in_uhz > in_config.min_refresh_in_uhz ?
		min_hardware_refresh_in_uhz : in_config.min_refresh_in_uhz;
	max_refresh_in_uhz = in_config.max_refresh_in_uhz;

	/* Full range may be larger than current video timing, so cap at nominal */
	if (max_refresh_in_uhz > nominal_field_rate_in_uhz)
		max_refresh_in_uhz = nominal_field_rate_in_uhz;

	/* Full range may be larger than current video timing, so cap at nominal */
	if (min_refresh_in_uhz > max_refresh_in_uhz)
		min_refresh_in_uhz = max_refresh_in_uhz;

	/* If a monitor reports exactly max refresh of 2x of min, enforce it on nominal */
	rounded_nominal_in_uhz =
			div_u64(nominal_field_rate_in_uhz + 50000, 100000) * 100000;
	if (in_config.max_refresh_in_uhz == (2 * in_config.min_refresh_in_uhz) &&
		in_config.max_refresh_in_uhz == rounded_nominal_in_uhz)
		min_refresh_in_uhz = div_u64(nominal_field_rate_in_uhz, 2);

	if (!vrr_settings_require_update(core_freesync,
			in_config, (u32)min_refresh_in_uhz, (u32)max_refresh_in_uhz,
			in_out_vrr))
		return;

	in_out_vrr.state = in_config.state;
	in_out_vrr.send_info_frame = in_config.vsif_supported;

	if (in_config.state == VRR_STATE_UNSUPPORTED) {
		in_out_vrr.state = VRR_STATE_UNSUPPORTED;
		in_out_vrr.supported = false;
		in_out_vrr.adjust.v_total_min = stream.timing.v_total;
		in_out_vrr.adjust.v_total_max = stream.timing.v_total;

		return;

	} else {
		in_out_vrr.min_refresh_in_uhz = (u32)min_refresh_in_uhz;
		in_out_vrr.max_duration_in_us =
				calc_duration_in_us_from_refresh_in_uhz(
						(u32)min_refresh_in_uhz);

		in_out_vrr.max_refresh_in_uhz = (u32)max_refresh_in_uhz;
		in_out_vrr.min_duration_in_us =
				calc_duration_in_us_from_refresh_in_uhz(
						(u32)max_refresh_in_uhz);

		if (in_config.state == VRR_STATE_ACTIVE_FIXED)
			in_out_vrr.fixed_refresh_in_uhz = in_config.fixed_refresh_in_uhz;
		else
			in_out_vrr.fixed_refresh_in_uhz = 0;

		{
			u64 rr_tmp = div_u64(in_out_vrr.max_refresh_in_uhz + 500000, 1000000) -
					div_u64(in_out_vrr.min_refresh_in_uhz + 500000, 1000000);
			ASSERT(rr_tmp <= 0xFFFFFFFF);
			refresh_range = (u32)rr_tmp;
		}

		in_out_vrr.supported = true;
	}

	in_out_vrr.fixed.ramping_active = in_config.ramping;

	in_out_vrr.btr.btr_enabled = in_config.btr;

	if (in_out_vrr.max_refresh_in_uhz < (2 * in_out_vrr.min_refresh_in_uhz))
		in_out_vrr.btr.btr_enabled = false;
	else {
		in_out_vrr.btr.margin_in_us = in_out_vrr.max_duration_in_us -
				2 * in_out_vrr.min_duration_in_us;
		if (in_out_vrr.btr.margin_in_us > BTR_MAX_MARGIN)
			in_out_vrr.btr.margin_in_us = BTR_MAX_MARGIN;
	}

	in_out_vrr.btr.btr_active = false;
	in_out_vrr.btr.inserted_duration_in_us = 0;
	in_out_vrr.btr.frames_to_insert = 0;
	in_out_vrr.btr.frame_counter = 0;
	in_out_vrr.fixed.fixed_active = false;
	in_out_vrr.fixed.target_refresh_in_uhz = 0;

	in_out_vrr.btr.mid_point_in_us =
				(in_out_vrr.min_duration_in_us +
				 in_out_vrr.max_duration_in_us) / 2;

	if (in_out_vrr.state == VRR_STATE_UNSUPPORTED) {
		in_out_vrr.adjust.v_total_min = stream.timing.v_total;
		in_out_vrr.adjust.v_total_max = stream.timing.v_total;
	} else if (in_out_vrr.state == VRR_STATE_DISABLED) {
		in_out_vrr.adjust.v_total_min = stream.timing.v_total;
		in_out_vrr.adjust.v_total_max = stream.timing.v_total;
	} else if (in_out_vrr.state == VRR_STATE_INACTIVE) {
		in_out_vrr.adjust.v_total_min = stream.timing.v_total;
		in_out_vrr.adjust.v_total_max = stream.timing.v_total;
	} else if (in_out_vrr.state == VRR_STATE_ACTIVE_VARIABLE &&
			refresh_range >= MIN_REFRESH_RANGE) {

		in_out_vrr.adjust.v_total_min =
			mod_freesync_calc_v_total_from_refresh(stream,
				in_out_vrr.max_refresh_in_uhz);
		in_out_vrr.adjust.v_total_max =
			mod_freesync_calc_v_total_from_refresh(stream,
				in_out_vrr.min_refresh_in_uhz);
	} else if (in_out_vrr.state == VRR_STATE_ACTIVE_FIXED) {
		in_out_vrr.fixed.target_refresh_in_uhz =
				in_out_vrr.fixed_refresh_in_uhz;
		if (in_out_vrr.fixed.ramping_active &&
				in_out_vrr.fixed.fixed_active) {
			/* Do not update vtotals if ramping is already active
			 * in order to continue ramp from current refresh.
			 */
			in_out_vrr.fixed.fixed_active = true;
		} else {
			in_out_vrr.fixed.fixed_active = true;
			in_out_vrr.adjust.v_total_min =
				mod_freesync_calc_v_total_from_refresh(stream,
					in_out_vrr.fixed.target_refresh_in_uhz);
			in_out_vrr.adjust.v_total_max =
				in_out_vrr.adjust.v_total_min;
		}
	} else {
		in_out_vrr.state = VRR_STATE_INACTIVE;
		in_out_vrr.adjust.v_total_min = stream.timing.v_total;
		in_out_vrr.adjust.v_total_max = stream.timing.v_total;
	}

	in_out_vrr.adjust.allow_otg_v_count_halt = (in_config.state == VRR_STATE_ACTIVE_FIXED) ? true : false;
}

pub unsafe fn mod_freesync_handle_preflip(*mut mod_freesyncmod_freesync, *const dc_plane_stateplane, *const dc_stream_statestream,
		u32 curr_time_stamp_in_us,
		*mut mod_vrr_paramsin_out_vrr)
{
	*mut core_freesynccore_freesync = core::ptr::null_mut();
	u32 last_render_time_in_us = 0;

	if (mod_freesync == core::ptr::null_mut())
		return;

	core_freesync = MOD_FREESYNC_TO_CORE(mod_freesync);

	if (in_out_vrr.supported &&
			in_out_vrr.state == VRR_STATE_ACTIVE_VARIABLE) {

		last_render_time_in_us = curr_time_stamp_in_us -
				plane.time.prev_update_time_in_us;

		if (in_out_vrr.btr.btr_enabled) {
			apply_below_the_range(core_freesync,
					stream,
					last_render_time_in_us,
					in_out_vrr);
		} else {
			apply_fixed_refresh(core_freesync,
				stream,
				last_render_time_in_us,
				in_out_vrr);
		}

		determine_flip_interval_workaround_req(in_out_vrr,
				curr_time_stamp_in_us);

	}
}

pub unsafe fn mod_freesync_handle_v_update(*mut mod_freesyncmod_freesync, *const dc_stream_statestream,
		*mut mod_vrr_paramsin_out_vrr)
{
	*mut core_freesynccore_freesync = core::ptr::null_mut();
	u32 cur_timestamp_in_us;
	u64 cur_tick;

	if ((mod_freesync == core::ptr::null_mut()) || (stream == core::ptr::null_mut()) || (in_out_vrr == core::ptr::null_mut()))
		return;

	core_freesync = MOD_FREESYNC_TO_CORE(mod_freesync);

	if (in_out_vrr.supported == false)
		return;

	cur_tick = dm_get_timestamp(core_freesync.dc->ctx);
	cur_timestamp_in_us = (u32)
			div_u64(dm_get_elapse_time_in_ns(core_freesync.dc->ctx, cur_tick, 0), 1000);

	in_out_vrr.flip_interval.vsyncs_between_flip++;
	in_out_vrr.flip_interval.v_update_timestamp_in_us = cur_timestamp_in_us;

	if (in_out_vrr.state == VRR_STATE_ACTIVE_VARIABLE &&
			(in_out_vrr.flip_interval.flip_interval_workaround_active ||
			(!in_out_vrr.flip_interval.flip_interval_workaround_active &&
			in_out_vrr.flip_interval.program_flip_interval_workaround))) {
		// set freesync vmin vmax to nominal for workaround
		in_out_vrr.adjust.v_total_min =
			mod_freesync_calc_v_total_from_refresh(
			stream, in_out_vrr.max_refresh_in_uhz);
		in_out_vrr.adjust.v_total_max =
				in_out_vrr.adjust.v_total_min;
		in_out_vrr.flip_interval.program_flip_interval_workaround = false;
		in_out_vrr.flip_interval.do_flip_interval_workaround_cleanup = true;
		return;
	}

	if (in_out_vrr.state != VRR_STATE_ACTIVE_VARIABLE &&
			in_out_vrr.flip_interval.do_flip_interval_workaround_cleanup) {
		in_out_vrr.flip_interval.do_flip_interval_workaround_cleanup = false;
		in_out_vrr.flip_interval.flip_interval_detect_counter = 0;
		in_out_vrr.flip_interval.vsyncs_between_flip = 0;
		in_out_vrr.flip_interval.vsync_to_flip_in_us = 0;
	}

	/* Below the Range Logic */

	/* Only execute if in fullscreen mode */
	if (in_out_vrr.state == VRR_STATE_ACTIVE_VARIABLE &&
					in_out_vrr.btr.btr_active) {
		/* TODO: pass in flag for Pre-DCE12 ASIC
		 * in order for frame variable duration to take affect,
		 * it needs to be done one VSYNC early, which is at
		 * frameCounter == 1.
		 * For DCE12 and newer updates to V_TOTAL_MIN/MAX
		 * will take affect on current frame
		 */
		if (in_out_vrr.btr.frames_to_insert ==
				in_out_vrr.btr.frame_counter) {
			in_out_vrr.adjust.v_total_min =
				calc_v_total_from_duration(stream,
				in_out_vrr,
				in_out_vrr.btr.inserted_duration_in_us);
			in_out_vrr.adjust.v_total_max =
				in_out_vrr.adjust.v_total_min;
		}

		if (in_out_vrr.btr.frame_counter > 0)
			in_out_vrr.btr.frame_counter--;

		/* Restore FreeSync */
		if (in_out_vrr.btr.frame_counter == 0) {
			in_out_vrr.adjust.v_total_min =
				mod_freesync_calc_v_total_from_refresh(stream,
				in_out_vrr.max_refresh_in_uhz);
			in_out_vrr.adjust.v_total_max =
				mod_freesync_calc_v_total_from_refresh(stream,
				in_out_vrr.min_refresh_in_uhz);
		}
	}

	/* If in fullscreen freesync mode or in video, do not program
	 * static screen ramp values
	 */
	if (in_out_vrr.state == VRR_STATE_ACTIVE_VARIABLE)
		in_out_vrr.fixed.ramping_active = false;

	/* Gradual Static Screen Ramping Logic
	 * Execute if ramp is active and user enabled freesync static screen
	 */
	if (in_out_vrr.state == VRR_STATE_ACTIVE_FIXED &&
				in_out_vrr.fixed.ramping_active) {
		update_v_total_for_static_ramp(
				core_freesync, stream, in_out_vrr);
	}

	/*
	 * If VRR is inactive, set vtotal min and max to nominal vtotal
	 */
	 if (in_out_vrr.state == VRR_STATE_INACTIVE) {
		in_out_vrr.adjust.v_total_min =
			mod_freesync_calc_v_total_from_refresh(stream,
				in_out_vrr.max_refresh_in_uhz);
		in_out_vrr.adjust.v_total_max = in_out_vrr.adjust.v_total_min;
		return;
	}
}

pub unsafe fn mod_freesync_calc_nominal_field_rate( *const dc_stream_statestream)
{
	u64 nominal_field_rate_in_uhz = 0;
	u32 total = stream.timing.h_total * stream.timing.v_total;

	/* Calculate nominal field rate for stream, rounded up to nearest integer */
	nominal_field_rate_in_uhz = stream.timing.pix_clk_100hz;
	nominal_field_rate_in_uhz *= 100000000ULL;

	nominal_field_rate_in_uhz =	div_u64(nominal_field_rate_in_uhz, total);

	return nominal_field_rate_in_uhz;
}

pub unsafe fn mod_freesync_get_freesync_enabled(*mut mod_vrr_paramspVrr)
{
	return (pVrr.state != VRR_STATE_UNSUPPORTED) && (pVrr.state != VRR_STATE_DISABLED);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
