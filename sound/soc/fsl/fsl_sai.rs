// SPDX-License-Identifier: GPL-2.0+
//
// Freescale ALSA SoC Digital Audio Interface (SAI) driver.
//
// Copyright 2012-2015 Freescale Semiconductor, Inc.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;
type u32 = c_uint;
type irqreturn_t = c_int;

const NULL: *mut c_void = ptr::null_mut();

extern "C" {
    static mut fsl_asoc_get_volsw: c_void;
    static mut fsl_asoc_put_volsw: c_void;
    static mut fsl_asoc_get_enum_double: c_void;
    static mut fsl_asoc_put_enum_double: c_void;
    static mut fsl_asoc_get_xr_sx: c_void;

    fn snd_soc_dai_get_drvdata(cpu_dai: *mut snd_soc_dai) -> *mut fsl_sai;
    fn dev_get_drvdata(dev: *mut device) -> *mut fsl_sai;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut fsl_sai;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut fsl_sai);
    fn of_device_get_match_data(dev: *mut device) -> *const fsl_sai_soc_data;

    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;

    fn pinctrl_lookup_state(pinctrl: *mut pinctrl, name: *const c_char) -> *mut pinctrl_state;
    fn pinctrl_select_state(pinctrl: *mut pinctrl, state: *mut pinctrl_state) -> c_int;
    fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl;

    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;

    fn fsl_asoc_reparent_pll_clocks(dev: *mut device, mclk: *mut clk, pll8k: *mut clk, pll11k: *mut clk, freq: c_uint);
    fn fsl_asoc_get_pll_clocks(dev: *mut device, pll8k: *mut *mut clk, pll11k: *mut *mut clk);
    fn fsl_asoc_constrain_rates(dst: *mut snd_pcm_hw_constraint_list, src: *const snd_pcm_hw_constraint_list,
                                pll8k: *mut clk, pll11k: *mut clk, unused: *mut c_void,
                                list: *mut c_uint);

    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, param: c_uint, step: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, param: c_uint,
                                  list: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, tx: *mut snd_dmaengine_dai_dma_data,
                                 rx: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;

    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;

    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint,
                                              res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, base: *mut c_void, config: *mut regmap_config) -> *mut regmap;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
                        flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *mut c_void, flags: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver,
                                       dai_drv: *mut snd_soc_dai_driver, num_dai: c_uint) -> c_int;
    fn imx_pcm_dma_init(pdev: *mut platform_device) -> c_int;

    fn of_property_count_u32_elems(np: *mut device_node, propname: *const c_char) -> c_int;
    fn of_property_read_u32_index(np: *mut device_node, propname: *const c_char, index: c_uint, out: *mut u32) -> c_int;
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out: *mut u32, sz: usize) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_string(np: *mut device_node, propname: *const c_char, out: *mut *const c_char) -> c_int;
    fn of_device_is_compatible(np: *mut device_node, compat: *const c_char) -> bool;
    fn of_alias_get_id(np: *mut device_node, stem: *const c_char) -> c_int;
    fn syscon_regmap_lookup_by_compatible(compat: *const c_char) -> *mut regmap;
    fn scmi_imx_misc_ctrl_set(ctrl: c_uint, val: u32) -> c_int;

    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn cpu_latency_qos_add_request(req: *mut pm_qos_request, value: c_int) -> c_int;
    fn cpu_latency_qos_remove_request(req: *mut pm_qos_request);

    fn udelay(usecs: c_ulong);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn hweight8(x: c_uint) -> c_uint;
    fn find_first_bit(addr: *const c_ulong, size: c_uint) -> c_int;
    fn find_next_bit(addr: *const c_ulong, size: c_uint, offset: c_int) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct pinctrl { _private: [u8; 0] }
#[repr(C)] struct pinctrl_state { _private: [u8; 0] }
#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] struct pm_qos_request { _private: [u8; 0] }

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct resource {
    start: c_ulong,
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: c_ulong,
    maxburst: c_uint,
    peripheral_config: *mut c_void,
    peripheral_size: usize,
}

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
}

#[repr(C)]
struct fsl_sai_dl_cfg {
    type_: u32,
    pins: [u32; 2],
    mask: [u32; 2],
    start_off: [u32; 2],
    next_off: [u32; 2],
}

#[repr(C)]
struct fsl_sai_verid {
    version: u32,
    feature: u32,
}

#[repr(C)]
struct fsl_sai_param {
    slot_num: u32,
    fifo_depth: u32,
    dataline: u32,
}

#[repr(C)]
struct fsl_sai_soc_data {
    use_imx_pcm: bool,
    use_edma: bool,
    fifo_depth: u32,
    pins: u32,
    reg_offset: c_uint,
    mclk0_is_mclk1: bool,
    flags: u32,
    max_register: u32,
    mclk_with_tere: bool,
    max_burst: [u32; 2],
}

#[repr(C)]
struct fsl_sai_audio_config {
    words_per_fifo: u32,
    n_fifos_dst: u32,
    stride_fifos_dst: u32,
    n_fifos_src: u32,
    stride_fifos_src: u32,
}

#[repr(C)]
struct fsl_sai {
    pdev: *mut platform_device,
    soc_data: *const fsl_sai_soc_data,
    regmap: *mut regmap,
    res: *mut resource,
    bus_clk: *mut clk,
    mclk_clk: [*mut clk; FSL_SAI_MCLK_MAX as usize],
    pll8k_clk: *mut clk,
    pll11k_clk: *mut clk,
    pinctrl: *mut pinctrl,
    pins_state: *mut pinctrl_state,
    synchronous: [bool; 2],
    is_pdm_mode: bool,
    is_dsp_mode: [bool; 2],
    is_consumer_mode: [bool; 2],
    is_lsb_first: bool,
    is_bit_clock_swap: bool,
    is_multi_fifo_dma: bool,
    mclk_direction_output: bool,
    slots: [c_int; 2],
    slot_width: [c_int; 2],
    bclk_ratio: c_uint,
    mclk_streams: c_uint,
    mclk_id: [u32; 2],
    dma_params_tx: snd_dmaengine_dai_dma_data,
    dma_params_rx: snd_dmaengine_dai_dma_data,
    audio_config: [fsl_sai_audio_config; 2],
    constraint_rates: snd_pcm_hw_constraint_list,
    constraint_rates_list: [c_uint; 20],
    dl_cfg: *mut fsl_sai_dl_cfg,
    dl_cfg_cnt: c_int,
    cpu_dai_drv: [snd_soc_dai_driver; 3],
    verid: fsl_sai_verid,
    param: fsl_sai_param,
    pm_qos_req: pm_qos_request,
}

#[repr(C)] struct snd_kcontrol_new { _private: [u8; 0] }

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32, u32, c_int, c_int) -> c_int>,
    xlate_tdm_slot_mask: Option<unsafe extern "C" fn(c_uint, *mut c_uint, *mut c_uint) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    symmetric_channels: c_uint,
    symmetric_sample_bits: c_uint,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    reg_stride: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    cache_type: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct platform_driver_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: platform_driver_driver,
}

const TX: usize = 1;
const RX: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SND_SOC_CLOCK_IN: c_int = 0;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_uint = 0x80;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 1;
const PMQOS_CPU_LATENCY: u32 = 1;
const CONFIG_SND_SOC_IMX_PCM_DMA: bool = false;

const FSL_SAI_MCLK_MAX: c_uint = 4;
const FSL_SAI_CLK_BUS: c_int = 0;
const FSL_SAI_CLK_MAST1: c_int = 1;
const FSL_SAI_CLK_MAST2: c_int = 2;
const FSL_SAI_CLK_MAST3: c_int = 3;
const FSL_SAI_MAXBURST_RX: u32 = 6;
const FSL_SAI_MAXBURST_TX: u32 = 6;
const FSL_SAI_DL_NUM: c_uint = 8;
const FSL_SAI_DL_DEFAULT: u32 = 0;
const FSL_SAI_DL_I2S: u32 = 1;
const FSL_SAI_DL_PDM: u32 = 2;
const FSL_SAI_AMIX_BYPASS: u32 = 0;
const FSL_SAI_AMIX_AUDMIX: u32 = 1;
const FSL_SAI_AMIX_NONE: u32 = 2;
const SCMI_IMX952_CTRL_BYPASS_AUDMIX: u32 = 0;
const IMX_DMATYPE_MULTI_SAI: u32 = 0;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SND_SOC_DAIFMT_PDM: c_uint = 7;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0010;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0020;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0030;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x0000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x0100;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0x0200;
const SND_SOC_DAIFMT_BP_FC: c_uint = 0x0300;

const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 1;
const SNDRV_PCM_RATE_KNOT: c_uint = 0;
const FSL_SAI_FORMATS: u64 = 0;

const FSL_SAI_CSR_SEIE: u32 = 1 << 18;
const FSL_SAI_CSR_FEIE: u32 = 1 << 16;
const FSL_SAI_CSR_xIE_SHIFT: u32 = 16;
const FSL_SAI_CSR_xF_SHIFT: u32 = 8;
const FSL_SAI_CSR_WSF: u32 = 1 << 20;
const FSL_SAI_CSR_SEF: u32 = 1 << 19;
const FSL_SAI_CSR_FEF: u32 = 1 << 18;
const FSL_SAI_CSR_FWF: u32 = 1 << 17;
const FSL_SAI_CSR_FRF: u32 = 1 << 16;
const FSL_SAI_CSR_xF_W_MASK: u32 = 0x01f0000;
const FSL_SAI_CSR_xF_MASK: u32 = 0x01f0000;
const FSL_SAI_CSR_FRDE: u32 = 1;
const FSL_SAI_CSR_TERE: u32 = 1 << 31;
const FSL_SAI_CSR_BCE: u32 = 1 << 28;
const FSL_SAI_CSR_FR: u32 = 1 << 25;
const FSL_SAI_CSR_SR: u32 = 1 << 24;
const FSL_SAI_CSR_xIE_MASK: u32 = 0x00ff0000;
const FSL_SAI_FLAGS: u32 = FSL_SAI_CSR_SEIE | FSL_SAI_CSR_FEIE;

const FSL_SAI_CR2_MSEL_BUS: u32 = 0;
const FSL_SAI_CR2_MSEL_MCLK1: u32 = 1 << 26;
const FSL_SAI_CR2_MSEL_MCLK2: u32 = 2 << 26;
const FSL_SAI_CR2_MSEL_MCLK3: u32 = 3 << 26;
const FSL_SAI_CR2_MSEL_MASK: u32 = 3 << 26;
const FSL_SAI_CR2_BCS: u32 = 1 << 25;
const FSL_SAI_CR2_BCP: u32 = 1 << 24;
const FSL_SAI_CR2_BCD_MSTR: u32 = 1 << 23;
const FSL_SAI_CR2_SYNC: u32 = 3 << 30;
const FSL_SAI_CR2_DIV_MASK: u32 = 0xff;
const FSL_SAI_CR2_BYP: u32 = 1 << 31;
const FSL_SAI_CR2_BCI: u32 = 1 << 28;

