// SPDX-License-Identifier: GPL-2.0
/*
 * NXP AUDMIX ALSA SoC Digital Audio Interface (DAI) driver
 *
 * Copyright 2017 NXP
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type bool_t = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_uint = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0x0004;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x4000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x1000;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0020;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const FSL_AUDMIX_FORMATS: u64 = 0;

const FSL_AUDMIX_CTR: c_uint = 0;
const FSL_AUDMIX_STR: c_uint = 4;
const FSL_AUDMIX_ATCR0: c_uint = 8;
const FSL_AUDMIX_ATIVAL0: c_uint = 12;
const FSL_AUDMIX_ATSTPUP0: c_uint = 16;
const FSL_AUDMIX_ATSTPDN0: c_uint = 20;
const FSL_AUDMIX_ATSTPTGT0: c_uint = 24;
const FSL_AUDMIX_ATTNVAL0: c_uint = 28;
const FSL_AUDMIX_ATSTP0: c_uint = 32;
const FSL_AUDMIX_ATCR1: c_uint = 36;
const FSL_AUDMIX_ATIVAL1: c_uint = 40;
const FSL_AUDMIX_ATSTPUP1: c_uint = 44;
const FSL_AUDMIX_ATSTPDN1: c_uint = 48;
const FSL_AUDMIX_ATSTPTGT1: c_uint = 52;
const FSL_AUDMIX_ATTNVAL1: c_uint = 56;
const FSL_AUDMIX_ATSTP1: c_uint = 60;

const FSL_AUDMIX_CTR_MIXCLK_SHIFT: c_uint = 0;
const FSL_AUDMIX_CTR_OUTSRC_SHIFT: c_uint = 2;
const FSL_AUDMIX_CTR_OUTWIDTH_SHIFT: c_uint = 4;
const FSL_AUDMIX_CTR_MASKRTDF_SHIFT: c_uint = 8;
const FSL_AUDMIX_CTR_MASKCKDF_SHIFT: c_uint = 9;
const FSL_AUDMIX_CTR_SYNCMODE_SHIFT: c_uint = 10;
const FSL_AUDMIX_CTR_SYNCSRC_SHIFT: c_uint = 11;
const FSL_AUDMIX_CTR_MIXCLK_MASK: c_uint = 0x3 << FSL_AUDMIX_CTR_MIXCLK_SHIFT;
const FSL_AUDMIX_CTR_OUTSRC_MASK: c_uint = 0x3 << FSL_AUDMIX_CTR_OUTSRC_SHIFT;
const FSL_AUDMIX_CTR_OUTCKPOL_MASK: c_uint = 0x1 << 7;

const fn FSL_AUDMIX_CTR_MIXCLK(x: u8) -> c_uint {
    ((x as c_uint) << FSL_AUDMIX_CTR_MIXCLK_SHIFT) & FSL_AUDMIX_CTR_MIXCLK_MASK
}

const fn FSL_AUDMIX_CTR_OUTSRC(x: c_uint) -> c_uint {
    (x << FSL_AUDMIX_CTR_OUTSRC_SHIFT) & FSL_AUDMIX_CTR_OUTSRC_MASK
}

const fn FSL_AUDMIX_CTR_OUTCKPOL(x: c_uint) -> c_uint {
    (x << 7) & FSL_AUDMIX_CTR_OUTCKPOL_MASK
}

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_kcontrol {
    private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 4],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_pcm_substream {
    stream: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver_id {
    id: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
    driver: *mut snd_soc_dai_driver_id,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsl_audmix {
    tdms: u8,
    lock: spinlock_t,
    regmap: *mut regmap,
    ipg_clk: *mut clk,
    pdev: *mut platform_device,
}

#[repr(C)]
pub struct soc_enum {
    reg: c_uint,
    shift_l: c_uint,
    items: c_uint,
    texts: *const *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    name: *const c_char,
    private_value: usize,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    id: c_uint,
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const c_char,
    controls: *const snd_kcontrol_new,
    num_controls: usize,
}

#[repr(C)]
pub struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    reg_bits: c_uint,
    reg_stride: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: usize,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    cache_type: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct platform_driver_inner {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: platform_driver_inner,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(comp: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(comp: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_update_bits(
        comp: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool_t;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool_t;
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: usize,
    ) -> c_int;
    fn of_find_property(np: *mut device_node, name: *const c_char, lenp: *mut c_int) -> *mut c_void;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: usize,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
}

const tdm_sel: [*const c_char; 2] = [c"TDM1".as_ptr(), c"TDM2".as_ptr()];
const mode_sel: [*const c_char; 4] = [
    c"Disabled".as_ptr(),
    c"TDM1".as_ptr(),
    c"TDM2".as_ptr(),
    c"Mixed".as_ptr(),
];
const width_sel: [*const c_char; 5] = [
    c"16b".as_ptr(),
    c"18b".as_ptr(),
    c"20b".as_ptr(),
    c"24b".as_ptr(),
    c"32b".as_ptr(),
];
const endis_sel: [*const c_char; 2] = [c"Disabled".as_ptr(), c"Enabled".as_ptr()];
const updn_sel: [*const c_char; 2] = [c"Downward".as_ptr(), c"Upward".as_ptr()];
const mask_sel: [*const c_char; 2] = [c"Unmask".as_ptr(), c"Mask".as_ptr()];

const fn SOC_ENUM_SINGLE_S<const N: usize>(
    xreg: c_uint,
    xshift: c_uint,
    xtexts: &[*const c_char; N],
) -> soc_enum {
    soc_enum {
        reg: xreg,
        shift_l: xshift,
        items: N as c_uint,
        texts: xtexts.as_ptr(),
    }
}

static fsl_audmix_enum: [soc_enum; 11] = [
    /* FSL_AUDMIX_CTR enums */
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_CTR, FSL_AUDMIX_CTR_MIXCLK_SHIFT, &tdm_sel),
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_CTR, FSL_AUDMIX_CTR_OUTSRC_SHIFT, &mode_sel),
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_CTR, FSL_AUDMIX_CTR_OUTWIDTH_SHIFT, &width_sel),
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_CTR, FSL_AUDMIX_CTR_MASKRTDF_SHIFT, &mask_sel),
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_CTR, FSL_AUDMIX_CTR_MASKCKDF_SHIFT, &mask_sel),
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_CTR, FSL_AUDMIX_CTR_SYNCMODE_SHIFT, &endis_sel),
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_CTR, FSL_AUDMIX_CTR_SYNCSRC_SHIFT, &tdm_sel),
    /* FSL_AUDMIX_ATCR0 enums */
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_ATCR0, 0, &endis_sel),
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_ATCR0, 1, &updn_sel),
    /* FSL_AUDMIX_ATCR1 enums */
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_ATCR1, 0, &endis_sel),
    SOC_ENUM_SINGLE_S(FSL_AUDMIX_ATCR1, 1, &updn_sel),
];

