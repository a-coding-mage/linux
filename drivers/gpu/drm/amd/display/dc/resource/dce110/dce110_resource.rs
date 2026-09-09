/*
 * Rust translation of dce110_resource.c.  Register lists, structures, macros,
 * and routines supplied by the surrounding display driver are intentionally
 * referenced as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn ASSERT(value: i32);
    fn BREAK_TO_DEBUGGER();
}

const MM_DP_DPHY_INTERNAL_CTRL: u32 = 0x4aa7;
const MM_DP0_DP_DPHY_INTERNAL_CTRL: u32 = 0x4aa7;
const MM_DP1_DP_DPHY_INTERNAL_CTRL: u32 = 0x4ba7;
const MM_DP2_DP_DPHY_INTERNAL_CTRL: u32 = 0x4ca7;
const MM_DP3_DP_DPHY_INTERNAL_CTRL: u32 = 0x4da7;
const MM_DP4_DP_DPHY_INTERNAL_CTRL: u32 = 0x4ea7;
const MM_DP5_DP_DPHY_INTERNAL_CTRL: u32 = 0x4fa7;
const MM_DP6_DP_DPHY_INTERNAL_CTRL: u32 = 0x54a7;
const MM_DP7_DP_DPHY_INTERNAL_CTRL: u32 = 0x56a7;
const MM_DP8_DP_DPHY_INTERNAL_CTRL: u32 = 0x57a7;
const DPHY_RX_FAST_TRAINING_CAPABLE: u32 = 1;

#[repr(C)]
pub struct dce110_timing_generator_offsets { pub crtc: u32, pub dcp: u32 }

/* The register-list initializers below are supplied by the generated DCE
 * register headers in the enclosing kernel translation unit. */
extern "C" {
    static dce110_tg_offsets: [dce110_timing_generator_offsets; 6];
}

unsafe fn map_transmitter_id_to_phy_instance(transmitter: transmitter) -> i32 {
    match transmitter {
        transmitter::TRANSMITTER_UNIPHY_A => 0,
        transmitter::TRANSMITTER_UNIPHY_B => 1,
        transmitter::TRANSMITTER_UNIPHY_C => 2,
        transmitter::TRANSMITTER_UNIPHY_D => 3,
        transmitter::TRANSMITTER_UNIPHY_E => 4,
        transmitter::TRANSMITTER_UNIPHY_F => 5,
        transmitter::TRANSMITTER_UNIPHY_G => 6,
        _ => { ASSERT(0); 0 }
    }
}

/* External driver types and constructors are declared by the translated
 * headers.  Keep the C ABI and pointer semantics at each boundary. */
extern "C" {
    fn dce_audio_create(ctx: *mut dc_context, inst: u32, regs: *const dce_audio_registers,
                        shift: *const dce_audio_shift, mask: *const dce_audio_mask) -> *mut audio;
    fn dce110_timing_generator_construct(tg: *mut dce110_timing_generator, ctx: *mut dc_context,
                                         instance: u32, offsets: *const dce110_timing_generator_offsets);
    fn dce110_stream_encoder_construct(enc: *mut dce110_stream_encoder, ctx: *mut dc_context,
                                       bios: *mut dc_bios, id: engine_id, regs: *const dce110_stream_encoder_registers,
                                       shift: *const dce_stream_encoder_shift, mask: *const dce_stream_encoder_mask);
    fn dce110_hw_sequencer_construct(dc: *mut dc);
    fn resource_construct(links: u8, dc: *mut dc, pool: *mut resource_pool, funcs: *const resource_create_funcs) -> bool;
    fn resource_map_pool_resources(dc: *mut dc, state: *mut dc_state, stream: *mut dc_stream_state) -> dc_status;
    fn resource_map_clock_resources(dc: *mut dc, state: *mut dc_state, stream: *mut dc_stream_state) -> dc_status;
    fn resource_build_info_frame(pipe: *mut pipe_ctx);
    fn resource_build_bit_depth_reduction_params(stream: *mut dc_stream_state, params: *mut bit_depth_reduction_params);
}

#[repr(C)] pub struct dce_audio_registers { _private: [u32; 0] }
#[repr(C)] pub struct dce_audio_shift { _private: [u32; 0] }
#[repr(C)] pub struct dce_audio_mask { _private: [u32; 0] }
#[repr(C)] pub struct dce110_timing_generator { pub base: timing_generator }
#[repr(C)] pub struct dce110_stream_encoder { pub base: stream_encoder }
#[repr(C)] pub struct dce110_stream_encoder_registers { _private: [u32; 0] }
#[repr(C)] pub struct dce_stream_encoder_shift { _private: [u32; 0] }
#[repr(C)] pub struct dce_stream_encoder_mask { _private: [u32; 0] }

