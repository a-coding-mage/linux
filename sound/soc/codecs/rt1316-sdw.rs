// SPDX-License-Identifier: GPL-2.0-only
//
// rt1316-sdw.c -- rt1316 SDCA ALSA SoC amplifier audio driver
//
// Copyright(c) 2021 Realtek Semiconductor Corp.
//
//
// C dependencies translated as external Rust dependencies:
// linux/delay.h, linux/device.h, linux/pm_runtime.h, linux/module.h,
// linux/regmap.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/sdw.h, sound/soc-dapm.h, sound/initval.h, "rt1316-sdw.h"

static rt1316_reg_defaults: [reg_default; 48] = [
    reg_default { reg: 0x3004, def: 0x00 },
    reg_default { reg: 0x3005, def: 0x00 },
    reg_default { reg: 0x3206, def: 0x00 },
    reg_default { reg: 0xc001, def: 0x00 },
    reg_default { reg: 0xc002, def: 0x00 },
    reg_default { reg: 0xc003, def: 0x00 },
    reg_default { reg: 0xc004, def: 0x00 },
    reg_default { reg: 0xc005, def: 0x00 },
    reg_default { reg: 0xc006, def: 0x00 },
    reg_default { reg: 0xc007, def: 0x00 },
    reg_default { reg: 0xc008, def: 0x00 },
    reg_default { reg: 0xc009, def: 0x00 },
    reg_default { reg: 0xc00a, def: 0x00 },
    reg_default { reg: 0xc00b, def: 0x00 },
    reg_default { reg: 0xc00c, def: 0x00 },
    reg_default { reg: 0xc00d, def: 0x00 },
    reg_default { reg: 0xc00e, def: 0x00 },
    reg_default { reg: 0xc00f, def: 0x00 },
    reg_default { reg: 0xc010, def: 0xa5 },
    reg_default { reg: 0xc011, def: 0x00 },
    reg_default { reg: 0xc012, def: 0xff },
    reg_default { reg: 0xc013, def: 0xff },
    reg_default { reg: 0xc014, def: 0x40 },
    reg_default { reg: 0xc015, def: 0x00 },
    reg_default { reg: 0xc016, def: 0x00 },
    reg_default { reg: 0xc017, def: 0x00 },
    reg_default { reg: 0xc605, def: 0x30 },
    reg_default { reg: 0xc700, def: 0x0a },
    reg_default { reg: 0xc701, def: 0xaa },
    reg_default { reg: 0xc702, def: 0x1a },
    reg_default { reg: 0xc703, def: 0x0a },
    reg_default { reg: 0xc710, def: 0x80 },
    reg_default { reg: 0xc711, def: 0x00 },
    reg_default { reg: 0xc712, def: 0x3e },
    reg_default { reg: 0xc713, def: 0x80 },
    reg_default { reg: 0xc714, def: 0x80 },
    reg_default { reg: 0xc715, def: 0x06 },
    reg_default { reg: 0xd101, def: 0x00 },
    reg_default { reg: 0xd102, def: 0x30 },
    reg_default { reg: 0xd103, def: 0x00 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_FU21, RT1316_SDCA_CTL_FU_MUTE, CH_L), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_FU21, RT1316_SDCA_CTL_FU_MUTE, CH_R), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_XU24, RT1316_SDCA_CTL_BYPASS, 0), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE23, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), def: 0x03 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE22, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), def: 0x03 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE24, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), def: 0x03 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_UDMPU21, RT1316_SDCA_CTL_UDMPU_CLUSTER, 0), def: 0x00 },
];

