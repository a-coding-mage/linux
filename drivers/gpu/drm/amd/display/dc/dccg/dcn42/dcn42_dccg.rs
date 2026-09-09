// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding repository:
// reg_helper.h, core_types.h, dcn35/dcn35_dccg.h, dcn42_dccg.h, dcn20/dcn20_dccg.h

#[inline]
unsafe fn to_dcn_dccg(dccg: *mut dccg) -> *mut dcn_dccg {
    container_of!(dccg, dcn_dccg, base)
}

pub unsafe extern "C" fn dccg42_otg_add_pixel(dccg: *mut dccg, otg_inst: u32) {
    let dccg_dcn = to_dcn_dccg(dccg);

    match otg_inst {
        0 => reg_update!(dccg_dcn, OTG_ADD_DROP_PIXEL_CNTL, OTG0_ADD_PIXEL, 1),
        1 => reg_update!(dccg_dcn, OTG_ADD_DROP_PIXEL_CNTL, OTG1_ADD_PIXEL, 1),
        2 => reg_update!(dccg_dcn, OTG_ADD_DROP_PIXEL_CNTL, OTG2_ADD_PIXEL, 1),
        3 => reg_update!(dccg_dcn, OTG_ADD_DROP_PIXEL_CNTL, OTG3_ADD_PIXEL, 1),
        _ => assert!(false),
    }
}

pub unsafe extern "C" fn dccg42_otg_drop_pixel(dccg: *mut dccg, otg_inst: u32) {
    let dccg_dcn = to_dcn_dccg(dccg);

    match otg_inst {
        0 => reg_update!(dccg_dcn, OTG_ADD_DROP_PIXEL_CNTL, OTG0_DROP_PIXEL, 1),
        1 => reg_update!(dccg_dcn, OTG_ADD_DROP_PIXEL_CNTL, OTG1_DROP_PIXEL, 1),
        2 => reg_update!(dccg_dcn, OTG_ADD_DROP_PIXEL_CNTL, OTG2_DROP_PIXEL, 1),
        3 => reg_update!(dccg_dcn, OTG_ADD_DROP_PIXEL_CNTL, OTG3_DROP_PIXEL, 1),
        _ => assert!(false),
    }
}

pub unsafe extern "C" fn dccg42_enable_global_fgcg(dccg: *mut dccg, mut enable: bool) {
    let dccg_dcn = to_dcn_dccg(dccg);

    // Temporary workaround for IOMMU mismatch issue.
    // Fine grain control via bit2 of debug flag.
    if (*(*dccg).ctx).dc.debug.disable_clock_gate
        || ((*(*dccg).ctx).dc.debug.iommu_mismatch_temp_wka & 0x4) != 0
    {
        enable = false;
    }

    reg_update!(dccg_dcn, DCCG_GLOBAL_FGCG_REP_CNTL, DCCG_GLOBAL_FGCG_REP_DIS, !enable);
}

pub unsafe extern "C" fn dccg42_get_global_fgcg_status(dccg: *mut dccg) -> bool {
    let dccg_dcn = to_dcn_dccg(dccg);
    let mut disabled: u32 = 0;

    reg_get!(dccg_dcn, DCCG_GLOBAL_FGCG_REP_CNTL, DCCG_GLOBAL_FGCG_REP_DIS, &mut disabled);
    (disabled & 0x1) != 0
}