const FSL_SAI_CR4_MF: u32 = 1 << 4;
const FSL_SAI_CR4_FSE: u32 = 1 << 3;
const FSL_SAI_CR4_FSP: u32 = 1 << 1;
const FSL_SAI_CR4_FSD_MSTR: u32 = 1;
const FSL_SAI_CR4_FCONT: u32 = 1 << 28;
const FSL_SAI_CR4_CHMOD: u32 = 1 << 27;
const FSL_SAI_CR4_MF_MASK: u32 = FSL_SAI_CR4_MF;
const FSL_SAI_CR4_FSE_MASK: u32 = FSL_SAI_CR4_FSE;
const FSL_SAI_CR4_FSP_MASK: u32 = FSL_SAI_CR4_FSP;
const FSL_SAI_CR4_FSD_MSTR_MASK: u32 = FSL_SAI_CR4_FSD_MSTR;
const FSL_SAI_CR4_SYWD_MASK: u32 = 0x1f00;
const FSL_SAI_CR4_FRSZ_MASK: u32 = 0x1f0000;
const FSL_SAI_CR4_CHMOD_MASK: u32 = FSL_SAI_CR4_CHMOD;
const FSL_SAI_CR4_FCONT_MASK: u32 = FSL_SAI_CR4_FCONT;
const FSL_SAI_CR4_FCOMB_MASK: u32 = 3 << 26;
const FSL_SAI_CR4_FCOMB_SOFT: u32 = 1 << 26;
const FSL_SAI_CR5_WNW_MASK: u32 = 0x1f000000;
const FSL_SAI_CR5_W0W_MASK: u32 = 0x001f0000;
const FSL_SAI_CR5_FBT_MASK: u32 = 0x00001f00;
const FSL_SAI_CR3_TRCE_MASK: u32 = 0xff0000;
const FSL_SAI_MCTL_MCLK_EN: u32 = 1 << 30;
const FSL_SAI_VERID_TSTMP_EN: u32 = 1;
const FSL_SAI_VERID_MAJOR_MASK: u32 = 0xff000000;
const FSL_SAI_VERID_MINOR_MASK: u32 = 0x00ff0000;
const FSL_SAI_VERID_MINOR_SHIFT: u32 = 16;
const FSL_SAI_VERID_FEATURE_MASK: u32 = 0x0000ffff;
const FSL_SAI_PARAM_SPF_MASK: u32 = 0x00000f00;
const FSL_SAI_PARAM_SPF_SHIFT: u32 = 8;
const FSL_SAI_PARAM_WPF_MASK: u32 = 0x000000f0;
const FSL_SAI_PARAM_WPF_SHIFT: u32 = 4;
const FSL_SAI_PARAM_DLN_MASK: u32 = 0x0000000f;

const FSL_SAI_TDR0: u32 = 0x20;
const FSL_SAI_TDR1: u32 = 0x24;
const FSL_SAI_TDR2: u32 = 0x28;
const FSL_SAI_TDR3: u32 = 0x2c;
const FSL_SAI_TDR4: u32 = 0x30;
const FSL_SAI_TDR5: u32 = 0x34;
const FSL_SAI_TDR6: u32 = 0x38;
const FSL_SAI_TDR7: u32 = 0x3c;
const FSL_SAI_TFR0: u32 = 0x40;
const FSL_SAI_TFR1: u32 = 0x44;
const FSL_SAI_TFR2: u32 = 0x48;
const FSL_SAI_TFR3: u32 = 0x4c;
const FSL_SAI_TFR4: u32 = 0x50;
const FSL_SAI_TFR5: u32 = 0x54;
const FSL_SAI_TFR6: u32 = 0x58;
const FSL_SAI_TFR7: u32 = 0x5c;
const FSL_SAI_TMR: u32 = 0x60;
const FSL_SAI_RDR0: u32 = 0xa0;
const FSL_SAI_RDR1: u32 = 0xa4;
const FSL_SAI_RDR2: u32 = 0xa8;
const FSL_SAI_RDR3: u32 = 0xac;
const FSL_SAI_RDR4: u32 = 0xb0;
const FSL_SAI_RDR5: u32 = 0xb4;
const FSL_SAI_RDR6: u32 = 0xb8;
const FSL_SAI_RDR7: u32 = 0xbc;
const FSL_SAI_RFR0: u32 = 0xc0;
const FSL_SAI_RFR1: u32 = 0xc4;
const FSL_SAI_RFR2: u32 = 0xc8;
const FSL_SAI_RFR3: u32 = 0xcc;
const FSL_SAI_RFR4: u32 = 0xd0;
const FSL_SAI_RFR5: u32 = 0xd4;
const FSL_SAI_RFR6: u32 = 0xd8;
const FSL_SAI_RFR7: u32 = 0xdc;
const FSL_SAI_RMR: u32 = 0xe0;
const FSL_SAI_MCTL: u32 = 0x100;
const FSL_SAI_MDIV: u32 = 0x104;
const FSL_SAI_VERID: u32 = 0xfc0;
const FSL_SAI_PARAM: u32 = 0xfc4;
const FSL_SAI_TTCTL: u32 = 0x110;
const FSL_SAI_TTCTN: u32 = 0x114;
const FSL_SAI_TBCTN: u32 = 0x118;
const FSL_SAI_TTCAP: u32 = 0x11c;
const FSL_SAI_RTCTL: u32 = 0x120;
const FSL_SAI_RTCTN: u32 = 0x124;
const FSL_SAI_RBCTN: u32 = 0x128;
const FSL_SAI_RTCAP: u32 = 0x12c;
const FSL_SAI_xTCTL_TSINC_SHIFT: u32 = 0;
const FSL_SAI_xTCTL_TSEN_SHIFT: u32 = 1;
const FSL_SAI_xTCTL_RTSC_SHIFT: u32 = 2;
const FSL_SAI_xTCTL_RBC_SHIFT: u32 = 3;
const IOMUXC_GPR1: u32 = 0;

const fn BIT(n: c_int) -> u32 { 1u32 << (n as u32) }
const fn FSL_SAI_TCSR(ofs: c_uint) -> u32 { ofs + 0x00 }
const fn FSL_SAI_TCR1(ofs: c_uint) -> u32 { ofs + 0x04 }
const fn FSL_SAI_TCR2(ofs: c_uint) -> u32 { ofs + 0x08 }
const fn FSL_SAI_TCR3(ofs: c_uint) -> u32 { ofs + 0x0c }
const fn FSL_SAI_TCR4(ofs: c_uint) -> u32 { ofs + 0x10 }
const fn FSL_SAI_TCR5(ofs: c_uint) -> u32 { ofs + 0x14 }
const fn FSL_SAI_RCSR(ofs: c_uint) -> u32 { ofs + 0x80 }
const fn FSL_SAI_RCR1(ofs: c_uint) -> u32 { ofs + 0x84 }
const fn FSL_SAI_RCR2(ofs: c_uint) -> u32 { ofs + 0x88 }
const fn FSL_SAI_RCR3(ofs: c_uint) -> u32 { ofs + 0x8c }
const fn FSL_SAI_RCR4(ofs: c_uint) -> u32 { ofs + 0x90 }
const fn FSL_SAI_RCR5(ofs: c_uint) -> u32 { ofs + 0x94 }
const fn FSL_SAI_xCSR(tx: bool, ofs: c_uint) -> u32 { if tx { FSL_SAI_TCSR(ofs) } else { FSL_SAI_RCSR(ofs) } }
const fn FSL_SAI_xCR1(tx: bool, ofs: c_uint) -> u32 { if tx { FSL_SAI_TCR1(ofs) } else { FSL_SAI_RCR1(ofs) } }
const fn FSL_SAI_xCR2(tx: bool, ofs: c_uint) -> u32 { if tx { FSL_SAI_TCR2(ofs) } else { FSL_SAI_RCR2(ofs) } }
const fn FSL_SAI_xCR3(tx: bool, ofs: c_uint) -> u32 { if tx { FSL_SAI_TCR3(ofs) } else { FSL_SAI_RCR3(ofs) } }
const fn FSL_SAI_xCR4(tx: bool, ofs: c_uint) -> u32 { if tx { FSL_SAI_TCR4(ofs) } else { FSL_SAI_RCR4(ofs) } }
const fn FSL_SAI_xCR5(tx: bool, ofs: c_uint) -> u32 { if tx { FSL_SAI_TCR5(ofs) } else { FSL_SAI_RCR5(ofs) } }
const fn FSL_SAI_xDR0(tx: bool) -> u32 { if tx { FSL_SAI_TDR0 } else { FSL_SAI_RDR0 } }
const fn FSL_SAI_xMR(tx: bool) -> u32 { if tx { FSL_SAI_TMR } else { FSL_SAI_RMR } }
const fn FSL_SAI_CR2_MSEL(id: u32) -> u32 { id << 26 }
const fn FSL_SAI_CR4_SYWD(width: u32) -> u32 { (width - 1) << 8 }
const fn FSL_SAI_CR4_FRSZ(slots: u32) -> u32 { (slots - 1) << 16 }
const fn FSL_SAI_CR5_WNW(width: u32) -> u32 { (width - 1) << 24 }
const fn FSL_SAI_CR5_W0W(width: u32) -> u32 { (width - 1) << 16 }
const fn FSL_SAI_CR5_FBT(bit: u32) -> u32 { bit << 8 }
const fn FSL_SAI_CR3_TRCE(mask: u32) -> u32 { mask << 16 }
const fn FSL_SAI_CR1_RFW_MASK(depth: u32) -> u32 { depth - 1 }
const fn GENMASK_U32(h: u32, l: u32) -> u32 { (!0u32 << l) & (!0u32 >> (31 - h)) }
const fn MCLK_DIR(index: c_int) -> u32 { 1u32 << (index as u32) }
const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint { N as c_uint }
const fn min_u32(a: u32, b: u32) -> u32 { if a < b { a } else { b } }
fn DIV_ROUND_CLOSEST(n: c_ulong, d: u32) -> u32 { ((n + (d as c_ulong / 2)) / d as c_ulong) as u32 }
fn DIV_ROUND_UP(n: u32, d: u32) -> u32 { (n + d - 1) / d }
fn IS_ERR_OR_NULL<T>(p: *const T) -> bool { p.is_null() || (p as isize) < 0 && (p as isize) > -4096 }
fn IS_ERR<T>(p: *const T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
fn PTR_ERR<T>(p: *const T) -> c_int { p as isize as c_int }
const fn IS_ENABLED(v: bool) -> bool { v }

static fsl_sai_rates: [c_uint; 20] = [
    8000, 11025, 12000, 16000, 22050,
    24000, 32000, 44100, 48000, 64000,
    88200, 96000, 176400, 192000, 352800,
    384000, 705600, 768000, 1411200, 2822400,
];

static fsl_sai_rate_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: ARRAY_SIZE(&fsl_sai_rates),
    list: fsl_sai_rates.as_ptr(),
};

static inc_mode: [*const c_char; 2] = [
    b"On enabled and bitcount increment\0".as_ptr() as *const c_char,
    b"On enabled\0".as_ptr() as *const c_char,
];

/* SOC_ENUM_SINGLE_DECL and FSL_ASOC_* kcontrol macro expansions are external dependency details. */
static fsl_sai_timestamp_ctrls: [snd_kcontrol_new; 14] = [
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
];

/**
 * fsl_sai_dir_is_synced - Check if stream is synced by the opposite stream
 *
 * SAI supports synchronous mode using bit/frame clocks of either Transmitter's
 * or Receiver's for both streams. This function is used to check if clocks of
 * the stream's are synced by the opposite stream.
 *
 * @sai: SAI context
 * @dir: stream direction
 */
unsafe fn fsl_sai_dir_is_synced(sai: *mut fsl_sai, dir: c_int) -> bool {
    let adir = if dir as usize == TX { RX } else { TX };
    !(*sai).synchronous[dir as usize] && (*sai).synchronous[adir]
}

unsafe extern "C" fn fsl_sai_get_pins_state(sai: *mut fsl_sai, bclk: u32) -> *mut pinctrl_state {
    let mut state: *mut pinctrl_state = ptr::null_mut();

    if (*sai).is_pdm_mode {
        /* DSD512@44.1kHz, DSD512@48kHz */
        if bclk >= 22579200 {
            state = pinctrl_lookup_state((*sai).pinctrl, b"dsd512\0".as_ptr() as *const c_char);
        }

        /* Get default DSD state */
        if IS_ERR_OR_NULL(state) {
            state = pinctrl_lookup_state((*sai).pinctrl, b"dsd\0".as_ptr() as *const c_char);
        }
    } else {
        /* 706k32b2c, 768k32b2c, etc */
        if bclk >= 45158400 {
            state = pinctrl_lookup_state((*sai).pinctrl, b"pcm_b2m\0".as_ptr() as *const c_char);
        }
    }

    /* Get default state */
    if IS_ERR_OR_NULL(state) {
        state = pinctrl_lookup_state((*sai).pinctrl, b"default\0".as_ptr() as *const c_char);
    }

    state
}