static rt1316_blind_write: [reg_sequence; 66] = [
    reg_sequence { reg: 0xc710, def: 0x17 },
    reg_sequence { reg: 0xc711, def: 0x80 },
    reg_sequence { reg: 0xc712, def: 0x26 },
    reg_sequence { reg: 0xc713, def: 0x06 },
    reg_sequence { reg: 0xc714, def: 0x80 },
    reg_sequence { reg: 0xc715, def: 0x06 },
    reg_sequence { reg: 0xc702, def: 0x0a },
    reg_sequence { reg: 0xc703, def: 0x0a },
    reg_sequence { reg: 0xc001, def: 0x45 },
    reg_sequence { reg: 0xc003, def: 0x00 },
    reg_sequence { reg: 0xc004, def: 0x11 },
    reg_sequence { reg: 0xc005, def: 0x00 },
    reg_sequence { reg: 0xc006, def: 0x00 },
    reg_sequence { reg: 0xc106, def: 0x00 },
    reg_sequence { reg: 0xc007, def: 0x11 },
    reg_sequence { reg: 0xc008, def: 0x11 },
    reg_sequence { reg: 0xc009, def: 0x00 },
    reg_sequence { reg: 0x2f0a, def: 0x00 },
    reg_sequence { reg: 0xd101, def: 0xf0 },
    reg_sequence { reg: 0xd103, def: 0x9b },
    reg_sequence { reg: 0x2f36, def: 0x8e },
    reg_sequence { reg: 0x3206, def: 0x80 },
    reg_sequence { reg: 0x3211, def: 0x0b },
    reg_sequence { reg: 0x3216, def: 0x06 },
    reg_sequence { reg: 0xc614, def: 0x20 },
    reg_sequence { reg: 0xc615, def: 0x0a },
    reg_sequence { reg: 0xc616, def: 0x02 },
    reg_sequence { reg: 0xc617, def: 0x00 },
    reg_sequence { reg: 0xc60b, def: 0x10 },
    reg_sequence { reg: 0xc60e, def: 0x05 },
    reg_sequence { reg: 0xc102, def: 0x00 },
    reg_sequence { reg: 0xc090, def: 0xb0 },
    reg_sequence { reg: 0xc00f, def: 0x01 },
    reg_sequence { reg: 0xc09c, def: 0x7b },
    reg_sequence { reg: 0xc602, def: 0x07 },
    reg_sequence { reg: 0xc603, def: 0x07 },
    reg_sequence { reg: 0xc0a3, def: 0x71 },
    reg_sequence { reg: 0xc00b, def: 0x30 },
    reg_sequence { reg: 0xc093, def: 0x80 },
    reg_sequence { reg: 0xc09d, def: 0x80 },
    reg_sequence { reg: 0xc0b0, def: 0x77 },
    reg_sequence { reg: 0xc010, def: 0xa5 },
    reg_sequence { reg: 0xc050, def: 0x83 },
    reg_sequence { reg: 0x2f55, def: 0x03 },
    reg_sequence { reg: 0x3217, def: 0xb5 },
    reg_sequence { reg: 0x3202, def: 0x02 },
    reg_sequence { reg: SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_XU24, RT1316_SDCA_CTL_BYPASS, 0), def: 0x00 },
    /* for IV sense */
    reg_sequence { reg: 0x2232, def: 0x80 },
    reg_sequence { reg: 0xc0b0, def: 0x77 },
    reg_sequence { reg: 0xc011, def: 0x00 },
    reg_sequence { reg: 0xc020, def: 0x00 },
    reg_sequence { reg: 0xc023, def: 0x00 },
    reg_sequence { reg: 0x3101, def: 0x00 },
    reg_sequence { reg: 0x3004, def: 0xa0 },
    reg_sequence { reg: 0x3005, def: 0xb1 },
    reg_sequence { reg: 0xc007, def: 0x11 },
    reg_sequence { reg: 0xc008, def: 0x11 },
    reg_sequence { reg: 0xc009, def: 0x00 },
    reg_sequence { reg: 0xc022, def: 0xd6 },
    reg_sequence { reg: 0xc025, def: 0xd6 },
    reg_sequence { reg: 0xd001, def: 0x03 },
    reg_sequence { reg: 0xd002, def: 0xbf },
    reg_sequence { reg: 0xd003, def: 0x03 },
    reg_sequence { reg: 0xd004, def: 0xbf },
];

