// SPDX-License-Identifier: GPL-2.0-only
//
// tegra210_peq.c - Tegra210 PEQ driver
//
// Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.

// C dependencies:
// linux/clk.h, linux/device.h, linux/io.h, linux/module.h, linux/of.h,
// linux/of_address.h, linux/platform_device.h, linux/pm_runtime.h,
// linux/regmap.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, tegra210_ope.h, tegra210_peq.h

static tegra210_peq_reg_defaults: [reg_default; 3] = [
    reg_default {
        reg: TEGRA210_PEQ_CFG,
        def: 0x00000013,
    },
    reg_default {
        reg: TEGRA210_PEQ_CFG_RAM_CTRL,
        def: 0x00004000,
    },
    reg_default {
        reg: TEGRA210_PEQ_CFG_RAM_SHIFT_CTRL,
        def: 0x00004000,
    },
];

static biquad_init_gains: [u32; TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH] = [
    1495012349, /* Pre-gain */

    /* Gains : b0, b1, a0, a1, a2 */
    536870912, (-1073741824i32) as u32, 536870912, 2143508246, (-1069773768i32) as u32, /* Band-0 */
    134217728, (-265414508i32) as u32, 131766272, 2140402222, (-1071252997i32) as u32,  /* Band-1 */
    268435456, (-233515765i32) as u32, (-33935948i32) as u32, 1839817267, (-773826124i32) as u32,   /* Band-2 */
    536870912, (-672537913i32) as u32, 139851540, 1886437554, (-824433167i32) as u32,   /* Band-3 */
    268435456, (-114439279i32) as u32, 173723964, 205743566, 278809729,     /* Band-4 */
    1, 0, 0, 0, 0, /* Band-5 */
    1, 0, 0, 0, 0, /* Band-6 */
    1, 0, 0, 0, 0, /* Band-7 */
    1, 0, 0, 0, 0, /* Band-8 */
    1, 0, 0, 0, 0, /* Band-9 */
    1, 0, 0, 0, 0, /* Band-10 */
    1, 0, 0, 0, 0, /* Band-11 */

    963423114, /* Post-gain */
];

static biquad_init_shifts: [u32; TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH] = [
    23, /* Pre-shift */
    30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, /* Shift for bands */
    28, /* Post-shift */
];

static mut biquad_coeff_buffer: [s32; TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH] =
    [0; TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH];

unsafe fn tegra210_peq_read_ram(
    regmap: *mut regmap,
    reg_ctrl: c_uint,
    reg_data: c_uint,
    ram_offset: c_uint,
    data: *mut c_uint,
    size: size_t,
) {
    let mut val: c_uint;
    let mut i: c_uint;

    val = ram_offset & TEGRA210_PEQ_RAM_CTRL_RAM_ADDR_MASK;
    val |= TEGRA210_PEQ_RAM_CTRL_ADDR_INIT_EN;
    val |= TEGRA210_PEQ_RAM_CTRL_SEQ_ACCESS_EN;
    val |= TEGRA210_PEQ_RAM_CTRL_RW_READ;

    regmap_write(regmap, reg_ctrl, val);

    /*
     * Since all ahub non-io modules work under same ahub clock it is not
     * necessary to check ahub read busy bit after every read.
     */
    i = 0;
    while (i as size_t) < size {
        regmap_read(regmap, reg_data, data.add(i as usize));
        i += 1;
    }
}

unsafe fn tegra210_peq_write_ram(
    regmap: *mut regmap,
    reg_ctrl: c_uint,
    reg_data: c_uint,
    ram_offset: c_uint,
    data: *mut c_uint,
    size: size_t,
) {
    let mut val: c_uint;
    let mut i: c_uint;

    val = ram_offset & TEGRA210_PEQ_RAM_CTRL_RAM_ADDR_MASK;
    val |= TEGRA210_PEQ_RAM_CTRL_ADDR_INIT_EN;
    val |= TEGRA210_PEQ_RAM_CTRL_SEQ_ACCESS_EN;
    val |= TEGRA210_PEQ_RAM_CTRL_RW_WRITE;

    regmap_write(regmap, reg_ctrl, val);

    i = 0;
    while (i as size_t) < size {
        regmap_write(regmap, reg_data, *data.add(i as usize));
        i += 1;
    }
}