#[repr(C)]
#[derive(Copy, Clone)]
struct fsl_audmix_state {
    tdms: u8,
    clk: u8,
    msg: [c_char; 64],
}

const fn msg64(bytes: &[u8]) -> [c_char; 64] {
    let mut out = [0 as c_char; 64];
    let mut i = 0;
    while i < bytes.len() && i < 63 {
        out[i] = bytes[i] as c_char;
        i += 1;
    }
    out
}

static prms: [[fsl_audmix_state; 4]; 4] = [
    [
        /* DIS->DIS, do nothing */
        fsl_audmix_state { tdms: 0, clk: 0, msg: msg64(b"") },
        /* DIS->TDM1*/
        fsl_audmix_state { tdms: 1, clk: 1, msg: msg64(b"DIS->TDM1: TDM1 not started!\n") },
        /* DIS->TDM2*/
        fsl_audmix_state { tdms: 2, clk: 2, msg: msg64(b"DIS->TDM2: TDM2 not started!\n") },
        /* DIS->MIX */
        fsl_audmix_state { tdms: 3, clk: 0, msg: msg64(b"DIS->MIX: Please start both TDMs!\n") },
    ],
    [
        /* TDM1->DIS */
        fsl_audmix_state { tdms: 1, clk: 0, msg: msg64(b"TDM1->DIS: TDM1 not started!\n") },
        /* TDM1->TDM1, do nothing */
        fsl_audmix_state { tdms: 0, clk: 0, msg: msg64(b"") },
        /* TDM1->TDM2 */
        fsl_audmix_state { tdms: 3, clk: 2, msg: msg64(b"TDM1->TDM2: Please start both TDMs!\n") },
        /* TDM1->MIX */
        fsl_audmix_state { tdms: 3, clk: 0, msg: msg64(b"TDM1->MIX: Please start both TDMs!\n") },
    ],
    [
        /* TDM2->DIS */
        fsl_audmix_state { tdms: 2, clk: 0, msg: msg64(b"TDM2->DIS: TDM2 not started!\n") },
        /* TDM2->TDM1 */
        fsl_audmix_state { tdms: 3, clk: 1, msg: msg64(b"TDM2->TDM1: Please start both TDMs!\n") },
        /* TDM2->TDM2, do nothing */
        fsl_audmix_state { tdms: 0, clk: 0, msg: msg64(b"") },
        /* TDM2->MIX */
        fsl_audmix_state { tdms: 3, clk: 0, msg: msg64(b"TDM2->MIX: Please start both TDMs!\n") },
    ],
    [
        /* MIX->DIS */
        fsl_audmix_state { tdms: 3, clk: 0, msg: msg64(b"MIX->DIS: Please start both TDMs!\n") },
        /* MIX->TDM1 */
        fsl_audmix_state { tdms: 3, clk: 1, msg: msg64(b"MIX->TDM1: Please start both TDMs!\n") },
        /* MIX->TDM2 */
        fsl_audmix_state { tdms: 3, clk: 2, msg: msg64(b"MIX->TDM2: Please start both TDMs!\n") },
        /* MIX->MIX, do nothing */
        fsl_audmix_state { tdms: 0, clk: 0, msg: msg64(b"") },
    ],
];