unsafe fn rt1316_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x2f0a | 0x2f36 | 0x3203..=0x320e | 0xc000..=0xc7b4 | 0xcf00..=0xcf03 | 0xd101..=0xd103 => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_UDMPU21, RT1316_SDCA_CTL_UDMPU_CLUSTER, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_FU21, RT1316_SDCA_CTL_FU_MUTE, CH_L) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_FU21, RT1316_SDCA_CTL_FU_MUTE, CH_R) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE23, RT1316_SDCA_CTL_REQ_POWER_STATE, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE27, RT1316_SDCA_CTL_REQ_POWER_STATE, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE22, RT1316_SDCA_CTL_REQ_POWER_STATE, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE24, RT1316_SDCA_CTL_REQ_POWER_STATE, 0) => true,
        _ => false,
    }
}

unsafe fn rt1316_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0xc000 | 0xc093 | 0xc09d | 0xc0a3 | 0xc201 | 0xc427..=0xc428 | 0xd102 => true,
        _ => false,
    }
}

static rt1316_sdw_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt1316_readable_register),
    volatile_reg: Some(rt1316_volatile_register),
    max_register: 0x4108ffff,
    reg_defaults: rt1316_reg_defaults.as_ptr(),
    num_reg_defaults: rt1316_reg_defaults.len(),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe fn rt1316_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop: *mut sdw_slave_prop = &mut (*slave).prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;

    (*prop).paging_support = true;

    /* first we need to allocate memory for set bits in port lists */
    (*prop).source_ports = 0x04; /* BITMAP: 00000100 */
    (*prop).sink_ports = 0x2; /* BITMAP:  00000010 */

    nval = hweight32((*prop).source_ports);
    (*prop).src_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval as usize, core::mem::size_of_val(&*(*prop).src_dpn_prop), GFP_KERNEL) as *mut sdw_dpn_prop;
    if (*prop).src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = (*prop).src_dpn_prop;
    addr = (*prop).source_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.add(i as usize)).num = bit;
            (*dpn.add(i as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(i as usize)).simple_ch_prep_sm = true;
            (*dpn.add(i as usize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = hweight32((*prop).sink_ports);
    (*prop).sink_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval as usize, core::mem::size_of_val(&*(*prop).sink_dpn_prop), GFP_KERNEL) as *mut sdw_dpn_prop;
    if (*prop).sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    j = 0;
    dpn = (*prop).sink_dpn_prop;
    addr = (*prop).sink_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.add(j as usize)).num = bit;
            (*dpn.add(j as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(j as usize)).simple_ch_prep_sm = true;
            (*dpn.add(j as usize)).ch_prep_timeout = 10;
            j += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 20;

    dev_dbg(&mut (*slave).dev, c_str!("%s\n"), __func__);

    0
}

unsafe fn rt1316_apply_bq_params(rt1316: *mut rt1316_sdw_priv) {
    let mut i: c_uint = 0;
    let mut reg: c_uint;
    let mut data: c_uint;

    while i < (*rt1316).bq_params_cnt {
        reg = *(*rt1316).bq_params.add(i as usize) as c_uint
            | ((*(*rt1316).bq_params.add((i + 1) as usize) as c_uint) << 8);
        data = *(*rt1316).bq_params.add((i + 2) as usize) as c_uint;
        regmap_write((*rt1316).regmap, reg, data);
        i += 3;
    }
}

unsafe fn rt1316_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt1316: *mut rt1316_sdw_priv = dev_get_drvdata(dev) as *mut rt1316_sdw_priv;

    if (*rt1316).hw_init {
        return 0;
    }

    regcache_cache_only((*rt1316).regmap, false);
    if (*rt1316).first_hw_init {
        regcache_cache_bypass((*rt1316).regmap, true);
    } else {
        /*
         *  PM runtime status is marked as 'active' only when a Slave reports as Attached
         */

        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    /* sw reset */
    regmap_write((*rt1316).regmap, 0xc000, 0x02);

    /* initial settings - blind write */
    regmap_multi_reg_write((*rt1316).regmap, rt1316_blind_write.as_ptr(), rt1316_blind_write.len());

    if (*rt1316).first_hw_init {
        regcache_cache_bypass((*rt1316).regmap, false);
        regcache_mark_dirty((*rt1316).regmap);
    } else {
        (*rt1316).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt1316).hw_init = true;

    pm_runtime_put_autosuspend(&mut (*slave).dev);

    dev_dbg(&mut (*slave).dev, c_str!("%s hw_init complete\n"), __func__);
    0
}

unsafe fn rt1316_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let rt1316: *mut rt1316_sdw_priv = dev_get_drvdata(&mut (*slave).dev) as *mut rt1316_sdw_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt1316).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt1316).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt1316_io_init(&mut (*slave).dev, slave)
}