unsafe extern "C" fn fsl_sai_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let sai = devid as *mut fsl_sai;
    let ofs = (*(*sai).soc_data).reg_offset;
    let dev = &mut (*(*sai).pdev).dev as *mut device;
    let mut flags: u32;
    let mut xcsr: u32 = 0;
    let mask: u32;
    let mut iret: irqreturn_t = IRQ_NONE;

    /*
     * Both IRQ status bits and IRQ mask bits are in the xCSR but
     * different shifts. And we here create a mask only for those
     * IRQs that we activated.
     */
    mask = (FSL_SAI_FLAGS >> FSL_SAI_CSR_xIE_SHIFT) << FSL_SAI_CSR_xF_SHIFT;

    /* Tx IRQ */
    regmap_read((*sai).regmap, FSL_SAI_TCSR(ofs), &mut xcsr);
    flags = xcsr & mask;

    if flags != 0 {
        iret = IRQ_HANDLED;
    } else {
        regmap_read((*sai).regmap, FSL_SAI_RCSR(ofs), &mut xcsr);
        flags = xcsr & mask;
        if flags == 0 {
            return iret;
        }
        iret = IRQ_HANDLED;
        if flags & FSL_SAI_CSR_WSF != 0 { dev_dbg(dev, b"isr: Start of Rx word detected\n\0".as_ptr() as *const c_char); }
        if flags & FSL_SAI_CSR_SEF != 0 { dev_dbg(dev, b"isr: Rx Frame sync error detected\n\0".as_ptr() as *const c_char); }
        if flags & FSL_SAI_CSR_FEF != 0 { dev_dbg(dev, b"isr: Receive overflow detected\n\0".as_ptr() as *const c_char); }
        if flags & FSL_SAI_CSR_FWF != 0 { dev_dbg(dev, b"isr: Enabled receive FIFO is full\n\0".as_ptr() as *const c_char); }
        if flags & FSL_SAI_CSR_FRF != 0 { dev_dbg(dev, b"isr: Receive FIFO watermark has been reached\n\0".as_ptr() as *const c_char); }
        flags &= FSL_SAI_CSR_xF_W_MASK;
        xcsr &= !FSL_SAI_CSR_xF_MASK;
        if flags != 0 { regmap_write((*sai).regmap, FSL_SAI_RCSR(ofs), flags | xcsr); }
        return iret;
    }

    if flags & FSL_SAI_CSR_WSF != 0 { dev_dbg(dev, b"isr: Start of Tx word detected\n\0".as_ptr() as *const c_char); }
    if flags & FSL_SAI_CSR_SEF != 0 { dev_dbg(dev, b"isr: Tx Frame sync error detected\n\0".as_ptr() as *const c_char); }
    if flags & FSL_SAI_CSR_FEF != 0 { dev_dbg(dev, b"isr: Transmit underrun detected\n\0".as_ptr() as *const c_char); }
    if flags & FSL_SAI_CSR_FWF != 0 { dev_dbg(dev, b"isr: Enabled transmit FIFO is empty\n\0".as_ptr() as *const c_char); }
    if flags & FSL_SAI_CSR_FRF != 0 { dev_dbg(dev, b"isr: Transmit FIFO watermark has been reached\n\0".as_ptr() as *const c_char); }

    flags &= FSL_SAI_CSR_xF_W_MASK;
    xcsr &= !FSL_SAI_CSR_xF_MASK;
    if flags != 0 { regmap_write((*sai).regmap, FSL_SAI_TCSR(ofs), flags | xcsr); }

    regmap_read((*sai).regmap, FSL_SAI_RCSR(ofs), &mut xcsr);
    flags = xcsr & mask;
    if flags == 0 { return iret; }
    iret = IRQ_HANDLED;
    if flags & FSL_SAI_CSR_WSF != 0 { dev_dbg(dev, b"isr: Start of Rx word detected\n\0".as_ptr() as *const c_char); }
    if flags & FSL_SAI_CSR_SEF != 0 { dev_dbg(dev, b"isr: Rx Frame sync error detected\n\0".as_ptr() as *const c_char); }
    if flags & FSL_SAI_CSR_FEF != 0 { dev_dbg(dev, b"isr: Receive overflow detected\n\0".as_ptr() as *const c_char); }
    if flags & FSL_SAI_CSR_FWF != 0 { dev_dbg(dev, b"isr: Enabled receive FIFO is full\n\0".as_ptr() as *const c_char); }
    if flags & FSL_SAI_CSR_FRF != 0 { dev_dbg(dev, b"isr: Receive FIFO watermark has been reached\n\0".as_ptr() as *const c_char); }
    flags &= FSL_SAI_CSR_xF_W_MASK;
    xcsr &= !FSL_SAI_CSR_xF_MASK;
    if flags != 0 { regmap_write((*sai).regmap, FSL_SAI_RCSR(ofs), flags | xcsr); }
    iret
}

unsafe extern "C" fn fsl_sai_set_dai_tdm_slot_tx(cpu_dai: *mut snd_soc_dai, _tx_mask: u32, _rx_mask: u32, slots: c_int, slot_width: c_int) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let tx = true;
    (*sai).slots[tx as usize] = slots;
    (*sai).slot_width[tx as usize] = slot_width;
    0
}

unsafe extern "C" fn fsl_sai_set_dai_tdm_slot_rx(cpu_dai: *mut snd_soc_dai, _tx_mask: u32, _rx_mask: u32, slots: c_int, slot_width: c_int) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let tx = false;
    (*sai).slots[tx as usize] = slots;
    (*sai).slot_width[tx as usize] = slot_width;
    0
}

unsafe extern "C" fn fsl_sai_set_dai_tdm_slot(cpu_dai: *mut snd_soc_dai, tx_mask: u32, rx_mask: u32, slots: c_int, slot_width: c_int) -> c_int {
    let ret = fsl_sai_set_dai_tdm_slot_tx(cpu_dai, tx_mask, rx_mask, slots, slot_width);
    if ret != 0 { return ret; }
    fsl_sai_set_dai_tdm_slot_rx(cpu_dai, tx_mask, rx_mask, slots, slot_width)
}

unsafe extern "C" fn fsl_sai_xlate_tdm_slot_mask(_slots: c_uint, _tx_mask: *mut c_uint, _rx_mask: *mut c_uint) -> c_int {
    /* Leave it empty, don't change the value of tx_mask and rx_mask */
    0
}

unsafe extern "C" fn fsl_sai_set_dai_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let sai = snd_soc_dai_get_drvdata(dai);
    (*sai).bclk_ratio = ratio;
    0
}

unsafe fn fsl_sai_set_dai_sysclk_tr(cpu_dai: *mut snd_soc_dai, clk_id: c_int, _freq: c_uint, tx: bool) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let ofs = (*(*sai).soc_data).reg_offset;
    let mut val_cr2: u32 = 0;

    match clk_id {
        FSL_SAI_CLK_BUS => val_cr2 |= FSL_SAI_CR2_MSEL_BUS,
        FSL_SAI_CLK_MAST1 => val_cr2 |= FSL_SAI_CR2_MSEL_MCLK1,
        FSL_SAI_CLK_MAST2 => val_cr2 |= FSL_SAI_CR2_MSEL_MCLK2,
        FSL_SAI_CLK_MAST3 => val_cr2 |= FSL_SAI_CR2_MSEL_MCLK3,
        _ => return -EINVAL,
    }

    regmap_update_bits((*sai).regmap, FSL_SAI_xCR2(tx, ofs), FSL_SAI_CR2_MSEL_MASK, val_cr2);
    0
}

unsafe fn fsl_sai_set_mclk_rate(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint) -> c_int {
    let sai = snd_soc_dai_get_drvdata(dai);
    fsl_asoc_reparent_pll_clocks((*dai).dev, (*sai).mclk_clk[clk_id as usize], (*sai).pll8k_clk, (*sai).pll11k_clk, freq);
    let ret = clk_set_rate((*sai).mclk_clk[clk_id as usize], freq);
    if ret < 0 {
        dev_err((*dai).dev, b"failed to set clock rate (%u): %d\n\0".as_ptr() as *const c_char, freq, ret);
    }
    ret
}