unsafe extern "C" fn fsl_audmix_state_trans(
    comp: *mut snd_soc_component,
    mask: *mut c_uint,
    ctr: *mut c_uint,
    prm: fsl_audmix_state,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(comp) as *mut fsl_audmix;
    /* Enforce all required TDMs are started */
    if ((*priv_).tdms & prm.tdms) != prm.tdms {
        dev_dbg((*comp).dev, c"%s".as_ptr(), prm.msg.as_ptr());
        return -EINVAL;
    }

    match prm.clk {
        1 | 2 => {
            /* Set mix clock */
            *mask |= FSL_AUDMIX_CTR_MIXCLK_MASK;
            *ctr |= FSL_AUDMIX_CTR_MIXCLK(prm.clk - 1);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn fsl_audmix_put_mix_clk_src(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(comp) as *mut fsl_audmix;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let reg_val: c_uint;
    let val: c_uint;
    let mix_clk: c_uint;

    if *item.add(0) >= (*e).items {
        return -EINVAL;
    }

    /* Get current state */
    reg_val = snd_soc_component_read(comp, FSL_AUDMIX_CTR);
    mix_clk = (reg_val & FSL_AUDMIX_CTR_MIXCLK_MASK) >> FSL_AUDMIX_CTR_MIXCLK_SHIFT;
    val = snd_soc_enum_item_to_val(e, *item.add(0));

    dev_dbg((*comp).dev, c"TDMs=x%08x, val=x%08x\n".as_ptr(), (*priv_).tdms as c_uint, val);

    /*
     * Ensure the current selected mixer clock is available
     * for configuration propagation
     */
    if ((*priv_).tdms & BIT(mix_clk)) == 0 {
        dev_err(
            (*comp).dev,
            c"Started TDM%d needed for config propagation!\n".as_ptr(),
            mix_clk + 1,
        );
        return -EINVAL;
    }

    if ((*priv_).tdms & BIT(val)) == 0 {
        dev_err(
            (*comp).dev,
            c"The selected clock source has no TDM%d enabled!\n".as_ptr(),
            val + 1,
        );
        return -EINVAL;
    }

    snd_soc_put_enum_double(kcontrol, ucontrol)
}

unsafe extern "C" fn fsl_audmix_put_out_src(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(comp) as *mut fsl_audmix;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let out_src: u32;
    let mix_clk: u32;
    let reg_val: c_uint;
    let val: c_uint;
    let mut mask: c_uint = 0;
    let mut ctr: c_uint = 0;
    let ret: c_int;

    if *item.add(0) >= (*e).items {
        return -EINVAL;
    }

    /* Get current state */
    reg_val = snd_soc_component_read(comp, FSL_AUDMIX_CTR);

    /* "From" state */
    out_src = (reg_val & FSL_AUDMIX_CTR_OUTSRC_MASK) >> FSL_AUDMIX_CTR_OUTSRC_SHIFT;
    mix_clk = (reg_val & FSL_AUDMIX_CTR_MIXCLK_MASK) >> FSL_AUDMIX_CTR_MIXCLK_SHIFT;

    /* "To" state */
    val = snd_soc_enum_item_to_val(e, *item.add(0));

    dev_dbg((*comp).dev, c"TDMs=x%08x, val=x%08x\n".as_ptr(), (*priv_).tdms as c_uint, val);

    /* Check if state is changing ... */
    if out_src == val {
        return 0;
    }
    /*
     * Ensure the current selected mixer clock is available
     * for configuration propagation
     */
    if ((*priv_).tdms & BIT(mix_clk)) == 0 {
        dev_err(
            (*comp).dev,
            c"Started TDM%d needed for config propagation!\n".as_ptr(),
            mix_clk + 1,
        );
        return -EINVAL;
    }

    /* Check state transition constraints */
    ret = fsl_audmix_state_trans(comp, &mut mask, &mut ctr, prms[out_src as usize][val as usize]);
    if ret != 0 {
        return ret;
    }

    /* Complete transition to new state */
    mask |= FSL_AUDMIX_CTR_OUTSRC_MASK;
    ctr |= FSL_AUDMIX_CTR_OUTSRC(val);

    snd_soc_component_update_bits(comp, FSL_AUDMIX_CTR, mask, ctr)
}

const fn SOC_ENUM_EXT(
    name: *const c_char,
    e: &'static soc_enum,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
) -> snd_kcontrol_new {
    snd_kcontrol_new { name, private_value: e as *const soc_enum as usize, get, put }
}

const fn SOC_ENUM(name: *const c_char, e: &'static soc_enum) -> snd_kcontrol_new {
    snd_kcontrol_new { name, private_value: e as *const soc_enum as usize, get: None, put: None }
}

const fn SOC_SINGLE(
    name: *const c_char,
    reg: c_uint,
    shift: c_uint,
    max: c_uint,
    invert: c_uint,
) -> snd_kcontrol_new {
    let _ = (reg, shift, max, invert);
    snd_kcontrol_new { name, private_value: 0, get: None, put: None }
}

static fsl_audmix_snd_controls: [snd_kcontrol_new; 21] = [
    /* FSL_AUDMIX_CTR controls */
    SOC_ENUM_EXT(c"Mixing Clock Source".as_ptr(), &fsl_audmix_enum[0], Some(snd_soc_get_enum_double), Some(fsl_audmix_put_mix_clk_src)),
    SOC_ENUM_EXT(c"Output Source".as_ptr(), &fsl_audmix_enum[1], Some(snd_soc_get_enum_double), Some(fsl_audmix_put_out_src)),
    SOC_ENUM(c"Output Width".as_ptr(), &fsl_audmix_enum[2]),
    SOC_ENUM(c"Frame Rate Diff Error".as_ptr(), &fsl_audmix_enum[3]),
    SOC_ENUM(c"Clock Freq Diff Error".as_ptr(), &fsl_audmix_enum[4]),
    SOC_ENUM(c"Sync Mode Config".as_ptr(), &fsl_audmix_enum[5]),
    SOC_ENUM(c"Sync Mode Clk Source".as_ptr(), &fsl_audmix_enum[6]),
    /* TDM1 Attenuation controls */
    SOC_ENUM(c"TDM1 Attenuation".as_ptr(), &fsl_audmix_enum[7]),
    SOC_ENUM(c"TDM1 Attenuation Direction".as_ptr(), &fsl_audmix_enum[8]),
    SOC_SINGLE(c"TDM1 Attenuation Step Divider".as_ptr(), FSL_AUDMIX_ATCR0, 2, 0x00fff, 0),
    SOC_SINGLE(c"TDM1 Attenuation Initial Value".as_ptr(), FSL_AUDMIX_ATIVAL0, 0, 0x3ffff, 0),
    SOC_SINGLE(c"TDM1 Attenuation Step Up Factor".as_ptr(), FSL_AUDMIX_ATSTPUP0, 0, 0x3ffff, 0),
    SOC_SINGLE(c"TDM1 Attenuation Step Down Factor".as_ptr(), FSL_AUDMIX_ATSTPDN0, 0, 0x3ffff, 0),
    SOC_SINGLE(c"TDM1 Attenuation Step Target".as_ptr(), FSL_AUDMIX_ATSTPTGT0, 0, 0x3ffff, 0),
    /* TDM2 Attenuation controls */
    SOC_ENUM(c"TDM2 Attenuation".as_ptr(), &fsl_audmix_enum[9]),
    SOC_ENUM(c"TDM2 Attenuation Direction".as_ptr(), &fsl_audmix_enum[10]),
    SOC_SINGLE(c"TDM2 Attenuation Step Divider".as_ptr(), FSL_AUDMIX_ATCR1, 2, 0x00fff, 0),
    SOC_SINGLE(c"TDM2 Attenuation Initial Value".as_ptr(), FSL_AUDMIX_ATIVAL1, 0, 0x3ffff, 0),
    SOC_SINGLE(c"TDM2 Attenuation Step Up Factor".as_ptr(), FSL_AUDMIX_ATSTPUP1, 0, 0x3ffff, 0),
    SOC_SINGLE(c"TDM2 Attenuation Step Down Factor".as_ptr(), FSL_AUDMIX_ATSTPDN1, 0, 0x3ffff, 0),
    SOC_SINGLE(c"TDM2 Attenuation Step Target".as_ptr(), FSL_AUDMIX_ATSTPTGT1, 0, 0x3ffff, 0),
];

unsafe extern "C" fn fsl_audmix_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let comp = (*dai).component;
    let mut mask: u32 = 0;
    let mut ctr: u32 = 0;

    /* AUDMIX is working in DSP_A format only */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {}
        _ => return -EINVAL,
    }

    /* For playback the AUDMIX is consumer, and for record is provider */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_BP_FP => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_NF => {
            /* Output data will be written on positive edge of the clock */
            ctr |= FSL_AUDMIX_CTR_OUTCKPOL(0);
        }
        SND_SOC_DAIFMT_NB_NF => {
            /* Output data will be written on negative edge of the clock */
            ctr |= FSL_AUDMIX_CTR_OUTCKPOL(1);
        }
        _ => return -EINVAL,
    }

    mask |= FSL_AUDMIX_CTR_OUTCKPOL_MASK;

    snd_soc_component_update_bits(comp, FSL_AUDMIX_CTR, mask, ctr)
}