unsafe fn rt1316_classd_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let rt1316: *mut rt1316_sdw_priv = snd_soc_component_get_drvdata(component) as *mut rt1316_sdw_priv;
    let ps0: u8 = 0x0;
    let ps3: u8 = 0x3;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt1316).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE23, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), ps0 as c_uint);
            regmap_write((*rt1316).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE27, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), ps0 as c_uint);
            regmap_write((*rt1316).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE22, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), ps0 as c_uint);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt1316).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE23, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), ps3 as c_uint);
            regmap_write((*rt1316).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE27, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), ps3 as c_uint);
            regmap_write((*rt1316).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE22, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), ps3 as c_uint);
        }
        _ => {}
    }

    0
}

unsafe fn rt1316_pde24_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let rt1316: *mut rt1316_sdw_priv = snd_soc_component_get_drvdata(component) as *mut rt1316_sdw_priv;
    let ps0: u8 = 0x0;
    let ps3: u8 = 0x3;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt1316).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE24, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), ps0 as c_uint);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt1316).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_PDE24, RT1316_SDCA_CTL_REQ_POWER_STATE, 0), ps3 as c_uint);
        }
        _ => {}
    }
    0
}

static rt1316_rx_data_ch_select: [&'static CStr; 10] = [
    c_str!("L,R"),
    c_str!("L,L"),
    c_str!("L,R"),
    c_str!("L,L+R"),
    c_str!("R,L"),
    c_str!("R,R"),
    c_str!("R,L+R"),
    c_str!("L+R,L"),
    c_str!("L+R,R"),
    c_str!("L+R,L+R"),
];

static rt1316_rx_data_ch_enum: soc_enum = SOC_ENUM_SINGLE_DECL(
    SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_UDMPU21, RT1316_SDCA_CTL_UDMPU_CLUSTER, 0),
    0,
    &rt1316_rx_data_ch_select,
);

static rt1316_dac_output_vol_select: [&'static CStr; 3] = [
    c_str!("immediately"),
    c_str!("zero crossing"),
    c_str!("zero crossing with soft ramp"),
];

static rt1316_dac_vol_ctl_enum: soc_enum = SOC_ENUM_SINGLE_DECL(0xc010, 6, &rt1316_dac_output_vol_select);

static rt1316_snd_controls: [snd_kcontrol_new; 10] = [
    /* I2S Data Channel Selection */
    SOC_ENUM(c_str!("RX Channel Select"), &rt1316_rx_data_ch_enum),

    /* XU24 Bypass Control */
    SOC_SINGLE(c_str!("XU24 Bypass Switch"), SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_XU24, RT1316_SDCA_CTL_BYPASS, 0), 0, 1, 0),

    /* Left/Right IV tag */
    SOC_SINGLE(c_str!("Left V Tag Select"), 0x3004, 0, 7, 0),
    SOC_SINGLE(c_str!("Left I Tag Select"), 0x3004, 4, 7, 0),
    SOC_SINGLE(c_str!("Right V Tag Select"), 0x3005, 0, 7, 0),
    SOC_SINGLE(c_str!("Right I Tag Select"), 0x3005, 4, 7, 0),

    /* IV mixer Control */
    SOC_DOUBLE(c_str!("Isense Mixer Switch"), 0xc605, 2, 0, 1, 1),
    SOC_DOUBLE(c_str!("Vsense Mixer Switch"), 0xc605, 3, 1, 1, 1),

    /* DAC Output Volume Control */
    SOC_ENUM(c_str!("DAC Output Vol Control"), &rt1316_dac_vol_ctl_enum),
];