unsafe extern "C" fn fsl_sai_set_dai_sysclk(cpu_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let mut ret: c_int;

    if dir == SND_SOC_CLOCK_IN { return 0; }
    if clk_id < 0 || clk_id >= FSL_SAI_MCLK_MAX as c_int {
        dev_err((*cpu_dai).dev, b"Unknown clock id: %d\n\0".as_ptr() as *const c_char, clk_id);
        return -EINVAL;
    }
    if IS_ERR_OR_NULL((*sai).mclk_clk[clk_id as usize]) {
        dev_err((*cpu_dai).dev, b"Unassigned clock: %d\n\0".as_ptr() as *const c_char, clk_id);
        return -EINVAL;
    }
    if (*sai).mclk_streams == 0 && freq > 0 {
        ret = fsl_sai_set_mclk_rate(cpu_dai, if clk_id != 0 { clk_id } else { FSL_SAI_CLK_MAST1 }, freq);
        if ret < 0 { return ret; }
    }
    ret = fsl_sai_set_dai_sysclk_tr(cpu_dai, clk_id, freq, true);
    if ret != 0 {
        dev_err((*cpu_dai).dev, b"Cannot set tx sysclk: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = fsl_sai_set_dai_sysclk_tr(cpu_dai, clk_id, freq, false);
    if ret != 0 { dev_err((*cpu_dai).dev, b"Cannot set rx sysclk: %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

unsafe fn fsl_sai_set_dai_fmt_tr(cpu_dai: *mut snd_soc_dai, fmt: c_uint, tx: bool) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let ofs = (*(*sai).soc_data).reg_offset;
    let mut val_cr2: u32 = 0;
    let mut val_cr4: u32 = 0;

    if (*sai).is_bit_clock_swap { val_cr2 |= FSL_SAI_CR2_BCS; }
    if !(*sai).is_lsb_first { val_cr4 |= FSL_SAI_CR4_MF; }

    (*sai).is_pdm_mode = false;
    (*sai).is_dsp_mode[tx as usize] = false;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            /*
             * Frame low, 1clk before data, one word length for frame sync,
             * frame sync starts one serial clock cycle earlier,
             * that is, together with the last bit of the previous
             * data word.
             */
            val_cr2 |= FSL_SAI_CR2_BCP;
            val_cr4 |= FSL_SAI_CR4_FSE | FSL_SAI_CR4_FSP;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            /*
             * Frame high, one word length for frame sync,
             * frame sync asserts with the first bit of the frame.
             */
            val_cr2 |= FSL_SAI_CR2_BCP;
        }
        SND_SOC_DAIFMT_DSP_A => {
            /*
             * Frame high, 1clk before data, one bit for frame sync,
             * frame sync starts one serial clock cycle earlier,
             * that is, together with the last bit of the previous
             * data word.
             */
            val_cr2 |= FSL_SAI_CR2_BCP;
            val_cr4 |= FSL_SAI_CR4_FSE;
            (*sai).is_dsp_mode[tx as usize] = true;
        }
        SND_SOC_DAIFMT_DSP_B => {
            /*
             * Frame high, one bit for frame sync,
             * frame sync asserts with the first bit of the frame.
             */
            val_cr2 |= FSL_SAI_CR2_BCP;
            (*sai).is_dsp_mode[tx as usize] = true;
        }
        SND_SOC_DAIFMT_PDM => {
            val_cr2 |= FSL_SAI_CR2_BCP;
            (*sai).is_pdm_mode = true;
        }
        SND_SOC_DAIFMT_RIGHT_J | _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_IF => { val_cr2 ^= FSL_SAI_CR2_BCP; val_cr4 ^= FSL_SAI_CR4_FSP; }
        SND_SOC_DAIFMT_IB_NF => { val_cr2 ^= FSL_SAI_CR2_BCP; }
        SND_SOC_DAIFMT_NB_IF => { val_cr4 ^= FSL_SAI_CR4_FSP; }
        SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            val_cr2 |= FSL_SAI_CR2_BCD_MSTR;
            val_cr4 |= FSL_SAI_CR4_FSD_MSTR;
            (*sai).is_consumer_mode[tx as usize] = false;
        }
        SND_SOC_DAIFMT_BC_FC => { (*sai).is_consumer_mode[tx as usize] = true; }
        SND_SOC_DAIFMT_BP_FC => {
            val_cr2 |= FSL_SAI_CR2_BCD_MSTR;
            (*sai).is_consumer_mode[tx as usize] = false;
        }
        SND_SOC_DAIFMT_BC_FP => {
            val_cr4 |= FSL_SAI_CR4_FSD_MSTR;
            (*sai).is_consumer_mode[tx as usize] = true;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits((*sai).regmap, FSL_SAI_xCR2(tx, ofs),
                       FSL_SAI_CR2_BCS | FSL_SAI_CR2_BCP | FSL_SAI_CR2_BCD_MSTR, val_cr2);
    regmap_update_bits((*sai).regmap, FSL_SAI_xCR4(tx, ofs),
                       FSL_SAI_CR4_MF | FSL_SAI_CR4_FSE | FSL_SAI_CR4_FSP | FSL_SAI_CR4_FSD_MSTR, val_cr4);
    0
}

unsafe extern "C" fn fsl_sai_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let mut ret = fsl_sai_set_dai_fmt_tr(cpu_dai, fmt, true);
    if ret != 0 {
        dev_err((*cpu_dai).dev, b"Cannot set tx format: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = fsl_sai_set_dai_fmt_tr(cpu_dai, fmt, false);
    if ret != 0 { dev_err((*cpu_dai).dev, b"Cannot set rx format: %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

unsafe extern "C" fn fsl_sai_set_dai_fmt_tx(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    fsl_sai_set_dai_fmt_tr(cpu_dai, fmt, true)
}

unsafe extern "C" fn fsl_sai_set_dai_fmt_rx(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    fsl_sai_set_dai_fmt_tr(cpu_dai, fmt, false)
}

unsafe fn fsl_sai_set_bclk(dai: *mut snd_soc_dai, tx: bool, freq: u32) -> c_int {
    let sai = snd_soc_dai_get_drvdata(dai);
    let ofs = (*(*sai).soc_data).reg_offset;
    let mut reg: c_uint;
    let mut savediv: u32 = 0;
    let mut bestdiff: u32 = freq;
    let adir = if tx { RX } else { TX };
    let dir = if tx { TX } else { RX };
    let mut id: u32;
    let support_1_1_ratio = (*sai).verid.version >= 0x0301;

    if (*sai).is_consumer_mode[tx as usize] { return 0; }
    id = if (*(*sai).soc_data).mclk0_is_mclk1 { 1 } else { 0 };

    while id < FSL_SAI_MCLK_MAX {
        let clk_rate = clk_get_rate((*sai).mclk_clk[id as usize]);
        if clk_rate == 0 { id += 1; continue; }
        let ratio = DIV_ROUND_CLOSEST(clk_rate, freq);
        if ratio == 0 || ratio > 512 { id += 1; continue; }
        if ratio == 1 && !support_1_1_ratio { id += 1; continue; }
        if (ratio & 1) != 0 && ratio > 1 { id += 1; continue; }
        let diff = ((clk_rate as i64) - (ratio as i64 * freq as i64)).abs() as u32;
        if diff != 0 && clk_rate / diff as c_ulong < 1000 { id += 1; continue; }
        dev_dbg((*dai).dev, b"ratio %d for freq %dHz based on clock %ldHz\n\0".as_ptr() as *const c_char, ratio, freq, clk_rate);
        if diff < bestdiff {
            savediv = ratio;
            (*sai).mclk_id[tx as usize] = id;
            bestdiff = diff;
        }
        if diff == 0 { break; }
        id += 1;
    }

    if savediv == 0 {
        dev_err((*dai).dev, b"failed to derive required %cx rate: %d\n\0".as_ptr() as *const c_char,
                if tx { b'T' as c_int } else { b'R' as c_int }, freq);
        return -EINVAL;
    }

    dev_dbg((*dai).dev, b"best fit: clock id=%d, div=%d, deviation =%d\n\0".as_ptr() as *const c_char,
            (*sai).mclk_id[tx as usize], savediv, bestdiff);

    if fsl_sai_dir_is_synced(sai, adir as c_int) {
        reg = FSL_SAI_xCR2(!tx, ofs);
    } else if !(*sai).synchronous[dir] {
        reg = FSL_SAI_xCR2(tx, ofs);
    } else {
        return 0;
    }

    regmap_update_bits((*sai).regmap, reg, FSL_SAI_CR2_MSEL_MASK, FSL_SAI_CR2_MSEL((*sai).mclk_id[tx as usize]));

    if savediv == 1 {
        regmap_update_bits((*sai).regmap, reg, FSL_SAI_CR2_DIV_MASK | FSL_SAI_CR2_BYP, FSL_SAI_CR2_BYP);
        if fsl_sai_dir_is_synced(sai, adir as c_int) {
            regmap_update_bits((*sai).regmap, FSL_SAI_xCR2(tx, ofs), FSL_SAI_CR2_BCI, FSL_SAI_CR2_BCI);
        } else {
            regmap_update_bits((*sai).regmap, FSL_SAI_xCR2(tx, ofs), FSL_SAI_CR2_BCI, 0);
        }
    } else {
        regmap_update_bits((*sai).regmap, reg, FSL_SAI_CR2_DIV_MASK | FSL_SAI_CR2_BYP, savediv / 2 - 1);
    }
    0
}

unsafe extern "C" fn fsl_sai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let ofs = (*(*sai).soc_data).reg_offset;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let channels = params_channels(params);
    let dl_cfg = (*sai).dl_cfg;
    let word_width = params_width(params);
    let mut trce_mask: c_int;
    let mut dl_cfg_idx: c_int = 0;
    let dl_cfg_cnt = (*sai).dl_cfg_cnt;
    let mut dl_type = FSL_SAI_DL_I2S;
    let mut val_cr4: u32 = 0;
    let mut val_cr5: u32 = 0;
    let mut slots = if channels == 1 { 2 } else { channels };
    let mut slot_width = word_width;
    let adir = if tx { RX } else { TX };
    let mut ret: c_int;

    if (*sai).slot_width[tx as usize] != 0 { slot_width = (*sai).slot_width[tx as usize] as u32; }
    if (*sai).slots[tx as usize] != 0 { slots = (*sai).slots[tx as usize] as u32; }
    else if (*sai).bclk_ratio != 0 { slots = (*sai).bclk_ratio / slot_width; }

    let mut pins = DIV_ROUND_UP(channels, slots);
    if (*sai).is_pdm_mode {
        pins = channels;
        dl_type = FSL_SAI_DL_PDM;
    }

    let mut i = 0;
    while i < dl_cfg_cnt {
        if (*dl_cfg.add(i as usize)).type_ == dl_type && (*dl_cfg.add(i as usize)).pins[tx as usize] == pins {
            dl_cfg_idx = i;
            break;
        }
        i += 1;
    }

    if hweight8((*dl_cfg.add(dl_cfg_idx as usize)).mask[tx as usize]) < pins {
        dev_err((*cpu_dai).dev, b"channel not supported\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    let bclk = params_rate(params) * if (*sai).bclk_ratio != 0 { (*sai).bclk_ratio } else { slots * slot_width };
    if !IS_ERR_OR_NULL((*sai).pinctrl) {
        (*sai).pins_state = fsl_sai_get_pins_state(sai, bclk);
        if !IS_ERR_OR_NULL((*sai).pins_state) {
            ret = pinctrl_select_state((*sai).pinctrl, (*sai).pins_state);
            if ret != 0 {
                dev_err((*cpu_dai).dev, b"failed to set proper pins state: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
    }

    if !(*sai).is_consumer_mode[tx as usize] {
        ret = fsl_sai_set_bclk(cpu_dai, tx, bclk);
        if ret != 0 { return ret; }
        if ((*sai).mclk_streams & BIT((*substream).stream)) == 0 {
            ret = clk_prepare_enable((*sai).mclk_clk[(*sai).mclk_id[tx as usize] as usize]);
            if ret != 0 { return ret; }
            (*sai).mclk_streams |= BIT((*substream).stream);
        }
    }

    if !(*sai).is_dsp_mode[tx as usize] && !(*sai).is_pdm_mode {
        val_cr4 |= FSL_SAI_CR4_SYWD(slot_width);
    }
    val_cr5 |= FSL_SAI_CR5_WNW(slot_width);
    val_cr5 |= FSL_SAI_CR5_W0W(slot_width);
    if (*sai).is_lsb_first { val_cr5 |= FSL_SAI_CR5_FBT(0); }
    else { val_cr5 |= FSL_SAI_CR5_FBT(word_width - 1); }
    val_cr4 |= FSL_SAI_CR4_FRSZ(slots);
    val_cr4 |= FSL_SAI_CR4_FCONT;
    if tx { val_cr4 |= FSL_SAI_CR4_CHMOD; }

    if fsl_sai_dir_is_synced(sai, adir as c_int) {
        regmap_update_bits((*sai).regmap, FSL_SAI_xCR4(!tx, ofs),
                           FSL_SAI_CR4_SYWD_MASK | FSL_SAI_CR4_FRSZ_MASK | FSL_SAI_CR4_CHMOD_MASK, val_cr4);
        regmap_update_bits((*sai).regmap, FSL_SAI_xCR5(!tx, ofs),
                           FSL_SAI_CR5_WNW_MASK | FSL_SAI_CR5_W0W_MASK | FSL_SAI_CR5_FBT_MASK, val_cr5);
    }

    if hweight8((*dl_cfg.add(dl_cfg_idx as usize)).mask[tx as usize]) <= 1 || (*sai).is_multi_fifo_dma {
        regmap_update_bits((*sai).regmap, FSL_SAI_xCR4(tx, ofs), FSL_SAI_CR4_FCOMB_MASK, 0);
    } else {
        regmap_update_bits((*sai).regmap, FSL_SAI_xCR4(tx, ofs), FSL_SAI_CR4_FCOMB_MASK, FSL_SAI_CR4_FCOMB_SOFT);
    }

    let dma_params = if tx { &mut (*sai).dma_params_tx } else { &mut (*sai).dma_params_rx };
    dma_params.addr = (*(*sai).res).start + FSL_SAI_xDR0(tx) as c_ulong + ((*dl_cfg.add(dl_cfg_idx as usize)).start_off[tx as usize] * 0x4) as c_ulong;

    if (*sai).is_multi_fifo_dma {
        (*sai).audio_config[tx as usize].words_per_fifo = min_u32(slots, channels);
        if tx {
            (*sai).audio_config[tx as usize].n_fifos_dst = pins;
            (*sai).audio_config[tx as usize].stride_fifos_dst = (*dl_cfg.add(dl_cfg_idx as usize)).next_off[tx as usize];
        } else {
            (*sai).audio_config[tx as usize].n_fifos_src = pins;
            (*sai).audio_config[tx as usize].stride_fifos_src = (*dl_cfg.add(dl_cfg_idx as usize)).next_off[tx as usize];
        }
        dma_params.maxburst = (*sai).audio_config[tx as usize].words_per_fifo * pins;
        dma_params.peripheral_config = &mut (*sai).audio_config[tx as usize] as *mut _ as *mut c_void;
        dma_params.peripheral_size = size_of::<fsl_sai_audio_config>();
        let watermark = if tx { (*(*sai).soc_data).fifo_depth - dma_params.maxburst } else { dma_params.maxburst - 1 };
        regmap_update_bits((*sai).regmap, FSL_SAI_xCR1(tx, ofs),
                           FSL_SAI_CR1_RFW_MASK((*(*sai).soc_data).fifo_depth), watermark);
    }

    i = 0;
    while i < (*(*sai).soc_data).pins as c_int {
        trce_mask = (1 << (i + 1)) - 1;
        if hweight8((*dl_cfg.add(dl_cfg_idx as usize)).mask[tx as usize] & trce_mask as u32) == pins { break; }
        i += 1;
    }
    trce_mask = (1 << (i + 1)) - 1;
    regmap_update_bits((*sai).regmap, FSL_SAI_xCR3(tx, ofs), FSL_SAI_CR3_TRCE_MASK,
                       FSL_SAI_CR3_TRCE((*dl_cfg.add(dl_cfg_idx as usize)).mask[tx as usize] & trce_mask as u32));

    if (*(*sai).soc_data).mclk_with_tere && (*sai).mclk_direction_output && !(*sai).is_consumer_mode[tx as usize] {
        regmap_update_bits((*sai).regmap, FSL_SAI_xCR4(tx, ofs), FSL_SAI_CR4_FSD_MSTR, 0);
    }

    regmap_update_bits((*sai).regmap, FSL_SAI_xCR4(tx, ofs),
                       FSL_SAI_CR4_SYWD_MASK | FSL_SAI_CR4_FRSZ_MASK | FSL_SAI_CR4_CHMOD_MASK | FSL_SAI_CR4_FCONT_MASK,
                       val_cr4);
    regmap_update_bits((*sai).regmap, FSL_SAI_xCR5(tx, ofs),
                       FSL_SAI_CR5_WNW_MASK | FSL_SAI_CR5_W0W_MASK | FSL_SAI_CR5_FBT_MASK, val_cr5);

    if (*(*sai).soc_data).mclk_with_tere && (*sai).mclk_direction_output && !(*sai).is_consumer_mode[tx as usize] {
        regmap_update_bits((*sai).regmap, FSL_SAI_xCR4(tx, ofs), FSL_SAI_CR4_FSD_MSTR, FSL_SAI_CR4_FSD_MSTR);
    }

    regmap_write((*sai).regmap, FSL_SAI_xMR(tx), !GENMASK_U32(min_u32(channels, slots) - 1, 0));
    0
}

unsafe extern "C" fn fsl_sai_hw_free(substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let ofs = (*(*sai).soc_data).reg_offset;
    let adir = if tx { RX } else { TX };
    let dir = if tx { TX } else { RX };
    regmap_write((*sai).regmap, FSL_SAI_xMR(tx), 0);
    regmap_update_bits((*sai).regmap, FSL_SAI_xCR3(tx, ofs), FSL_SAI_CR3_TRCE_MASK, 0);
    if !(*sai).is_consumer_mode[tx as usize] {
        let adir_active = ((*sai).mclk_streams & BIT(1 - (*substream).stream)) != 0;
        if fsl_sai_dir_is_synced(sai, adir as c_int) && !adir_active {
            regmap_update_bits((*sai).regmap, FSL_SAI_xCR2(!tx, ofs), FSL_SAI_CR2_BCI | FSL_SAI_CR2_BYP, 0);
        }
        if !fsl_sai_dir_is_synced(sai, dir as c_int) || !adir_active {
            regmap_update_bits((*sai).regmap, FSL_SAI_xCR2(tx, ofs), FSL_SAI_CR2_BCI | FSL_SAI_CR2_BYP, 0);
        }
        if ((*sai).mclk_streams & BIT((*substream).stream)) != 0 {
            clk_disable_unprepare((*sai).mclk_clk[(*sai).mclk_id[tx as usize] as usize]);
            (*sai).mclk_streams &= !BIT((*substream).stream);
        }
    }
    0
}

unsafe fn fsl_sai_config_disable(sai: *mut fsl_sai, dir: c_int) {
    let ofs = (*(*sai).soc_data).reg_offset;
    let tx = dir as usize == TX;
    let mut xcsr: u32 = 0;
    let mut count: u32 = 100;
    let mask = if (*(*sai).soc_data).mclk_with_tere && (*sai).mclk_direction_output {
        FSL_SAI_CSR_TERE
    } else {
        FSL_SAI_CSR_TERE | FSL_SAI_CSR_BCE
    };

    regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), mask, 0);
    loop {
        udelay(10);
        regmap_read((*sai).regmap, FSL_SAI_xCSR(tx, ofs), &mut xcsr);
        count -= 1;
        if count == 0 || (xcsr & FSL_SAI_CSR_TERE) == 0 { break; }
    }
    regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), FSL_SAI_CSR_FR, FSL_SAI_CSR_FR);
    regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), FSL_SAI_CSR_SR, FSL_SAI_CSR_SR);
    regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), FSL_SAI_CSR_SR, 0);
}

unsafe extern "C" fn fsl_sai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let ofs = (*(*sai).soc_data).reg_offset;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let adir = if tx { RX } else { TX };
    let dir = if tx { TX } else { RX };
    let mut xcsr: u32 = 0;

    regmap_update_bits((*sai).regmap, FSL_SAI_TCR2(ofs), FSL_SAI_CR2_SYNC,
                       if (*sai).synchronous[TX] { FSL_SAI_CR2_SYNC } else { 0 });
    regmap_update_bits((*sai).regmap, FSL_SAI_RCR2(ofs), FSL_SAI_CR2_SYNC,
                       if (*sai).synchronous[RX] { FSL_SAI_CR2_SYNC } else { 0 });

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), FSL_SAI_CSR_FRDE, FSL_SAI_CSR_FRDE);
            regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), FSL_SAI_CSR_TERE, FSL_SAI_CSR_TERE);
            if fsl_sai_dir_is_synced(sai, adir as c_int) {
                regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(!tx, ofs), FSL_SAI_CSR_TERE, FSL_SAI_CSR_TERE);
            }
            regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), FSL_SAI_CSR_xIE_MASK, FSL_SAI_FLAGS);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), FSL_SAI_CSR_FRDE, 0);
            regmap_update_bits((*sai).regmap, FSL_SAI_xCSR(tx, ofs), FSL_SAI_CSR_xIE_MASK, 0);
            regmap_read((*sai).regmap, FSL_SAI_xCSR(!tx, ofs), &mut xcsr);
            if fsl_sai_dir_is_synced(sai, adir as c_int) && (xcsr & FSL_SAI_CSR_FRDE) == 0 {
                fsl_sai_config_disable(sai, adir as c_int);
            }
            if !fsl_sai_dir_is_synced(sai, dir as c_int) || (xcsr & FSL_SAI_CSR_FRDE) == 0 {
                fsl_sai_config_disable(sai, dir as c_int);
            }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn fsl_sai_startup(substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai);
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    if (*(*sai).soc_data).use_edma {
        snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
                                   if tx { (*sai).dma_params_tx.maxburst } else { (*sai).dma_params_rx.maxburst });
    }
    if (*sai).is_consumer_mode[tx as usize] {
        snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &fsl_sai_rate_constraints)
    } else {
        snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &(*sai).constraint_rates)
    }
}