unsafe extern "C" fn fsl_audmix_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut fsl_audmix;

    /* Capture stream shall not be handled */
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        return 0;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            /* scoped_guard(spinlock_irqsave, &priv->lock) */
            (*priv_).tdms |= BIT((*(*dai).driver).id) as u8;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /* scoped_guard(spinlock_irqsave, &priv->lock) */
            (*priv_).tdms &= !(BIT((*(*dai).driver).id) as u8);
        }
        _ => return -EINVAL,
    }

    0
}

static fsl_audmix_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(fsl_audmix_dai_set_fmt),
    trigger: Some(fsl_audmix_dai_trigger),
};

static mut fsl_audmix_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        id: 0,
        name: c"audmix-0".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"AUDMIX-Playback-0".as_ptr(),
            channels_min: 8,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: FSL_AUDMIX_FORMATS,
        },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rate_min: 0, rate_max: 0, rates: 0, formats: 0 },
        ops: &fsl_audmix_dai_ops,
    },
    snd_soc_dai_driver {
        id: 1,
        name: c"audmix-1".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"AUDMIX-Playback-1".as_ptr(),
            channels_min: 8,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: FSL_AUDMIX_FORMATS,
        },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rate_min: 0, rate_max: 0, rates: 0, formats: 0 },
        ops: &fsl_audmix_dai_ops,
    },
    snd_soc_dai_driver {
        id: 2,
        name: c"audmix-2".as_ptr(),
        playback: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rate_min: 0, rate_max: 0, rates: 0, formats: 0 },
        capture: snd_soc_pcm_stream {
            stream_name: c"AUDMIX-Capture-0".as_ptr(),
            channels_min: 8,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: FSL_AUDMIX_FORMATS,
        },
        ops: &fsl_audmix_dai_ops,
    },
];

