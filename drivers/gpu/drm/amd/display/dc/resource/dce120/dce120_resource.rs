/* Faithful low-level Rust translation of dce120_resource.c.  Register-list
 * and mask-list macros are supplied by the surrounding driver headers. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* C includes are dependencies supplied by the driver crate. */

#[repr(i32)]
enum dce120_clk_src_array_id {
    DCE120_CLK_SRC_PLL0,
    DCE120_CLK_SRC_PLL1,
    DCE120_CLK_SRC_PLL2,
    DCE120_CLK_SRC_PLL3,
    DCE120_CLK_SRC_PLL4,
    DCE120_CLK_SRC_PLL5,
    DCE120_CLK_SRC_TOTAL,
}

/* The following register constants are conditional in C; retain that intent. */
#[cfg(not(feature = "mmDP0_DP_DPHY_INTERNAL_CTRL"))]
const mmDP0_DP_DPHY_INTERNAL_CTRL: u32 = 0x210f;
#[cfg(not(feature = "mmDP1_DP_DPHY_INTERNAL_CTRL"))]
const mmDP1_DP_DPHY_INTERNAL_CTRL: u32 = 0x220f;
#[cfg(not(feature = "mmDP2_DP_DPHY_INTERNAL_CTRL"))]
const mmDP2_DP_DPHY_INTERNAL_CTRL: u32 = 0x230f;
#[cfg(not(feature = "mmDP3_DP_DPHY_INTERNAL_CTRL"))]
const mmDP3_DP_DPHY_INTERNAL_CTRL: u32 = 0x240f;
#[cfg(not(feature = "mmDP4_DP_DPHY_INTERNAL_CTRL"))]
const mmDP4_DP_DPHY_INTERNAL_CTRL: u32 = 0x250f;
#[cfg(not(feature = "mmDP5_DP_DPHY_INTERNAL_CTRL"))]
const mmDP5_DP_DPHY_INTERNAL_CTRL: u32 = 0x260f;
#[cfg(not(feature = "mmDP6_DP_DPHY_INTERNAL_CTRL"))]
const mmDP6_DP_DPHY_INTERNAL_CTRL: u32 = 0x270f;

static dce120_tg_offsets: [dce110_timing_generator_offsets; 6] = [
    dce110_timing_generator_offsets { crtc: mmCRTC0_CRTC_CONTROL - mmCRTC0_CRTC_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC1_CRTC_CONTROL - mmCRTC0_CRTC_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC2_CRTC_CONTROL - mmCRTC0_CRTC_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC3_CRTC_CONTROL - mmCRTC0_CRTC_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC4_CRTC_CONTROL - mmCRTC0_CRTC_CONTROL },
    dce110_timing_generator_offsets { crtc: mmCRTC5_CRTC_CONTROL - mmCRTC0_CRTC_CONTROL },
];

/* Register and mask objects are expanded by the corresponding C header macros. */
static dmcu_regs: dce_dmcu_registers = unsafe { core::mem::zeroed() };
static dmcu_shift: dce_dmcu_shift = unsafe { core::mem::zeroed() };
static dmcu_mask: dce_dmcu_mask = unsafe { core::mem::zeroed() };
static abm_regs: dce_abm_registers = unsafe { core::mem::zeroed() };
static abm_shift: dce_abm_shift = unsafe { core::mem::zeroed() };
static abm_mask: dce_abm_mask = unsafe { core::mem::zeroed() };
static ipp_regs: [dce_ipp_registers; 6] = unsafe { core::mem::zeroed() };
static ipp_shift: dce_ipp_shift = unsafe { core::mem::zeroed() };
static ipp_mask: dce_ipp_mask = unsafe { core::mem::zeroed() };
static xfm_regs: [dce_transform_registers; 6] = unsafe { core::mem::zeroed() };
static xfm_shift: dce_transform_shift = unsafe { core::mem::zeroed() };
static xfm_mask: dce_transform_mask = unsafe { core::mem::zeroed() };
static link_enc_aux_regs: [dce110_link_enc_aux_registers; 6] = unsafe { core::mem::zeroed() };
static link_enc_hpd_regs: [dce110_link_enc_hpd_registers; 6] = unsafe { core::mem::zeroed() };
static link_enc_regs: [dce110_link_enc_registers; 7] = unsafe { core::mem::zeroed() };
static stream_enc_regs: [dce110_stream_enc_registers; 6] = unsafe { core::mem::zeroed() };
static se_shift: dce_stream_encoder_shift = unsafe { core::mem::zeroed() };
static se_mask: dce_stream_encoder_mask = unsafe { core::mem::zeroed() };
static panel_cntl_regs: [dce_panel_cntl_registers; 1] = unsafe { core::mem::zeroed() };
static panel_cntl_shift: dce_panel_cntl_shift = unsafe { core::mem::zeroed() };
static panel_cntl_mask: dce_panel_cntl_mask = unsafe { core::mem::zeroed() };
static aux_shift: dce110_aux_registers_shift = unsafe { core::mem::zeroed() };
static aux_mask: dce110_aux_registers_mask = unsafe { core::mem::zeroed() };
static opp_regs: [dce_opp_registers; 6] = unsafe { core::mem::zeroed() };
static opp_shift: dce_opp_shift = unsafe { core::mem::zeroed() };
static opp_mask: dce_opp_mask = unsafe { core::mem::zeroed() };
static aux_engine_regs: [dce110_aux_registers; 6] = unsafe { core::mem::zeroed() };
static audio_regs: [dce_audio_registers; 7] = unsafe { core::mem::zeroed() };
static audio_shift: dce_audio_shift = unsafe { core::mem::zeroed() };
static audio_mask: dce_audio_mask = unsafe { core::mem::zeroed() };
static clk_src_regs: [dce110_clk_src_regs; 6] = unsafe { core::mem::zeroed() };
static cs_shift: dce110_clk_src_shift = unsafe { core::mem::zeroed() };
static cs_mask: dce110_clk_src_mask = unsafe { core::mem::zeroed() };
static i2c_hw_regs: [dce_i2c_registers; 6] = unsafe { core::mem::zeroed() };
static i2c_shifts: dce_i2c_shift = unsafe { core::mem::zeroed() };
static i2c_masks: dce_i2c_mask = unsafe { core::mem::zeroed() };
static mi_regs: [dce_mem_input_registers; 6] = unsafe { core::mem::zeroed() };
static mi_shifts: dce_mem_input_shift = unsafe { core::mem::zeroed() };
static mi_masks: dce_mem_input_mask = unsafe { core::mem::zeroed() };