unsafe extern "C" fn fsl_sai_dai_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = dev_get_drvdata((*cpu_dai).dev);
    let ofs = (*(*sai).soc_data).reg_offset;
    regmap_update_bits((*sai).regmap, FSL_SAI_TCSR(ofs), FSL_SAI_CSR_SR, FSL_SAI_CSR_SR);
    regmap_update_bits((*sai).regmap, FSL_SAI_RCSR(ofs), FSL_SAI_CSR_SR, FSL_SAI_CSR_SR);
    regmap_update_bits((*sai).regmap, FSL_SAI_TCSR(ofs), FSL_SAI_CSR_SR, 0);
    regmap_update_bits((*sai).regmap, FSL_SAI_RCSR(ofs), FSL_SAI_CSR_SR, 0);
    regmap_update_bits((*sai).regmap, FSL_SAI_TCR1(ofs),
                       FSL_SAI_CR1_RFW_MASK((*(*sai).soc_data).fifo_depth),
                       (*(*sai).soc_data).fifo_depth - (*sai).dma_params_tx.maxburst);
    regmap_update_bits((*sai).regmap, FSL_SAI_RCR1(ofs),
                       FSL_SAI_CR1_RFW_MASK((*(*sai).soc_data).fifo_depth),
                       (*sai).dma_params_rx.maxburst - 1);
    snd_soc_dai_init_dma_data(cpu_dai, &mut (*sai).dma_params_tx, &mut (*sai).dma_params_rx);
    0
}

static fsl_sai_pcm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_sai_dai_probe), set_bclk_ratio: Some(fsl_sai_set_dai_bclk_ratio),
    set_sysclk: Some(fsl_sai_set_dai_sysclk), set_fmt: Some(fsl_sai_set_dai_fmt),
    set_tdm_slot: Some(fsl_sai_set_dai_tdm_slot), xlate_tdm_slot_mask: None,
    hw_params: Some(fsl_sai_hw_params), hw_free: Some(fsl_sai_hw_free),
    trigger: Some(fsl_sai_trigger), startup: Some(fsl_sai_startup),
};

static fsl_sai_pcm_dai_tx_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_sai_dai_probe), set_bclk_ratio: Some(fsl_sai_set_dai_bclk_ratio),
    set_sysclk: Some(fsl_sai_set_dai_sysclk), set_fmt: Some(fsl_sai_set_dai_fmt_tx),
    set_tdm_slot: Some(fsl_sai_set_dai_tdm_slot_tx), xlate_tdm_slot_mask: Some(fsl_sai_xlate_tdm_slot_mask),
    hw_params: Some(fsl_sai_hw_params), hw_free: Some(fsl_sai_hw_free),
    trigger: Some(fsl_sai_trigger), startup: Some(fsl_sai_startup),
};

static fsl_sai_pcm_dai_rx_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_sai_dai_probe), set_bclk_ratio: Some(fsl_sai_set_dai_bclk_ratio),
    set_sysclk: Some(fsl_sai_set_dai_sysclk), set_fmt: Some(fsl_sai_set_dai_fmt_rx),
    set_tdm_slot: Some(fsl_sai_set_dai_tdm_slot_rx), xlate_tdm_slot_mask: Some(fsl_sai_xlate_tdm_slot_mask),
    hw_params: Some(fsl_sai_hw_params), hw_free: Some(fsl_sai_hw_free),
    trigger: Some(fsl_sai_trigger), startup: Some(fsl_sai_startup),
};