pub unsafe extern "C" fn dccg42_set_physymclk(
    dccg: *mut dccg,
    phy_inst: i32,
    clk_src: physymclk_clock_source,
    force_enable: bool,
) {
    let dccg_dcn = to_dcn_dccg(dccg);
    let optimize = (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.physymclk;

    match phy_inst {
        0 => set_physymclk!(dccg_dcn, PHYASYMCLK, optimize, clk_src, force_enable),
        1 => set_physymclk!(dccg_dcn, PHYBSYMCLK, optimize, clk_src, force_enable),
        2 => set_physymclk!(dccg_dcn, PHYCSYMCLK, optimize, clk_src, force_enable),
        3 => set_physymclk!(dccg_dcn, PHYDSYMCLK, optimize, clk_src, force_enable),
        4 => set_physymclk!(dccg_dcn, PHYESYMCLK, optimize, clk_src, force_enable),
        _ => {
            break_to_debugger!();
            return;
        }
    }
}

unsafe fn dccg42_disable_hdmistreamclk(dccg: *mut dccg) {
    let dccg_dcn = to_dcn_dccg(dccg);

    reg_update_2!(dccg_dcn, HDMISTREAMCLK_CNTL,
        HDMISTREAMCLK0_EN, 0,
        HDMISTREAMCLK0_SRC_SEL, 0);
    reg_update!(dccg_dcn, DCCG_GATE_DISABLE_CNTL6,
        HDMISTREAMCLK0_ROOT_GATE_DISABLE,
        if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.hdmistream { 0 } else { 1 });
}

unsafe fn dccg42_disable_hdmicharclk(dccg: *mut dccg, hpo_inst: i32) {
    let dccg_dcn = to_dcn_dccg(dccg);

    assert!(hpo_inst >= 0);
    reg_write!(dccg_dcn, HDMICHARCLK_CLOCK_CNTL[hpo_inst as usize], 0);
    reg_update!(dccg_dcn, DCCG_GATE_DISABLE_CNTL2, HDMICHARCLK0_GATE_DISABLE, 0);
    reg_update!(dccg_dcn, DCCG_GATE_DISABLE_CNTL4, HDMICHARCLK0_ROOT_GATE_DISABLE,
        if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.hdmichar { 0 } else { 1 });
}

pub unsafe extern "C" fn dccg42_set_pixel_rate_div(
    dccg: *mut dccg,
    otg_inst: u32,
    tmds_div: pixel_rate_div,
    _unused: pixel_rate_div,
) {
    let dccg_dcn = to_dcn_dccg(dccg);
    let mut cur_tmds_div: u32 = PIXEL_RATE_DIV_NA;
    let mut dp_dto_int: u32 = 0;
    let mut reg_val: u32;

    // only 2 and 4 are valid on dcn401
    if tmds_div != PIXEL_RATE_DIV_BY_2 && tmds_div != PIXEL_RATE_DIV_BY_4 {
        return;
    }

    dccg401_get_pixel_rate_div(dccg, otg_inst, &mut cur_tmds_div, &mut dp_dto_int);
    if tmds_div == cur_tmds_div {
        return;
    }

    // encode enum to register value
    reg_val = if tmds_div == PIXEL_RATE_DIV_BY_4 { 1 } else { 0 };
    match otg_inst {
        0 => reg_update!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG0_TMDS_PIXEL_RATE_DIV, reg_val),
        1 => reg_update!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG1_TMDS_PIXEL_RATE_DIV, reg_val),
        2 => reg_update!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG2_TMDS_PIXEL_RATE_DIV, reg_val),
        3 => reg_update!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG3_TMDS_PIXEL_RATE_DIV, reg_val),
        _ => { break_to_debugger!(); return; }
    }
}

pub unsafe extern "C" fn dccg42_trigger_dio_fifo_resync(dccg: *mut dccg) {
    let dccg_dcn = to_dcn_dccg(dccg);
    reg_update!(dccg_dcn, DISPCLK_FREQ_CHANGE_CNTL, RESYNC_FIFO_LEVEL_ADJUST_EN, 1);
    reg_update!(dccg_dcn, DISPCLK_FREQ_CHANGE_CNTL, RESYNC_FIFO_LEVEL_ADJUST_EN, 0);
    reg_wait!(dccg_dcn, DISPCLK_FREQ_CHANGE_CNTL, DISPCLK_FREQ_RAMP_DONE, 1, 50, 2000);
}

