/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

// C dependencies: dm_services.h, dc.h, core_status.h, core_types.h, resource.h

macro_rules! surface_trace {
    ($dc:expr, $($arg:tt)*) => {
        if (*$dc).debug.surface_trace {
            DC_LOG_IF_TRACE!($($arg)*);
        }
    };
}

macro_rules! clock_trace {
    ($dc:expr, $($arg:tt)*) => {
        if (*$dc).debug.clock_trace {
            DC_LOG_BANDWIDTH_CALCS!($($arg)*);
        }
    };
}

pub unsafe fn update_surface_trace(
    dc: *mut dc,
    updates: *const dc_surface_update,
    surface_count: i32,
) {
    let mut i = 0;
    while i < surface_count {
        let update = &*updates.add(i as usize);

        surface_trace!(dc, "Update {}\n", i);
        if !update.flip_addr.is_null() {
            surface_trace!(dc,
                "flip_addr->address.type = {};\nflip_addr->address.grph.addr.quad_part = 0x{:X};\nflip_addr->address.grph.meta_addr.quad_part = 0x{:X};\nflip_addr->flip_immediate = {};\n",
                (*update.flip_addr).address.type_,
                (*update.flip_addr).address.grph.addr.quad_part,
                (*update.flip_addr).address.grph.meta_addr.quad_part,
                (*update.flip_addr).flip_immediate);
        }

        if !update.plane_info.is_null() {
            let p = &*update.plane_info;
            surface_trace!(dc,
                "plane_info->color_space = {};\nplane_info->format = {};\nplane_info->plane_size.surface_pitch = {};\nplane_info->plane_size.surface_size.height = {};\nplane_info->plane_size.surface_size.width = {};\nplane_info->plane_size.surface_size.x = {};\nplane_info->plane_size.surface_size.y = {};\nplane_info->rotation = {};\nplane_info->stereo_format = {};\n",
                p.color_space, p.format, p.plane_size.surface_pitch,
                p.plane_size.surface_size.height, p.plane_size.surface_size.width,
                p.plane_size.surface_size.x, p.plane_size.surface_size.y,
                p.rotation, p.stereo_format);
            surface_trace!(dc,
                "plane_info->tiling_info.gfx8.num_banks = {};\nplane_info->tiling_info.gfx8.bank_width = {};\nplane_info->tiling_info.gfx8.bank_width_c = {};\nplane_info->tiling_info.gfx8.bank_height = {};\nplane_info->tiling_info.gfx8.bank_height_c = {};\nplane_info->tiling_info.gfx8.tile_aspect = {};\nplane_info->tiling_info.gfx8.tile_aspect_c = {};\nplane_info->tiling_info.gfx8.tile_split = {};\nplane_info->tiling_info.gfx8.tile_split_c = {};\nplane_info->tiling_info.gfx8.tile_mode = {};\nplane_info->tiling_info.gfx8.tile_mode_c = {};\n",
                p.tiling_info.gfx8.num_banks, p.tiling_info.gfx8.bank_width,
                p.tiling_info.gfx8.bank_width_c, p.tiling_info.gfx8.bank_height,
                p.tiling_info.gfx8.bank_height_c, p.tiling_info.gfx8.tile_aspect,
                p.tiling_info.gfx8.tile_aspect_c, p.tiling_info.gfx8.tile_split,
                p.tiling_info.gfx8.tile_split_c, p.tiling_info.gfx8.tile_mode,
                p.tiling_info.gfx8.tile_mode_c);
            surface_trace!(dc,
                "plane_info->tiling_info.gfx8.pipe_config = {};\nplane_info->tiling_info.gfx8.array_mode = {};\nplane_info->visible = {};\nplane_info->per_pixel_alpha = {};\n",
                p.tiling_info.gfx8.pipe_config, p.tiling_info.gfx8.array_mode,
                p.visible, p.per_pixel_alpha);
            surface_trace!(dc, "surface->tiling_info.gfx9.swizzle = {};\n", p.tiling_info.gfx9.swizzle);
        }

        if !update.scaling_info.is_null() {
            let s = &*update.scaling_info;
            surface_trace!(dc,
                "scaling_info->src_rect.x = {};\nscaling_info->src_rect.y = {};\nscaling_info->src_rect.width = {};\nscaling_info->src_rect.height = {};\nscaling_info->dst_rect.x = {};\nscaling_info->dst_rect.y = {};\nscaling_info->dst_rect.width = {};\nscaling_info->dst_rect.height = {};\nscaling_info->clip_rect.x = {};\nscaling_info->clip_rect.y = {};\nscaling_info->clip_rect.width = {};\nscaling_info->clip_rect.height = {};\nscaling_info->scaling_quality.h_taps = {};\nscaling_info->scaling_quality.v_taps = {};\nscaling_info->scaling_quality.h_taps_c = {};\nscaling_info->scaling_quality.v_taps_c = {};\n",
                s.src_rect.x, s.src_rect.y, s.src_rect.width, s.src_rect.height,
                s.dst_rect.x, s.dst_rect.y, s.dst_rect.width, s.dst_rect.height,
                s.clip_rect.x, s.clip_rect.y, s.clip_rect.width, s.clip_rect.height,
                s.scaling_quality.h_taps, s.scaling_quality.v_taps,
                s.scaling_quality.h_taps_c, s.scaling_quality.v_taps_c);
        }
        surface_trace!(dc, "\n");
        i += 1;
    }
    surface_trace!(dc, "\n");
}