unsafe extern "C" fn fsl_sai_dai_resume(component: *mut snd_soc_component) -> c_int {
    let sai = snd_soc_component_get_drvdata(component);
    let dev = &mut (*(*sai).pdev).dev as *mut device;
    if !IS_ERR_OR_NULL((*sai).pinctrl) && !IS_ERR_OR_NULL((*sai).pins_state) {
        let ret = pinctrl_select_state((*sai).pinctrl, (*sai).pins_state);
        if ret != 0 {
            dev_err(dev, b"failed to set proper pins state: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn fsl_sai_component_probe(component: *mut snd_soc_component) -> c_int {
    let sai = snd_soc_component_get_drvdata(component);
    if ((*sai).verid.feature & FSL_SAI_VERID_TSTMP_EN) != 0 {
        snd_soc_add_component_controls(component, fsl_sai_timestamp_ctrls.as_ptr(), ARRAY_SIZE(&fsl_sai_timestamp_ctrls));
    }
    0
}

static mut fsl_sai_dai_template: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: b"sai-tx-rx\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { stream_name: b"CPU-Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 32, rate_min: 8000, rate_max: 2822400, rates: SNDRV_PCM_RATE_KNOT, formats: FSL_SAI_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: b"CPU-Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 32, rate_min: 8000, rate_max: 2822400, rates: SNDRV_PCM_RATE_KNOT, formats: FSL_SAI_FORMATS },
        ops: &fsl_sai_pcm_dai_ops, symmetric_rate: 0, symmetric_channels: 0, symmetric_sample_bits: 0,
    },
    snd_soc_dai_driver {
        name: b"sai-tx\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { stream_name: b"SAI-Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 32, rate_min: 8000, rate_max: 2822400, rates: SNDRV_PCM_RATE_KNOT, formats: FSL_SAI_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rate_min: 0, rate_max: 0, rates: 0, formats: 0 },
        ops: &fsl_sai_pcm_dai_tx_ops, symmetric_rate: 0, symmetric_channels: 0, symmetric_sample_bits: 0,
    },
    snd_soc_dai_driver {
        name: b"sai-rx\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rate_min: 0, rate_max: 0, rates: 0, formats: 0 },
        capture: snd_soc_pcm_stream { stream_name: b"SAI-Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 32, rate_min: 8000, rate_max: 2822400, rates: SNDRV_PCM_RATE_KNOT, formats: FSL_SAI_FORMATS },
        ops: &fsl_sai_pcm_dai_rx_ops, symmetric_rate: 0, symmetric_channels: 0, symmetric_sample_bits: 0,
    },
];

static fsl_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"fsl-sai\0".as_ptr() as *const c_char,
    probe: Some(fsl_sai_component_probe),
    resume: Some(fsl_sai_dai_resume),
    legacy_dai_naming: 1,
};

static fsl_sai_reg_defaults_ofs0: [reg_default; 21] = [
    reg_default { reg: FSL_SAI_TCR1(0), def: 0 }, reg_default { reg: FSL_SAI_TCR2(0), def: 0 },
    reg_default { reg: FSL_SAI_TCR3(0), def: 0 }, reg_default { reg: FSL_SAI_TCR4(0), def: 0 },
    reg_default { reg: FSL_SAI_TCR5(0), def: 0 }, reg_default { reg: FSL_SAI_TDR0, def: 0 },
    reg_default { reg: FSL_SAI_TDR1, def: 0 }, reg_default { reg: FSL_SAI_TDR2, def: 0 },
    reg_default { reg: FSL_SAI_TDR3, def: 0 }, reg_default { reg: FSL_SAI_TDR4, def: 0 },
    reg_default { reg: FSL_SAI_TDR5, def: 0 }, reg_default { reg: FSL_SAI_TDR6, def: 0 },
    reg_default { reg: FSL_SAI_TDR7, def: 0 }, reg_default { reg: FSL_SAI_TMR, def: 0 },
    reg_default { reg: FSL_SAI_TTCTL, def: 0 }, reg_default { reg: FSL_SAI_RCR1(0), def: 0 },
    reg_default { reg: FSL_SAI_RCR2(0), def: 0 }, reg_default { reg: FSL_SAI_RCR3(0), def: 0 },
    reg_default { reg: FSL_SAI_RCR4(0), def: 0 }, reg_default { reg: FSL_SAI_RCR5(0), def: 0 },
    reg_default { reg: FSL_SAI_RMR, def: 0 },
];

static fsl_sai_reg_defaults_ofs8: [reg_default; 24] = [
    reg_default { reg: FSL_SAI_TCR1(8), def: 0 }, reg_default { reg: FSL_SAI_TCR2(8), def: 0 },
    reg_default { reg: FSL_SAI_TCR3(8), def: 0 }, reg_default { reg: FSL_SAI_TCR4(8), def: 0 },
    reg_default { reg: FSL_SAI_TCR5(8), def: 0 }, reg_default { reg: FSL_SAI_TDR0, def: 0 },
    reg_default { reg: FSL_SAI_TDR1, def: 0 }, reg_default { reg: FSL_SAI_TDR2, def: 0 },
    reg_default { reg: FSL_SAI_TDR3, def: 0 }, reg_default { reg: FSL_SAI_TDR4, def: 0 },
    reg_default { reg: FSL_SAI_TDR5, def: 0 }, reg_default { reg: FSL_SAI_TDR6, def: 0 },
    reg_default { reg: FSL_SAI_TDR7, def: 0 }, reg_default { reg: FSL_SAI_TMR, def: 0 },
    reg_default { reg: FSL_SAI_TTCTL, def: 0 }, reg_default { reg: FSL_SAI_RCR1(8), def: 0 },
    reg_default { reg: FSL_SAI_RCR2(8), def: 0 }, reg_default { reg: FSL_SAI_RCR3(8), def: 0 },
    reg_default { reg: FSL_SAI_RCR4(8), def: 0 }, reg_default { reg: FSL_SAI_RCR5(8), def: 0 },
    reg_default { reg: FSL_SAI_RMR, def: 0 }, reg_default { reg: FSL_SAI_RTCTL, def: 0 },
    reg_default { reg: FSL_SAI_MCTL, def: 0 }, reg_default { reg: FSL_SAI_MDIV, def: 0 },
];

unsafe extern "C" fn fsl_sai_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    let sai = dev_get_drvdata(dev);
    let ofs = (*(*sai).soc_data).reg_offset;
    if reg >= FSL_SAI_TCSR(ofs) && reg <= FSL_SAI_TCR5(ofs) { return true; }
    if reg >= FSL_SAI_RCSR(ofs) && reg <= FSL_SAI_RCR5(ofs) { return true; }
    matches!(reg,
        FSL_SAI_TFR0|FSL_SAI_TFR1|FSL_SAI_TFR2|FSL_SAI_TFR3|FSL_SAI_TFR4|FSL_SAI_TFR5|FSL_SAI_TFR6|FSL_SAI_TFR7|
        FSL_SAI_TMR|FSL_SAI_RDR0|FSL_SAI_RDR1|FSL_SAI_RDR2|FSL_SAI_RDR3|FSL_SAI_RDR4|FSL_SAI_RDR5|FSL_SAI_RDR6|
        FSL_SAI_RDR7|FSL_SAI_RFR0|FSL_SAI_RFR1|FSL_SAI_RFR2|FSL_SAI_RFR3|FSL_SAI_RFR4|FSL_SAI_RFR5|FSL_SAI_RFR6|
        FSL_SAI_RFR7|FSL_SAI_RMR|FSL_SAI_MCTL|FSL_SAI_MDIV|FSL_SAI_VERID|FSL_SAI_PARAM|FSL_SAI_TTCTN|FSL_SAI_RTCTN|
        FSL_SAI_TTCTL|FSL_SAI_TBCTN|FSL_SAI_TTCAP|FSL_SAI_RTCTL|FSL_SAI_RBCTN|FSL_SAI_RTCAP)
}

unsafe extern "C" fn fsl_sai_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    let sai = dev_get_drvdata(dev);
    let ofs = (*(*sai).soc_data).reg_offset;
    if reg == FSL_SAI_TCSR(ofs) || reg == FSL_SAI_RCSR(ofs) { return true; }
    if ofs == 8 && (reg == FSL_SAI_VERID || reg == FSL_SAI_PARAM) { return true; }
    matches!(reg,
        FSL_SAI_TFR0|FSL_SAI_TFR1|FSL_SAI_TFR2|FSL_SAI_TFR3|FSL_SAI_TFR4|FSL_SAI_TFR5|FSL_SAI_TFR6|FSL_SAI_TFR7|
        FSL_SAI_RFR0|FSL_SAI_RFR1|FSL_SAI_RFR2|FSL_SAI_RFR3|FSL_SAI_RFR4|FSL_SAI_RFR5|FSL_SAI_RFR6|FSL_SAI_RFR7|
        FSL_SAI_RDR0|FSL_SAI_RDR1|FSL_SAI_RDR2|FSL_SAI_RDR3|FSL_SAI_RDR4|FSL_SAI_RDR5|FSL_SAI_RDR6|FSL_SAI_RDR7|
        FSL_SAI_TTCTN|FSL_SAI_RTCTN|FSL_SAI_TTCTL|FSL_SAI_TBCTN|FSL_SAI_TTCAP|FSL_SAI_RTCTL|FSL_SAI_RBCTN|FSL_SAI_RTCAP)
}

unsafe extern "C" fn fsl_sai_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    let sai = dev_get_drvdata(dev);
    let ofs = (*(*sai).soc_data).reg_offset;
    if reg >= FSL_SAI_TCSR(ofs) && reg <= FSL_SAI_TCR5(ofs) { return true; }
    if reg >= FSL_SAI_RCSR(ofs) && reg <= FSL_SAI_RCR5(ofs) { return true; }
    matches!(reg, FSL_SAI_TDR0|FSL_SAI_TDR1|FSL_SAI_TDR2|FSL_SAI_TDR3|FSL_SAI_TDR4|FSL_SAI_TDR5|FSL_SAI_TDR6|FSL_SAI_TDR7|FSL_SAI_TMR|FSL_SAI_RMR|FSL_SAI_MCTL|FSL_SAI_MDIV|FSL_SAI_TTCTL|FSL_SAI_RTCTL)
}

static mut fsl_sai_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: FSL_SAI_RMR,
    reg_defaults: fsl_sai_reg_defaults_ofs0.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&fsl_sai_reg_defaults_ofs0),
    readable_reg: Some(fsl_sai_readable_reg), volatile_reg: Some(fsl_sai_volatile_reg),
    writeable_reg: Some(fsl_sai_writeable_reg), cache_type: REGCACHE_FLAT,
};

unsafe fn fsl_sai_check_version(dev: *mut device) -> c_int {
    let sai = dev_get_drvdata(dev);
    let ofs = (*(*sai).soc_data).reg_offset;
    let mut val: c_uint = 0;
    if FSL_SAI_TCSR(ofs) == FSL_SAI_VERID { return 0; }
    let mut ret = regmap_read((*sai).regmap, FSL_SAI_VERID, &mut val);
    if ret < 0 { return ret; }
    dev_dbg(dev, b"VERID: 0x%016X\n\0".as_ptr() as *const c_char, val);
    (*sai).verid.version = val & (FSL_SAI_VERID_MAJOR_MASK | FSL_SAI_VERID_MINOR_MASK);
    (*sai).verid.version >>= FSL_SAI_VERID_MINOR_SHIFT;
    (*sai).verid.feature = val & FSL_SAI_VERID_FEATURE_MASK;
    ret = regmap_read((*sai).regmap, FSL_SAI_PARAM, &mut val);
    if ret < 0 { return ret; }
    dev_dbg(dev, b"PARAM: 0x%016X\n\0".as_ptr() as *const c_char, val);
    (*sai).param.slot_num = 1 << ((val & FSL_SAI_PARAM_SPF_MASK) >> FSL_SAI_PARAM_SPF_SHIFT);
    (*sai).param.fifo_depth = 1 << ((val & FSL_SAI_PARAM_WPF_MASK) >> FSL_SAI_PARAM_WPF_SHIFT);
    (*sai).param.dataline = val & FSL_SAI_PARAM_DLN_MASK;
    0
}