static fsl_audmix_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"fsl-audmix-dai".as_ptr(),
    controls: fsl_audmix_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&fsl_audmix_snd_controls),
};

unsafe extern "C" fn fsl_audmix_readable_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        FSL_AUDMIX_CTR
        | FSL_AUDMIX_STR
        | FSL_AUDMIX_ATCR0
        | FSL_AUDMIX_ATIVAL0
        | FSL_AUDMIX_ATSTPUP0
        | FSL_AUDMIX_ATSTPDN0
        | FSL_AUDMIX_ATSTPTGT0
        | FSL_AUDMIX_ATTNVAL0
        | FSL_AUDMIX_ATSTP0
        | FSL_AUDMIX_ATCR1
        | FSL_AUDMIX_ATIVAL1
        | FSL_AUDMIX_ATSTPUP1
        | FSL_AUDMIX_ATSTPDN1
        | FSL_AUDMIX_ATSTPTGT1
        | FSL_AUDMIX_ATTNVAL1
        | FSL_AUDMIX_ATSTP1 => true,
        _ => false,
    }
}

unsafe extern "C" fn fsl_audmix_writeable_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        FSL_AUDMIX_CTR
        | FSL_AUDMIX_ATCR0
        | FSL_AUDMIX_ATIVAL0
        | FSL_AUDMIX_ATSTPUP0
        | FSL_AUDMIX_ATSTPDN0
        | FSL_AUDMIX_ATSTPTGT0
        | FSL_AUDMIX_ATCR1
        | FSL_AUDMIX_ATIVAL1
        | FSL_AUDMIX_ATSTPUP1
        | FSL_AUDMIX_ATSTPDN1
        | FSL_AUDMIX_ATSTPTGT1 => true,
        _ => false,
    }
}