pub unsafe fn post_surface_trace(dc: *mut dc) {
    surface_trace!(dc, "post surface process.\n");
}

pub unsafe fn context_clock_trace(dc: *mut dc, context: *mut dc_state) {
    clock_trace!(dc, "Current: dispclk_khz:{}  max_dppclk_khz:{}  dcfclk_khz:{}\ndcfclk_deep_sleep_khz:{}  fclk_khz:{}  socclk_khz:{}\n",
        (*context).bw_ctx.bw.dcn.clk.dispclk_khz, (*context).bw_ctx.bw.dcn.clk.dppclk_khz,
        (*context).bw_ctx.bw.dcn.clk.dcfclk_khz, (*context).bw_ctx.bw.dcn.clk.dcfclk_deep_sleep_khz,
        (*context).bw_ctx.bw.dcn.clk.fclk_khz, (*context).bw_ctx.bw.dcn.clk.socclk_khz);
    clock_trace!(dc, "Calculated: dispclk_khz:{}  max_dppclk_khz:{}  dcfclk_khz:{}\ndcfclk_deep_sleep_khz:{}  fclk_khz:{}  socclk_khz:{}\n",
        (*context).bw_ctx.bw.dcn.clk.dispclk_khz, (*context).bw_ctx.bw.dcn.clk.dppclk_khz,
        (*context).bw_ctx.bw.dcn.clk.dcfclk_khz, (*context).bw_ctx.bw.dcn.clk.dcfclk_deep_sleep_khz,
        (*context).bw_ctx.bw.dcn.clk.fclk_khz, (*context).bw_ctx.bw.dcn.clk.socclk_khz);
}