unsafe fn fsl_sai_reset_hw(dev: *mut device) -> c_int {
    let sai = dev_get_drvdata(dev);
    let ofs = (*(*sai).soc_data).reg_offset;
    let mut ret = regmap_write((*sai).regmap, FSL_SAI_TCSR(ofs), 0);
    if ret != 0 {
        dev_err(dev, b"Failed to clear TCSR: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = regmap_write((*sai).regmap, FSL_SAI_RCSR(ofs), 0);
    if ret != 0 {
        dev_err(dev, b"Failed to clear RCSR: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}

/*
 * Calculate the offset between first two datalines, don't
 * different offset in one case.
 */
unsafe fn fsl_sai_calc_dl_off(dl_mask_in: c_ulong) -> c_uint {
    let dl_mask = dl_mask_in;
    let fbidx = find_first_bit(&dl_mask, FSL_SAI_DL_NUM);
    let nbidx = find_next_bit(&dl_mask, FSL_SAI_DL_NUM, fbidx + 1);
    let offset = nbidx - fbidx - 1;
    if offset < 0 || offset >= (FSL_SAI_DL_NUM as c_int - 1) { 0 } else { offset as c_uint }
}

unsafe fn fsl_sai_read_dlcfg(sai: *mut fsl_sai) -> c_int {
    let pdev = (*sai).pdev;
    let np = (*pdev).dev.of_node;
    let dev = &mut (*pdev).dev as *mut device;
    let propname = b"fsl,dataline\0".as_ptr() as *const c_char;
    let mut elems = of_property_count_u32_elems(np, propname);
    if elems <= 0 { elems = 0; }
    else if elems % 3 != 0 {
        dev_err(dev, b"Number of elements must be divisible to 3.\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let num_cfg = elems / 3;
    let cfg = devm_kcalloc(dev, (num_cfg + 1) as usize, size_of::<fsl_sai_dl_cfg>(), GFP_KERNEL) as *mut fsl_sai_dl_cfg;
    if cfg.is_null() { return -ENOMEM; }

    let soc_dl = BIT((*(*sai).soc_data).pins as c_int) - 1;
    (*cfg.add(0)).type_ = FSL_SAI_DL_DEFAULT;
    (*cfg.add(0)).pins[0] = (*(*sai).soc_data).pins;
    (*cfg.add(0)).mask[0] = soc_dl;
    (*cfg.add(0)).start_off[0] = 0;
    (*cfg.add(0)).next_off[0] = 0;
    (*cfg.add(0)).pins[1] = (*(*sai).soc_data).pins;
    (*cfg.add(0)).mask[1] = soc_dl;
    (*cfg.add(0)).start_off[1] = 0;
    (*cfg.add(0)).next_off[1] = 0;

    let mut index = 0;
    let mut i = 1;
    while i < num_cfg + 1 {
        let mut rx: u32 = 0;
        let mut tx: u32 = 0;
        let mut type_: u32 = 0;
        if of_property_read_u32_index(np, propname, index as c_uint, &mut type_) != 0 { return -EINVAL; }
        index += 1;
        if of_property_read_u32_index(np, propname, index as c_uint, &mut rx) != 0 { return -EINVAL; }
        index += 1;
        if of_property_read_u32_index(np, propname, index as c_uint, &mut tx) != 0 { return -EINVAL; }
        index += 1;
        if (rx & !soc_dl) != 0 || (tx & !soc_dl) != 0 {
            dev_err(dev, b"dataline cfg[%d] setting error, mask is 0x%x\n\0".as_ptr() as *const c_char, i, soc_dl);
            return -EINVAL;
        }
        rx &= soc_dl;
        tx &= soc_dl;
        (*cfg.add(i as usize)).type_ = type_;
        (*cfg.add(i as usize)).pins[0] = hweight8(rx);
        (*cfg.add(i as usize)).mask[0] = rx;
        let mut dl_mask = rx as c_ulong;
        (*cfg.add(i as usize)).start_off[0] = find_first_bit(&mut dl_mask, FSL_SAI_DL_NUM) as u32;
        (*cfg.add(i as usize)).next_off[0] = fsl_sai_calc_dl_off(rx as c_ulong);
        (*cfg.add(i as usize)).pins[1] = hweight8(tx);
        (*cfg.add(i as usize)).mask[1] = tx;
        dl_mask = tx as c_ulong;
        (*cfg.add(i as usize)).start_off[1] = find_first_bit(&mut dl_mask, FSL_SAI_DL_NUM) as u32;
        (*cfg.add(i as usize)).next_off[1] = fsl_sai_calc_dl_off(tx as c_ulong);
        i += 1;
    }
    (*sai).dl_cfg = cfg;
    (*sai).dl_cfg_cnt = num_cfg + 1;
    0
}

unsafe extern "C" fn fsl_sai_runtime_suspend(dev: *mut device) -> c_int {
    let sai = dev_get_drvdata(dev);
    if ((*sai).mclk_streams & BIT(SNDRV_PCM_STREAM_CAPTURE)) != 0 {
        clk_disable_unprepare((*sai).mclk_clk[(*sai).mclk_id[0] as usize]);
    }
    if ((*sai).mclk_streams & BIT(SNDRV_PCM_STREAM_PLAYBACK)) != 0 {
        clk_disable_unprepare((*sai).mclk_clk[(*sai).mclk_id[1] as usize]);
    }
    clk_disable_unprepare((*sai).bus_clk);
    if ((*(*sai).soc_data).flags & PMQOS_CPU_LATENCY) != 0 {
        cpu_latency_qos_remove_request(&mut (*sai).pm_qos_req);
    }
    regcache_cache_only((*sai).regmap, true);
    0
}

unsafe extern "C" fn fsl_sai_runtime_resume(dev: *mut device) -> c_int {
    let sai = dev_get_drvdata(dev);
    let ofs = (*(*sai).soc_data).reg_offset;
    let mut ret = clk_prepare_enable((*sai).bus_clk);
    if ret != 0 {
        dev_err(dev, b"failed to enable bus clock: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if ((*sai).mclk_streams & BIT(SNDRV_PCM_STREAM_PLAYBACK)) != 0 {
        ret = clk_prepare_enable((*sai).mclk_clk[(*sai).mclk_id[1] as usize]);
        if ret != 0 { goto_disable_bus_clk(sai); return ret; }
    }
    if ((*sai).mclk_streams & BIT(SNDRV_PCM_STREAM_CAPTURE)) != 0 {
        ret = clk_prepare_enable((*sai).mclk_clk[(*sai).mclk_id[0] as usize]);
        if ret != 0 {
            if ((*sai).mclk_streams & BIT(SNDRV_PCM_STREAM_PLAYBACK)) != 0 {
                clk_disable_unprepare((*sai).mclk_clk[(*sai).mclk_id[1] as usize]);
            }
            goto_disable_bus_clk(sai);
            return ret;
        }
    }
    if ((*(*sai).soc_data).flags & PMQOS_CPU_LATENCY) != 0 {
        cpu_latency_qos_add_request(&mut (*sai).pm_qos_req, 0);
    }
    regcache_cache_only((*sai).regmap, false);
    regcache_mark_dirty((*sai).regmap);
    regmap_update_bits((*sai).regmap, FSL_SAI_TCSR(ofs), FSL_SAI_CSR_SR, FSL_SAI_CSR_SR);
    regmap_update_bits((*sai).regmap, FSL_SAI_RCSR(ofs), FSL_SAI_CSR_SR, FSL_SAI_CSR_SR);
    usleep_range(1000, 2000);
    regmap_update_bits((*sai).regmap, FSL_SAI_TCSR(ofs), FSL_SAI_CSR_SR, 0);
    regmap_update_bits((*sai).regmap, FSL_SAI_RCSR(ofs), FSL_SAI_CSR_SR, 0);
    ret = regcache_sync((*sai).regmap);
    if ret != 0 {
        if ((*sai).mclk_streams & BIT(SNDRV_PCM_STREAM_CAPTURE)) != 0 {
            clk_disable_unprepare((*sai).mclk_clk[(*sai).mclk_id[0] as usize]);
        }
        if ((*sai).mclk_streams & BIT(SNDRV_PCM_STREAM_PLAYBACK)) != 0 {
            clk_disable_unprepare((*sai).mclk_clk[(*sai).mclk_id[1] as usize]);
        }
        goto_disable_bus_clk(sai);
        return ret;
    }
    if (*(*sai).soc_data).mclk_with_tere && (*sai).mclk_direction_output {
        regmap_update_bits((*sai).regmap, FSL_SAI_TCSR(ofs), FSL_SAI_CSR_TERE, FSL_SAI_CSR_TERE);
    }
    0
}

unsafe fn goto_disable_bus_clk(sai: *mut fsl_sai) {
    clk_disable_unprepare((*sai).bus_clk);
}

unsafe extern "C" fn fsl_sai_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let dev = &mut (*pdev).dev as *mut device;
    let sai = devm_kzalloc(dev, size_of::<fsl_sai>(), GFP_KERNEL) as *mut fsl_sai;
    if sai.is_null() { return -ENOMEM; }
    (*sai).pdev = pdev;
    (*sai).soc_data = of_device_get_match_data(dev);
    (*sai).is_lsb_first = of_property_read_bool(np, b"lsb-first\0".as_ptr() as *const c_char);
    (*sai).is_bit_clock_swap = of_property_read_bool(np, b"fsl,sai-bit-clock-swap\0".as_ptr() as *const c_char);

    let base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut (*sai).res);
    if IS_ERR(base) { return PTR_ERR(base); }
    if (*(*sai).soc_data).reg_offset == 8 {
        fsl_sai_regmap_config.reg_defaults = fsl_sai_reg_defaults_ofs8.as_ptr();
        fsl_sai_regmap_config.max_register = FSL_SAI_MDIV;
        fsl_sai_regmap_config.num_reg_defaults = ARRAY_SIZE(&fsl_sai_reg_defaults_ofs8);
    }
    (*sai).regmap = devm_regmap_init_mmio(dev, base, &mut fsl_sai_regmap_config);
    if IS_ERR((*sai).regmap) {
        dev_err(dev, b"regmap init failed\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*sai).regmap);
    }
    (*sai).bus_clk = devm_clk_get(dev, b"bus\0".as_ptr() as *const c_char);
    if IS_ERR((*sai).bus_clk) && PTR_ERR((*sai).bus_clk) != -EPROBE_DEFER {
        (*sai).bus_clk = devm_clk_get(dev, b"sai\0".as_ptr() as *const c_char);
    }
    if IS_ERR((*sai).bus_clk) {
        dev_err(dev, b"failed to get bus clock: %ld\n\0".as_ptr() as *const c_char, PTR_ERR((*sai).bus_clk));
        return PTR_ERR((*sai).bus_clk);
    }
    let mut i = 1;
    while i < FSL_SAI_MCLK_MAX as c_int {
        let mut tmp = [0 as c_char; 8];
        sprintf(tmp.as_mut_ptr(), b"mclk%d\0".as_ptr() as *const c_char, i);
        (*sai).mclk_clk[i as usize] = devm_clk_get(dev, tmp.as_ptr());
        if IS_ERR((*sai).mclk_clk[i as usize]) {
            dev_err(dev, b"failed to get mclk%d clock: %ld\n\0".as_ptr() as *const c_char, i, PTR_ERR((*sai).mclk_clk[i as usize]));
            (*sai).mclk_clk[i as usize] = ptr::null_mut();
        }
        i += 1;
    }
    if (*(*sai).soc_data).mclk0_is_mclk1 { (*sai).mclk_clk[0] = (*sai).mclk_clk[1]; }
    else { (*sai).mclk_clk[0] = (*sai).bus_clk; }
    fsl_asoc_get_pll_clocks(dev, &mut (*sai).pll8k_clk, &mut (*sai).pll11k_clk);
    fsl_asoc_constrain_rates(&mut (*sai).constraint_rates, &fsl_sai_rate_constraints,
                             (*sai).pll8k_clk, (*sai).pll11k_clk, ptr::null_mut(),
                             (*sai).constraint_rates_list.as_mut_ptr());

    let mut dmas = [0u32; 4];
    let mut ret = of_property_read_u32_array(np, b"dmas\0".as_ptr() as *const c_char, dmas.as_mut_ptr(), 4);
    if !(*(*sai).soc_data).use_edma && ret == 0 && dmas[2] == IMX_DMATYPE_MULTI_SAI {
        (*sai).is_multi_fifo_dma = true;
    }
    ret = fsl_sai_read_dlcfg(sai);
    if ret < 0 {
        dev_err(dev, b"failed to read dlcfg %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    memcpy((*sai).cpu_dai_drv.as_mut_ptr() as *mut c_void, fsl_sai_dai_template.as_ptr() as *const c_void,
           size_of::<snd_soc_dai_driver>() * fsl_sai_dai_template.len());

    (*sai).synchronous[RX] = true;
    (*sai).synchronous[TX] = false;
    (*sai).cpu_dai_drv[0].symmetric_rate = 1;
    (*sai).cpu_dai_drv[0].symmetric_channels = 1;
    (*sai).cpu_dai_drv[0].symmetric_sample_bits = 1;

    if of_property_read_bool(np, b"fsl,sai-synchronous-rx\0".as_ptr() as *const c_char) &&
       of_property_read_bool(np, b"fsl,sai-asynchronous\0".as_ptr() as *const c_char) {
        dev_err(dev, b"invalid binding for synchronous mode\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if of_property_read_bool(np, b"fsl,sai-synchronous-rx\0".as_ptr() as *const c_char) {
        (*sai).synchronous[RX] = false;
        (*sai).synchronous[TX] = true;
    } else if of_property_read_bool(np, b"fsl,sai-asynchronous\0".as_ptr() as *const c_char) {
        (*sai).synchronous[RX] = false;
        (*sai).synchronous[TX] = false;
        (*sai).cpu_dai_drv[0].symmetric_rate = 0;
        (*sai).cpu_dai_drv[0].symmetric_channels = 0;
        (*sai).cpu_dai_drv[0].symmetric_sample_bits = 0;
    }
    (*sai).mclk_direction_output = of_property_read_bool(np, b"fsl,sai-mclk-direction-output\0".as_ptr() as *const c_char);
    if (*sai).mclk_direction_output && of_device_is_compatible(np, b"fsl,imx6ul-sai\0".as_ptr() as *const c_char) {
        let gpr = syscon_regmap_lookup_by_compatible(b"fsl,imx6ul-iomuxc-gpr\0".as_ptr() as *const c_char);
        if IS_ERR(gpr) {
            dev_err(dev, b"cannot find iomuxc registers\n\0".as_ptr() as *const c_char);
            return PTR_ERR(gpr);
        }
        let index = of_alias_get_id(np, b"sai\0".as_ptr() as *const c_char);
        if index < 0 { return index; }
        regmap_update_bits(gpr, IOMUXC_GPR1, MCLK_DIR(index), MCLK_DIR(index));
    }

    (*sai).dma_params_rx.addr = (*(*sai).res).start + FSL_SAI_RDR0 as c_ulong;
    (*sai).dma_params_tx.addr = (*(*sai).res).start + FSL_SAI_TDR0 as c_ulong;
    (*sai).dma_params_rx.maxburst = if (*(*sai).soc_data).max_burst[RX] != 0 { (*(*sai).soc_data).max_burst[RX] } else { FSL_SAI_MAXBURST_RX };
    (*sai).dma_params_tx.maxburst = if (*(*sai).soc_data).max_burst[TX] != 0 { (*(*sai).soc_data).max_burst[TX] } else { FSL_SAI_MAXBURST_TX };
    (*sai).pinctrl = devm_pinctrl_get(dev);

    platform_set_drvdata(pdev, sai);
    pm_runtime_enable(dev);
    if !pm_runtime_enabled(dev) {
        ret = fsl_sai_runtime_resume(dev);
        if ret != 0 { pm_runtime_disable(dev); return ret; }
    }
    ret = pm_runtime_resume_and_get(dev);
    if ret < 0 { pm_runtime_disable(dev); return ret; }
    ret = fsl_sai_check_version(dev);
    if ret < 0 { dev_warn(dev, b"Error reading SAI version: %d\n\0".as_ptr() as *const c_char, ret); }
    ret = fsl_sai_reset_hw(dev);
    if ret < 0 { dev_warn(dev, b"Failed to reset hardware: %d\n\0".as_ptr() as *const c_char, ret); }
    if (*sai).mclk_direction_output && (*(*sai).soc_data).max_register >= FSL_SAI_MCTL {
        regmap_update_bits((*sai).regmap, FSL_SAI_MCTL, FSL_SAI_MCTL_MCLK_EN, FSL_SAI_MCTL_MCLK_EN);
    }
    ret = pm_runtime_put_sync(dev);
    if ret < 0 && ret != -ENOSYS {
        if !pm_runtime_status_suspended(dev) { fsl_sai_runtime_suspend(dev); }
        pm_runtime_disable(dev);
        return ret;
    }
    ret = devm_request_irq(dev, irq, fsl_sai_isr, IRQF_SHARED, (*np).name, sai as *mut c_void);
    if ret != 0 {
        dev_err(dev, b"failed to claim irq %u\n\0".as_ptr() as *const c_char, irq);
        if !pm_runtime_status_suspended(dev) { fsl_sai_runtime_suspend(dev); }
        pm_runtime_disable(dev);
        return ret;
    }
    if of_device_is_compatible(np, b"fsl,imx952-sai\0".as_ptr() as *const c_char) {
        let mut strp: *const c_char = ptr::null();
        if of_property_read_string(np, b"fsl,sai-amix-mode\0".as_ptr() as *const c_char, &mut strp) == 0 {
            let val = if strcmp(strp, b"bypass\0".as_ptr() as *const c_char) == 0 { FSL_SAI_AMIX_BYPASS }
                      else if strcmp(strp, b"audmix\0".as_ptr() as *const c_char) == 0 { FSL_SAI_AMIX_AUDMIX }
                      else { FSL_SAI_AMIX_NONE };
            if val < FSL_SAI_AMIX_NONE {
                ret = scmi_imx_misc_ctrl_set(SCMI_IMX952_CTRL_BYPASS_AUDMIX, val);
                if ret != 0 {
                    dev_err_probe(dev, ret, b"Error setting audmix mode\n\0".as_ptr() as *const c_char);
                    if !pm_runtime_status_suspended(dev) { fsl_sai_runtime_suspend(dev); }
                    pm_runtime_disable(dev);
                    return ret;
                }
            }
        }
    }
    if (*(*sai).soc_data).use_imx_pcm {
        ret = imx_pcm_dma_init(pdev);
        if ret != 0 {
            dev_err_probe(dev, ret, b"PCM DMA init failed\n\0".as_ptr() as *const c_char);
            if !IS_ENABLED(CONFIG_SND_SOC_IMX_PCM_DMA) {
                dev_err(dev, b"Error: You must enable the imx-pcm-dma support!\n\0".as_ptr() as *const c_char);
            }
            if !pm_runtime_status_suspended(dev) { fsl_sai_runtime_suspend(dev); }
            pm_runtime_disable(dev);
            return ret;
        }
    } else {
        ret = devm_snd_dmaengine_pcm_register(dev, ptr::null_mut(), 0);
        if ret != 0 {
            dev_err_probe(dev, ret, b"Registering PCM dmaengine failed\n\0".as_ptr() as *const c_char);
            if !pm_runtime_status_suspended(dev) { fsl_sai_runtime_suspend(dev); }
            pm_runtime_disable(dev);
            return ret;
        }
    }
    ret = devm_snd_soc_register_component(dev, &fsl_component, (*sai).cpu_dai_drv.as_mut_ptr(), ARRAY_SIZE(&fsl_sai_dai_template));
    if ret != 0 {
        if !pm_runtime_status_suspended(dev) { fsl_sai_runtime_suspend(dev); }
        pm_runtime_disable(dev);
    }
    ret
}

unsafe extern "C" fn fsl_sai_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        fsl_sai_runtime_suspend(&mut (*pdev).dev);
    }
}

macro_rules! soc_data {
    ($use_imx_pcm:expr, $use_edma:expr, $fifo_depth:expr, $pins:expr, $reg_offset:expr,
     $mclk0_is_mclk1:expr, $flags:expr, $max_register:expr) => {
        fsl_sai_soc_data { use_imx_pcm: $use_imx_pcm, use_edma: $use_edma, fifo_depth: $fifo_depth, pins: $pins,
            reg_offset: $reg_offset, mclk0_is_mclk1: $mclk0_is_mclk1, flags: $flags,
            max_register: $max_register, mclk_with_tere: false, max_burst: [0, 0] }
    };
}

static fsl_sai_vf610_data: fsl_sai_soc_data = soc_data!(false, false, 32, 1, 0, false, 0, FSL_SAI_RMR);
static fsl_sai_imx6sx_data: fsl_sai_soc_data = soc_data!(true, false, 32, 1, 0, true, 0, FSL_SAI_RMR);
static fsl_sai_imx7ulp_data: fsl_sai_soc_data = soc_data!(true, false, 16, 2, 8, false, PMQOS_CPU_LATENCY, FSL_SAI_RMR);
static fsl_sai_imx8mq_data: fsl_sai_soc_data = soc_data!(true, false, 128, 8, 8, false, 0, FSL_SAI_RMR);
static fsl_sai_imx8qm_data: fsl_sai_soc_data = soc_data!(true, true, 64, 4, 0, false, 0, FSL_SAI_RMR);
static fsl_sai_imx8mm_data: fsl_sai_soc_data = soc_data!(true, false, 128, 8, 8, false, 0, FSL_SAI_MCTL);
static fsl_sai_imx8mn_data: fsl_sai_soc_data = soc_data!(true, false, 128, 8, 8, false, 0, FSL_SAI_MDIV);
static fsl_sai_imx8mp_data: fsl_sai_soc_data = fsl_sai_soc_data { use_imx_pcm: true, use_edma: false, fifo_depth: 128, reg_offset: 8, mclk0_is_mclk1: false, pins: 8, flags: 0, max_register: FSL_SAI_MDIV, mclk_with_tere: true, max_burst: [0, 0] };
static fsl_sai_imx8ulp_data: fsl_sai_soc_data = soc_data!(true, true, 16, 4, 8, false, PMQOS_CPU_LATENCY, FSL_SAI_RTCAP);
static fsl_sai_imx93_data: fsl_sai_soc_data = fsl_sai_soc_data { use_imx_pcm: true, use_edma: true, fifo_depth: 128, reg_offset: 8, mclk0_is_mclk1: false, pins: 4, flags: 0, max_register: FSL_SAI_MCTL, mclk_with_tere: false, max_burst: [8, 8] };
static fsl_sai_imx95_data: fsl_sai_soc_data = fsl_sai_soc_data { use_imx_pcm: true, use_edma: true, fifo_depth: 128, reg_offset: 8, mclk0_is_mclk1: false, pins: 8, flags: 0, max_register: FSL_SAI_MCTL, mclk_with_tere: false, max_burst: [8, 8] };

static fsl_sai_ids: [of_device_id; 13] = [
    of_device_id { compatible: b"fsl,vf610-sai\0".as_ptr() as *const c_char, data: &fsl_sai_vf610_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx6sx-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx6sx_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx6ul-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx6sx_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx7ulp-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx7ulp_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8mq-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx8mq_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8qm-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx8qm_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8mm-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx8mm_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8mp-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx8mp_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8ulp-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx8ulp_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8mn-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx8mn_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx93-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx93_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx95-sai\0".as_ptr() as *const c_char, data: &fsl_sai_imx95_data as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, fsl_sai_ids); */

static fsl_sai_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(fsl_sai_runtime_suspend),
    runtime_resume: Some(fsl_sai_runtime_resume),
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
};

static mut fsl_sai_driver: platform_driver = platform_driver {
    probe: Some(fsl_sai_probe),
    remove: Some(fsl_sai_remove),
    driver: platform_driver_driver {
        name: b"fsl-sai\0".as_ptr() as *const c_char,
        pm: &fsl_sai_pm_ops,
        of_match_table: fsl_sai_ids.as_ptr(),
    },
};
/* module_platform_driver(fsl_sai_driver); */

/* MODULE_DESCRIPTION("Freescale Soc SAI Interface"); */
/* MODULE_AUTHOR("Xiubo Li, <Li.Xiubo@freescale.com>"); */
/* MODULE_ALIAS("platform:fsl-sai"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