static rt1316_sto_dac: snd_kcontrol_new = SOC_DAPM_DOUBLE_R(
    c_str!("Switch"),
    SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_FU21, RT1316_SDCA_CTL_FU_MUTE, CH_L),
    SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1316_SDCA_ENT_FU21, RT1316_SDCA_CTL_FU_MUTE, CH_R),
    0,
    1,
    1,
);

static rt1316_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    /* Audio Interface */
    SND_SOC_DAPM_AIF_IN(c_str!("DP1RX"), c_str!("DP1 Playback"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(c_str!("DP2TX"), c_str!("DP2 Capture"), 0, SND_SOC_NOPM, 0, 0),

    /* Digital Interface */
    SND_SOC_DAPM_SWITCH(c_str!("DAC"), SND_SOC_NOPM, 0, 0, &rt1316_sto_dac),

    /* Output Lines */
    SND_SOC_DAPM_PGA_E(c_str!("CLASS D"), SND_SOC_NOPM, 0, 0, core::ptr::null(), 0, Some(rt1316_classd_event), SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_OUTPUT(c_str!("SPOL")),
    SND_SOC_DAPM_OUTPUT(c_str!("SPOR")),

    SND_SOC_DAPM_SUPPLY(c_str!("PDE 24"), SND_SOC_NOPM, 0, 0, Some(rt1316_pde24_event), SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA(c_str!("I Sense"), SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA(c_str!("V Sense"), SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SIGGEN(c_str!("I Gen")),
    SND_SOC_DAPM_SIGGEN(c_str!("V Gen")),
];

static rt1316_dapm_routes: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route { sink: c_str!("DAC"), control: c_str!("Switch"), source: c_str!("DP1RX") },
    snd_soc_dapm_route { sink: c_str!("CLASS D"), control: core::ptr::null(), source: c_str!("DAC") },
    snd_soc_dapm_route { sink: c_str!("SPOL"), control: core::ptr::null(), source: c_str!("CLASS D") },
    snd_soc_dapm_route { sink: c_str!("SPOR"), control: core::ptr::null(), source: c_str!("CLASS D") },

    snd_soc_dapm_route { sink: c_str!("I Sense"), control: core::ptr::null(), source: c_str!("I Gen") },
    snd_soc_dapm_route { sink: c_str!("V Sense"), control: core::ptr::null(), source: c_str!("V Gen") },
    snd_soc_dapm_route { sink: c_str!("I Sense"), control: core::ptr::null(), source: c_str!("PDE 24") },
    snd_soc_dapm_route { sink: c_str!("V Sense"), control: core::ptr::null(), source: c_str!("PDE 24") },
    snd_soc_dapm_route { sink: c_str!("DP2TX"), control: core::ptr::null(), source: c_str!("I Sense") },
    snd_soc_dapm_route { sink: c_str!("DP2TX"), control: core::ptr::null(), source: c_str!("V Sense") },
];

unsafe fn rt1316_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

unsafe fn rt1316_sdw_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, core::ptr::null_mut());
}

unsafe fn rt1316_sdw_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let rt1316: *mut rt1316_sdw_priv = snd_soc_component_get_drvdata(component) as *mut rt1316_sdw_priv;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream: *mut sdw_stream_runtime;
    let retval: c_int;

    dev_dbg((*dai).dev, c_str!("%s %s"), __func__, (*dai).name);
    sdw_stream = snd_soc_dai_get_dma_data(dai, substream) as *mut sdw_stream_runtime;

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if (*rt1316).sdw_slave.is_null() {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    /* port 1 for playback */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        port_config.num = 1;
    } else {
        port_config.num = 2;
    }

    retval = sdw_stream_add_slave((*rt1316).sdw_slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        dev_err((*dai).dev, c_str!("%s: Unable to configure port\n"), __func__);
        return retval;
    }

    0
}

unsafe fn rt1316_sdw_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let rt1316: *mut rt1316_sdw_priv = snd_soc_component_get_drvdata(component) as *mut rt1316_sdw_priv;
    let sdw_stream: *mut sdw_stream_runtime = snd_soc_dai_get_dma_data(dai, substream) as *mut sdw_stream_runtime;

    if (*rt1316).sdw_slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*rt1316).sdw_slave, sdw_stream);
    0
}