unsafe fn dccg42_init(dccg: *mut dccg) {
    let res_pool = (*(*dccg).ctx).dc.res_pool;

    for i in 0..(*res_pool).hpo_dp_stream_enc_count {
        dccg35_disable_symclk32_se(dccg, i);
    }
    if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.symclk32_le {
        for i in 0..(*res_pool).hpo_dp_link_enc_count { dccg401_disable_symclk32_le(dccg, i); }
    }
    if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.dpstream {
        for i in 0..(*res_pool).hpo_dp_stream_enc_count { dccg401_disable_dpstreamclk(dccg, i); }
    }
    dccg42_disable_hdmistreamclk(dccg);
    if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.hdmichar { dccg42_disable_hdmicharclk(dccg, 0); }
    if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.dpp {
        for i in 0..(*res_pool).pipe_count { dccg35_dpp_root_clock_control(dccg, i, true); }
    }
}

static DccgFuncs dccg42_funcs = DccgFuncs {
    enable_hdmicharclk: Some(dccg401_enable_hdmicharclk), disable_hdmicharclk: Some(dccg42_disable_hdmicharclk),
    set_hdmistreamclk: Some(dccg35_set_hdmistreamclk), set_hdmistreamclk_root_clock_gating: Some(dccg35_set_hdmistreamclk_root_clock_gating),
    update_dpp_dto: Some(dccg35_update_dpp_dto), dpp_root_clock_control: Some(dccg35_dpp_root_clock_control),
    get_dccg_ref_freq: Some(dccg401_get_dccg_ref_freq), dccg_init: Some(dccg42_init), set_dpstreamclk: Some(dccg401_set_dpstreamclk),
    enable_symclk32_se: Some(dccg31_enable_symclk32_se), disable_symclk32_se: Some(dccg35_disable_symclk32_se),
    enable_symclk32_le: Some(dccg401_enable_symclk32_le), disable_symclk32_le: Some(dccg401_disable_symclk32_le),
    set_symclk32_le_root_clock_gating: Some(dccg31_set_symclk32_le_root_clock_gating), set_physymclk: Some(dccg42_set_physymclk),
    set_dtbclk_dto: None, set_dto_dscclk: Some(dccg401_set_dto_dscclk), set_ref_dscclk: Some(dccg401_set_ref_dscclk),
    set_valid_pixel_rate: None, set_fifo_errdet_ovr_en: Some(dccg2_set_fifo_errdet_ovr_en), set_audio_dtbclk_dto: None,
    otg_add_pixel: Some(dccg42_otg_add_pixel), otg_drop_pixel: Some(dccg42_otg_drop_pixel), disable_dsc: Some(dccg35_disable_dscclk),
    enable_dsc: Some(dccg35_enable_dscclk), set_pixel_rate_div: Some(dccg42_set_pixel_rate_div), get_pixel_rate_div: Some(dccg401_get_pixel_rate_div),
    trigger_dio_fifo_resync: Some(dccg42_trigger_dio_fifo_resync), set_dp_dto: Some(dccg401_set_dp_dto), enable_symclk_se: Some(dccg35_enable_symclk_se),
    disable_symclk_se: Some(dccg35_disable_symclk_se), set_dtbclk_p_src: Some(dccg401_set_dtbclk_p_src), dccg_root_gate_disable_control: Some(dccg35_root_gate_disable_control),
    dccg_read_reg_state: Some(dccg31_read_reg_state), dccg_enable_global_fgcg: Some(dccg42_enable_global_fgcg), allow_clock_gating: Some(dccg2_allow_clock_gating),
    dccg_get_global_fgcg_status: Some(dccg42_get_global_fgcg_status),
};

pub unsafe extern "C" fn dccg42_create(ctx: *mut dc_context, regs: *const dccg_registers, dccg_shift: *const dccg_shift, dccg_mask: *const dccg_mask) -> *mut dccg {
    let dccg_dcn = kzalloc!(core::mem::size_of::<dcn_dccg>(), GFP_KERNEL) as *mut dcn_dccg;
    if dccg_dcn.is_null() { break_to_debugger!(); return core::ptr::null_mut(); }
    (*dccg_dcn).base.ctx = ctx;
    (*dccg_dcn).base.funcs = &dccg42_funcs;
    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;
    &mut (*dccg_dcn).base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
