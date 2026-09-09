// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Faithful low-level translation of dc_spl.c.  Types and helper routines
// supplied by dc_spl.h and the filter headers are intentionally external.

const MIN_VIEWPORT_SIZE: i32 = 12;

#[inline]
fn identity_ratio(ratio: spl_fixed31_32) -> bool {
    unsafe { spl_fixpt_u3d19(ratio) == (1_i64 << 19) }
}

unsafe fn spl_is_yuv420(format: spl_pixel_format) -> bool {
    format >= SPL_PIXEL_FORMAT_420BPP8 && format <= SPL_PIXEL_FORMAT_420BPP10
}

unsafe fn spl_is_yuv422(format: spl_pixel_format) -> bool {
    format >= SPL_PIXEL_FORMAT_422BPP8 && format <= SPL_PIXEL_FORMAT_422BPP12
}

unsafe fn spl_is_rgb8(format: spl_pixel_format) -> bool {
    format == SPL_PIXEL_FORMAT_ARGB8888
}

unsafe fn spl_is_video_format(format: spl_pixel_format) -> bool {
    format >= SPL_PIXEL_FORMAT_VIDEO_BEGIN && format <= SPL_PIXEL_FORMAT_VIDEO_END
}

unsafe fn spl_is_subsampled_format(format: spl_pixel_format) -> bool {
    format >= SPL_PIXEL_FORMAT_SUBSAMPLED_BEGIN && format <= SPL_PIXEL_FORMAT_SUBSAMPLED_END
}

unsafe fn intersect_rec(r0: *const spl_rect, r1: *const spl_rect) -> spl_rect {
    let mut rec = spl_rect { x: (*r0).x.max((*r1).x),
        width: 0, y: (*r0).y.max((*r1).y), height: 0 };
    let r0_x_end = (*r0).x + (*r0).width;
    let r1_x_end = (*r1).x + (*r1).width;
    let r0_y_end = (*r0).y + (*r0).height;
    let r1_y_end = (*r1).y + (*r1).height;
    rec.width = if r0_x_end > r1_x_end { r1_x_end - rec.x } else { r0_x_end - rec.x };
    rec.height = if r0_y_end > r1_y_end { r1_y_end - rec.y } else { r0_y_end - rec.y };
    if rec.width < 0 || rec.height < 0 { spl_rect { x: 0, y: 0, width: 0, height: 0 } } else { rec }
}

unsafe fn shift_rec(rec_in: *const spl_rect, x: i32, y: i32) -> spl_rect {
    let mut rec = *rec_in;
    rec.x += x;
    rec.y += y;
    rec
}

unsafe fn spl_opp_adjust_rect(rec: *mut spl_rect, adjust: *const spl_opp_adjust) {
    if (*rec).x + (*adjust).x >= 0 { (*rec).x += (*adjust).x; }
    if (*rec).y + (*adjust).y >= 0 { (*rec).y += (*adjust).y; }
    if (*rec).width + (*adjust).width >= 1 { (*rec).width += (*adjust).width; }
    if (*rec).height + (*adjust).height >= 1 { (*rec).height += (*adjust).height; }
}

unsafe fn spl_clamp_viewport(viewport: *mut spl_rect, min_viewport_size: i32) {
    let min_size = if min_viewport_size == 0 { MIN_VIEWPORT_SIZE } else { min_viewport_size };
    if (*viewport).height < min_size { (*viewport).height = min_size; }
    if (*viewport).width < min_size { (*viewport).width = min_size; }
}

// The remaining declarations mirror the C implementation and retain its
// external data types and call boundaries.  Filter programming is delegated
// to the corresponding external SPL helpers, exactly as in the C source.
extern "C" {
    fn spl_calculate_scaler_params(spl_in: *mut spl_in, spl_out: *mut spl_out) -> bool;
    fn spl_get_number_of_taps(spl_in: *mut spl_in, spl_out: *mut spl_out) -> bool;
}