/*
 * slave_ops: callbacks for get_clock_stop_mode, clock_stop and
 * port_prep are not defined for now
 */
static rt1316_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt1316_read_prop),
    update_status: Some(rt1316_update_status),
};

unsafe fn rt1316_sdw_parse_dt(rt1316: *mut rt1316_sdw_priv, dev: *mut device) -> c_int {
    let mut ret: c_int = 0;

    device_property_read_u32(dev, c_str!("realtek,bq-params-cnt"), &mut (*rt1316).bq_params_cnt);
    if (*rt1316).bq_params_cnt != 0 {
        (*rt1316).bq_params = devm_kzalloc(dev, (*rt1316).bq_params_cnt as usize, GFP_KERNEL) as *mut u8;
        if (*rt1316).bq_params.is_null() {
            dev_err(dev, c_str!("%s: Could not allocate bq_params memory\n"), __func__);
            ret = -ENOMEM;
        } else {
            ret = device_property_read_u8_array(dev, c_str!("realtek,bq-params"), (*rt1316).bq_params, (*rt1316).bq_params_cnt);
            if ret < 0 {
                dev_err(dev, c_str!("%s: Could not read list of realtek,bq-params\n"), __func__);
            }
        }
    }

    dev_dbg(dev, c_str!("bq_params_cnt=%d\n"), (*rt1316).bq_params_cnt);
    ret
}

unsafe fn rt1316_sdw_component_probe(component: *mut snd_soc_component) -> c_int {
    let rt1316: *mut rt1316_sdw_priv = snd_soc_component_get_drvdata(component) as *mut rt1316_sdw_priv;
    let ret: c_int;

    (*rt1316).component = component;
    rt1316_sdw_parse_dt(rt1316, &mut (*(*rt1316).sdw_slave).dev);

    if !(*rt1316).first_hw_init {
        return 0;
    }

    ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    /* apply BQ params */
    rt1316_apply_bq_params(rt1316);

    0
}

static soc_component_sdw_rt1316: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt1316_sdw_component_probe),
    controls: rt1316_snd_controls.as_ptr(),
    num_controls: rt1316_snd_controls.len(),
    dapm_widgets: rt1316_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt1316_dapm_widgets.len(),
    dapm_routes: rt1316_dapm_routes.as_ptr(),
    num_dapm_routes: rt1316_dapm_routes.len(),
    endianness: 1,
};

static rt1316_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt1316_sdw_hw_params),
    hw_free: Some(rt1316_sdw_pcm_hw_free),
    set_stream: Some(rt1316_set_sdw_stream),
    shutdown: Some(rt1316_sdw_shutdown),
};

const RT1316_STEREO_RATES: c_uint = SNDRV_PCM_RATE_48000;
const RT1316_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

static mut rt1316_sdw_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: c_str!("rt1316-aif"),
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("DP1 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: RT1316_STEREO_RATES,
            formats: RT1316_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("DP2 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: RT1316_STEREO_RATES,
            formats: RT1316_FORMATS,
        },
        ops: &rt1316_aif_dai_ops,
    },
];