unsafe fn tegra210_peq_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let ope: *mut tegra210_ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let mask: c_uint = (1 << fls((*mc).max)) - 1;
    let mut val: c_uint = 0;

    regmap_read((*ope).peq_regmap, (*mc).reg, &mut val);

    (*ucontrol).value.integer.value[0] = ((val >> (*mc).shift) & mask) as c_long;

    if !(*mc).invert {
        return 0;
    }

    (*ucontrol).value.integer.value[0] =
        ((*mc).max as c_long) - (*ucontrol).value.integer.value[0];

    0
}

unsafe fn tegra210_peq_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let ope: *mut tegra210_ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let mask: c_uint = (1 << fls((*mc).max)) - 1;
    let mut change: bool = false;
    let mut val: c_uint;

    val = ((*ucontrol).value.integer.value[0] as c_uint) & mask;

    if (*mc).invert {
        val = (*mc).max - val;
    }

    val = val << (*mc).shift;

    regmap_update_bits_check(
        (*ope).peq_regmap,
        (*mc).reg,
        mask << (*mc).shift,
        val,
        &mut change,
    );

    if change { 1 } else { 0 }
}

unsafe fn tegra210_peq_ram_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let params: *mut tegra_soc_bytes = (*kcontrol).private_value as *mut tegra_soc_bytes;
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let ope: *mut tegra210_ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let val_bytes: c_int = snd_soc_component_regmap_val_bytes(cmpnt);
    let mut i: u32;
    let reg_ctrl: u32 = (*params).soc.base;
    let reg_data: u32 = reg_ctrl + val_bytes as u32;
    let data: *mut s32 = biquad_coeff_buffer.as_mut_ptr();

    pm_runtime_get_sync((*cmpnt).dev);

    tegra210_peq_read_ram(
        (*ope).peq_regmap,
        reg_ctrl,
        reg_data,
        (*params).shift,
        data as *mut c_uint,
        (*params).soc.num_regs as size_t,
    );

    pm_runtime_put_sync((*cmpnt).dev);

    i = 0;
    while i < (*params).soc.num_regs {
        (*ucontrol).value.integer.value[i as usize] = *data.add(i as usize) as c_long;
        i += 1;
    }

    0
}

unsafe fn tegra210_peq_ram_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let params: *mut tegra_soc_bytes = (*kcontrol).private_value as *mut tegra_soc_bytes;
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let ope: *mut tegra210_ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let val_bytes: c_int = snd_soc_component_regmap_val_bytes(cmpnt);
    let mut i: u32;
    let reg_ctrl: u32 = (*params).soc.base;
    let reg_data: u32 = reg_ctrl + val_bytes as u32;
    let data: *mut s32 = biquad_coeff_buffer.as_mut_ptr();

    i = 0;
    while i < (*params).soc.num_regs {
        *data.add(i as usize) = (*ucontrol).value.integer.value[i as usize] as s32;
        i += 1;
    }

    pm_runtime_get_sync((*cmpnt).dev);

    tegra210_peq_write_ram(
        (*ope).peq_regmap,
        reg_ctrl,
        reg_data,
        (*params).shift,
        data as *mut c_uint,
        (*params).soc.num_regs as size_t,
    );

    pm_runtime_put_sync((*cmpnt).dev);

    1
}

unsafe fn tegra210_peq_param_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let params: *mut soc_bytes = (*kcontrol).private_value as *mut soc_bytes;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).value.integer.min = INT_MIN;
    (*uinfo).value.integer.max = INT_MAX;
    (*uinfo).count = (*params).num_regs;

    0
}

macro_rules! TEGRA210_PEQ_GAIN_PARAMS_CTRL {
    ($chan:expr) => {
        TEGRA_SOC_BYTES_EXT!(
            concat!("PEQ Channel-", stringify!($chan), " Biquad Gain Params"),
            TEGRA210_PEQ_CFG_RAM_CTRL,
            TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH,
            (TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH * $chan),
            0xffffffff,
            tegra210_peq_ram_get,
            tegra210_peq_ram_put,
            tegra210_peq_param_info
        )
    };
}

macro_rules! TEGRA210_PEQ_SHIFT_PARAMS_CTRL {
    ($chan:expr) => {
        TEGRA_SOC_BYTES_EXT!(
            concat!("PEQ Channel-", stringify!($chan), " Biquad Shift Params"),
            TEGRA210_PEQ_CFG_RAM_SHIFT_CTRL,
            TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH,
            (TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH * $chan),
            0x1f,
            tegra210_peq_ram_get,
            tegra210_peq_ram_put,
            tegra210_peq_param_info
        )
    };
}

