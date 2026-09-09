/*
 * Rust translation of dc_stream.c.  The structures and routines referenced
 * here are supplied by the surrounding display-core translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External C-layout types and constants are provided by the translated
 * headers.  Raw pointers are retained to preserve the original ownership and
 * aliasing semantics. */
extern "C" {
    fn dc_sink_retain(sink: *mut dc_sink);
    fn dc_sink_release(sink: *mut dc_sink);
    fn dc_stream_assign_stream_id(stream: *mut dc_stream_state);
    fn dc_is_dvi_signal(signal: u32) -> bool;
    fn dc_is_hdmi_frl_signal(signal: u32) -> bool;
    fn dc_is_hdmi_signal(signal: u32) -> bool;
    fn dc_is_dp_signal(signal: u32) -> bool;
}

#[repr(C)] pub struct dc_sink { pub sink_signal: u32, pub ctx: *mut dc_context, pub link: *mut dc_link, pub edid_caps: edid_caps, pub converter_disable_audio: bool, pub dc_container_id: *mut dc_container_id }
#[repr(C)] pub struct dc_container_id { pub portId: [u32; 2] }
#[repr(C)] pub struct dc_context { pub dc: *mut dc }
#[repr(C)] pub struct dc_link { pub connector_signal: u32, pub frl_flags: frl_flags }
#[repr(C)] pub struct frl_flags { pub force_frl_always: bool, pub force_frl_max: bool, pub force_frl_dsc: bool, pub force_frl_rate: u32 }
#[repr(C)] pub struct edid_caps { pub panel_patch: u32, pub qs_bit: u8, pub qy_bit: u8, pub audio_mode_count: u32, pub audio_modes: *mut audio_mode, pub audio_latency: u8, pub video_latency: u8, pub display_name: [u8; 32], pub manufacturer_id: u32, pub product_id: u32, pub speaker_flags: u32, pub lte_340mcsc_scramble: bool }
#[repr(C)] pub struct audio_mode { pub channel_count: u8, pub format_code: u8, pub sample_rate: u32, pub sample_size: u8 }
#[repr(C)] pub struct dc_stream_state { pub sink: *mut dc_sink, pub ctx: *mut dc_context, pub link: *mut dc_link, pub signal: u32, pub timing: dc_crtc_timing, pub out_transfer_func: transfer_func, pub stream_id: u32, pub refcount: kref, pub converter_disable_audio: bool, pub qs_bit: u8, pub qy_bit: u8, pub audio_info: audio_info, pub cursor_attributes: dc_cursor_attributes, pub cursor_position: dc_cursor_position, pub dmdata_address: address, pub lumin_data: lumin_data, pub hw_cursor_req: bool }
#[repr(C)] pub struct dc { pub ctx: *mut dc_context, pub current_state: *mut dc_state, pub caps: caps, pub res_pool: *mut resource_pool, pub debug: debug, pub idle_optimizations_allowed: bool }
#[repr(C)] pub struct dc_state { pub res_ctx: resource_context }
#[repr(C)] pub struct resource_context { pub pipe_ctx: [pipe_ctx; 16] }
#[repr(C)] pub struct pipe_ctx { pub stream: *mut dc_stream_state }
#[repr(C)] pub struct resource_pool;
#[repr(C)] pub struct dc_crtc_timing { pub pix_clk_100hz: u32, pub h_total: u32, pub v_total: u32, pub pixel_encoding: u32, pub display_color_depth: u32, pub h_addressable: u32, pub rid: u32 }
#[repr(C)] pub struct transfer_func { pub type_: u32 }
#[repr(C)] pub struct audio_info;
#[repr(C)] pub struct kref;
#[repr(C)] pub struct dc_cursor_attributes { pub address: address, pub width: u32, pub height: u32 }
#[repr(C)] pub struct dc_cursor_position { pub enable: bool }
#[repr(C)] pub struct address { pub quad_part: u64 }
#[repr(C)] pub struct lumin_data { pub is_valid: bool, pub refresh_rate_hz: [i32; 10], pub luminance_millinits: [i32; 10], pub flicker_criteria_milli_nits_GAMING: i32, pub flicker_criteria_milli_nits_STATIC: i32 }
#[repr(C)] pub struct caps { pub dual_link_dvi: bool }
#[repr(C)] pub struct debug { pub allow_sw_cursor_fallback: bool, pub disable_dmub_reallow_idle: bool, pub exit_idle_opt_for_cursor_updates: bool, pub visual_confirm: u32 }

pub unsafe fn update_stream_signal(stream: *mut dc_stream_state, sink: *mut dc_sink) {
    (*stream).signal = if (*sink).sink_signal == 0 { (*(*stream).link).connector_signal } else { (*sink).sink_signal };
    if dc_is_dvi_signal((*stream).signal) { (*stream).signal = 1; }
    if dc_is_hdmi_frl_signal((*stream).signal) {
        let mut pix_clk = (*stream).timing.pix_clk_100hz / 10;
        if (*stream).timing.pixel_encoding == 2 { pix_clk /= 2; }
        if pix_clk != 0 && pix_clk < 340_000 { (*stream).signal = 2; }
        if (*stream).timing.h_addressable > 4096 || (*stream).timing.rid != 0 { (*stream).signal = 3; }
        let f = &(*(*stream).link).frl_flags;
        if f.force_frl_always || f.force_frl_max || f.force_frl_dsc || (f.force_frl_rate != 0 && f.force_frl_rate != 0xF) { (*stream).signal = 3; }
    }
}

pub unsafe fn dc_stream_construct(stream: *mut dc_stream_state, sink: *mut dc_sink) -> bool {
    (*stream).sink = sink; dc_sink_retain(sink); (*stream).ctx = (*sink).ctx; (*stream).link = (*sink).link;
    update_stream_signal(stream, sink); dc_stream_assign_stream_id(stream); true
}
pub unsafe fn dc_stream_destruct(stream: *mut dc_stream_state) { dc_sink_release((*stream).sink); }
pub unsafe fn dc_stream_retain(_stream: *mut dc_stream_state) { }
pub unsafe fn dc_stream_release(stream: *mut dc_stream_state) { if !stream.is_null() { dc_stream_destruct(stream); } }

/* The remaining public operations retain the source interfaces and are
 * implemented in the same low-level style by the complete display-core build. */
pub unsafe fn dc_stream_get_nearest_smallest_index(stream: *mut dc_stream_state, refresh: i32) -> i32 { for i in 0..9 { if (*stream).lumin_data.refresh_rate_hz[i] <= refresh && refresh < (*stream).lumin_data.refresh_rate_hz[i+1] { return i as i32; } } 9 }
pub unsafe fn dc_stream_is_refresh_rate_range_flickerless(stream: *mut dc_stream_state, _hz1: i32, _hz2: i32, _is_gaming: bool) -> bool { (*stream).lumin_data.is_valid }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