unsafe fn rt1316_sdw_init(dev: *mut device, regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt1316: *mut rt1316_sdw_priv;
    let ret: c_int;

    rt1316 = devm_kzalloc(dev, core::mem::size_of::<rt1316_sdw_priv>(), GFP_KERNEL) as *mut rt1316_sdw_priv;
    if rt1316.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, rt1316 as *mut c_void);
    (*rt1316).sdw_slave = slave;
    (*rt1316).regmap = regmap;

    regcache_cache_only((*rt1316).regmap, true);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt1316).hw_init = false;
    (*rt1316).first_hw_init = false;

    ret = devm_snd_soc_register_component(
        dev,
        &soc_component_sdw_rt1316,
        rt1316_sdw_dai.as_mut_ptr(),
        rt1316_sdw_dai.len(),
    );
    if ret < 0 {
        return ret;
    }

    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);

    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);

    pm_runtime_enable(dev);

    /* important note: the device is NOT tagged as 'active' and will remain
     * 'suspended' until the hardware is enumerated/initialized. This is required
     * to make sure the ASoC framework use of pm_runtime_get_sync() does not silently
     * fail with -EACCESS because of race conditions between card creation and enumeration
     */

    dev_dbg(dev, c_str!("%s\n"), __func__);

    0
}

unsafe fn rt1316_sdw_probe(slave: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw(slave, &rt1316_sdw_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt1316_sdw_init(&mut (*slave).dev, regmap, slave)
}

unsafe fn rt1316_sdw_remove(slave: *mut sdw_slave) {
    pm_runtime_disable(&mut (*slave).dev);
}

static rt1316_id: [sdw_device_id; 2] = [
    SDW_SLAVE_ENTRY_EXT(0x025d, 0x1316, 0x3, 0x1, 0),
    sdw_device_id {},
];
MODULE_DEVICE_TABLE!(sdw, rt1316_id);

unsafe fn rt1316_dev_suspend(dev: *mut device) -> c_int {
    let rt1316: *mut rt1316_sdw_priv = dev_get_drvdata(dev) as *mut rt1316_sdw_priv;

    if !(*rt1316).hw_init {
        return 0;
    }

    regcache_cache_only((*rt1316).regmap, true);

    0
}

const RT1316_PROBE_TIMEOUT: c_int = 5000;

unsafe fn rt1316_dev_resume(dev: *mut device) -> c_int {
    let slave: *mut sdw_slave = dev_to_sdw_dev(dev);
    let rt1316: *mut rt1316_sdw_priv = dev_get_drvdata(dev) as *mut rt1316_sdw_priv;
    let mut ret: c_int;

    if !(*rt1316).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, RT1316_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt1316).regmap, false);
    ret = regcache_sync((*rt1316).regmap);
    if ret != 0 {
        regcache_cache_only((*rt1316).regmap, true);
        regcache_mark_dirty((*rt1316).regmap);
        return ret;
    }

    0
}

static rt1316_pm: dev_pm_ops = dev_pm_ops {
    /* SYSTEM_SLEEP_PM_OPS(rt1316_dev_suspend, rt1316_dev_resume) */
    suspend: Some(rt1316_dev_suspend),
    resume: Some(rt1316_dev_resume),
    /* RUNTIME_PM_OPS(rt1316_dev_suspend, rt1316_dev_resume, NULL) */
    runtime_suspend: Some(rt1316_dev_suspend),
    runtime_resume: Some(rt1316_dev_resume),
    runtime_idle: None,
};

static mut rt1316_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: c_str!("rt1316-sdca"),
        pm: pm_ptr(&rt1316_pm),
    },
    probe: Some(rt1316_sdw_probe),
    remove: Some(rt1316_sdw_remove),
    ops: &rt1316_slave_ops,
    id_table: rt1316_id.as_ptr(),
};
module_sdw_driver!(rt1316_sdw_driver);

MODULE_DESCRIPTION!("ASoC RT1316 driver SDCA SDW");
MODULE_AUTHOR!("Shuming Fan <shumingf@realtek.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