static tegra210_peq_controls: [snd_kcontrol_new; 18] = [
    SOC_SINGLE_EXT!(
        "PEQ Active",
        TEGRA210_PEQ_CFG,
        TEGRA210_PEQ_CFG_MODE_SHIFT,
        1,
        0,
        tegra210_peq_get,
        tegra210_peq_put
    ),

    SOC_SINGLE_EXT!(
        "PEQ Biquad Stages",
        TEGRA210_PEQ_CFG,
        TEGRA210_PEQ_CFG_BIQUAD_STAGES_SHIFT,
        TEGRA210_PEQ_MAX_BIQUAD_STAGES - 1,
        0,
        tegra210_peq_get,
        tegra210_peq_put
    ),

    TEGRA210_PEQ_GAIN_PARAMS_CTRL!(0),
    TEGRA210_PEQ_GAIN_PARAMS_CTRL!(1),
    TEGRA210_PEQ_GAIN_PARAMS_CTRL!(2),
    TEGRA210_PEQ_GAIN_PARAMS_CTRL!(3),
    TEGRA210_PEQ_GAIN_PARAMS_CTRL!(4),
    TEGRA210_PEQ_GAIN_PARAMS_CTRL!(5),
    TEGRA210_PEQ_GAIN_PARAMS_CTRL!(6),
    TEGRA210_PEQ_GAIN_PARAMS_CTRL!(7),

    TEGRA210_PEQ_SHIFT_PARAMS_CTRL!(0),
    TEGRA210_PEQ_SHIFT_PARAMS_CTRL!(1),
    TEGRA210_PEQ_SHIFT_PARAMS_CTRL!(2),
    TEGRA210_PEQ_SHIFT_PARAMS_CTRL!(3),
    TEGRA210_PEQ_SHIFT_PARAMS_CTRL!(4),
    TEGRA210_PEQ_SHIFT_PARAMS_CTRL!(5),
    TEGRA210_PEQ_SHIFT_PARAMS_CTRL!(6),
    TEGRA210_PEQ_SHIFT_PARAMS_CTRL!(7),
];

unsafe fn tegra210_peq_wr_reg(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA210_PEQ_SOFT_RESET | TEGRA210_PEQ_CG => true,
        TEGRA210_PEQ_CFG..=TEGRA210_PEQ_CFG_RAM_SHIFT_DATA => true,
        _ => false,
    }
}

unsafe fn tegra210_peq_rd_reg(dev: *mut device, reg: c_uint) -> bool {
    if tegra210_peq_wr_reg(dev, reg) {
        return true;
    }

    match reg {
        TEGRA210_PEQ_STATUS => true,
        _ => false,
    }
}

unsafe fn tegra210_peq_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA210_PEQ_SOFT_RESET | TEGRA210_PEQ_STATUS => true,
        TEGRA210_PEQ_CFG_RAM_CTRL..=TEGRA210_PEQ_CFG_RAM_SHIFT_DATA => true,
        _ => false,
    }
}

unsafe fn tegra210_peq_precious_reg(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA210_PEQ_CFG_RAM_DATA | TEGRA210_PEQ_CFG_RAM_SHIFT_DATA => true,
        _ => false,
    }
}

static tegra210_peq_regmap_config: regmap_config = regmap_config {
    name: b"peq\0".as_ptr() as *const c_char,
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA210_PEQ_CFG_RAM_SHIFT_DATA,
    writeable_reg: Some(tegra210_peq_wr_reg),
    readable_reg: Some(tegra210_peq_rd_reg),
    volatile_reg: Some(tegra210_peq_volatile_reg),
    precious_reg: Some(tegra210_peq_precious_reg),
    reg_defaults: tegra210_peq_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(tegra210_peq_reg_defaults),
    reg_default_cb: Some(regmap_default_zero_cb),
    cache_type: REGCACHE_FLAT,
};

pub unsafe fn tegra210_peq_restore(
    regmap: *mut regmap,
    biquad_gains: *mut u32,
    biquad_shifts: *mut u32,
) {
    let mut i: c_uint;

    i = 0;
    while i < TEGRA210_PEQ_MAX_CHANNELS {
        tegra210_peq_write_ram(
            regmap,
            TEGRA210_PEQ_CFG_RAM_CTRL,
            TEGRA210_PEQ_CFG_RAM_DATA,
            i * TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH,
            biquad_gains,
            TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH as size_t,
        );

        tegra210_peq_write_ram(
            regmap,
            TEGRA210_PEQ_CFG_RAM_SHIFT_CTRL,
            TEGRA210_PEQ_CFG_RAM_SHIFT_DATA,
            i * TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH,
            biquad_shifts,
            TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH as size_t,
        );

        i += 1;
    }
}