static fsl_audmix_reg: [reg_default; 16] = [
    reg_default { reg: FSL_AUDMIX_CTR, def: 0x00060 },
    reg_default { reg: FSL_AUDMIX_STR, def: 0x00003 },
    reg_default { reg: FSL_AUDMIX_ATCR0, def: 0x00000 },
    reg_default { reg: FSL_AUDMIX_ATIVAL0, def: 0x3FFFF },
    reg_default { reg: FSL_AUDMIX_ATSTPUP0, def: 0x2AAAA },
    reg_default { reg: FSL_AUDMIX_ATSTPDN0, def: 0x30000 },
    reg_default { reg: FSL_AUDMIX_ATSTPTGT0, def: 0x00010 },
    reg_default { reg: FSL_AUDMIX_ATTNVAL0, def: 0x00000 },
    reg_default { reg: FSL_AUDMIX_ATSTP0, def: 0x00000 },
    reg_default { reg: FSL_AUDMIX_ATCR1, def: 0x00000 },
    reg_default { reg: FSL_AUDMIX_ATIVAL1, def: 0x3FFFF },
    reg_default { reg: FSL_AUDMIX_ATSTPUP1, def: 0x2AAAA },
    reg_default { reg: FSL_AUDMIX_ATSTPDN1, def: 0x30000 },
    reg_default { reg: FSL_AUDMIX_ATSTPTGT1, def: 0x00010 },
    reg_default { reg: FSL_AUDMIX_ATTNVAL1, def: 0x00000 },
    reg_default { reg: FSL_AUDMIX_ATSTP1, def: 0x00000 },
];

static fsl_audmix_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: FSL_AUDMIX_ATSTP1,
    reg_defaults: fsl_audmix_reg.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&fsl_audmix_reg),
    readable_reg: Some(fsl_audmix_readable_reg),
    writeable_reg: Some(fsl_audmix_writeable_reg),
    cache_type: REGCACHE_FLAT,
};

static fsl_audmix_ids: [of_device_id; 3] = [
    of_device_id { compatible: c"fsl,imx8qm-audmix".as_ptr() },
    of_device_id { compatible: c"fsl,imx952-audmix".as_ptr() },
    of_device_id { compatible: ptr::null() }, /* sentinel */
];
/* MODULE_DEVICE_TABLE(of, fsl_audmix_ids); */

unsafe extern "C" fn fsl_audmix_runtime_resume(dev: *mut device) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut fsl_audmix;
    let ret: c_int;

    ret = clk_prepare_enable((*priv_).ipg_clk);
    if ret != 0 {
        dev_err(dev, c"Failed to enable IPG clock: %d\n".as_ptr(), ret);
        return ret;
    }

    regcache_cache_only((*priv_).regmap, false);
    regcache_mark_dirty((*priv_).regmap);

    regcache_sync((*priv_).regmap)
}

unsafe extern "C" fn fsl_audmix_runtime_suspend(dev: *mut device) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut fsl_audmix;

    regcache_cache_only((*priv_).regmap, true);

    clk_disable_unprepare((*priv_).ipg_clk);

    0
}