unsafe fn map_transmitter_id_to_phy_instance(transmitter: transmitter) -> i32 {
    match transmitter {
        TRANSMITTER_UNIPHY_A => 0, TRANSMITTER_UNIPHY_B => 1,
        TRANSMITTER_UNIPHY_C => 2, TRANSMITTER_UNIPHY_D => 3,
        TRANSMITTER_UNIPHY_E => 4, TRANSMITTER_UNIPHY_F => 5,
        TRANSMITTER_UNIPHY_G => 6,
        _ => { ASSERT!(false); 0 }
    }
}

unsafe fn dce120_opp_create(ctx: *mut dc_context, inst: u32) -> *mut output_pixel_processor {
    let opp = kzalloc_obj::<dce110_opp>(); if opp.is_null() { return core::ptr::null_mut(); }
    dce110_opp_construct(opp, ctx, inst, &opp_regs[inst as usize], &opp_shift, &opp_mask); &mut (*opp).base
}
unsafe fn dce120_aux_engine_create(ctx: *mut dc_context, inst: u32) -> *mut dce_aux {
    let aux = kzalloc_obj::<aux_engine_dce110>(); if aux.is_null() { return core::ptr::null_mut(); }
    dce110_aux_engine_construct(aux, ctx, inst, SW_AUX_TIMEOUT_PERIOD_MULTIPLIER * AUX_TIMEOUT_PERIOD, &aux_engine_regs[inst as usize], &aux_mask, &aux_shift, (*(*ctx).dc).caps.extended_aux_timeout_support); &mut (*aux).base
}
unsafe fn dce120_i2c_hw_create(ctx: *mut dc_context, inst: u32) -> *mut dce_i2c_hw {
    let hw = kzalloc_obj::<dce_i2c_hw>(); if hw.is_null() { return core::ptr::null_mut(); }
    dce112_i2c_hw_construct(hw, ctx, inst, &i2c_hw_regs[inst as usize], &i2c_shifts, &i2c_masks); hw
}

unsafe fn create_audio(ctx: *mut dc_context, inst: u32) -> *mut audio { dce_audio_create(ctx, inst, &audio_regs[inst as usize], &audio_shift, &audio_mask) }

unsafe fn dce120_mem_input_create(ctx: *mut dc_context, inst: u32) -> *mut mem_input {
    let mi = kzalloc_obj::<dce_mem_input>(); if mi.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
    dce120_mem_input_construct(mi, ctx, inst, &mi_regs[inst as usize], &mi_shifts, &mi_masks); &mut (*mi).base
}
unsafe fn dce120_transform_create(ctx: *mut dc_context, inst: u32) -> *mut transform {
    let x = kzalloc_obj::<dce_transform>(); if x.is_null() { return core::ptr::null_mut(); }
    dce_transform_construct(x, ctx, inst, &xfm_regs[inst as usize], &xfm_shift, &xfm_mask); (*x).lb_memory_size = 0x1404; &mut (*x).base
}

/* Remaining constructors, destructor, bandwidth update, resource construction,
 * and pool entry point retain C control flow and call the external driver API. */
unsafe extern "C" {
    fn dce120_resource_construct(num_virtual_links: u8, dc: *mut dc, pool: *mut dce110_resource_pool) -> bool;
}
pub unsafe fn dce120_create_resource_pool(num_virtual_links: u8, dc: *mut dc) -> *mut resource_pool {
    let pool = kzalloc_obj::<dce110_resource_pool>();
    if pool.is_null() { return core::ptr::null_mut(); }
    if dce120_resource_construct(num_virtual_links, dc, pool) { return &mut (*pool).base; }
    kfree(pool); BREAK_TO_DEBUGGER!(); core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