pub unsafe fn tegra210_peq_save(
    regmap: *mut regmap,
    biquad_gains: *mut u32,
    biquad_shifts: *mut u32,
) {
    let mut i: c_uint;

    i = 0;
    while i < TEGRA210_PEQ_MAX_CHANNELS {
        tegra210_peq_read_ram(
            regmap,
            TEGRA210_PEQ_CFG_RAM_CTRL,
            TEGRA210_PEQ_CFG_RAM_DATA,
            i * TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH,
            biquad_gains,
            TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH as size_t,
        );

        tegra210_peq_read_ram(
            regmap,
            TEGRA210_PEQ_CFG_RAM_SHIFT_CTRL,
            TEGRA210_PEQ_CFG_RAM_SHIFT_DATA,
            i * TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH,
            biquad_shifts,
            TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH as size_t,
        );

        i += 1;
    }
}

pub unsafe fn tegra210_peq_component_init(cmpnt: *mut snd_soc_component) -> c_int {
    let ope: *mut tegra210_ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let mut i: c_uint;

    pm_runtime_get_sync((*cmpnt).dev);
    regmap_update_bits(
        (*ope).peq_regmap,
        TEGRA210_PEQ_CFG,
        TEGRA210_PEQ_CFG_MODE_MASK,
        0 << TEGRA210_PEQ_CFG_MODE_SHIFT,
    );
    regmap_update_bits(
        (*ope).peq_regmap,
        TEGRA210_PEQ_CFG,
        TEGRA210_PEQ_CFG_BIQUAD_STAGES_MASK,
        (TEGRA210_PEQ_BIQUAD_INIT_STAGE - 1) << TEGRA210_PEQ_CFG_BIQUAD_STAGES_SHIFT,
    );

    /* Initialize PEQ AHUB RAM with default params */
    i = 0;
    while i < TEGRA210_PEQ_MAX_CHANNELS {
        /* Set default gain params */
        tegra210_peq_write_ram(
            (*ope).peq_regmap,
            TEGRA210_PEQ_CFG_RAM_CTRL,
            TEGRA210_PEQ_CFG_RAM_DATA,
            i * TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH,
            biquad_init_gains.as_ptr() as *mut u32,
            TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH as size_t,
        );

        /* Set default shift params */
        tegra210_peq_write_ram(
            (*ope).peq_regmap,
            TEGRA210_PEQ_CFG_RAM_SHIFT_CTRL,
            TEGRA210_PEQ_CFG_RAM_SHIFT_DATA,
            i * TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH,
            biquad_init_shifts.as_ptr() as *mut u32,
            TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH as size_t,
        );

        i += 1;
    }

    pm_runtime_put_sync((*cmpnt).dev);

    snd_soc_add_component_controls(
        cmpnt,
        tegra210_peq_controls.as_ptr(),
        ARRAY_SIZE!(tegra210_peq_controls),
    );

    0
}

pub unsafe fn tegra210_peq_regmap_init(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let ope: *mut tegra210_ope = dev_get_drvdata(dev) as *mut tegra210_ope;
    let mut child: *mut device_node;
    let mut mem: resource = core::mem::zeroed();
    let mut regs: *mut c_void;
    let mut err: c_int;

    child = of_get_child_by_name((*dev).of_node, b"equalizer\0".as_ptr() as *const c_char);
    if child.is_null() {
        return dev_err_probe(
            dev,
            -ENODEV,
            b"missing 'equalizer' DT child node\n\0".as_ptr() as *const c_char,
        );
    }

    err = of_address_to_resource(child, 0, &mut mem);
    of_node_put(child);
    if err < 0 {
        return dev_err_probe(
            dev,
            err,
            b"failed to get PEQ resource\n\0".as_ptr() as *const c_char,
        );
    }

    mem.flags = IORESOURCE_MEM;
    regs = devm_ioremap_resource(dev, &mut mem);
    if IS_ERR(regs) {
        return PTR_ERR(regs) as c_int;
    }
    (*ope).peq_regmap = devm_regmap_init_mmio(dev, regs, &tegra210_peq_regmap_config);
    if IS_ERR((*ope).peq_regmap as *mut c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*ope).peq_regmap as *mut c_void) as c_int,
            b"PEQ regmap init failed\n\0".as_ptr() as *const c_char,
        );
    }

    regcache_cache_only((*ope).peq_regmap, true);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