pub fn dc_status_to_str(status: dc_status) -> *const u8 {
    match status {
        DC_OK => b"DC OK\0".as_ptr(),
        DC_NO_CONTROLLER_RESOURCE => b"No controller resource\0".as_ptr(),
        DC_NO_STREAM_ENC_RESOURCE => b"No stream encoder\0".as_ptr(),
        DC_NO_CLOCK_SOURCE_RESOURCE => b"No clock source\0".as_ptr(),
        DC_FAIL_CONTROLLER_VALIDATE => b"Controller validation failure\0".as_ptr(),
        DC_FAIL_ENC_VALIDATE => b"Encoder validation failure\0".as_ptr(),
        DC_FAIL_ATTACH_SURFACES => b"Surfaces attachment failure\0".as_ptr(),
        DC_FAIL_DETACH_SURFACES => b"Surfaces detachment failure\0".as_ptr(),
        DC_FAIL_SURFACE_VALIDATE => b"Surface validation failure\0".as_ptr(),
        DC_NO_DP_LINK_BANDWIDTH => b"No DP link bandwidth\0".as_ptr(),
        DC_EXCEED_DONGLE_CAP => b"Exceed dongle capability\0".as_ptr(),
        DC_SURFACE_PIXEL_FORMAT_UNSUPPORTED => b"Unsupported pixel format\0".as_ptr(),
        DC_FAIL_BANDWIDTH_VALIDATE => b"Bandwidth validation failure (BW and Watermark)\0".as_ptr(),
        DC_FAIL_SCALING => b"Scaling failure\0".as_ptr(),
        DC_FAIL_DP_LINK_TRAINING => b"DP link training failure\0".as_ptr(),
        DC_FAIL_DSC_VALIDATE => b"DSC validation failure\0".as_ptr(),
        DC_NO_DSC_RESOURCE => b"No DSC resource\0".as_ptr(),
        DC_FAIL_UNSUPPORTED_1 => b"Unsupported\0".as_ptr(),
        DC_FAIL_HDMI_FRL_LINK_TRAINING => b"HDMI frl link training failure\0".as_ptr(),
        DC_NO_HDMI_FRL_LINK_BANDWIDTH => b"No DHMI frl link bandwidth\0".as_ptr(),
        DC_FAIL_CLK_EXCEED_MAX => b"Clk exceed max failure\0".as_ptr(),
        DC_FAIL_CLK_BELOW_MIN => b"Fail clk below minimum\0".as_ptr(),
        DC_FAIL_CLK_BELOW_CFG_REQUIRED => b"Fail clk below required CFG (hard_min in PPLIB)\0".as_ptr(),
        DC_NOT_SUPPORTED => b"The operation is not supported.\0".as_ptr(),
        DC_UNSUPPORTED_VALUE => b"The value specified is not supported.\0".as_ptr(),
        DC_NO_LINK_ENC_RESOURCE => b"No link encoder resource\0".as_ptr(),
        DC_FAIL_DP_PAYLOAD_ALLOCATION => b"Fail dp payload allocation\0".as_ptr(),
        DC_FAIL_DP_LINK_BANDWIDTH => b"Insufficient DP link bandwidth\0".as_ptr(),
        DC_FAIL_HW_CURSOR_SUPPORT => b"HW Cursor not supported\0".as_ptr(),
        DC_FAIL_DP_TUNNEL_BW_VALIDATE => b"Fail DP Tunnel BW validation\0".as_ptr(),
        DC_ERROR_UNEXPECTED => b"Unexpected error\0".as_ptr(),
        _ => b"Unexpected status error\0".as_ptr(),
    }
}

pub fn dc_pixel_encoding_to_str(pixel_encoding: dc_pixel_encoding) -> *const u8 {
    match pixel_encoding {
        PIXEL_ENCODING_RGB => b"RGB\0".as_ptr(),
        PIXEL_ENCODING_YCBCR422 => b"YUV422\0".as_ptr(),
        PIXEL_ENCODING_YCBCR444 => b"YUV444\0".as_ptr(),
        PIXEL_ENCODING_YCBCR420 => b"YUV420\0".as_ptr(),
        _ => b"Unknown\0".as_ptr(),
    }
}

pub fn dc_color_depth_to_str(color_depth: dc_color_depth) -> *const u8 {
    match color_depth {
        COLOR_DEPTH_666 => b"6-bpc\0".as_ptr(),
        COLOR_DEPTH_888 => b"8-bpc\0".as_ptr(),
        COLOR_DEPTH_101010 => b"10-bpc\0".as_ptr(),
        COLOR_DEPTH_121212 => b"12-bpc\0".as_ptr(),
        COLOR_DEPTH_141414 => b"14-bpc\0".as_ptr(),
        COLOR_DEPTH_161616 => b"16-bpc\0".as_ptr(),
        COLOR_DEPTH_999 => b"9-bpc\0".as_ptr(),
        COLOR_DEPTH_111111 => b"11-bpc\0".as_ptr(),
        _ => b"Unknown\0".as_ptr(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