/*
+// C source: // SPDX-License-Identifier: MIT
// C source: //
// C source: // Copyright 2024 Advanced Micro Devices, Inc.
// C source: 
// C source: #include "dc_spl.h"
// C source: #include "dc_spl_scl_easf_filters.h"
// C source: #include "dc_spl_isharp_filters.h"
// C source: #include "spl_debug.h"
// C source: 
// C source: #define IDENTITY_RATIO(ratio) (SPL_NAMESPACE(spl_fixpt_u3d19(ratio)) == (1 << 19))
// C source: #define MIN_VIEWPORT_SIZE 12
// C source: static bool spl_is_yuv420(enum spl_pixel_format format)
// C source: {
// C source: 	if ((format >= SPL_PIXEL_FORMAT_420BPP8) &&
// C source: 		(format <= SPL_PIXEL_FORMAT_420BPP10))
// C source: 		return true;
// C source: 
// C source: 	return false;
// C source: }
// C source: 
// C source: static bool spl_is_yuv422(enum spl_pixel_format format)
// C source: {
// C source: 	if ((format >= SPL_PIXEL_FORMAT_422BPP8) &&
// C source: 		(format <= SPL_PIXEL_FORMAT_422BPP12))
// C source: 		return true;
// C source: 
// C source: 	return false;
// C source: }
// C source: 
// C source: static bool spl_is_rgb8(enum spl_pixel_format format)
// C source: {
// C source: 	if (format == SPL_PIXEL_FORMAT_ARGB8888)
// C source: 		return true;
// C source: 
// C source: 	return false;
// C source: }
// C source: 
// C source: static bool spl_is_video_format(enum spl_pixel_format format)
// C source: {
// C source: 	if (format >= SPL_PIXEL_FORMAT_VIDEO_BEGIN
// C source: 		&& format <= SPL_PIXEL_FORMAT_VIDEO_END)
// C source: 		return true;
// C source: 	else
// C source: 		return false;
// C source: }
// C source: 
// C source: static bool spl_is_subsampled_format(enum spl_pixel_format format)
// C source: {
// C source: 	if (format >= SPL_PIXEL_FORMAT_SUBSAMPLED_BEGIN
// C source: 		&& format <= SPL_PIXEL_FORMAT_SUBSAMPLED_END)
// C source: 		return true;
// C source: 	else
// C source: 		return false;
// C source: }
// C source: 
// C source: static struct spl_rect intersect_rec(const struct spl_rect *r0, const struct spl_rect *r1)
// C source: {
// C source: 	struct spl_rect rec;
// C source: 	int r0_x_end = r0->x + r0->width;
// C source: 	int r1_x_end = r1->x + r1->width;
// C source: 	int r0_y_end = r0->y + r0->height;
// C source: 	int r1_y_end = r1->y + r1->height;
// C source: 
// C source: 	rec.x = r0->x > r1->x ? r0->x : r1->x;
// C source: 	rec.width = r0_x_end > r1_x_end ? r1_x_end - rec.x : r0_x_end - rec.x;
// C source: 	rec.y = r0->y > r1->y ? r0->y : r1->y;
// C source: 	rec.height = r0_y_end > r1_y_end ? r1_y_end - rec.y : r0_y_end - rec.y;
// C source: 
// C source: 	/* in case that there is no intersection */
// C source: 	if (rec.width < 0 || rec.height < 0)
// C source: 		memset(&rec, 0, sizeof(rec));
// C source: 
// C source: 	return rec;
// C source: }
// C source: 
// C source: static struct spl_rect shift_rec(const struct spl_rect *rec_in, int x, int y)
// C source: {
// C source: 	struct spl_rect rec_out = *rec_in;
// C source: 
// C source: 	rec_out.x += x;
// C source: 	rec_out.y += y;
// C source: 
// C source: 	return rec_out;
// C source: }
// C source: 
// C source: static void spl_opp_adjust_rect(struct spl_rect *rec, const struct spl_opp_adjust *adjust)
// C source: {
// C source: 	if ((rec->x + adjust->x) >= 0)
// C source: 		rec->x += adjust->x;
// C source: 
// C source: 	if ((rec->y + adjust->y) >= 0)
// C source: 		rec->y += adjust->y;
// C source: 
// C source: 	if ((rec->width + adjust->width) >= 1)
// C source: 		rec->width += adjust->width;
// C source: 
// C source: 	if ((rec->height + adjust->height) >= 1)
// C source: 		rec->height += adjust->height;
// C source: }
// C source: 
// C source: static struct spl_rect calculate_plane_rec_in_timing_active(
// C source: 		struct spl_in *spl_in,
// C source: 		const struct spl_rect *rec_in)
// C source: {
// C source: 	/*
// C source: 	 * The following diagram shows an example where we map a 1920x1200
// C source: 	 * desktop to a 2560x1440 timing with a plane rect in the middle
// C source: 	 * of the screen. To map a plane rect from Stream Source to Timing
// C source: 	 * Active space, we first multiply stream scaling ratios (i.e 2304/1920
// C source: 	 * horizontal and 1440/1200 vertical) to the plane's x and y, then
// C source: 	 * we add stream destination offsets (i.e 128 horizontal, 0 vertical).
// C source: 	 * This will give us a plane rect's position in Timing Active. However
// C source: 	 * we have to remove the fractional. The rule is that we find left/right
// C source: 	 * and top/bottom positions and round the value to the adjacent integer.
// C source: 	 *
// C source: 	 * Stream Source Space
// C source: 	 * ------------
// C source: 	 *        __________________________________________________
// C source: 	 *       |Stream Source (1920 x 1200) ^                     |
// C source: 	 *       |                            y                     |
// C source: 	 *       |         <------- w --------|>                    |
// C source: 	 *       |          __________________V                     |
// C source: 	 *       |<-- x -->|Plane//////////////| ^                  |
// C source: 	 *       |         |(pre scale)////////| |                  |
// C source: 	 *       |         |///////////////////| |                  |
// C source: 	 *       |         |///////////////////| h                  |
// C source: 	 *       |         |///////////////////| |                  |
// C source: 	 *       |         |///////////////////| |                  |
// C source: 	 *       |         |///////////////////| V                  |
// C source: 	 *       |                                                  |
// C source: 	 *       |                                                  |
// C source: 	 *       |__________________________________________________|
// C source: 	 *
// C source: 	 *
// C source: 	 * Timing Active Space
// C source: 	 * ---------------------------------
// C source: 	 *
// C source: 	 *       Timing Active (2560 x 1440)
// C source: 	 *        __________________________________________________
// C source: 	 *       |*****|  Stteam Destination (2304 x 1440)    |*****|
// C source: 	 *       |*****|                                      |*****|
// C source: 	 *       |<128>|                                      |*****|
// C source: 	 *       |*****|     __________________               |*****|
// C source: 	 *       |*****|    |Plane/////////////|              |*****|
// C source: 	 *       |*****|    |(post scale)//////|              |*****|
// C source: 	 *       |*****|    |//////////////////|              |*****|
// C source: 	 *       |*****|    |//////////////////|              |*****|
// C source: 	 *       |*****|    |//////////////////|              |*****|
// C source: 	 *       |*****|    |//////////////////|              |*****|
// C source: 	 *       |*****|                                      |*****|
// C source: 	 *       |*****|                                      |*****|
// C source: 	 *       |*****|                                      |*****|
// C source: 	 *       |*****|______________________________________|*****|
// C source: 	 *
// C source: 	 * So the resulting formulas are shown below:
// C source: 	 *
// C source: 	 * recout_x = 128 + round(plane_x * 2304 / 1920)
// C source: 	 * recout_w = 128 + round((plane_x + plane_w) * 2304 / 1920) - recout_x
// C source: 	 * recout_y = 0 + round(plane_y * 1440 / 1200)
// C source: 	 * recout_h = 0 + round((plane_y + plane_h) * 1440 / 1200) - recout_y
// C source: 	 *
// C source: 	 * NOTE: fixed point division is not error free. To reduce errors
// C source: 	 * introduced by fixed point division, we divide only after
// C source: 	 * multiplication is complete.
// C source: 	 */
// C source: 	const struct spl_rect *stream_src = &spl_in->basic_out.src_rect;
// C source: 	const struct spl_rect *stream_dst = &spl_in->basic_out.dst_rect;
// C source: 	struct spl_rect rec_out = {0};
// C source: 	struct spl_fixed31_32 temp;
// C source: 
// C source: 
// C source: 	temp = SPL_NAMESPACE(spl_fixpt_from_fraction(
// C source: 			rec_in->x * (long long)stream_dst->width,
// C source: 			stream_src->width));
// C source: 	rec_out.x = stream_dst->x + spl_fixpt_round(temp);
// C source: 
// C source: 	temp = SPL_NAMESPACE(spl_fixpt_from_fraction(
// C source: 			(rec_in->x + rec_in->width) * (long long)stream_dst->width,
// C source: 			stream_src->width));
// C source: 	rec_out.width = stream_dst->x + spl_fixpt_round(temp) - rec_out.x;
// C source: 
// C source: 	temp = SPL_NAMESPACE(spl_fixpt_from_fraction(
// C source: 			rec_in->y * (long long)stream_dst->height,
// C source: 			stream_src->height));
// C source: 	rec_out.y = stream_dst->y + spl_fixpt_round(temp);
// C source: 
// C source: 	temp = SPL_NAMESPACE(spl_fixpt_from_fraction(
// C source: 			(rec_in->y + rec_in->height) * (long long)stream_dst->height,
// C source: 			stream_src->height));
// C source: 	rec_out.height = stream_dst->y + spl_fixpt_round(temp) - rec_out.y;
// C source: 
// C source: 	return rec_out;
// C source: }
// C source: 
// C source: static struct spl_rect calculate_mpc_slice_in_timing_active(
// C source: 		struct spl_in *spl_in,
// C source: 		struct spl_rect *plane_clip_rec)
// C source: {
// C source: 	bool use_recout_width_aligned =
// C source: 		spl_in->basic_in.num_h_slices_recout_width_align.use_recout_width_aligned;
// C source: 	int mpc_slice_count =
// C source: 		spl_in->basic_in.num_h_slices_recout_width_align.num_slices_recout_width.mpc_num_h_slices;
// C source: 	int recout_width_align =
// C source: 		spl_in->basic_in.num_h_slices_recout_width_align.num_slices_recout_width.mpc_recout_width_align;
// C source: 	int mpc_slice_idx = spl_in->basic_in.mpc_h_slice_index;
// C source: 	int epimo = mpc_slice_count - plane_clip_rec->width % mpc_slice_count - 1;
// C source: 	struct spl_rect mpc_rec;
// C source: 
// C source: 	if (spl_in->basic_in.custom_width != 0) {
// C source: 		mpc_rec.width = spl_in->basic_in.custom_width;
// C source: 		mpc_rec.x = spl_in->basic_in.custom_x;
// C source: 		mpc_rec.height = plane_clip_rec->height;
// C source: 		mpc_rec.y = plane_clip_rec->y;
// C source: 	} else if (use_recout_width_aligned) {
// C source: 		mpc_rec.width = recout_width_align;
// C source: 		if ((mpc_rec.width * (mpc_slice_idx + 1)) > plane_clip_rec->width) {
// C source: 			mpc_rec.width = plane_clip_rec->width % recout_width_align;
// C source: 			mpc_rec.x = plane_clip_rec->x + recout_width_align * mpc_slice_idx;
// C source: 		} else
// C source: 			mpc_rec.x = plane_clip_rec->x + mpc_rec.width * mpc_slice_idx;
// C source: 		mpc_rec.height = plane_clip_rec->height;
// C source: 		mpc_rec.y = plane_clip_rec->y;
// C source: 
// C source: 	} else {
// C source: 		mpc_rec.width = plane_clip_rec->width / mpc_slice_count;
// C source: 		mpc_rec.x = plane_clip_rec->x + mpc_rec.width * mpc_slice_idx;
// C source: 		mpc_rec.height = plane_clip_rec->height;
// C source: 		mpc_rec.y = plane_clip_rec->y;
// C source: 	}
// C source: 	SPL_ASSERT(mpc_slice_count == 1 ||
// C source: 			spl_in->basic_out.view_format != SPL_VIEW_3D_SIDE_BY_SIDE ||
// C source: 			mpc_rec.width % 2 == 0);
// C source: 
// C source: 	/* extra pixels in the division remainder need to go to pipes after
// C source: 	 * the extra pixel index minus one(epimo) defined here as:
// C source: 	 */
// C source: 	if ((use_recout_width_aligned == false) &&
// C source: 		mpc_slice_idx > epimo && spl_in->basic_in.custom_width == 0) {
// C source: 		mpc_rec.x += mpc_slice_idx - epimo - 1;
// C source: 		mpc_rec.width += 1;
// C source: 	}
// C source: 
// C source: 	if (spl_in->basic_out.view_format == SPL_VIEW_3D_TOP_AND_BOTTOM) {
// C source: 		SPL_ASSERT(mpc_rec.height % 2 == 0);
// C source: 		mpc_rec.height /= 2;
// C source: 	}
// C source: 	return mpc_rec;
// C source: }
// C source: 
// C source: static struct spl_rect calculate_odm_slice_in_timing_active(struct spl_in *spl_in)
// C source: {
// C source: 	int odm_slice_count = spl_in->basic_out.odm_combine_factor;
// C source: 	int odm_slice_idx = spl_in->odm_slice_index;
// C source: 	bool is_last_odm_slice = (odm_slice_idx + 1) == odm_slice_count;
// C source: 	int h_active = spl_in->basic_out.output_size.width;
// C source: 	int v_active = spl_in->basic_out.output_size.height;
// C source: 	int odm_slice_width;
// C source: 	struct spl_rect odm_rec;
// C source: 
// C source: 	if (spl_in->basic_out.odm_combine_factor > 0) {
// C source: 		odm_slice_width = h_active / odm_slice_count;
// C source: 		/*
// C source: 		 * deprecated, caller must pass in odm slice rect i.e OPP input
// C source: 		 * rect in timing active for the new interface.
// C source: 		 */
// C source: 		if (spl_in->basic_out.use_two_pixels_per_container && (odm_slice_width % 2))
// C source: 			odm_slice_width++;
// C source: 
// C source: 		odm_rec.x = odm_slice_width * odm_slice_idx;
// C source: 		odm_rec.width = is_last_odm_slice ?
// C source: 			/* last slice width is the reminder of h_active */
// C source: 			h_active - odm_slice_width * (odm_slice_count - 1) :
// C source: 			/* odm slice width is the floor of h_active / count */
// C source: 			odm_slice_width;
// C source: 		odm_rec.y = 0;
// C source: 		odm_rec.height = v_active;
// C source: 
// C source: 		return odm_rec;
// C source: 	}
// C source: 
// C source: 	return spl_in->basic_out.odm_slice_rect;
// C source: }
// C source: 
// C source: static void spl_calculate_recout(struct spl_in *spl_in, struct spl_scratch *spl_scratch, struct spl_out *spl_out)
// C source: {
// C source: 	/*
// C source: 	 * A plane clip represents the desired plane size and position in Stream
// C source: 	 * Source Space. Stream Source is the destination where all planes are
// C source: 	 * blended (i.e. positioned, scaled and overlaid). It is a canvas where
// C source: 	 * all planes associated with the current stream are drawn together.
// C source: 	 * After Stream Source is completed, we will further scale and
// C source: 	 * reposition the entire canvas of the stream source to Stream
// C source: 	 * Destination in Timing Active Space. This could be due to display
// C source: 	 * overscan adjustment where we will need to rescale and reposition all
// C source: 	 * the planes so they can fit into a TV with overscan or downscale
// C source: 	 * upscale features such as GPU scaling or VSR.
// C source: 	 *
// C source: 	 * This two step blending is a virtual procedure in software. In
// C source: 	 * hardware there is no such thing as Stream Source. all planes are
// C source: 	 * blended once in Timing Active Space. Software virtualizes a Stream
// C source: 	 * Source space to decouple the math complicity so scaling param
// C source: 	 * calculation focuses on one step at a time.
// C source: 	 *
// C source: 	 * In the following two diagrams, user applied 10% overscan adjustment
// C source: 	 * so the Stream Source needs to be scaled down a little before mapping
// C source: 	 * to Timing Active Space. As a result the Plane Clip is also scaled
// C source: 	 * down by the same ratio, Plane Clip position (i.e. x and y) with
// C source: 	 * respect to Stream Source is also scaled down. To map it in Timing
// C source: 	 * Active Space additional x and y offsets from Stream Destination are
// C source: 	 * added to Plane Clip as well.
// C source: 	 *
// C source: 	 * Stream Source Space
// C source: 	 * ------------
// C source: 	 *        __________________________________________________
// C source: 	 *       |Stream Source (3840 x 2160) ^                     |
// C source: 	 *       |                            y                     |
// C source: 	 *       |                            |                     |
// C source: 	 *       |          __________________V                     |
// C source: 	 *       |<-- x -->|Plane Clip/////////|                    |
// C source: 	 *       |         |(pre scale)////////|                    |
// C source: 	 *       |         |///////////////////|                    |
// C source: 	 *       |         |///////////////////|                    |
// C source: 	 *       |         |///////////////////|                    |
// C source: 	 *       |         |///////////////////|                    |
// C source: 	 *       |         |///////////////////|                    |
// C source: 	 *       |                                                  |
// C source: 	 *       |                                                  |
// C source: 	 *       |__________________________________________________|
// C source: 	 *
// C source: 	 *
// C source: 	 * Timing Active Space (3840 x 2160)
// C source: 	 * ---------------------------------
// C source: 	 *
// C source: 	 *       Timing Active
// C source: 	 *        __________________________________________________
// C source: 	 *       | y_____________________________________________   |
// C source: 	 *       |x |Stream Destination (3456 x 1944)            |  |
// C source: 	 *       |  |                                            |  |
// C source: 	 *       |  |        __________________                  |  |
// C source: 	 *       |  |       |Plane Clip////////|                 |  |
// C source: 	 *       |  |       |(post scale)//////|                 |  |
// C source: 	 *       |  |       |//////////////////|                 |  |
// C source: 	 *       |  |       |//////////////////|                 |  |
// C source: 	 *       |  |       |//////////////////|                 |  |
// C source: 	 *       |  |       |//////////////////|                 |  |
// C source: 	 *       |  |                                            |  |
// C source: 	 *       |  |                                            |  |
// C source: 	 *       |  |____________________________________________|  |
// C source: 	 *       |__________________________________________________|
// C source: 	 *
// C source: 	 *
// C source: 	 * In Timing Active Space a plane clip could be further sliced into
// C source: 	 * pieces called MPC slices. Each Pipe Context is responsible for
// C source: 	 * processing only one MPC slice so the plane processing workload can be
// C source: 	 * distributed to multiple DPP Pipes. MPC slices could be blended
// C source: 	 * together to a single ODM slice. Each ODM slice is responsible for
// C source: 	 * processing a portion of Timing Active divided horizontally so the
// C source: 	 * output pixel processing workload can be distributed to multiple OPP
// C source: 	 * pipes. All ODM slices are mapped together in ODM block so all MPC
// C source: 	 * slices belong to different ODM slices could be pieced together to
// C source: 	 * form a single image in Timing Active. MPC slices must belong to
// C source: 	 * single ODM slice. If an MPC slice goes across ODM slice boundary, it
// C source: 	 * needs to be divided into two MPC slices one for each ODM slice.
// C source: 	 *
// C source: 	 * In the following diagram the output pixel processing workload is
// C source: 	 * divided horizontally into two ODM slices one for each OPP blend tree.
// C source: 	 * OPP0 blend tree is responsible for processing left half of Timing
// C source: 	 * Active, while OPP2 blend tree is responsible for processing right
// C source: 	 * half.
// C source: 	 *
// C source: 	 * The plane has two MPC slices. However since the right MPC slice goes
// C source: 	 * across ODM boundary, two DPP pipes are needed one for each OPP blend
// C source: 	 * tree. (i.e. DPP1 for OPP0 blend tree and DPP2 for OPP2 blend tree).
// C source: 	 *
// C source: 	 * Assuming that we have a Pipe Context associated with OPP0 and DPP1
// C source: 	 * working on processing the plane in the diagram. We want to know the
// C source: 	 * width and height of the shaded rectangle and its relative position
// C source: 	 * with respect to the ODM slice0. This is called the recout of the pipe
// C source: 	 * context.
// C source: 	 *
// C source: 	 * Planes can be at arbitrary size and position and there could be an
// C source: 	 * arbitrary number of MPC and ODM slices. The algorithm needs to take
// C source: 	 * all scenarios into account.
// C source: 	 *
// C source: 	 * Timing Active Space (3840 x 2160)
// C source: 	 * ---------------------------------
// C source: 	 *
// C source: 	 *       Timing Active
// C source: 	 *        __________________________________________________
// C source: 	 *       |OPP0(ODM slice0)^        |OPP2(ODM slice1)        |
// C source: 	 *       |                y        |                        |
// C source: 	 *       |                |  <- w ->                        |
// C source: 	 *       |           _____V________|____                    |
// C source: 	 *       |          |DPP0 ^  |DPP1 |DPP2|                   |
// C source: 	 *       |<------ x |-----|->|/////|    |                   |
// C source: 	 *       |          |     |  |/////|    |                   |
// C source: 	 *       |          |     h  |/////|    |                   |
// C source: 	 *       |          |     |  |/////|    |                   |
// C source: 	 *       |          |_____V__|/////|____|                   |
// C source: 	 *       |                         |                        |
// C source: 	 *       |                         |                        |
// C source: 	 *       |                         |                        |
// C source: 	 *       |_________________________|________________________|
// C source: 	 *
// C source: 	 *
// C source: 	 */
// C source: 	struct spl_rect plane_clip;
// C source: 	struct spl_rect mpc_slice_of_plane_clip;
// C source: 	struct spl_rect odm_slice;
// C source: 	struct spl_rect overlapping_area;
// C source: 
// C source: 	plane_clip = calculate_plane_rec_in_timing_active(spl_in,
// C source: 			&spl_in->basic_in.clip_rect);
// C source: 	/* guard plane clip from drawing beyond stream dst here */
// C source: 	plane_clip = intersect_rec(&plane_clip,
// C source: 				&spl_in->basic_out.dst_rect);
// C source: 	mpc_slice_of_plane_clip = calculate_mpc_slice_in_timing_active(
// C source: 			spl_in, &plane_clip);
// C source: 	odm_slice = calculate_odm_slice_in_timing_active(spl_in);
// C source: 	overlapping_area = intersect_rec(&mpc_slice_of_plane_clip, &odm_slice);
// C source: 
// C source: 	if (overlapping_area.height > 0 &&
// C source: 			overlapping_area.width > 0) {
// C source: 		/* shift the overlapping area so it is with respect to current
// C source: 		 * ODM slice's position
// C source: 		 */
// C source: 		spl_scratch->scl_data.recout = shift_rec(
// C source: 				&overlapping_area,
// C source: 				-odm_slice.x, -odm_slice.y);
// C source: 		spl_scratch->scl_data.recout.height -=
// C source: 			spl_in->debug.visual_confirm_base_offset;
// C source: 		spl_scratch->scl_data.recout.height -=
// C source: 			spl_in->debug.visual_confirm_dpp_offset;
// C source: 	} else
// C source: 		/* if there is no overlap, zero recout */
// C source: 		memset(&spl_scratch->scl_data.recout, 0,
// C source: 				sizeof(struct spl_rect));
// C source: }
// C source: 
// C source: /* Calculate scaling ratios */
// C source: static void spl_calculate_scaling_ratios(struct spl_in *spl_in,
// C source: 		struct spl_scratch *spl_scratch,
// C source: 		struct spl_out *spl_out)
// C source: {
// C source: 	const int in_w = spl_in->basic_out.src_rect.width;
// C source: 	const int in_h = spl_in->basic_out.src_rect.height;
// C source: 	const int out_w = spl_in->basic_out.dst_rect.width;
// C source: 	const int out_h = spl_in->basic_out.dst_rect.height;
// C source: 	struct spl_rect surf_src = spl_in->basic_in.src_rect;
// C source: 
// C source: 	/*Swap surf_src height and width since scaling ratios are in recout rotation*/
// C source: 	if (spl_in->basic_in.rotation == SPL_ROTATION_ANGLE_90 ||
// C source: 		spl_in->basic_in.rotation == SPL_ROTATION_ANGLE_270)
// C source: 		spl_swap(surf_src.height, surf_src.width);
// C source: 
// C source: 	spl_scratch->scl_data.ratios.horz = SPL_NAMESPACE(spl_fixpt_from_fraction(
// C source: 					surf_src.width,
// C source: 					spl_in->basic_in.dst_rect.width));
// C source: 	spl_scratch->scl_data.ratios.vert = SPL_NAMESPACE(spl_fixpt_from_fraction(
// C source: 					surf_src.height,
// C source: 					spl_in->basic_in.dst_rect.height));
// C source: 
// C source: 	if (spl_in->basic_out.view_format == SPL_VIEW_3D_SIDE_BY_SIDE)
// C source: 		spl_scratch->scl_data.ratios.horz.value *= 2;
// C source: 	else if (spl_in->basic_out.view_format == SPL_VIEW_3D_TOP_AND_BOTTOM)
// C source: 		spl_scratch->scl_data.ratios.vert.value *= 2;
// C source: 
// C source: 	spl_scratch->scl_data.ratios.vert.value = spl_div64_s64(
// C source: 		spl_scratch->scl_data.ratios.vert.value * in_h, out_h);
// C source: 	spl_scratch->scl_data.ratios.horz.value = spl_div64_s64(
// C source: 		spl_scratch->scl_data.ratios.horz.value * in_w, out_w);
// C source: 
// C source: 	spl_scratch->scl_data.ratios.horz_c = spl_scratch->scl_data.ratios.horz;
// C source: 	spl_scratch->scl_data.ratios.vert_c = spl_scratch->scl_data.ratios.vert;
// C source: 
// C source: 	if (spl_is_yuv420(spl_in->basic_in.format)) {
// C source: 		spl_scratch->scl_data.ratios.horz_c.value /= 2;
// C source: 		spl_scratch->scl_data.ratios.vert_c.value /= 2;
// C source: 	} else if (spl_is_yuv422(spl_in->basic_in.format)) {
// C source: 		if (spl_in->basic_in.rotation == SPL_ROTATION_ANGLE_90 ||
// C source: 			spl_in->basic_in.rotation == SPL_ROTATION_ANGLE_270)
// C source: 			spl_scratch->scl_data.ratios.vert_c.value /= 2;
// C source: 		else
// C source: 			spl_scratch->scl_data.ratios.horz_c.value /= 2;
// C source: 	}
// C source: 	spl_scratch->scl_data.ratios.horz = spl_fixpt_truncate(
// C source: 			spl_scratch->scl_data.ratios.horz, 19);
// C source: 	spl_scratch->scl_data.ratios.vert = spl_fixpt_truncate(
// C source: 			spl_scratch->scl_data.ratios.vert, 19);
// C source: 	spl_scratch->scl_data.ratios.horz_c = spl_fixpt_truncate(
// C source: 			spl_scratch->scl_data.ratios.horz_c, 19);
// C source: 	spl_scratch->scl_data.ratios.vert_c = spl_fixpt_truncate(
// C source: 			spl_scratch->scl_data.ratios.vert_c, 19);
// C source: 
// C source: 	/*
// C source: 	 * Coefficient table and some registers are different based on ratio
// C source: 	 * that is output/input.  Currently we calculate input/output
// C source: 	 * Store 1/ratio in recip_ratio for those lookups
// C source: 	 */
// C source: 	spl_scratch->scl_data.recip_ratios.horz = SPL_NAMESPACE(spl_fixpt_recip(
// C source: 			spl_scratch->scl_data.ratios.horz));
// C source: 	spl_scratch->scl_data.recip_ratios.vert = SPL_NAMESPACE(spl_fixpt_recip(
// C source: 			spl_scratch->scl_data.ratios.vert));
// C source: 	spl_scratch->scl_data.recip_ratios.horz_c = SPL_NAMESPACE(spl_fixpt_recip(
// C source: 			spl_scratch->scl_data.ratios.horz_c));
// C source: 	spl_scratch->scl_data.recip_ratios.vert_c = SPL_NAMESPACE(spl_fixpt_recip(
// C source: 			spl_scratch->scl_data.ratios.vert_c));
// C source: }
// C source: 
// C source: /* Calculate Viewport size */
// C source: static void spl_calculate_viewport_size(struct spl_in *spl_in, struct spl_scratch *spl_scratch)
// C source: {
// C source: 	spl_scratch->scl_data.viewport.width = spl_fixpt_ceil(spl_fixpt_mul_int(spl_scratch->scl_data.ratios.horz,
// C source: 							spl_scratch->scl_data.recout.width));
// C source: 	spl_scratch->scl_data.viewport.height = spl_fixpt_ceil(spl_fixpt_mul_int(spl_scratch->scl_data.ratios.vert,
// C source: 							spl_scratch->scl_data.recout.height));
// C source: 	spl_scratch->scl_data.viewport_c.width = spl_fixpt_ceil(spl_fixpt_mul_int(spl_scratch->scl_data.ratios.horz_c,
// C source: 						spl_scratch->scl_data.recout.width));
// C source: 	spl_scratch->scl_data.viewport_c.height = spl_fixpt_ceil(spl_fixpt_mul_int(spl_scratch->scl_data.ratios.vert_c,
// C source: 						spl_scratch->scl_data.recout.height));
// C source: 	if (spl_in->basic_in.rotation == SPL_ROTATION_ANGLE_90 ||
// C source: 			spl_in->basic_in.rotation == SPL_ROTATION_ANGLE_270) {
// C source: 		spl_swap(spl_scratch->scl_data.viewport.width, spl_scratch->scl_data.viewport.height);
// C source: 		spl_swap(spl_scratch->scl_data.viewport_c.width, spl_scratch->scl_data.viewport_c.height);
// C source: 	}
// C source: }
// C source: 
// C source: static void spl_get_vp_scan_direction(enum spl_rotation_angle rotation,
// C source: 			   bool horizontal_mirror,
// C source: 			   bool *orthogonal_rotation,
// C source: 			   bool *flip_vert_scan_dir,
// C source: 			   bool *flip_horz_scan_dir)
// C source: {
// C source: 	*orthogonal_rotation = false;
// C source: 	*flip_vert_scan_dir = false;
// C source: 	*flip_horz_scan_dir = false;
// C source: 	if (rotation == SPL_ROTATION_ANGLE_180) {
// C source: 		*flip_vert_scan_dir = true;
// C source: 		*flip_horz_scan_dir = true;
// C source: 	} else if (rotation == SPL_ROTATION_ANGLE_90) {
// C source: 		*orthogonal_rotation = true;
// C source: 		*flip_horz_scan_dir = true;
// C source: 	} else if (rotation == SPL_ROTATION_ANGLE_270) {
// C source: 		*orthogonal_rotation = true;
// C source: 		*flip_vert_scan_dir = true;
// C source: 	}
// C source: 
// C source: 	if (horizontal_mirror)
// C source: 		*flip_horz_scan_dir = !*flip_horz_scan_dir;
// C source: }
// C source: 
// C source: /*
// C source:  * We completely calculate vp offset, size and inits here based entirely on scaling
// C source:  * ratios and recout for pixel perfect pipe combine.
// C source:  */
// C source: static void spl_calculate_init_and_vp(bool flip_scan_dir,
// C source: 				int recout_offset_within_recout_full,
// C source: 				int recout_size,
// C source: 				int src_size,
// C source: 				int taps,
// C source: 				struct spl_fixed31_32 ratio,
// C source: 				struct spl_fixed31_32 init_adj,
// C source: 				struct spl_fixed31_32 *init,
// C source: 				int *vp_offset,
// C source: 				int *vp_size)
// C source: {
// C source: 	struct spl_fixed31_32 temp;
// C source: 	int int_part;
// C source: 
// C source: 	/*
// C source: 	 * First of the taps starts sampling pixel number <init_int_part> corresponding to recout
// C source: 	 * pixel 1. Next recout pixel samples int part of <init + scaling ratio> and so on.
// C source: 	 * All following calculations are based on this logic.
// C source: 	 *
// C source: 	 * Init calculated according to formula:
// C source: 	 * init = (scaling_ratio + number_of_taps + 1) / 2
// C source: 	 * init_bot = init + scaling_ratio
// C source: 	 * to get pixel perfect combine add the fraction from calculating vp offset
// C source: 	 */
// C source: 	temp = spl_fixpt_mul_int(ratio, recout_offset_within_recout_full);
// C source: 	*vp_offset = spl_fixpt_floor(temp);
// C source: 	temp.value &= 0xffffffff;
// C source: 	*init = spl_fixpt_add(spl_fixpt_div_int(spl_fixpt_add_int(ratio, taps + 1), 2), temp);
// C source: 	*init = spl_fixpt_add(*init, init_adj);
// C source: 	*init = spl_fixpt_truncate(*init, 19);
// C source: 
// C source: 	/*
// C source: 	 * If viewport has non 0 offset and there are more taps than covered by init then
// C source: 	 * we should decrease the offset and increase init so we are never sampling
// C source: 	 * outside of viewport.
// C source: 	 */
// C source: 	int_part = spl_fixpt_floor(*init);
// C source: 	if (int_part < taps) {
// C source: 		int_part = taps - int_part;
// C source: 		if (int_part > *vp_offset)
// C source: 			int_part = *vp_offset;
// C source: 		*vp_offset -= int_part;
// C source: 		*init = spl_fixpt_add_int(*init, int_part);
// C source: 	}
// C source: 	/*
// C source: 	 * If taps are sampling outside of viewport at end of recout and there are more pixels
// C source: 	 * available in the surface we should increase the viewport size, regardless set vp to
// C source: 	 * only what is used.
// C source: 	 */
// C source: 	temp = spl_fixpt_add(*init, spl_fixpt_mul_int(ratio, recout_size - 1));
// C source: 	*vp_size = spl_fixpt_floor(temp);
// C source: 	if (*vp_size + *vp_offset > src_size)
// C source: 		*vp_size = src_size - *vp_offset;
// C source: 
// C source: 	/* We did all the math assuming we are scanning same direction as display does,
// C source: 	 * however mirror/rotation changes how vp scans vs how it is offset. If scan direction
// C source: 	 * is flipped we simply need to calculate offset from the other side of plane.
// C source: 	 * Note that outside of viewport all scaling hardware works in recout space.
// C source: 	 */
// C source: 	if (flip_scan_dir)
// C source: 		*vp_offset = src_size - *vp_offset - *vp_size;
// C source: }
// C source: 
// C source: /*Calculate inits and viewport */
// C source: static void spl_calculate_inits_and_viewports(struct spl_in *spl_in,
// C source: 		struct spl_scratch *spl_scratch)
// C source: {
// C source: 	struct spl_rect src = spl_in->basic_in.src_rect;
// C source: 	struct spl_rect recout_dst_in_active_timing;
// C source: 	struct spl_rect recout_clip_in_active_timing;
// C source: 	struct spl_rect recout_clip_in_recout_dst;
// C source: 	struct spl_rect overlap_in_active_timing;
// C source: 	struct spl_rect odm_slice = calculate_odm_slice_in_timing_active(spl_in);
// C source: 	int vp_hc_div = spl_is_subsampled_format(spl_in->basic_in.format) ? 2 : 1;
// C source: 	int vp_vc_div = spl_is_yuv420(spl_in->basic_in.format) ? 2 : 1;
// C source: 	bool orthogonal_rotation, flip_vert_scan_dir, flip_horz_scan_dir;
// C source: 	struct spl_fixed31_32 init_adj_h = spl_fixpt_zero;
// C source: 	struct spl_fixed31_32 init_adj_v = spl_fixpt_zero;
// C source: 
// C source: 	recout_clip_in_active_timing = shift_rec(
// C source: 			&spl_scratch->scl_data.recout, odm_slice.x, odm_slice.y);
// C source: 	recout_dst_in_active_timing = calculate_plane_rec_in_timing_active(
// C source: 			spl_in, &spl_in->basic_in.dst_rect);
// C source: 	overlap_in_active_timing = intersect_rec(&recout_clip_in_active_timing,
// C source: 			&recout_dst_in_active_timing);
// C source: 	if (overlap_in_active_timing.width > 0 &&
// C source: 			overlap_in_active_timing.height > 0)
// C source: 		recout_clip_in_recout_dst = shift_rec(&overlap_in_active_timing,
// C source: 				-recout_dst_in_active_timing.x,
// C source: 				-recout_dst_in_active_timing.y);
// C source: 	else
// C source: 		memset(&recout_clip_in_recout_dst, 0, sizeof(struct spl_rect));
// C source: 	/*
// C source: 	 * Work in recout rotation since that requires less transformations
// C source: 	 */
// C source: 	spl_get_vp_scan_direction(
// C source: 			spl_in->basic_in.rotation,
// C source: 			spl_in->basic_in.horizontal_mirror,
// C source: 			&orthogonal_rotation,
// C source: 			&flip_vert_scan_dir,
// C source: 			&flip_horz_scan_dir);
// C source: 
// C source: 	if (spl_is_subsampled_format(spl_in->basic_in.format)) {
// C source: 		/* this gives the direction of the cositing (negative will move
// C source: 		 * left, right otherwise)
// C source: 		 */
// C source: 		int h_sign = flip_horz_scan_dir ? -1 : 1;
// C source: 		int v_sign = flip_vert_scan_dir ? -1 : 1;
// C source: 
// C source: 		switch (spl_in->basic_in.cositing) {
// C source: 		case CHROMA_COSITING_TOPLEFT:
// C source: 			init_adj_h = SPL_NAMESPACE(spl_fixpt_from_fraction(h_sign, 4));
// C source: 			init_adj_v = SPL_NAMESPACE(spl_fixpt_from_fraction(v_sign, 4));
// C source: 			break;
// C source: 		case CHROMA_COSITING_LEFT:
// C source: 			init_adj_h = SPL_NAMESPACE(spl_fixpt_from_fraction(h_sign, 4));
// C source: 			init_adj_v = spl_fixpt_zero;
// C source: 			break;
// C source: 		case CHROMA_COSITING_NONE:
// C source: 		default:
// C source: 			init_adj_h = spl_fixpt_zero;
// C source: 			init_adj_v = spl_fixpt_zero;
// C source: 			break;
// C source: 		}
// C source: 	}
// C source: 
// C source: 	if (orthogonal_rotation) {
// C source: 		spl_swap(src.width, src.height);
// C source: 		spl_swap(flip_vert_scan_dir, flip_horz_scan_dir);
// C source: 		spl_swap(vp_hc_div, vp_vc_div);
// C source: 		spl_swap(init_adj_h, init_adj_v);
// C source: 	}
// C source: 
// C source: 	spl_calculate_init_and_vp(
// C source: 			flip_horz_scan_dir,
// C source: 			recout_clip_in_recout_dst.x,
// C source: 			spl_scratch->scl_data.recout.width,
// C source: 			src.width,
// C source: 			spl_scratch->scl_data.taps.h_taps,
// C source: 			spl_scratch->scl_data.ratios.horz,
// C source: 			spl_fixpt_zero,
// C source: 			&spl_scratch->scl_data.inits.h,
// C source: 			&spl_scratch->scl_data.viewport.x,
// C source: 			&spl_scratch->scl_data.viewport.width);
// C source: 	spl_calculate_init_and_vp(
// C source: 			flip_horz_scan_dir,
// C source: 			recout_clip_in_recout_dst.x,
// C source: 			spl_scratch->scl_data.recout.width,
// C source: 			src.width / vp_hc_div,
// C source: 			spl_scratch->scl_data.taps.h_taps_c,
// C source: 			spl_scratch->scl_data.ratios.horz_c,
// C source: 			init_adj_h,
// C source: 			&spl_scratch->scl_data.inits.h_c,
// C source: 			&spl_scratch->scl_data.viewport_c.x,
// C source: 			&spl_scratch->scl_data.viewport_c.width);
// C source: 	spl_calculate_init_and_vp(
// C source: 			flip_vert_scan_dir,
// C source: 			recout_clip_in_recout_dst.y,
// C source: 			spl_scratch->scl_data.recout.height,
// C source: 			src.height,
// C source: 			spl_scratch->scl_data.taps.v_taps,
// C source: 			spl_scratch->scl_data.ratios.vert,
// C source: 			spl_fixpt_zero,
// C source: 			&spl_scratch->scl_data.inits.v,
// C source: 			&spl_scratch->scl_data.viewport.y,
// C source: 			&spl_scratch->scl_data.viewport.height);
// C source: 	spl_calculate_init_and_vp(
// C source: 			flip_vert_scan_dir,
// C source: 			recout_clip_in_recout_dst.y,
// C source: 			spl_scratch->scl_data.recout.height,
// C source: 			src.height / vp_vc_div,
// C source: 			spl_scratch->scl_data.taps.v_taps_c,
// C source: 			spl_scratch->scl_data.ratios.vert_c,
// C source: 			init_adj_v,
// C source: 			&spl_scratch->scl_data.inits.v_c,
// C source: 			&spl_scratch->scl_data.viewport_c.y,
// C source: 			&spl_scratch->scl_data.viewport_c.height);
// C source: 	if (orthogonal_rotation) {
// C source: 		spl_swap(spl_scratch->scl_data.viewport.x, spl_scratch->scl_data.viewport.y);
// C source: 		spl_swap(spl_scratch->scl_data.viewport.width, spl_scratch->scl_data.viewport.height);
// C source: 		spl_swap(spl_scratch->scl_data.viewport_c.x, spl_scratch->scl_data.viewport_c.y);
// C source: 		spl_swap(spl_scratch->scl_data.viewport_c.width, spl_scratch->scl_data.viewport_c.height);
// C source: 		spl_swap(vp_hc_div, vp_vc_div);
// C source: 	}
// C source: 	spl_scratch->scl_data.viewport.x += src.x;
// C source: 	spl_scratch->scl_data.viewport.y += src.y;
// C source: 	SPL_ASSERT(src.x % vp_hc_div == 0 && src.y % vp_vc_div == 0);
// C source: 	spl_scratch->scl_data.viewport_c.x += src.x / vp_hc_div;
// C source: 	spl_scratch->scl_data.viewport_c.y += src.y / vp_vc_div;
// C source: }
// C source: 
// C source: static void spl_handle_3d_recout(struct spl_in *spl_in, struct spl_rect *recout)
// C source: {
// C source: 	/*
// C source: 	 * Handle side by side and top bottom 3d recout offsets after vp calculation
// C source: 	 * since 3d is special and needs to calculate vp as if there is no recout offset
// C source: 	 * This may break with rotation, good thing we aren't mixing hw rotation and 3d
// C source: 	 */
// C source: 	if (spl_in->basic_in.mpc_h_slice_index) {
// C source: 		SPL_ASSERT(spl_in->basic_in.rotation == SPL_ROTATION_ANGLE_0 ||
// C source: 			(spl_in->basic_out.view_format != SPL_VIEW_3D_TOP_AND_BOTTOM &&
// C source: 					spl_in->basic_out.view_format != SPL_VIEW_3D_SIDE_BY_SIDE));
// C source: 		if (spl_in->basic_out.view_format == SPL_VIEW_3D_TOP_AND_BOTTOM)
// C source: 			recout->y += recout->height;
// C source: 		else if (spl_in->basic_out.view_format == SPL_VIEW_3D_SIDE_BY_SIDE)
// C source: 			recout->x += recout->width;
// C source: 	}
// C source: }
// C source: 
// C source: static void spl_clamp_viewport(struct spl_rect *viewport, int min_viewport_size)
// C source: {
// C source: 	if (min_viewport_size == 0)
// C source: 		min_viewport_size = MIN_VIEWPORT_SIZE;
// C source: 	/* Clamp minimum viewport size */
// C source: 	if (viewport->height < min_viewport_size)
// C source: 		viewport->height = min_viewport_size;
// C source: 	if (viewport->width < min_viewport_size)
// C source: 		viewport->width = min_viewport_size;
// C source: }
// C source: 
// C source: static enum scl_mode spl_get_dscl_mode(const struct spl_in *spl_in,
// C source: 				const struct spl_scaler_data *data,
// C source: 				bool enable_isharp, bool enable_easf)
// C source: {
// C source: 	(void)enable_easf;
// C source: 	const long long one = spl_fixpt_one.value;
// C source: 	enum spl_pixel_format pixel_format = spl_in->basic_in.format;
// C source: 
// C source: 	/* Bypass if ratio is 1:1 with no ISHARP or force scale on */
// C source: 	if (data->ratios.horz.value == one
// C source: 			&& data->ratios.vert.value == one
// C source: 			&& data->ratios.horz_c.value == one
// C source: 			&& data->ratios.vert_c.value == one
// C source: 			&& !spl_in->basic_out.always_scale
// C source: 			&& !enable_isharp)
// C source: 		return SCL_MODE_SCALING_444_BYPASS;
// C source: 
// C source: 	if (!spl_is_subsampled_format(pixel_format)) {
// C source: 		if (spl_is_video_format(pixel_format))
// C source: 			return SCL_MODE_SCALING_444_YCBCR_ENABLE;
// C source: 		else
// C source: 			return SCL_MODE_SCALING_444_RGB_ENABLE;
// C source: 	}
// C source: 
// C source: 	/*
// C source: 	 * Bypass YUV if Y is 1:1 with no ISHARP
// C source: 	 * Do not bypass UV at 1:1 for cositing to be applied
// C source: 	 */
// C source: 	if (!enable_isharp) {
// C source: 		if (data->ratios.horz.value == one && data->ratios.vert.value == one && !spl_in->basic_out.always_scale)
// C source: 			return SCL_MODE_SCALING_420_LUMA_BYPASS;
// C source: 	}
// C source: 
// C source: 	return SCL_MODE_SCALING_420_YCBCR_ENABLE;
// C source: }
// C source: 
// C source: static void spl_choose_lls_policy(enum spl_pixel_format format,
// C source: 	enum linear_light_scaling *lls_pref)
// C source: {
// C source: 	if (spl_is_subsampled_format(format))
// C source: 		*lls_pref = LLS_PREF_NO;
// C source: 	else /* RGB or YUV444 */
// C source: 		*lls_pref = LLS_PREF_YES;
// C source: }
// C source: 
// C source: /* Enable EASF ?*/
// C source: static bool enable_easf(struct spl_in *spl_in, struct spl_scratch *spl_scratch)
// C source: {
// C source: 	int vratio = 0;
// C source: 	int hratio = 0;
// C source: 	bool skip_easf = false;
// C source: 
// C source: 	if (spl_in->disable_easf)
// C source: 		skip_easf = true;
// C source: 
// C source: 	vratio = spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert);
// C source: 	hratio = spl_fixpt_ceil(spl_scratch->scl_data.ratios.horz);
// C source: 
// C source: 	/*
// C source: 	 * No EASF support for downscaling > 2:1
// C source: 	 * EASF support for upscaling or downscaling up to 2:1
// C source: 	 */
// C source: 	if ((vratio > 2) || (hratio > 2))
// C source: 		skip_easf = true;
// C source: 
// C source: 	/*
// C source: 	 * If lls_pref is LLS_PREF_DONT_CARE, then use pixel format
// C source: 	 *  to determine whether to use LINEAR or NONLINEAR scaling
// C source: 	 */
// C source: 	if (spl_in->lls_pref == LLS_PREF_DONT_CARE)
// C source: 		spl_choose_lls_policy(spl_in->basic_in.format,
// C source: 			&spl_in->lls_pref);
// C source: 
// C source: 	/* Check for linear scaling or EASF preferred */
// C source: 	if (spl_in->lls_pref != LLS_PREF_YES && !spl_in->prefer_easf)
// C source: 		skip_easf = true;
// C source: 
// C source: 	return skip_easf;
// C source: }
// C source: 
// C source: /* Check if video is in fullscreen mode */
// C source: static bool spl_is_video_fullscreen(struct spl_in *spl_in)
// C source: {
// C source: 	if (spl_is_video_format(spl_in->basic_in.format) && spl_in->is_fullscreen)
// C source: 		return true;
// C source: 	return false;
// C source: }
// C source: 
// C source: static bool spl_get_isharp_en(struct spl_in *spl_in,
// C source: 	struct spl_scratch *spl_scratch)
// C source: {
// C source: 	bool enable_isharp = false;
// C source: 	int vratio = 0;
// C source: 	int hratio = 0;
// C source: 	struct spl_taps taps = spl_scratch->scl_data.taps;
// C source: 	bool fullscreen = spl_is_video_fullscreen(spl_in);
// C source: 
// C source: 	/* Return if adaptive sharpness is disabled */
// C source: 	if (spl_in->adaptive_sharpness.enable == false)
// C source: 		return enable_isharp;
// C source: 
// C source: 	vratio = spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert);
// C source: 	hratio = spl_fixpt_ceil(spl_scratch->scl_data.ratios.horz);
// C source: 
// C source: 	/* No iSHARP support for downscaling */
// C source: 	if (vratio > 1 || hratio > 1)
// C source: 		return enable_isharp;
// C source: 
// C source: 	// Scaling is up to 1:1 (no scaling) or upscaling
// C source: 
// C source: 	/*
// C source: 	 * Apply sharpness to RGB and YUV (NV12/P010)
// C source: 	 *  surfaces based on policy setting
// C source: 	 */
// C source: 	if (!spl_is_video_format(spl_in->basic_in.format) &&
// C source: 		(spl_in->sharpen_policy == SHARPEN_YUV))
// C source: 		return enable_isharp;
// C source: 	else if ((spl_is_video_format(spl_in->basic_in.format) && !fullscreen) &&
// C source: 		(spl_in->sharpen_policy == SHARPEN_RGB_FULLSCREEN_YUV))
// C source: 		return enable_isharp;
// C source: 	else if (!spl_in->is_fullscreen &&
// C source: 			spl_in->sharpen_policy == SHARPEN_FULLSCREEN_ALL)
// C source: 		return enable_isharp;
// C source: 
// C source: 	/*
// C source: 	 * Apply sharpness if supports horizontal taps 4,6 AND
// C source: 	 *  vertical taps 3, 4, 6
// C source: 	 */
// C source: 	if ((taps.h_taps == 4 || taps.h_taps == 6) &&
// C source: 		(taps.v_taps == 3 || taps.v_taps == 4 || taps.v_taps == 6))
// C source: 		enable_isharp = true;
// C source: 
// C source: 	return enable_isharp;
// C source: }
// C source: 
// C source: /* Calculate number of tap with adaptive scaling off */
// C source: static void spl_get_taps_non_adaptive_scaler(
// C source: 		struct spl_scratch *spl_scratch,
// C source: 		const struct spl_taps *in_taps,
// C source: 		bool is_horz_subsampled,
// C source: 		bool is_vert_subsampled)
// C source: {
// C source: 	bool check_max_downscale = false;
// C source: 
// C source: 	if (in_taps->h_taps == 0) {
// C source: 		if (spl_fixpt_ceil(spl_scratch->scl_data.ratios.horz) > 1)
// C source: 			spl_scratch->scl_data.taps.h_taps = spl_min(2 * spl_fixpt_ceil(
// C source: 				spl_scratch->scl_data.ratios.horz), 8);
// C source: 		else
// C source: 			spl_scratch->scl_data.taps.h_taps = 4;
// C source: 	} else
// C source: 		spl_scratch->scl_data.taps.h_taps = in_taps->h_taps;
// C source: 
// C source: 	if (in_taps->v_taps == 0) {
// C source: 		if (spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert) > 1)
// C source: 			spl_scratch->scl_data.taps.v_taps = spl_min(2 * spl_fixpt_ceil(
// C source: 				spl_scratch->scl_data.ratios.vert), 8);
// C source: 		else
// C source: 			spl_scratch->scl_data.taps.v_taps = 4;
// C source: 	} else
// C source: 		spl_scratch->scl_data.taps.v_taps = in_taps->v_taps;
// C source: 
// C source: 	if (in_taps->v_taps_c == 0) {
// C source: 		if (spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert_c) > 1)
// C source: 			spl_scratch->scl_data.taps.v_taps_c = spl_min(2 * spl_fixpt_ceil(
// C source: 				spl_scratch->scl_data.ratios.vert_c), 8);
// C source: 		else
// C source: 			spl_scratch->scl_data.taps.v_taps_c = 4;
// C source: 	} else
// C source: 		spl_scratch->scl_data.taps.v_taps_c = in_taps->v_taps_c;
// C source: 
// C source: 	if (in_taps->h_taps_c == 0) {
// C source: 		if (spl_fixpt_ceil(spl_scratch->scl_data.ratios.horz_c) > 1)
// C source: 			spl_scratch->scl_data.taps.h_taps_c = spl_min(2 * spl_fixpt_ceil(
// C source: 				spl_scratch->scl_data.ratios.horz_c), 8);
// C source: 		else
// C source: 			spl_scratch->scl_data.taps.h_taps_c = 4;
// C source: 	} else if ((in_taps->h_taps_c % 2) != 0 && in_taps->h_taps_c != 1)
// C source: 		/* Only 1 and even h_taps_c are supported by hw */
// C source: 		spl_scratch->scl_data.taps.h_taps_c = in_taps->h_taps_c - 1;
// C source: 	else
// C source: 		spl_scratch->scl_data.taps.h_taps_c = in_taps->h_taps_c;
// C source: 
// C source: 
// C source: 	/*
// C source: 	 * Max downscale supported is 6.0x.  Add ASSERT to catch if go beyond that
// C source: 	 */
// C source: 	check_max_downscale = spl_fixpt_le(spl_scratch->scl_data.ratios.horz,
// C source: 		SPL_NAMESPACE(spl_fixpt_from_fraction(6, 1)));
// C source: 	SPL_ASSERT(check_max_downscale);
// C source: 	check_max_downscale = spl_fixpt_le(spl_scratch->scl_data.ratios.vert,
// C source: 		SPL_NAMESPACE(spl_fixpt_from_fraction(6, 1)));
// C source: 	SPL_ASSERT(check_max_downscale);
// C source: 	check_max_downscale = spl_fixpt_le(spl_scratch->scl_data.ratios.horz_c,
// C source: 		SPL_NAMESPACE(spl_fixpt_from_fraction(6, 1)));
// C source: 	SPL_ASSERT(check_max_downscale);
// C source: 	check_max_downscale = spl_fixpt_le(spl_scratch->scl_data.ratios.vert_c,
// C source: 		SPL_NAMESPACE(spl_fixpt_from_fraction(6, 1)));
// C source: 	SPL_ASSERT(check_max_downscale);
// C source: 
// C source: 
// C source: 	if (IDENTITY_RATIO(spl_scratch->scl_data.ratios.horz))
// C source: 		spl_scratch->scl_data.taps.h_taps = 1;
// C source: 	if (IDENTITY_RATIO(spl_scratch->scl_data.ratios.vert))
// C source: 		spl_scratch->scl_data.taps.v_taps = 1;
// C source: 	if (IDENTITY_RATIO(spl_scratch->scl_data.ratios.horz_c) && !is_horz_subsampled)
// C source: 		spl_scratch->scl_data.taps.h_taps_c = 1;
// C source: 	if (IDENTITY_RATIO(spl_scratch->scl_data.ratios.vert_c) && !is_vert_subsampled)
// C source: 		spl_scratch->scl_data.taps.v_taps_c = 1;
// C source: }
// C source: 
// C source: /* Calculate optimal number of taps */
// C source: static bool spl_get_optimal_number_of_taps(
// C source: 	  int max_downscale_src_width, struct spl_in *spl_in, struct spl_scratch *spl_scratch,
// C source: 	  const struct spl_taps *in_taps, bool *enable_easf_v, bool *enable_easf_h,
// C source: 	  bool *enable_isharp)
// C source: {
// C source: 	int num_part_y, num_part_c;
// C source: 	unsigned int max_taps_y, max_taps_c;
// C source: 	unsigned int min_taps_y, min_taps_c;
// C source: 	enum lb_memory_config lb_config;
// C source: 	bool skip_easf          = false;
// C source: 	bool is_horz_subsampled = spl_is_subsampled_format(spl_in->basic_in.format);
// C source: 	bool is_vert_subsampled = spl_is_yuv420(spl_in->basic_in.format);
// C source: 
// C source: 	if (spl_scratch->scl_data.viewport.width > spl_scratch->scl_data.h_active &&
// C source: 		max_downscale_src_width != 0 &&
// C source: 		spl_scratch->scl_data.viewport.width > max_downscale_src_width) {
// C source: 		spl_get_taps_non_adaptive_scaler(spl_scratch, in_taps, is_horz_subsampled, is_vert_subsampled);
// C source: 		*enable_easf_v = false;
// C source: 		*enable_easf_h = false;
// C source: 		*enable_isharp = false;
// C source: 		return false;
// C source: 	}
// C source: 
// C source: 	/* Disable adaptive scaler and sharpener when integer scaling is enabled */
// C source: 	if (spl_in->scaling_quality.integer_scaling) {
// C source: 		spl_get_taps_non_adaptive_scaler(spl_scratch, in_taps, is_horz_subsampled, is_vert_subsampled);
// C source: 		*enable_easf_v = false;
// C source: 		*enable_easf_h = false;
// C source: 		*enable_isharp = false;
// C source: 		return true;
// C source: 	}
// C source: 
// C source: 	/* Check if we are using EASF or not */
// C source: 	skip_easf = enable_easf(spl_in, spl_scratch);
// C source: 
// C source: 	/*
// C source: 	 * Set default taps if none are provided
// C source: 	 * From programming guide: taps = min{ ceil(2*H_RATIO,1), 8} for downscaling
// C source: 	 * taps = 4 for upscaling
// C source: 	 */
// C source: 	if (skip_easf) {
// C source: 		spl_get_taps_non_adaptive_scaler(spl_scratch, in_taps, is_horz_subsampled, is_vert_subsampled);
// C source: 	}
// C source: 	else {
// C source: 		if (spl_is_subsampled_format(spl_in->basic_in.format)) {
// C source: 			spl_scratch->scl_data.taps.h_taps = 6;
// C source: 			spl_scratch->scl_data.taps.v_taps = 6;
// C source: 			spl_scratch->scl_data.taps.h_taps_c = 4;
// C source: 			spl_scratch->scl_data.taps.v_taps_c = 4;
// C source: 		} else { /* RGB / YUV444 */
// C source: 			spl_scratch->scl_data.taps.h_taps = 6;
// C source: 			spl_scratch->scl_data.taps.v_taps = 6;
// C source: 			spl_scratch->scl_data.taps.h_taps_c = 6;
// C source: 			spl_scratch->scl_data.taps.v_taps_c = 6;
// C source: 		}
// C source: 
// C source: 		/* Override mode: keep EASF enabled but use input taps if valid */
// C source: 		if (spl_in->override_easf) {
// C source: 			spl_scratch->scl_data.taps.h_taps = (in_taps->h_taps != 0) ? in_taps->h_taps : spl_scratch->scl_data.taps.h_taps;
// C source: 			spl_scratch->scl_data.taps.v_taps = (in_taps->v_taps != 0) ? in_taps->v_taps : spl_scratch->scl_data.taps.v_taps;
// C source: 			spl_scratch->scl_data.taps.h_taps_c = (in_taps->h_taps_c != 0) ? in_taps->h_taps_c : spl_scratch->scl_data.taps.h_taps_c;
// C source: 			spl_scratch->scl_data.taps.v_taps_c = (in_taps->v_taps_c != 0) ? in_taps->v_taps_c : spl_scratch->scl_data.taps.v_taps_c;
// C source: 
// C source: 			if ((spl_scratch->scl_data.taps.h_taps > 6) || (spl_scratch->scl_data.taps.v_taps > 6))
// C source: 				skip_easf = true;
// C source: 			if ((spl_scratch->scl_data.taps.h_taps > 1) && (spl_scratch->scl_data.taps.h_taps % 2))
// C source: 				spl_scratch->scl_data.taps.h_taps--;
// C source: 			if ((spl_scratch->scl_data.taps.h_taps_c > 1) && (spl_scratch->scl_data.taps.h_taps_c % 2))
// C source: 				spl_scratch->scl_data.taps.h_taps_c--;
// C source: 		}
// C source: 	}
// C source: 
// C source: 	/*Ensure we can support the requested number of vtaps*/
// C source: 	min_taps_y = spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert);
// C source: 	min_taps_c = spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert_c);
// C source: 
// C source: 	/* Use LB_MEMORY_CONFIG_3 for 4:2:0 */
// C source: 	if (spl_is_yuv420(spl_in->basic_in.format))
// C source: 		lb_config = LB_MEMORY_CONFIG_3;
// C source: 	else
// C source: 		lb_config = LB_MEMORY_CONFIG_0;
// C source: 	// Determine max vtap support by calculating how much line buffer can fit
// C source: 	spl_in->callbacks.spl_calc_lb_num_partitions(spl_in->basic_out.alpha_en, &spl_scratch->scl_data,
// C source: 			lb_config, &num_part_y, &num_part_c);
// C source: 	/* MAX_V_TAPS = MIN (NUM_LINES - MAX(CEILING(V_RATIO,1)-2, 0), 8) */
// C source: 	if (spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert) > 2)
// C source: 		if ((spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert) - 2) > num_part_y)
// C source: 			max_taps_y = 0;
// C source: 		else
// C source: 			max_taps_y = num_part_y - (spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert) - 2);
// C source: 	else
// C source: 		max_taps_y = num_part_y;
// C source: 
// C source: 	if (spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert_c) > 2)
// C source: 		if ((spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert_c) - 2) > num_part_c)
// C source: 			max_taps_c = 0;
// C source: 		else
// C source: 			max_taps_c = num_part_c - (spl_fixpt_ceil(spl_scratch->scl_data.ratios.vert_c) - 2);
// C source: 	else
// C source: 		max_taps_c = num_part_c;
// C source: 
// C source: 	if (max_taps_y < min_taps_y)
// C source: 		return false;
// C source: 	else if (max_taps_c < min_taps_c)
// C source: 		return false;
// C source: 
// C source: 	if (spl_scratch->scl_data.taps.v_taps > max_taps_y)
// C source: 		spl_scratch->scl_data.taps.v_taps = max_taps_y;
// C source: 
// C source: 	if (spl_scratch->scl_data.taps.v_taps_c > max_taps_c)
// C source: 		spl_scratch->scl_data.taps.v_taps_c = max_taps_c;
// C source: 
// C source: 	if (!skip_easf) {
// C source: 		/*
// C source: 		 * RGB ( L + NL ) and Linear HDR support 6x6, 6x4, 6x3, 4x4, 4x3
// C source: 		 * NL YUV420 only supports 6x6, 6x4 for Y and 4x4 for UV
// C source: 		 *
// C source: 		 * If LB does not support 3, 4, or 6 taps, then disable EASF_V
// C source: 		 *  and only enable EASF_H.  So for RGB, support 6x2, 4x2
// C source: 		 *  and for NL YUV420, support 6x2 for Y and 4x2 for UV
// C source: 		 *
// C source: 		 * All other cases, have to disable EASF_V and EASF_H
// C source: 		 *
// C source: 		 * If optimal no of taps is 5, then set it to 4
// C source: 		 * If optimal no of taps is 7 or 8, then fine since max tap is 6
// C source: 		 *
// C source: 		 */
// C source: 		if (spl_scratch->scl_data.taps.v_taps == 5)
// C source: 			spl_scratch->scl_data.taps.v_taps = 4;
// C source: 
// C source: 		if (spl_scratch->scl_data.taps.v_taps_c == 5)
// C source: 			spl_scratch->scl_data.taps.v_taps_c = 4;
// C source: 
// C source: 		if (spl_scratch->scl_data.taps.h_taps == 5)
// C source: 			spl_scratch->scl_data.taps.h_taps = 4;
// C source: 
// C source: 		if (spl_scratch->scl_data.taps.h_taps_c == 5)
// C source: 			spl_scratch->scl_data.taps.h_taps_c = 4;
// C source: 
// C source: 		if (spl_is_video_format(spl_in->basic_in.format)) {
// C source: 			if (spl_scratch->scl_data.taps.h_taps <= 4) {
// C source: 				*enable_easf_v = false;
// C source: 				*enable_easf_h = false;
// C source: 			} else if (spl_scratch->scl_data.taps.v_taps <= 3) {
// C source: 				*enable_easf_v = false;
// C source: 				*enable_easf_h = true;
// C source: 			} else {
// C source: 				*enable_easf_v = true;
// C source: 				*enable_easf_h = true;
// C source: 			}
// C source: 			SPL_ASSERT((spl_scratch->scl_data.taps.v_taps > 1) &&
// C source: 				(spl_scratch->scl_data.taps.v_taps_c > 1));
// C source: 		} else { /* RGB */
// C source: 			if (spl_scratch->scl_data.taps.h_taps <= 3) {
// C source: 				*enable_easf_v = false;
// C source: 				*enable_easf_h = false;
// C source: 			} else if (spl_scratch->scl_data.taps.v_taps < 3) {
// C source: 				*enable_easf_v = false;
// C source: 				*enable_easf_h = true;
// C source: 			} else {
// C source: 				*enable_easf_v = true;
// C source: 				*enable_easf_h = true;
// C source: 			}
// C source: 			SPL_ASSERT(spl_scratch->scl_data.taps.v_taps > 1);
// C source: 		}
// C source: 	} else {
// C source: 		*enable_easf_v = false;
// C source: 		*enable_easf_h = false;
// C source: 	} // end of if prefer_easf
// C source: 
// C source: 	/* Sharpener requires scaler to be enabled, including for 1:1
// C source: 	 * Check if ISHARP can be enabled
// C source: 	 * If ISHARP is not enabled, set taps to 1 if ratio is 1:1
// C source: 	 *  except for chroma taps.  Keep previous taps so it can
// C source: 	 *  handle cositing
// C source: 	 */
// C source: 
// C source: 	*enable_isharp = spl_get_isharp_en(spl_in, spl_scratch);
// C source: 	if (!*enable_isharp && !spl_in->basic_out.always_scale)	{
// C source: 		if ((IDENTITY_RATIO(spl_scratch->scl_data.ratios.horz)) &&
// C source: 			(IDENTITY_RATIO(spl_scratch->scl_data.ratios.vert))) {
// C source: 			spl_scratch->scl_data.taps.h_taps = 1;
// C source: 			spl_scratch->scl_data.taps.v_taps = 1;
// C source: 			if (IDENTITY_RATIO(spl_scratch->scl_data.ratios.horz_c) && !is_horz_subsampled)
// C source: 				spl_scratch->scl_data.taps.h_taps_c = 1;
// C source: 
// C source: 			if (IDENTITY_RATIO(spl_scratch->scl_data.ratios.vert_c) && !is_vert_subsampled)
// C source: 				spl_scratch->scl_data.taps.v_taps_c = 1;
// C source: 
// C source: 			*enable_easf_v = false;
// C source: 			*enable_easf_h = false;
// C source: 		} else {
// C source: 			if ((!*enable_easf_h) &&
// C source: 				(IDENTITY_RATIO(spl_scratch->scl_data.ratios.horz)))
// C source: 				spl_scratch->scl_data.taps.h_taps = 1;
// C source: 
// C source: 			if ((!*enable_easf_v) &&
// C source: 				(IDENTITY_RATIO(spl_scratch->scl_data.ratios.vert)))
// C source: 				spl_scratch->scl_data.taps.v_taps = 1;
// C source: 
// C source: 			if ((!*enable_easf_h) && !is_horz_subsampled &&
// C source: 				(IDENTITY_RATIO(spl_scratch->scl_data.ratios.horz_c)))
// C source: 				spl_scratch->scl_data.taps.h_taps_c = 1;
// C source: 
// C source: 			if ((!*enable_easf_v) && !is_vert_subsampled &&
// C source: 				(IDENTITY_RATIO(spl_scratch->scl_data.ratios.vert_c)))
// C source: 				spl_scratch->scl_data.taps.v_taps_c = 1;
// C source: 
// C source: 		}
// C source: 	}
// C source: 	return true;
// C source: }
// C source: 
// C source: static void spl_set_black_color_data(enum spl_pixel_format format,
// C source: 			struct scl_black_color *scl_black_color)
// C source: {
// C source: 	bool ycbcr = spl_is_video_format(format);
// C source: 	if (ycbcr)	{
// C source: 		scl_black_color->offset_rgb_y = BLACK_OFFSET_RGB_Y;
// C source: 		scl_black_color->offset_rgb_cbcr = BLACK_OFFSET_CBCR;
// C source: 	}	else {
// C source: 		scl_black_color->offset_rgb_y = 0x0;
// C source: 		scl_black_color->offset_rgb_cbcr = 0x0;
// C source: 	}
// C source: }
// C source: 
// C source: static void spl_set_manual_ratio_init_data(struct dscl_prog_data *dscl_prog_data,
// C source: 		const struct spl_scaler_data *scl_data)
// C source: {
// C source: 	struct spl_fixed31_32 bot;
// C source: 
// C source: 	dscl_prog_data->ratios.h_scale_ratio = SPL_NAMESPACE(spl_fixpt_u3d19(
// C source: 			scl_data->ratios.horz)) << 5;
// C source: 	dscl_prog_data->ratios.v_scale_ratio = SPL_NAMESPACE(spl_fixpt_u3d19(
// C source: 			scl_data->ratios.vert)) << 5;
// C source: 	dscl_prog_data->ratios.h_scale_ratio_c = SPL_NAMESPACE(spl_fixpt_u3d19(
// C source: 			scl_data->ratios.horz_c)) << 5;
// C source: 	dscl_prog_data->ratios.v_scale_ratio_c = SPL_NAMESPACE(spl_fixpt_u3d19(
// C source: 			scl_data->ratios.vert_c)) << 5;
// C source: 	/*
// C source: 	 * 0.24 format for fraction, first five bits zeroed
// C source: 	 */
// C source: 	dscl_prog_data->init.h_filter_init_frac =
// C source: 			SPL_NAMESPACE(spl_fixpt_u0d19(scl_data->inits.h)) << 5;
// C source: 	dscl_prog_data->init.h_filter_init_int =
// C source: 			spl_fixpt_floor(scl_data->inits.h);
// C source: 	dscl_prog_data->init.h_filter_init_frac_c =
// C source: 			SPL_NAMESPACE(spl_fixpt_u0d19(scl_data->inits.h_c)) << 5;
// C source: 	dscl_prog_data->init.h_filter_init_int_c =
// C source: 			spl_fixpt_floor(scl_data->inits.h_c);
// C source: 	dscl_prog_data->init.v_filter_init_frac =
// C source: 			SPL_NAMESPACE(spl_fixpt_u0d19(scl_data->inits.v)) << 5;
// C source: 	dscl_prog_data->init.v_filter_init_int =
// C source: 			spl_fixpt_floor(scl_data->inits.v);
// C source: 	dscl_prog_data->init.v_filter_init_frac_c =
// C source: 			SPL_NAMESPACE(spl_fixpt_u0d19(scl_data->inits.v_c)) << 5;
// C source: 	dscl_prog_data->init.v_filter_init_int_c =
// C source: 			spl_fixpt_floor(scl_data->inits.v_c);
// C source: 
// C source: 	bot = spl_fixpt_add(scl_data->inits.v, scl_data->ratios.vert);
// C source: 	dscl_prog_data->init.v_filter_init_bot_frac = SPL_NAMESPACE(spl_fixpt_u0d19(bot)) << 5;
// C source: 	dscl_prog_data->init.v_filter_init_bot_int = spl_fixpt_floor(bot);
// C source: 	bot = spl_fixpt_add(scl_data->inits.v_c, scl_data->ratios.vert_c);
// C source: 	dscl_prog_data->init.v_filter_init_bot_frac_c = SPL_NAMESPACE(spl_fixpt_u0d19(bot)) << 5;
// C source: 	dscl_prog_data->init.v_filter_init_bot_int_c = spl_fixpt_floor(bot);
// C source: }
// C source: 
// C source: static void spl_set_taps_data(struct dscl_prog_data *dscl_prog_data,
// C source: 		const struct spl_scaler_data *scl_data)
// C source: {
// C source: 	dscl_prog_data->taps.v_taps = scl_data->taps.v_taps - 1;
// C source: 	dscl_prog_data->taps.h_taps = scl_data->taps.h_taps - 1;
// C source: 	dscl_prog_data->taps.v_taps_c = scl_data->taps.v_taps_c - 1;
// C source: 	dscl_prog_data->taps.h_taps_c = scl_data->taps.h_taps_c - 1;
// C source: }
// C source: 
// C source: /* Populate dscl prog data structure from scaler data calculated by SPL */
// C source: static void spl_set_dscl_prog_data(struct spl_in *spl_in, struct spl_scratch *spl_scratch,
// C source: 	struct spl_out *spl_out, bool enable_easf_v, bool enable_easf_h, bool enable_isharp)
// C source: {
// C source: 	struct dscl_prog_data *dscl_prog_data = spl_out->dscl_prog_data;
// C source: 
// C source: 	const struct spl_scaler_data *data = &spl_scratch->scl_data;
// C source: 
// C source: 	struct scl_black_color *scl_black_color = &dscl_prog_data->scl_black_color;
// C source: 
// C source: 	bool enable_easf = enable_easf_v || enable_easf_h;
// C source: 
// C source: 	// Set values for recout
// C source: 	dscl_prog_data->recout = spl_scratch->scl_data.recout;
// C source: 	// Set values for MPC Size
// C source: 	dscl_prog_data->mpc_size.width = spl_scratch->scl_data.h_active;
// C source: 	dscl_prog_data->mpc_size.height = spl_scratch->scl_data.v_active;
// C source: 
// C source: 	// SCL_MODE - Set SCL_MODE data
// C source: 	dscl_prog_data->dscl_mode = spl_get_dscl_mode(spl_in, data, enable_isharp,
// C source: 		enable_easf);
// C source: 
// C source: 	// SCL_BLACK_COLOR
// C source: 	spl_set_black_color_data(spl_in->basic_in.format, scl_black_color);
// C source: 
// C source: 	/* Manually calculate scale ratio and init values */
// C source: 	spl_set_manual_ratio_init_data(dscl_prog_data, data);
// C source: 
// C source: 	// Set HTaps/VTaps
// C source: 	spl_set_taps_data(dscl_prog_data, data);
// C source: 	// Set viewport
// C source: 	dscl_prog_data->viewport = spl_scratch->scl_data.viewport;
// C source: 	// Set viewport_c
// C source: 	dscl_prog_data->viewport_c = spl_scratch->scl_data.viewport_c;
// C source: 	// Set filters data
// C source: 	SPL_NAMESPACE(spl_set_filters_data(dscl_prog_data, data, enable_easf_v, enable_easf_h));
// C source: }
// C source: 
// C source: /* Calculate C0-C3 coefficients based on HDR_mult */
// C source: static void spl_calculate_c0_c3_hdr(struct dscl_prog_data *dscl_prog_data, uint32_t sdr_white_level_nits)
// C source: {
// C source: 	struct spl_fixed31_32 hdr_mult, c0_mult, c1_mult, c2_mult;
// C source: 	struct spl_fixed31_32 c0_calc, c1_calc, c2_calc;
// C source: 	struct spl_custom_float_format fmt;
// C source: 	uint32_t hdr_multx100_int;
// C source: 
// C source: 	if ((sdr_white_level_nits >= 80) && (sdr_white_level_nits <= 480))
// C source: 		hdr_multx100_int = sdr_white_level_nits * 100 / 80;
// C source: 	else
// C source: 		hdr_multx100_int = 100; /* default for 80 nits otherwise */
// C source: 
// C source: 	hdr_mult = SPL_NAMESPACE(spl_fixpt_from_fraction((long long)hdr_multx100_int, 100LL));
// C source: 	c0_mult = SPL_NAMESPACE(spl_fixpt_from_fraction(2126LL, 10000LL));
// C source: 	c1_mult = SPL_NAMESPACE(spl_fixpt_from_fraction(7152LL, 10000LL));
// C source: 	c2_mult = SPL_NAMESPACE(spl_fixpt_from_fraction(722LL, 10000LL));
// C source: 
// C source: 	c0_calc = SPL_NAMESPACE(spl_fixpt_mul(hdr_mult, SPL_NAMESPACE(spl_fixpt_mul(c0_mult,
// C source: 		SPL_NAMESPACE(spl_fixpt_from_fraction(16384LL, 125LL))))));
// C source: 	c1_calc = SPL_NAMESPACE(spl_fixpt_mul(hdr_mult, SPL_NAMESPACE(spl_fixpt_mul(c1_mult,
// C source: 		SPL_NAMESPACE(spl_fixpt_from_fraction(16384LL, 125LL))))));
// C source: 	c2_calc = SPL_NAMESPACE(spl_fixpt_mul(hdr_mult, SPL_NAMESPACE(spl_fixpt_mul(c2_mult,
// C source: 		SPL_NAMESPACE(spl_fixpt_from_fraction(16384LL, 125LL))))));
// C source: 
// C source: 	fmt.exponenta_bits = 5;
// C source: 	fmt.mantissa_bits = 10;
// C source: 	fmt.sign = true;
// C source: 
// C source: 	// fp1.5.10, C0 coefficient (LN_rec709:  HDR_MULT * 0.212600 * 2^14/125)
// C source: 	SPL_NAMESPACE(spl_convert_to_custom_float_format(c0_calc, &fmt,
// C source: 		&dscl_prog_data->easf_matrix_c0));
// C source: 	// fp1.5.10, C1 coefficient (LN_rec709:  HDR_MULT * 0.715200 * 2^14/125)
// C source: 	SPL_NAMESPACE(spl_convert_to_custom_float_format(c1_calc, &fmt,
// C source: 		&dscl_prog_data->easf_matrix_c1));
// C source: 	// fp1.5.10, C2 coefficient (LN_rec709:  HDR_MULT * 0.072200 * 2^14/125)
// C source: 	SPL_NAMESPACE(spl_convert_to_custom_float_format(c2_calc, &fmt,
// C source: 		&dscl_prog_data->easf_matrix_c2));
// C source: 	dscl_prog_data->easf_matrix_c3 = 0x0; // fp1.5.10, C3 coefficient
// C source: }
// C source: 
// C source: /* Set EASF data */
// C source: static void spl_set_easf_data(struct spl_scratch *spl_scratch, struct spl_out *spl_out, bool enable_easf_v,
// C source: 	bool enable_easf_h, enum linear_light_scaling lls_pref,
// C source: 	enum spl_pixel_format format, enum system_setup setup,
// C source: 	uint32_t sdr_white_level_nits)
// C source: {
// C source: 	struct dscl_prog_data *dscl_prog_data = spl_out->dscl_prog_data;
// C source: 	if (enable_easf_v) {
// C source: 		dscl_prog_data->easf_v_en = true;
// C source: 		dscl_prog_data->easf_v_ring = 0;
// C source: 		dscl_prog_data->easf_v_sharp_factor = 1;
// C source: 		dscl_prog_data->easf_v_bf1_en = 1;	// 1-bit, BF1 calculation enable, 0=disable, 1=enable
// C source: 		dscl_prog_data->easf_v_bf2_mode = 0xF;	// 4-bit, BF2 calculation mode
// C source: 		/* 2-bit, BF3 chroma mode correction calculation mode */
// C source: 		dscl_prog_data->easf_v_bf3_mode = SPL_NAMESPACE(spl_get_v_bf3_mode(
// C source: 			spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10 [ minCoef ]*/
// C source: 		dscl_prog_data->easf_v_ringest_3tap_dntilt_uptilt =
// C source: 			SPL_NAMESPACE(spl_get_3tap_dntilt_uptilt_offset(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10 [ upTiltMaxVal ]*/
// C source: 		dscl_prog_data->easf_v_ringest_3tap_uptilt_max =
// C source: 			SPL_NAMESPACE(spl_get_3tap_uptilt_maxval(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10 [ dnTiltSlope ]*/
// C source: 		dscl_prog_data->easf_v_ringest_3tap_dntilt_slope =
// C source: 			SPL_NAMESPACE(spl_get_3tap_dntilt_slope(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10 [ upTilt1Slope ]*/
// C source: 		dscl_prog_data->easf_v_ringest_3tap_uptilt1_slope =
// C source: 			SPL_NAMESPACE(spl_get_3tap_uptilt1_slope(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10 [ upTilt2Slope ]*/
// C source: 		dscl_prog_data->easf_v_ringest_3tap_uptilt2_slope =
// C source: 			SPL_NAMESPACE(spl_get_3tap_uptilt2_slope(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10 [ upTilt2Offset ]*/
// C source: 		dscl_prog_data->easf_v_ringest_3tap_uptilt2_offset =
// C source: 			SPL_NAMESPACE(spl_get_3tap_uptilt2_offset(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10; (2.0) Ring reducer gain for 4 or 6-tap mode [H_REDUCER_GAIN4] */
// C source: 		dscl_prog_data->easf_v_ringest_eventap_reduceg1 =
// C source: 			SPL_NAMESPACE(spl_get_reducer_gain4(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10; (2.5) Ring reducer gain for 6-tap mode [V_REDUCER_GAIN6] */
// C source: 		dscl_prog_data->easf_v_ringest_eventap_reduceg2 =
// C source: 			SPL_NAMESPACE(spl_get_reducer_gain6(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10; (-0.135742) Ring gain for 6-tap set to -139/1024 */
// C source: 		dscl_prog_data->easf_v_ringest_eventap_gain1 =
// C source: 			SPL_NAMESPACE(spl_get_gainRing4(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		/* FP1.5.10; (-0.024414) Ring gain for 6-tap set to -25/1024 */
// C source: 		dscl_prog_data->easf_v_ringest_eventap_gain2 =
// C source: 			SPL_NAMESPACE(spl_get_gainRing6(spl_scratch->scl_data.taps.v_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.vert));
// C source: 		dscl_prog_data->easf_v_bf_maxa = 63; //Vertical Max BF value A in U0.6 format.Selected if V_FCNTL == 0
// C source: 		dscl_prog_data->easf_v_bf_maxb = 63; //Vertical Max BF value A in U0.6 format.Selected if V_FCNTL == 1
// C source: 		dscl_prog_data->easf_v_bf_mina = 0;	//Vertical Min BF value A in U0.6 format.Selected if V_FCNTL == 0
// C source: 		dscl_prog_data->easf_v_bf_minb = 0;	//Vertical Min BF value A in U0.6 format.Selected if V_FCNTL == 1
// C source: 		if (lls_pref == LLS_PREF_YES)	{
// C source: 			dscl_prog_data->easf_v_bf2_flat1_gain = 4;	// U1.3, BF2 Flat1 Gain control
// C source: 			dscl_prog_data->easf_v_bf2_flat2_gain = 8;	// U4.0, BF2 Flat2 Gain control
// C source: 			dscl_prog_data->easf_v_bf2_roc_gain = 4;	// U2.2, Rate Of Change control
// C source: 
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg0 = 0x600;	// S0.10, BF1 PWL Segment 0 = -512
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg0 = 0;	// U0.6, BF1 Base PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg0 = 3;	// S7.3, BF1 Slope PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg1 = 0x7EC;	// S0.10, BF1 PWL Segment 1 = -20
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg1 = 12;	// U0.6, BF1 Base PWL Segment 1
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg1 = 326;	// S7.3, BF1 Slope PWL Segment 1
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg2 = 0;	// S0.10, BF1 PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg2 = 63;	// U0.6, BF1 Base PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg2 = 0;	// S7.3, BF1 Slope PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg3 = 16;	// S0.10, BF1 PWL Segment 3
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg3 = 63;	// U0.6, BF1 Base PWL Segment 3
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg3 = 0x7C8;	// S7.3, BF1 Slope PWL Segment 3 = -56
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg4 = 32;	// S0.10, BF1 PWL Segment 4
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg4 = 56;	// U0.6, BF1 Base PWL Segment 4
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg4 = 0x7D0;	// S7.3, BF1 Slope PWL Segment 4 = -48
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg5 = 48;	// S0.10, BF1 PWL Segment 5
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg5 = 50;	// U0.6, BF1 Base PWL Segment 5
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg5 = 0x710;	// S7.3, BF1 Slope PWL Segment 5 = -240
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg6 = 64;	// S0.10, BF1 PWL Segment 6
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg6 = 20;	// U0.6, BF1 Base PWL Segment 6
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg6 = 0x760;	// S7.3, BF1 Slope PWL Segment 6 = -160
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg7 = 80;	// S0.10, BF1 PWL Segment 7
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg7 = 0;	// U0.6, BF1 Base PWL Segment 7
// C source: 
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set0 = 0x000;	// FP0.6.6, BF3 Input value PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set0 = 63;	// S0.6, BF3 Base PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set0 = 0x12C5;	// FP1.6.6, BF3 Slope PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set1 =
// C source: 				0x0B37; // FP0.6.6, BF3 Input value PWL Segment 1 (0.0078125 * 125^3)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set1 = 62;	// S0.6, BF3 Base PWL Segment 1
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set1 =
// C source: 				0x13B8;	// FP1.6.6, BF3 Slope PWL Segment 1
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set2 =
// C source: 				0x0BB7;	// FP0.6.6, BF3 Input value PWL Segment 2 (0.03125 * 125^3)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set2 = 20;	// S0.6, BF3 Base PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set2 =
// C source: 				0x1356;	// FP1.6.6, BF3 Slope PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set3 =
// C source: 				0x0BF7;	// FP0.6.6, BF3 Input value PWL Segment 3 (0.0625 * 125^3)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set3 = 0;	// S0.6, BF3 Base PWL Segment 3
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set3 =
// C source: 				0x136B;	// FP1.6.6, BF3 Slope PWL Segment 3
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set4 =
// C source: 				0x0C37;	// FP0.6.6, BF3 Input value PWL Segment 4 (0.125 * 125^3)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set4 = 0x4E;	// S0.6, BF3 Base PWL Segment 4 = -50
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set4 =
// C source: 				0x1200;	// FP1.6.6, BF3 Slope PWL Segment 4
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set5 =
// C source: 				0x0CF7;	// FP0.6.6, BF3 Input value PWL Segment 5 (1.0 * 125^3)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set5 = 0x41;	// S0.6, BF3 Base PWL Segment 5 = -63
// C source: 		}	else	{
// C source: 			dscl_prog_data->easf_v_bf2_flat1_gain = 13;	// U1.3, BF2 Flat1 Gain control
// C source: 			dscl_prog_data->easf_v_bf2_flat2_gain = 15;	// U4.0, BF2 Flat2 Gain control
// C source: 			dscl_prog_data->easf_v_bf2_roc_gain = 14;	// U2.2, Rate Of Change control
// C source: 
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg0 = 0x440;	// S0.10, BF1 PWL Segment 0 = -960
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg0 = 0;	// U0.6, BF1 Base PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg0 = 2;	// S7.3, BF1 Slope PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg1 = 0x7C4;	// S0.10, BF1 PWL Segment 1 = -60
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg1 = 12;	// U0.6, BF1 Base PWL Segment 1
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg1 = 109;	// S7.3, BF1 Slope PWL Segment 1
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg2 = 0;	// S0.10, BF1 PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg2 = 63;	// U0.6, BF1 Base PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg2 = 0;	// S7.3, BF1 Slope PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg3 = 48;	// S0.10, BF1 PWL Segment 3
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg3 = 63;	// U0.6, BF1 Base PWL Segment 3
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg3 = 0x7ED;	// S7.3, BF1 Slope PWL Segment 3 = -19
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg4 = 96;	// S0.10, BF1 PWL Segment 4
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg4 = 56;	// U0.6, BF1 Base PWL Segment 4
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg4 = 0x7F0;	// S7.3, BF1 Slope PWL Segment 4 = -16
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg5 = 144;	// S0.10, BF1 PWL Segment 5
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg5 = 50;	// U0.6, BF1 Base PWL Segment 5
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg5 = 0x7B0;	// S7.3, BF1 Slope PWL Segment 5 = -80
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg6 = 192;	// S0.10, BF1 PWL Segment 6
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg6 = 20;	// U0.6, BF1 Base PWL Segment 6
// C source: 			dscl_prog_data->easf_v_bf1_pwl_slope_seg6 = 0x7CB;	// S7.3, BF1 Slope PWL Segment 6 = -53
// C source: 			dscl_prog_data->easf_v_bf1_pwl_in_seg7 = 240;	// S0.10, BF1 PWL Segment 7
// C source: 			dscl_prog_data->easf_v_bf1_pwl_base_seg7 = 0;	// U0.6, BF1 Base PWL Segment 7
// C source: 
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set0 = 0x000;	// FP0.6.6, BF3 Input value PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set0 = 63;	// S0.6, BF3 Base PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set0 = 0x0000;	// FP1.6.6, BF3 Slope PWL Segment 0
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set1 =
// C source: 				0x06C0; // FP0.6.6, BF3 Input value PWL Segment 1 (0.0625)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set1 = 63;	// S0.6, BF3 Base PWL Segment 1
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set1 = 0x1896;	// FP1.6.6, BF3 Slope PWL Segment 1
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set2 =
// C source: 				0x0700;	// FP0.6.6, BF3 Input value PWL Segment 2 (0.125)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set2 = 20;	// S0.6, BF3 Base PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set2 = 0x1810;	// FP1.6.6, BF3 Slope PWL Segment 2
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set3 =
// C source: 				0x0740;	// FP0.6.6, BF3 Input value PWL Segment 3 (0.25)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set3 = 0;	// S0.6, BF3 Base PWL Segment 3
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set3 =
// C source: 				0x1878;	// FP1.6.6, BF3 Slope PWL Segment 3
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set4 =
// C source: 				0x0761;	// FP0.6.6, BF3 Input value PWL Segment 4 (0.375)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set4 = 0x44;	// S0.6, BF3 Base PWL Segment 4 = -60
// C source: 			dscl_prog_data->easf_v_bf3_pwl_slope_set4 = 0x1760;	// FP1.6.6, BF3 Slope PWL Segment 4
// C source: 			dscl_prog_data->easf_v_bf3_pwl_in_set5 =
// C source: 				0x0780;	// FP0.6.6, BF3 Input value PWL Segment 5 (0.5)
// C source: 			dscl_prog_data->easf_v_bf3_pwl_base_set5 = 0x41;	// S0.6, BF3 Base PWL Segment 5 = -63
// C source: 		}
// C source: 	} else
// C source: 		dscl_prog_data->easf_v_en = false;
// C source: 
// C source: 	if (enable_easf_h) {
// C source: 		dscl_prog_data->easf_h_en = true;
// C source: 		dscl_prog_data->easf_h_ring = 0;
// C source: 		dscl_prog_data->easf_h_sharp_factor = 1;
// C source: 		dscl_prog_data->easf_h_bf1_en =
// C source: 			1;	// 1-bit, BF1 calculation enable, 0=disable, 1=enable
// C source: 		dscl_prog_data->easf_h_bf2_mode =
// C source: 			0xF;	// 4-bit, BF2 calculation mode
// C source: 		/* 2-bit, BF3 chroma mode correction calculation mode */
// C source: 		dscl_prog_data->easf_h_bf3_mode = SPL_NAMESPACE(spl_get_h_bf3_mode(
// C source: 			spl_scratch->scl_data.recip_ratios.horz));
// C source: 		/* FP1.5.10; (2.0) Ring reducer gain for 4 or 6-tap mode [H_REDUCER_GAIN4] */
// C source: 		dscl_prog_data->easf_h_ringest_eventap_reduceg1 =
// C source: 			SPL_NAMESPACE(spl_get_reducer_gain4(spl_scratch->scl_data.taps.h_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.horz));
// C source: 		/* FP1.5.10; (2.5) Ring reducer gain for 6-tap mode [V_REDUCER_GAIN6] */
// C source: 		dscl_prog_data->easf_h_ringest_eventap_reduceg2 =
// C source: 			SPL_NAMESPACE(spl_get_reducer_gain6(spl_scratch->scl_data.taps.h_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.horz));
// C source: 		/* FP1.5.10; (-0.135742) Ring gain for 6-tap set to -139/1024 */
// C source: 		dscl_prog_data->easf_h_ringest_eventap_gain1 =
// C source: 			SPL_NAMESPACE(spl_get_gainRing4(spl_scratch->scl_data.taps.h_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.horz));
// C source: 		/* FP1.5.10; (-0.024414) Ring gain for 6-tap set to -25/1024 */
// C source: 		dscl_prog_data->easf_h_ringest_eventap_gain2 =
// C source: 			SPL_NAMESPACE(spl_get_gainRing6(spl_scratch->scl_data.taps.h_taps,
// C source: 				spl_scratch->scl_data.recip_ratios.horz));
// C source: 		dscl_prog_data->easf_h_bf_maxa = 63; //Horz Max BF value A in U0.6 format.Selected if H_FCNTL==0
// C source: 		dscl_prog_data->easf_h_bf_maxb = 63; //Horz Max BF value B in U0.6 format.Selected if H_FCNTL==1
// C source: 		dscl_prog_data->easf_h_bf_mina = 0;	//Horz Min BF value B in U0.6 format.Selected if H_FCNTL==0
// C source: 		dscl_prog_data->easf_h_bf_minb = 0;	//Horz Min BF value B in U0.6 format.Selected if H_FCNTL==1
// C source: 		if (lls_pref == LLS_PREF_YES)	{
// C source: 			dscl_prog_data->easf_h_bf2_flat1_gain = 4;	// U1.3, BF2 Flat1 Gain control
// C source: 			dscl_prog_data->easf_h_bf2_flat2_gain = 8;	// U4.0, BF2 Flat2 Gain control
// C source: 			dscl_prog_data->easf_h_bf2_roc_gain = 4;	// U2.2, Rate Of Change control
// C source: 
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg0 = 0x600;	// S0.10, BF1 PWL Segment 0 = -512
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg0 = 0;	// U0.6, BF1 Base PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg0 = 3;	// S7.3, BF1 Slope PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg1 = 0x7EC;	// S0.10, BF1 PWL Segment 1 = -20
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg1 = 12;	// U0.6, BF1 Base PWL Segment 1
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg1 = 326;	// S7.3, BF1 Slope PWL Segment 1
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg2 = 0;	// S0.10, BF1 PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg2 = 63;	// U0.6, BF1 Base PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg2 = 0;	// S7.3, BF1 Slope PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg3 = 16;	// S0.10, BF1 PWL Segment 3
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg3 = 63;	// U0.6, BF1 Base PWL Segment 3
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg3 = 0x7C8;	// S7.3, BF1 Slope PWL Segment 3 = -56
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg4 = 32;	// S0.10, BF1 PWL Segment 4
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg4 = 56;	// U0.6, BF1 Base PWL Segment 4
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg4 = 0x7D0;	// S7.3, BF1 Slope PWL Segment 4 = -48
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg5 = 48;	// S0.10, BF1 PWL Segment 5
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg5 = 50;	// U0.6, BF1 Base PWL Segment 5
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg5 = 0x710;	// S7.3, BF1 Slope PWL Segment 5 = -240
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg6 = 64;	// S0.10, BF1 PWL Segment 6
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg6 = 20;	// U0.6, BF1 Base PWL Segment 6
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg6 = 0x760;	// S7.3, BF1 Slope PWL Segment 6 = -160
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg7 = 80;	// S0.10, BF1 PWL Segment 7
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg7 = 0;	// U0.6, BF1 Base PWL Segment 7
// C source: 
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set0 = 0x000;	// FP0.6.6, BF3 Input value PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set0 = 63;	// S0.6, BF3 Base PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set0 = 0x12C5;	// FP1.6.6, BF3 Slope PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set1 =
// C source: 				0x0B37;	// FP0.6.6, BF3 Input value PWL Segment 1 (0.0078125 * 125^3)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set1 = 62;	// S0.6, BF3 Base PWL Segment 1
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set1 =	0x13B8;	// FP1.6.6, BF3 Slope PWL Segment 1
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set2 =
// C source: 				0x0BB7;	// FP0.6.6, BF3 Input value PWL Segment 2 (0.03125 * 125^3)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set2 = 20;	// S0.6, BF3 Base PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set2 =	0x1356;	// FP1.6.6, BF3 Slope PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set3 =
// C source: 				0x0BF7;	// FP0.6.6, BF3 Input value PWL Segment 3 (0.0625 * 125^3)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set3 = 0;	// S0.6, BF3 Base PWL Segment 3
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set3 =	0x136B;	// FP1.6.6, BF3 Slope PWL Segment 3
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set4 =
// C source: 				0x0C37;	// FP0.6.6, BF3 Input value PWL Segment 4 (0.125 * 125^3)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set4 = 0x4E;	// S0.6, BF3 Base PWL Segment 4 = -50
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set4 = 0x1200;	// FP1.6.6, BF3 Slope PWL Segment 4
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set5 =
// C source: 				0x0CF7;	// FP0.6.6, BF3 Input value PWL Segment 5 (1.0 * 125^3)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set5 = 0x41;	// S0.6, BF3 Base PWL Segment 5 = -63
// C source: 		} else {
// C source: 			dscl_prog_data->easf_h_bf2_flat1_gain = 13;	// U1.3, BF2 Flat1 Gain control
// C source: 			dscl_prog_data->easf_h_bf2_flat2_gain = 15;	// U4.0, BF2 Flat2 Gain control
// C source: 			dscl_prog_data->easf_h_bf2_roc_gain = 14;	// U2.2, Rate Of Change control
// C source: 
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg0 = 0x440;	// S0.10, BF1 PWL Segment 0 = -960
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg0 = 0;	// U0.6, BF1 Base PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg0 = 2;	// S7.3, BF1 Slope PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg1 = 0x7C4;	// S0.10, BF1 PWL Segment 1 = -60
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg1 = 12;	// U0.6, BF1 Base PWL Segment 1
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg1 = 109;	// S7.3, BF1 Slope PWL Segment 1
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg2 = 0;	// S0.10, BF1 PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg2 = 63;	// U0.6, BF1 Base PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg2 = 0;	// S7.3, BF1 Slope PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg3 = 48;	// S0.10, BF1 PWL Segment 3
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg3 = 63;	// U0.6, BF1 Base PWL Segment 3
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg3 = 0x7ED;	// S7.3, BF1 Slope PWL Segment 3 = -19
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg4 = 96;	// S0.10, BF1 PWL Segment 4
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg4 = 56;	// U0.6, BF1 Base PWL Segment 4
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg4 = 0x7F0;	// S7.3, BF1 Slope PWL Segment 4 = -16
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg5 = 144;	// S0.10, BF1 PWL Segment 5
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg5 = 50;	// U0.6, BF1 Base PWL Segment 5
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg5 = 0x7B0;	// S7.3, BF1 Slope PWL Segment 5 = -80
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg6 = 192;	// S0.10, BF1 PWL Segment 6
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg6 = 20;	// U0.6, BF1 Base PWL Segment 6
// C source: 			dscl_prog_data->easf_h_bf1_pwl_slope_seg6 = 0x7CB;	// S7.3, BF1 Slope PWL Segment 6 = -53
// C source: 			dscl_prog_data->easf_h_bf1_pwl_in_seg7 = 240;	// S0.10, BF1 PWL Segment 7
// C source: 			dscl_prog_data->easf_h_bf1_pwl_base_seg7 = 0;	// U0.6, BF1 Base PWL Segment 7
// C source: 
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set0 = 0x000;	// FP0.6.6, BF3 Input value PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set0 = 63;	// S0.6, BF3 Base PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set0 = 0x0000;	// FP1.6.6, BF3 Slope PWL Segment 0
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set1 =
// C source: 				0x06C0;	// FP0.6.6, BF3 Input value PWL Segment 1 (0.0625)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set1 = 63;	// S0.6, BF3 Base PWL Segment 1
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set1 = 0x1896;	// FP1.6.6, BF3 Slope PWL Segment 1
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set2 =
// C source: 				0x0700;	// FP0.6.6, BF3 Input value PWL Segment 2 (0.125)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set2 = 20;	// S0.6, BF3 Base PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set2 = 0x1810;	// FP1.6.6, BF3 Slope PWL Segment 2
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set3 =
// C source: 				0x0740;	// FP0.6.6, BF3 Input value PWL Segment 3 (0.25)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set3 = 0;	// S0.6, BF3 Base PWL Segment 3
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set3 = 0x1878;	// FP1.6.6, BF3 Slope PWL Segment 3
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set4 =
// C source: 				0x0761;	// FP0.6.6, BF3 Input value PWL Segment 4 (0.375)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set4 = 0x44;	// S0.6, BF3 Base PWL Segment 4 = -60
// C source: 			dscl_prog_data->easf_h_bf3_pwl_slope_set4 = 0x1760;	// FP1.6.6, BF3 Slope PWL Segment 4
// C source: 			dscl_prog_data->easf_h_bf3_pwl_in_set5 =
// C source: 				0x0780;	// FP0.6.6, BF3 Input value PWL Segment 5 (0.5)
// C source: 			dscl_prog_data->easf_h_bf3_pwl_base_set5 = 0x41;	// S0.6, BF3 Base PWL Segment 5 = -63
// C source: 		} // if (lls_pref == LLS_PREF_YES)
// C source: 	} else
// C source: 		dscl_prog_data->easf_h_en = false;
// C source: 
// C source: 	if (lls_pref == LLS_PREF_YES)	{
// C source: 		dscl_prog_data->easf_ltonl_en = 1;	// Linear input
// C source: 		if ((setup == HDR_L) && (spl_is_rgb8(format))) {
// C source: 			/* Calculate C0-C3 coefficients based on HDR multiplier */
// C source: 			spl_calculate_c0_c3_hdr(dscl_prog_data, sdr_white_level_nits);
// C source: 		} else { // HDR_L ( DWM ) and SDR_L
// C source: 			dscl_prog_data->easf_matrix_c0 =
// C source: 				0x4EF7;	// fp1.5.10, C0 coefficient (LN_rec709:  0.2126 * (2^14)/125 = 27.86590720)
// C source: 			dscl_prog_data->easf_matrix_c1 =
// C source: 				0x55DC;	// fp1.5.10, C1 coefficient (LN_rec709:  0.7152 * (2^14)/125 = 93.74269440)
// C source: 			dscl_prog_data->easf_matrix_c2 =
// C source: 				0x48BB;	// fp1.5.10, C2 coefficient (LN_rec709:  0.0722 * (2^14)/125 = 9.46339840)
// C source: 			dscl_prog_data->easf_matrix_c3 =
// C source: 				0x0;	// fp1.5.10, C3 coefficient
// C source: 		}
// C source: 	}	else	{
// C source: 		dscl_prog_data->easf_ltonl_en = 0;	// Non-Linear input
// C source: 		dscl_prog_data->easf_matrix_c0 =
// C source: 			0x3434;	// fp1.5.10, C0 coefficient (LN_BT2020:  0.262695312500000)
// C source: 		dscl_prog_data->easf_matrix_c1 =
// C source: 			0x396D;	// fp1.5.10, C1 coefficient (LN_BT2020:  0.678222656250000)
// C source: 		dscl_prog_data->easf_matrix_c2 =
// C source: 			0x2B97;	// fp1.5.10, C2 coefficient (LN_BT2020:  0.059295654296875)
// C source: 		dscl_prog_data->easf_matrix_c3 =
// C source: 			0x0;	// fp1.5.10, C3 coefficient
// C source: 	}
// C source: 
// C source: 	if (spl_is_subsampled_format(format)) { /* TODO: 0 = RGB, 1 = YUV */
// C source: 		dscl_prog_data->easf_matrix_mode = 1;
// C source: 		/*
// C source: 		 * 2-bit, BF3 chroma mode correction calculation mode
// C source: 		 * Needs to be disabled for YUV420 mode
// C source: 		 * Override lookup value
// C source: 		 */
// C source: 		dscl_prog_data->easf_v_bf3_mode = 0;
// C source: 		dscl_prog_data->easf_h_bf3_mode = 0;
// C source: 	} else
// C source: 		dscl_prog_data->easf_matrix_mode = 0;
// C source: 
// C source: }
// C source: 
// C source: /*Set isharp noise detection */
// C source: static void spl_set_isharp_noise_det_mode(struct dscl_prog_data *dscl_prog_data,
// C source: 	const struct spl_scaler_data *data)
// C source: {
// C source: 	// ISHARP_NOISEDET_MODE
// C source: 	// 0: 3x5 as VxH
// C source: 	// 1: 4x5 as VxH
// C source: 	// 2:
// C source: 	// 3: 5x5 as VxH
// C source: 	if (data->taps.v_taps == 6)
// C source: 		dscl_prog_data->isharp_noise_det.mode = 3;
// C source: 	else if (data->taps.v_taps == 4)
// C source: 		dscl_prog_data->isharp_noise_det.mode = 1;
// C source: 	else if (data->taps.v_taps == 3)
// C source: 		dscl_prog_data->isharp_noise_det.mode = 0;
// C source: };
// C source: /* Set Sharpener data */
// C source: static void spl_set_isharp_data(struct dscl_prog_data *dscl_prog_data,
// C source: 		struct adaptive_sharpness adp_sharpness, bool enable_isharp,
// C source: 		enum linear_light_scaling lls_pref, enum spl_pixel_format format,
// C source: 		const struct spl_scaler_data *data, struct spl_fixed31_32 ratio,
// C source: 		enum system_setup setup, enum scale_to_sharpness_policy scale_to_sharpness_policy)
// C source: {
// C source: 	(void)format;
// C source: 	/* Turn off sharpener if not required */
// C source: 	if (!enable_isharp) {
// C source: 		dscl_prog_data->isharp_en = 0;
// C source: 		return;
// C source: 	}
// C source: 
// C source: 	SPL_NAMESPACE(spl_build_isharp_1dlut_from_reference_curve(ratio, setup, adp_sharpness,
// C source: 		scale_to_sharpness_policy));
// C source: 	memcpy(dscl_prog_data->isharp_delta, SPL_NAMESPACE(spl_get_pregen_filter_isharp_1D_lut(setup)),
// C source: 		sizeof(uint32_t) * ISHARP_LUT_TABLE_SIZE);
// C source: 	dscl_prog_data->sharpness_level = adp_sharpness.sharpness_level;
// C source: 
// C source: 	dscl_prog_data->isharp_en = 1;	// ISHARP_EN
// C source: 	// Set ISHARP_NOISEDET_MODE if htaps = 6-tap
// C source: 	if (data->taps.h_taps == 6) {
// C source: 		dscl_prog_data->isharp_noise_det.enable = 1;	/* ISHARP_NOISEDET_EN */
// C source: 		spl_set_isharp_noise_det_mode(dscl_prog_data, data);	/* ISHARP_NOISEDET_MODE */
// C source: 	} else
// C source: 		dscl_prog_data->isharp_noise_det.enable = 0;	// ISHARP_NOISEDET_EN
// C source: 	// Program noise detection threshold
// C source: 	dscl_prog_data->isharp_noise_det.uthreshold = 24;	// ISHARP_NOISEDET_UTHRE
// C source: 	dscl_prog_data->isharp_noise_det.dthreshold = 4;	// ISHARP_NOISEDET_DTHRE
// C source: 	// Program noise detection gain
// C source: 	dscl_prog_data->isharp_noise_det.pwl_start_in = 3;	// ISHARP_NOISEDET_PWL_START_IN
// C source: 	dscl_prog_data->isharp_noise_det.pwl_end_in = 13;	// ISHARP_NOISEDET_PWL_END_IN
// C source: 	dscl_prog_data->isharp_noise_det.pwl_slope = 1623;	// ISHARP_NOISEDET_PWL_SLOPE
// C source: 
// C source: 	if (lls_pref == LLS_PREF_NO) /* ISHARP_FMT_MODE */
// C source: 		dscl_prog_data->isharp_fmt.mode = 1;
// C source: 	else
// C source: 		dscl_prog_data->isharp_fmt.mode = 0;
// C source: 
// C source: 	dscl_prog_data->isharp_fmt.norm = 0x3C00;	// ISHARP_FMT_NORM
// C source: 	dscl_prog_data->isharp_lba.mode = 0;	// ISHARP_LBA_MODE
// C source: 
// C source: 	if (setup == SDR_L) {
// C source: 		// ISHARP_LBA_PWL_SEG0: ISHARP Local Brightness Adjustment PWL Segment 0
// C source: 		dscl_prog_data->isharp_lba.in_seg[0] = 0;	// ISHARP LBA PWL for Seg 0. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[0] = 0;	// ISHARP LBA PWL for Seg 0. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[0] = 62;	// ISHARP LBA for Seg 0. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG1: ISHARP LBA PWL Segment 1
// C source: 		dscl_prog_data->isharp_lba.in_seg[1] = 130;	// ISHARP LBA PWL for Seg 1. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[1] = 63; // ISHARP LBA PWL for Seg 1. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[1] = 0; // ISHARP LBA for Seg 1. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG2: ISHARP LBA PWL Segment 2
// C source: 		dscl_prog_data->isharp_lba.in_seg[2] = 450; // ISHARP LBA PWL for Seg 2. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[2] = 63; // ISHARP LBA PWL for Seg 2. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[2] = 0x18D; // ISHARP LBA for Seg 2. SLOPE value in S5.3 format = -115
// C source: 		// ISHARP_LBA_PWL_SEG3: ISHARP LBA PWL Segment 3
// C source: 		dscl_prog_data->isharp_lba.in_seg[3] = 520; // ISHARP LBA PWL for Seg 3.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[3] = 0; // ISHARP LBA PWL for Seg 3. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[3] = 0; // ISHARP LBA for Seg 3. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG4: ISHARP LBA PWL Segment 4
// C source: 		dscl_prog_data->isharp_lba.in_seg[4] = 520; // ISHARP LBA PWL for Seg 4.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[4] = 0; // ISHARP LBA PWL for Seg 4. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[4] = 0; // ISHARP LBA for Seg 4. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG5: ISHARP LBA PWL Segment 5
// C source: 		dscl_prog_data->isharp_lba.in_seg[5] = 520; // ISHARP LBA PWL for Seg 5.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[5] = 0;	// ISHARP LBA PWL for Seg 5. BASE value in U0.6 format
// C source: 	} else if (setup == HDR_L) {
// C source: 		// ISHARP_LBA_PWL_SEG0: ISHARP Local Brightness Adjustment PWL Segment 0
// C source: 		dscl_prog_data->isharp_lba.in_seg[0] = 0;	// ISHARP LBA PWL for Seg 0. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[0] = 0;	// ISHARP LBA PWL for Seg 0. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[0] = 32;	// ISHARP LBA for Seg 0. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG1: ISHARP LBA PWL Segment 1
// C source: 		dscl_prog_data->isharp_lba.in_seg[1] = 254;	// ISHARP LBA PWL for Seg 1. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[1] = 63; // ISHARP LBA PWL for Seg 1. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[1] = 0; // ISHARP LBA for Seg 1. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG2: ISHARP LBA PWL Segment 2
// C source: 		dscl_prog_data->isharp_lba.in_seg[2] = 559; // ISHARP LBA PWL for Seg 2. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[2] = 63; // ISHARP LBA PWL for Seg 2. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[2] = 0x10C; // ISHARP LBA for Seg 2. SLOPE value in S5.3 format = -244
// C source: 		// ISHARP_LBA_PWL_SEG3: ISHARP LBA PWL Segment 3
// C source: 		dscl_prog_data->isharp_lba.in_seg[3] = 592; // ISHARP LBA PWL for Seg 3.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[3] = 0; // ISHARP LBA PWL for Seg 3. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[3] = 0; // ISHARP LBA for Seg 3. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG4: ISHARP LBA PWL Segment 4
// C source: 		dscl_prog_data->isharp_lba.in_seg[4] = 1023; // ISHARP LBA PWL for Seg 4.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[4] = 0; // ISHARP LBA PWL for Seg 4. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[4] = 0; // ISHARP LBA for Seg 4. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG5: ISHARP LBA PWL Segment 5
// C source: 		dscl_prog_data->isharp_lba.in_seg[5] = 1023; // ISHARP LBA PWL for Seg 5.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[5] = 0;	// ISHARP LBA PWL for Seg 5. BASE value in U0.6 format
// C source: 	} else {
// C source: 		// ISHARP_LBA_PWL_SEG0: ISHARP Local Brightness Adjustment PWL Segment 0
// C source: 		dscl_prog_data->isharp_lba.in_seg[0] = 0;	// ISHARP LBA PWL for Seg 0. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[0] = 0;	// ISHARP LBA PWL for Seg 0. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[0] = 40;	// ISHARP LBA for Seg 0. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG1: ISHARP LBA PWL Segment 1
// C source: 		dscl_prog_data->isharp_lba.in_seg[1] = 204;	// ISHARP LBA PWL for Seg 1. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[1] = 63; // ISHARP LBA PWL for Seg 1. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[1] = 0; // ISHARP LBA for Seg 1. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG2: ISHARP LBA PWL Segment 2
// C source: 		dscl_prog_data->isharp_lba.in_seg[2] = 818; // ISHARP LBA PWL for Seg 2. INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[2] = 63; // ISHARP LBA PWL for Seg 2. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[2] = 0x1D9; // ISHARP LBA for Seg 2. SLOPE value in S5.3 format = -39
// C source: 		// ISHARP_LBA_PWL_SEG3: ISHARP LBA PWL Segment 3
// C source: 		dscl_prog_data->isharp_lba.in_seg[3] = 1023; // ISHARP LBA PWL for Seg 3.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[3] = 0; // ISHARP LBA PWL for Seg 3. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[3] = 0; // ISHARP LBA for Seg 3. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG4: ISHARP LBA PWL Segment 4
// C source: 		dscl_prog_data->isharp_lba.in_seg[4] = 1023; // ISHARP LBA PWL for Seg 4.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[4] = 0; // ISHARP LBA PWL for Seg 4. BASE value in U0.6 format
// C source: 		dscl_prog_data->isharp_lba.slope_seg[4] = 0; // ISHARP LBA for Seg 4. SLOPE value in S5.3 format
// C source: 		// ISHARP_LBA_PWL_SEG5: ISHARP LBA PWL Segment 5
// C source: 		dscl_prog_data->isharp_lba.in_seg[5] = 1023; // ISHARP LBA PWL for Seg 5.INPUT value in U0.10 format
// C source: 		dscl_prog_data->isharp_lba.base_seg[5] = 0;	// ISHARP LBA PWL for Seg 5. BASE value in U0.6 format
// C source: 	}
// C source: 
// C source: 	// Program the nldelta soft clip values
// C source: 	if (lls_pref == LLS_PREF_YES) {
// C source: 		dscl_prog_data->isharp_nldelta_sclip.enable_p = 0;	/* ISHARP_NLDELTA_SCLIP_EN_P */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.pivot_p = 0;	/* ISHARP_NLDELTA_SCLIP_PIVOT_P */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.slope_p = 0;	/* ISHARP_NLDELTA_SCLIP_SLOPE_P */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.enable_n = 1;	/* ISHARP_NLDELTA_SCLIP_EN_N */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.pivot_n = 71;	/* ISHARP_NLDELTA_SCLIP_PIVOT_N */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.slope_n = 16;	/* ISHARP_NLDELTA_SCLIP_SLOPE_N */
// C source: 	} else {
// C source: 		dscl_prog_data->isharp_nldelta_sclip.enable_p = 1;	/* ISHARP_NLDELTA_SCLIP_EN_P */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.pivot_p = 70;	/* ISHARP_NLDELTA_SCLIP_PIVOT_P */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.slope_p = 24;	/* ISHARP_NLDELTA_SCLIP_SLOPE_P */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.enable_n = 1;	/* ISHARP_NLDELTA_SCLIP_EN_N */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.pivot_n = 70;	/* ISHARP_NLDELTA_SCLIP_PIVOT_N */
// C source: 		dscl_prog_data->isharp_nldelta_sclip.slope_n = 24;	/* ISHARP_NLDELTA_SCLIP_SLOPE_N */
// C source: 	}
// C source: 
// C source: 	// Set the values as per lookup table
// C source: 	SPL_NAMESPACE(spl_set_blur_scale_data(dscl_prog_data, data));
// C source: }
// C source: 
// C source: static void determine_upsp_values(struct spl_in *spl_in, struct dscl_prog_data *dscl_prog_data)
// C source: {
// C source: 	dscl_prog_data->upsp_mode = spl_in->upsp_mode;
// C source: 
// C source: 	if (dscl_prog_data->upsp_mode == UPSP_BYPASS) { //Set all UPSP register fields to 0 if bypass
// C source: 		dscl_prog_data->upsp_v_num_taps = UPSP_2_TAPS;
// C source: 		dscl_prog_data->upsp_h_num_taps = UPSP_2_TAPS;
// C source: 		dscl_prog_data->upsp_boundary_mode = UPSP_BOUNDARY_EDGE;
// C source: 		dscl_prog_data->upsp_v_init_int = 0x0;
// C source: 		dscl_prog_data->upsp_v_init_frac = 0x0;
// C source: 		dscl_prog_data->upsp_v_coef_tap0_p0 = 0x0;
// C source: 		dscl_prog_data->upsp_v_coef_tap1_p0 = 0x0;
// C source: 		dscl_prog_data->upsp_v_coef_tap2_p0 = 0x0;
// C source: 		dscl_prog_data->upsp_v_coef_tap3_p0 = 0x0;
// C source: 		dscl_prog_data->upsp_v_coef_tap0_p1 = 0x0;
// C source: 		dscl_prog_data->upsp_v_coef_tap1_p1 = 0x0;
// C source: 		dscl_prog_data->upsp_v_coef_tap2_p1 = 0x0;
// C source: 		dscl_prog_data->upsp_v_coef_tap3_p1 = 0x0;
// C source: 		dscl_prog_data->upsp_h_init_int = 0x0;
// C source: 		dscl_prog_data->upsp_h_init_frac = 0x0;
// C source: 		dscl_prog_data->upsp_h_coef_tap0_p0 = 0x0;
// C source: 		dscl_prog_data->upsp_h_coef_tap1_p0 = 0x0;
// C source: 		dscl_prog_data->upsp_h_coef_tap2_p0 = 0x0;
// C source: 		dscl_prog_data->upsp_h_coef_tap3_p0 = 0x0;
// C source: 		dscl_prog_data->upsp_h_coef_tap0_p1 = 0x0;
// C source: 		dscl_prog_data->upsp_h_coef_tap1_p1 = 0x0;
// C source: 		dscl_prog_data->upsp_h_coef_tap2_p1 = 0x0;
// C source: 		dscl_prog_data->upsp_h_coef_tap3_p1 = 0x0;
// C source: 		dscl_prog_data->upsp_clamp_max = 0x0;
// C source: 		dscl_prog_data->upsp_clamp_min = 0x0;
// C source: 	} else {
// C source: 		dscl_prog_data->upsp_v_num_taps = UPSP_4_TAPS;
// C source: 		dscl_prog_data->upsp_h_num_taps = UPSP_4_TAPS;
// C source: 		dscl_prog_data->upsp_boundary_mode = UPSP_BOUNDARY_EDGE;
// C source: 		dscl_prog_data->upsp_clamp_max = 0xFFF;//4095
// C source: 		dscl_prog_data->upsp_clamp_min = 0x0;
// C source: 
// C source: 		if (spl_in->basic_in.cositing == CHROMA_COSITING_TOPLEFT) { //Vertical Subsampling: Co-sited
// C source: 			if (dscl_prog_data->upsp_v_num_taps == UPSP_4_TAPS) {
// C source: 				dscl_prog_data->upsp_v_init_int = 0x3;
// C source: 				dscl_prog_data->upsp_v_init_frac = 0x0;
// C source: 				dscl_prog_data->upsp_v_coef_tap0_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap1_p0 = 0x40;
// C source: 				dscl_prog_data->upsp_v_coef_tap2_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap3_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap0_p1 = 0xFC;
// C source: 				dscl_prog_data->upsp_v_coef_tap1_p1 = 0x24;
// C source: 				dscl_prog_data->upsp_v_coef_tap2_p1 = 0x24;
// C source: 				dscl_prog_data->upsp_v_coef_tap3_p1 = 0xFC;
// C source: 			} else { //2 taps
// C source: 				dscl_prog_data->upsp_v_init_int = 0x2;
// C source: 				dscl_prog_data->upsp_v_init_frac = 0x0;
// C source: 				dscl_prog_data->upsp_v_coef_tap0_p0 = 0x40;
// C source: 				dscl_prog_data->upsp_v_coef_tap1_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap2_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap3_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap0_p1 = 0x20;
// C source: 				dscl_prog_data->upsp_v_coef_tap1_p1 = 0x20;
// C source: 				dscl_prog_data->upsp_v_coef_tap2_p1 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap3_p1 = 0x00;
// C source: 			}
// C source: 		} else { //Vertical Subsampling: Interstitial
// C source: 			if (dscl_prog_data->upsp_v_num_taps == UPSP_4_TAPS) {
// C source: 				dscl_prog_data->upsp_v_init_int = 0x2;
// C source: 				dscl_prog_data->upsp_v_init_frac = 0x1;
// C source: 				dscl_prog_data->upsp_v_coef_tap0_p0 = 0xFB;
// C source: 				dscl_prog_data->upsp_v_coef_tap1_p0 = 0x2F;
// C source: 				dscl_prog_data->upsp_v_coef_tap2_p0 = 0x19;
// C source: 				dscl_prog_data->upsp_v_coef_tap3_p0 = 0xFD;
// C source: 				dscl_prog_data->upsp_v_coef_tap0_p1 = 0xFD;
// C source: 				dscl_prog_data->upsp_v_coef_tap1_p1 = 0x19;
// C source: 				dscl_prog_data->upsp_v_coef_tap2_p1 = 0x2F;
// C source: 				dscl_prog_data->upsp_v_coef_tap3_p1 = 0xFB;
// C source: 			} else { //2 taps
// C source: 				dscl_prog_data->upsp_v_init_int = 0x1;
// C source: 				dscl_prog_data->upsp_v_init_frac = 0x1;
// C source: 				dscl_prog_data->upsp_v_coef_tap0_p0 = 0x28;
// C source: 				dscl_prog_data->upsp_v_coef_tap1_p0 = 0x18;
// C source: 				dscl_prog_data->upsp_v_coef_tap2_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap3_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap0_p1 = 0x18;
// C source: 				dscl_prog_data->upsp_v_coef_tap1_p1 = 0x28;
// C source: 				dscl_prog_data->upsp_v_coef_tap2_p1 = 0x00;
// C source: 				dscl_prog_data->upsp_v_coef_tap3_p1 = 0x00;
// C source: 			}
// C source: 		}
// C source: 		if (spl_in->basic_in.cositing == CHROMA_COSITING_LEFT || spl_in->basic_in.cositing == CHROMA_COSITING_TOPLEFT) { //Horizontal Subsampling: Co-sited
// C source: 			if (dscl_prog_data->upsp_h_num_taps == UPSP_4_TAPS) {
// C source: 				dscl_prog_data->upsp_h_init_int = 0x3;
// C source: 				dscl_prog_data->upsp_h_init_frac = 0x0;
// C source: 				dscl_prog_data->upsp_h_coef_tap0_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap1_p0 = 0x40;
// C source: 				dscl_prog_data->upsp_h_coef_tap2_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap3_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap0_p1 = 0xFC;
// C source: 				dscl_prog_data->upsp_h_coef_tap1_p1 = 0x24;
// C source: 				dscl_prog_data->upsp_h_coef_tap2_p1 = 0x24;
// C source: 				dscl_prog_data->upsp_h_coef_tap3_p1 = 0xFC;
// C source: 			} else { //2 taps
// C source: 				dscl_prog_data->upsp_h_init_int = 0x2;
// C source: 				dscl_prog_data->upsp_h_init_frac = 0x0;
// C source: 				dscl_prog_data->upsp_h_coef_tap0_p0 = 0x40;
// C source: 				dscl_prog_data->upsp_h_coef_tap1_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap2_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap3_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap0_p1 = 0x20;
// C source: 				dscl_prog_data->upsp_h_coef_tap1_p1 = 0x20;
// C source: 				dscl_prog_data->upsp_h_coef_tap2_p1 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap3_p1 = 0x00;
// C source: 			}
// C source: 		} else { //Horizontal Subsampling: Interstitial
// C source: 			if (dscl_prog_data->upsp_h_num_taps == UPSP_4_TAPS) {
// C source: 				dscl_prog_data->upsp_h_init_int = 0x2;
// C source: 				dscl_prog_data->upsp_h_init_frac = 0x1;
// C source: 				dscl_prog_data->upsp_h_coef_tap0_p0 = 0xFB;
// C source: 				dscl_prog_data->upsp_h_coef_tap1_p0 = 0x2F;
// C source: 				dscl_prog_data->upsp_h_coef_tap2_p0 = 0x19;
// C source: 				dscl_prog_data->upsp_h_coef_tap3_p0 = 0xFD;
// C source: 				dscl_prog_data->upsp_h_coef_tap0_p1 = 0xFD;
// C source: 				dscl_prog_data->upsp_h_coef_tap1_p1 = 0x19;
// C source: 				dscl_prog_data->upsp_h_coef_tap2_p1 = 0x2F;
// C source: 				dscl_prog_data->upsp_h_coef_tap3_p1 = 0xFB;
// C source: 			} else { //2 taps
// C source: 				dscl_prog_data->upsp_h_init_int = 0x1;
// C source: 				dscl_prog_data->upsp_h_init_frac = 0x1;
// C source: 				dscl_prog_data->upsp_h_coef_tap0_p0 = 0x28;
// C source: 				dscl_prog_data->upsp_h_coef_tap1_p0 = 0x18;
// C source: 				dscl_prog_data->upsp_h_coef_tap2_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap3_p0 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap0_p1 = 0x18;
// C source: 				dscl_prog_data->upsp_h_coef_tap1_p1 = 0x28;
// C source: 				dscl_prog_data->upsp_h_coef_tap2_p1 = 0x00;
// C source: 				dscl_prog_data->upsp_h_coef_tap3_p1 = 0x00;
// C source: 			}
// C source: 		}
// C source: 	}
// C source: }
// C source: 
// C source: /* Calculate recout, scaling ratio, and viewport, then get optimal number of taps */
// C source: static bool spl_calculate_number_of_taps(struct spl_in *spl_in, struct spl_scratch *spl_scratch, struct spl_out *spl_out,
// C source: 	bool *enable_easf_v, bool *enable_easf_h, bool *enable_isharp)
// C source: {
// C source: 	bool res = false;
// C source: 
// C source: 	memset(spl_scratch, 0, sizeof(struct spl_scratch));
// C source: 	spl_scratch->scl_data.h_active = spl_in->h_active;
// C source: 	spl_scratch->scl_data.v_active = spl_in->v_active;
// C source: 
// C source: 	// All SPL calls
// C source: 	/* recout calculation */
// C source: 	/* depends on h_active */
// C source: 	spl_calculate_recout(spl_in, spl_scratch, spl_out);
// C source: 	/* depends on pixel format */
// C source: 	spl_calculate_scaling_ratios(spl_in, spl_scratch, spl_out);
// C source: 	/* Adjust recout for opp if needed */
// C source: 	spl_opp_adjust_rect(&spl_scratch->scl_data.recout, &spl_in->basic_in.opp_recout_adjust);
// C source: 	/* depends on scaling ratios and recout, does not calculate offset yet */
// C source: 	spl_calculate_viewport_size(spl_in, spl_scratch);
// C source: 
// C source: 	res = spl_get_optimal_number_of_taps(
// C source: 			  spl_in->basic_out.max_downscale_src_width, spl_in,
// C source: 			  spl_scratch, &spl_in->scaling_quality, enable_easf_v,
// C source: 			  enable_easf_h, enable_isharp);
// C source: 	return res;
// C source: }
// C source: 
// C source: /* Calculate scaler parameters */
// C source: bool SPL_NAMESPACE(spl_calculate_scaler_params(struct spl_in *spl_in, struct spl_out *spl_out))
// C source: {
// C source: 	bool res = false;
// C source: 	bool enable_easf_v = false;
// C source: 	bool enable_easf_h = false;
// C source: 	int vratio = 0;
// C source: 	int hratio = 0;
// C source: 	struct spl_scratch spl_scratch;
// C source: 	struct spl_fixed31_32 isharp_scale_ratio;
// C source: 	enum system_setup setup;
// C source: 	bool enable_isharp = false;
// C source: 	const struct spl_scaler_data *data = &spl_scratch.scl_data;
// C source: 
// C source: 	determine_upsp_values(spl_in, spl_out->dscl_prog_data);
// C source: 
// C source: 	res = spl_calculate_number_of_taps(spl_in, &spl_scratch, spl_out,
// C source: 		&enable_easf_v, &enable_easf_h, &enable_isharp);
// C source: 
// C source: 	/*
// C source: 	 * Depends on recout, scaling ratios, h_active and taps
// C source: 	 * May need to re-check lb size after this in some obscure scenario
// C source: 	 */
// C source: 	if (res)
// C source: 		spl_calculate_inits_and_viewports(spl_in, &spl_scratch);
// C source: 	// Handle 3d recout
// C source: 	spl_handle_3d_recout(spl_in, &spl_scratch.scl_data.recout);
// C source: 	// Clamp
// C source: 	spl_clamp_viewport(&spl_scratch.scl_data.viewport, spl_in->min_viewport_size);
// C source: 
// C source: 	// Save all calculated parameters in dscl_prog_data structure to program hw registers
// C source: 	spl_set_dscl_prog_data(spl_in, &spl_scratch, spl_out, enable_easf_v, enable_easf_h, enable_isharp);
// C source: 
// C source: 	if (!res)
// C source: 		return res;
// C source: 
// C source: 	if (spl_in->lls_pref == LLS_PREF_YES) {
// C source: 		if (spl_in->is_hdr_on)
// C source: 			setup = HDR_L;
// C source: 		else
// C source: 			setup = SDR_L;
// C source: 	} else {
// C source: 		if (spl_in->is_hdr_on)
// C source: 			setup = HDR_NL;
// C source: 		else
// C source: 			setup = SDR_NL;
// C source: 	}
// C source: 
// C source: 	// Set EASF
// C source: 	spl_set_easf_data(&spl_scratch, spl_out, enable_easf_v, enable_easf_h, spl_in->lls_pref,
// C source: 		spl_in->basic_in.format, setup, spl_in->sdr_white_level_nits);
// C source: 
// C source: 	// Set iSHARP
// C source: 	vratio = spl_fixpt_ceil(spl_scratch.scl_data.ratios.vert);
// C source: 	hratio = spl_fixpt_ceil(spl_scratch.scl_data.ratios.horz);
// C source: 	if (vratio <= hratio)
// C source: 		isharp_scale_ratio = spl_scratch.scl_data.recip_ratios.vert;
// C source: 	else
// C source: 		isharp_scale_ratio = spl_scratch.scl_data.recip_ratios.horz;
// C source: 
// C source: 	spl_set_isharp_data(spl_out->dscl_prog_data, spl_in->adaptive_sharpness, enable_isharp,
// C source: 		spl_in->lls_pref, spl_in->basic_in.format, data, isharp_scale_ratio, setup,
// C source: 		spl_in->debug.scale_to_sharpness_policy);
// C source: 
// C source: 	return res;
// C source: }
// C source: 
// C source: /* External interface to get number of taps only */
// C source: bool SPL_NAMESPACE(spl_get_number_of_taps(struct spl_in *spl_in, struct spl_out *spl_out))
// C source: {
// C source: 	bool res = false;
// C source: 	bool enable_easf_v = false;
// C source: 	bool enable_easf_h = false;
// C source: 	bool enable_isharp = false;
// C source: 	struct spl_scratch spl_scratch;
// C source: 	struct dscl_prog_data *dscl_prog_data = spl_out->dscl_prog_data;
// C source: 	const struct spl_scaler_data *data = &spl_scratch.scl_data;
// C source: 
// C source: 	res = spl_calculate_number_of_taps(spl_in, &spl_scratch, spl_out,
// C source: 		&enable_easf_v, &enable_easf_h, &enable_isharp);
// C source: 	spl_set_taps_data(dscl_prog_data, data);
// C source: 	return res;
// C source: }

*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