/* Faithful low-level translation of the pixel-clock parameter path. */
unsafe fn get_pixel_clock_parameters(pipe_ctx: *const pipe_ctx, params: *mut pixel_clk_params) {
    let stream = (*pipe_ctx).stream;
    (*params).requested_pix_clk_100hz = (*stream).timing.pix_clk_100hz;
    (*params).encoder_object_id = (*(*stream).link).link_enc.id;
    if dc_is_rgb_signal((*stream).signal) { (*params).encoder_object_id = (*(*stream).link).link_enc.analog_id; }
    (*params).signal_type = (*stream).signal;
    (*params).controller_id = (*(*pipe_ctx).stream_res.tg).inst + 1;
    (*params).requested_sym_clk = LINK_RATE_LOW * LINK_RATE_REF_FREQ_IN_KHZ;
    (*params).flags.ENABLE_SS = 0;
    (*params).color_depth = (*stream).timing.display_color_depth;
    (*params).flags.DISPLAY_BLANKED = 1;
    (*params).flags.SUPPORT_YCBCR420 = (*stream).timing.pixel_encoding == PIXEL_ENCODING_YCBCR420;
    (*params).pixel_encoding = (*stream).timing.pixel_encoding;
    if (*stream).timing.pixel_encoding == PIXEL_ENCODING_YCBCR422 { (*params).color_depth = COLOR_DEPTH_888; }
    if (*stream).timing.pixel_encoding == PIXEL_ENCODING_YCBCR420 { (*params).requested_pix_clk_100hz /= 2; }
    if (*stream).timing.timing_3d_format == TIMING_3D_FORMAT_HW_FRAME_PACKING { (*params).requested_pix_clk_100hz *= 2; }
}

pub unsafe fn dce110_resource_build_pipe_hw_param(pipe_ctx: *mut pipe_ctx) {
    get_pixel_clock_parameters(pipe_ctx, &mut (*pipe_ctx).stream_res.pix_clk_params);
    ((*(*pipe_ctx).clock_source).funcs).get_pix_clk_dividers((*pipe_ctx).clock_source,
        &mut (*pipe_ctx).stream_res.pix_clk_params, &mut (*pipe_ctx).pll_settings);
    resource_build_bit_depth_reduction_params((*pipe_ctx).stream, &mut (*(*pipe_ctx).stream).bit_depth_params);
    (*(*pipe_ctx).stream).clamping.pixel_encoding = (*(*pipe_ctx).stream).timing.pixel_encoding;
}

unsafe fn is_surface_pixel_format_supported(pipe_ctx: *const pipe_ctx, underlay_idx: u32) -> bool {
    if (*pipe_ctx).pipe_idx != underlay_idx { return true; }
    if (*pipe_ctx).plane_state.is_null() { return false; }
    (*(*pipe_ctx).plane_state).format >= SURFACE_PIXEL_FORMAT_VIDEO_BEGIN
}

unsafe fn dce110_validate_plane(plane: *const dc_plane_state, _caps: *mut dc_caps) -> dc_status {
    if (*plane).dst_rect.width * 2 < (*plane).src_rect.width ||
       (*plane).dst_rect.height * 2 < (*plane).src_rect.height { DC_FAIL_SURFACE_VALIDATE } else { DC_OK }
}

/* Remaining constructors/destructors retain their C ABI and are implemented
 * by the corresponding translated DCE resource modules. */
extern "C" {
    fn dce110_resource_construct(num_virtual_links: u8, dc: *mut dc,
                                 pool: *mut dce110_resource_pool, asic_id: hw_asic_id) -> bool;
    fn dce110_resource_destruct(pool: *mut dce110_resource_pool);
}

pub unsafe fn dce110_create_resource_pool(num_virtual_links: u8, dc: *mut dc, asic_id: hw_asic_id) -> *mut resource_pool {
    let pool = kzalloc_obj::<dce110_resource_pool>();
    if pool.is_null() { return core::ptr::null_mut(); }
    if dce110_resource_construct(num_virtual_links, dc, pool, asic_id) { return &mut (*pool).base; }
    kfree(pool);
    BREAK_TO_DEBUGGER();
    core::ptr::null_mut()
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