unsafe extern "C" fn fsl_audmix_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let priv_: *mut fsl_audmix;
    let regs: *mut c_void;
    let mut ret: c_int;

    priv_ = devm_kzalloc(dev, size_of::<fsl_audmix>(), GFP_KERNEL) as *mut fsl_audmix;
    if priv_.is_null() {
        return -ENOMEM;
    }

    /* Get the addresses */
    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*priv_).regmap = devm_regmap_init_mmio(dev, regs, &fsl_audmix_regmap_config);
    if IS_ERR((*priv_).regmap as *const c_void) {
        dev_err(dev, c"failed to init regmap\n".as_ptr());
        return PTR_ERR((*priv_).regmap as *const c_void);
    }

    (*priv_).ipg_clk = devm_clk_get(dev, c"ipg".as_ptr());
    if IS_ERR((*priv_).ipg_clk as *const c_void) {
        dev_err(dev, c"failed to get ipg clock\n".as_ptr());
        return PTR_ERR((*priv_).ipg_clk as *const c_void);
    }

    spin_lock_init(&mut (*priv_).lock);
    platform_set_drvdata(pdev, priv_ as *mut c_void);
    pm_runtime_enable(dev);
    if !pm_runtime_enabled(dev) {
        ret = fsl_audmix_runtime_resume(dev);
        if ret != 0 {
            goto_err_disable_pm(dev, ret);
            return ret;
        }
    }

    ret = pm_runtime_resume_and_get(dev);
    if ret < 0 {
        goto_err_pm_get_sync(dev);
        pm_runtime_disable(dev);
        return ret;
    }

    /* To enable regmap cache only when runtime PM enabled */
    pm_runtime_put(dev);

    ret = devm_snd_soc_register_component(
        dev,
        &fsl_audmix_component,
        fsl_audmix_dai.as_mut_ptr(),
        ARRAY_SIZE(&fsl_audmix_dai),
    );
    if ret != 0 {
        dev_err(dev, c"failed to register ASoC DAI\n".as_ptr());
        goto_err_pm_get_sync(dev);
        pm_runtime_disable(dev);
        return ret;
    }

    /*
     * If dais property exist, then register the imx-audmix card driver.
     * otherwise, it should be linked by audio graph card.
     */
    if !of_find_property(ptr::null_mut(), c"dais".as_ptr(), ptr::null_mut()).is_null() {
        (*priv_).pdev = platform_device_register_data(dev, c"imx-audmix".as_ptr(), 0, ptr::null(), 0);
        if IS_ERR((*priv_).pdev as *const c_void) {
            ret = PTR_ERR((*priv_).pdev as *const c_void);
            dev_err(dev, c"failed to register platform: %d\n".as_ptr(), ret);
            goto_err_pm_get_sync(dev);
            pm_runtime_disable(dev);
            return ret;
        }
    }

    0
}

unsafe fn goto_err_pm_get_sync(dev: *mut device) {
    if !pm_runtime_status_suspended(dev) {
        fsl_audmix_runtime_suspend(dev);
    }
}

unsafe fn goto_err_disable_pm(dev: *mut device, _ret: c_int) {
    pm_runtime_disable(dev);
}

unsafe extern "C" fn fsl_audmix_remove(pdev: *mut platform_device) {
    let priv_ = dev_get_drvdata(&mut (*pdev).dev) as *mut fsl_audmix;

    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        fsl_audmix_runtime_suspend(&mut (*pdev).dev);
    }

    if !(*priv_).pdev.is_null() {
        platform_device_unregister((*priv_).pdev);
    }
}

static fsl_audmix_pm: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(fsl_audmix_runtime_suspend),
    runtime_resume: Some(fsl_audmix_runtime_resume),
};
/* SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume) */

static mut fsl_audmix_driver: platform_driver = platform_driver {
    probe: Some(fsl_audmix_probe),
    remove: Some(fsl_audmix_remove),
    driver: platform_driver_inner {
        name: c"fsl-audmix".as_ptr(),
        of_match_table: fsl_audmix_ids.as_ptr(),
        pm: &fsl_audmix_pm,
    },
};
/* module_platform_driver(fsl_audmix_driver); */

/* MODULE_DESCRIPTION("NXP AUDMIX ASoC DAI driver"); */
/* MODULE_AUTHOR("Viorel Suman <viorel.suman@nxp.com>"); */
/* MODULE_ALIAS("platform:fsl-audmix"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
