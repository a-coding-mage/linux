// SPDX-License-Identifier: GPL-2.0
// Copyright 2019 NXP
//
// Rust source-level translation of soc/fsl/fsl_xcvr.c.
// C include dependencies intentionally remain external: linux bit/clk/firmware/
// interrupt/module/of_platform/pm_runtime/regmap/reset, ALSA SoC/PCM helpers,
// fsl_xcvr.h, fsl_utils.h, and imx-pcm.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;
type u8_t = u8;
type u32_t = u32;
type ulong_t = usize;
type phys_addr_t = usize;
type irqreturn_t = c_int;

const FSL_XCVR_CAPDS_SIZE: usize = 256;
const SPDIF_NUM_RATES: usize = 7;
const FSL_XCVR_SPDIF_RX_FREQ: u32_t = 175000000;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_int = 0;
const IORESOURCE_MEM: c_uint = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_CTL_ELEM_TYPE_BYTES: c_int = 4;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_int = 5;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_PCM: c_int = 3;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_WRITE: c_uint = 1 << 1;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint =
    SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 1 << 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 2;
const SNDRV_PCM_RATE_KNOT: u32_t = 1 << 31;
const SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE: u64 = 1 << 0;
const REGCACHE_FLAT: c_int = 1;

const FSL_XCVR_MODE_SPDIF: u32_t = 0;
const FSL_XCVR_MODE_ARC: u32_t = 1;
const FSL_XCVR_MODE_EARC: u32_t = 2;

fn BIT(n: c_uint) -> u32_t {
    1u32.wrapping_shl(n)
}

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

fn min_i32(a: c_int, b: c_int) -> c_int {
    if a < b { a } else { b }
}

fn ilog2(mut v: u32_t) -> u32_t {
    let mut r = 0;
    while v > 1 {
        v >>= 1;
        r += 1;
    }
    r
}

fn bitrev32(x: u32_t) -> u32_t {
    x.reverse_bits()
}

// Register constants and bitfield helpers are supplied by fsl_xcvr.h.
extern "C" {
    static FSL_XCVR_VERSION: c_uint;
    static FSL_XCVR_EXT_CTRL: c_uint;
    static FSL_XCVR_EXT_STATUS: c_uint;
    static FSL_XCVR_EXT_IER0: c_uint;
    static FSL_XCVR_EXT_IER1: c_uint;
    static FSL_XCVR_EXT_ISR: c_uint;
    static FSL_XCVR_EXT_ISR_SET: c_uint;
    static FSL_XCVR_EXT_ISR_CLR: c_uint;
    static FSL_XCVR_EXT_ISR_TOG: c_uint;
    static FSL_XCVR_IER: c_uint;
    static FSL_XCVR_ISR: c_uint;
    static FSL_XCVR_ISR_SET: c_uint;
    static FSL_XCVR_ISR_CLR: c_uint;
    static FSL_XCVR_ISR_TOG: c_uint;
    static FSL_XCVR_PHY_AI_CTRL: c_uint;
    static FSL_XCVR_PHY_AI_CTRL_SET: c_uint;
    static FSL_XCVR_PHY_AI_CTRL_CLR: c_uint;
    static FSL_XCVR_PHY_AI_CTRL_TOG: c_uint;
    static FSL_XCVR_PHY_AI_WDATA: c_uint;
    static FSL_XCVR_PHY_AI_RDATA: c_uint;
    static FSL_XCVR_CLK_CTRL: c_uint;
    static FSL_XCVR_RX_DPTH_CTRL: c_uint;
    static FSL_XCVR_RX_DPTH_CTRL_SET: c_uint;
    static FSL_XCVR_RX_DPTH_CTRL_CLR: c_uint;
    static FSL_XCVR_RX_DPTH_CTRL_TOG: c_uint;
    static FSL_XCVR_RX_CS_DATA_0: c_uint;
    static FSL_XCVR_RX_CS_DATA_1: c_uint;
    static FSL_XCVR_RX_CS_DATA_2: c_uint;
    static FSL_XCVR_RX_CS_DATA_3: c_uint;
    static FSL_XCVR_RX_CS_DATA_4: c_uint;
    static FSL_XCVR_RX_CS_DATA_5: c_uint;
    static FSL_XCVR_RX_DPTH_CNTR_CTRL: c_uint;
    static FSL_XCVR_RX_DPTH_CNTR_CTRL_SET: c_uint;
    static FSL_XCVR_RX_DPTH_CNTR_CTRL_CLR: c_uint;
    static FSL_XCVR_RX_DPTH_CNTR_CTRL_TOG: c_uint;
    static FSL_XCVR_RX_DPTH_TSCR: c_uint;
    static FSL_XCVR_RX_DPTH_BCR: c_uint;
    static FSL_XCVR_RX_DPTH_BCTR: c_uint;
    static FSL_XCVR_RX_DPTH_BCRR: c_uint;
    static FSL_XCVR_TX_DPTH_CTRL: c_uint;
    static FSL_XCVR_TX_DPTH_CTRL_SET: c_uint;
    static FSL_XCVR_TX_DPTH_CTRL_CLR: c_uint;
    static FSL_XCVR_TX_DPTH_CTRL_TOG: c_uint;
    static FSL_XCVR_TX_CS_DATA_0: c_uint;
    static FSL_XCVR_TX_CS_DATA_1: c_uint;
    static FSL_XCVR_TX_CS_DATA_2: c_uint;
    static FSL_XCVR_TX_CS_DATA_3: c_uint;
    static FSL_XCVR_TX_CS_DATA_4: c_uint;
    static FSL_XCVR_TX_CS_DATA_5: c_uint;
    static FSL_XCVR_TX_DPTH_CNTR_CTRL: c_uint;
    static FSL_XCVR_TX_DPTH_CNTR_CTRL_SET: c_uint;
    static FSL_XCVR_TX_DPTH_CNTR_CTRL_CLR: c_uint;
    static FSL_XCVR_TX_DPTH_CNTR_CTRL_TOG: c_uint;
    static FSL_XCVR_TX_DPTH_TSCR: c_uint;
    static FSL_XCVR_TX_DPTH_BCR: c_uint;
    static FSL_XCVR_TX_DPTH_BCTR: c_uint;
    static FSL_XCVR_TX_DPTH_BCRR: c_uint;
    static FSL_XCVR_DEBUG_REG_0: c_uint;
    static FSL_XCVR_DEBUG_REG_1: c_uint;
    static FSL_XCVR_MAX_REG: c_uint;

    static FSL_XCVR_PHY_CTRL: c_uint;
    static FSL_XCVR_PHY_STATUS: c_uint;
    static FSL_XCVR_PHY_ANALOG_TRIM: c_uint;
    static FSL_XCVR_PHY_SLEW_RATE_TRIM: c_uint;
    static FSL_XCVR_PHY_DATA_TEST_DELAY: c_uint;
    static FSL_XCVR_PHY_TEST_CTRL: c_uint;
    static FSL_XCVR_PHY_DIFF_CDR_CTRL: c_uint;
    static FSL_XCVR_PHY_CTRL2: c_uint;
    static FSL_XCVR_PHY_CTRL2_TOG: c_uint;
    static FSL_XCVR_PLL_STAT0_TOG: c_uint;
    static FSL_XCVR_GP_PLL_STATUS_TOG: c_uint;
}

extern "C" {
    fn FSL_XCVR_PLL_PDIVx(log2: u32_t, idx: u32_t) -> u32_t;
    fn FSL_XCVR_EXT_CTRL_CMDC_RESET(tx: bool_t) -> u32_t;
    fn FSL_XCVR_EXT_CTRL_DPTH_RESET(tx: bool_t) -> u32_t;
    fn FSL_XCVR_EXT_CTRL_DMA_DIS(tx: bool_t) -> u32_t;
    fn FSL_XCVR_EXT_CTRL_PAGE(page: c_int) -> u32_t;
    fn FSL_XCVR_EXT_CTRL_RX_FWM(v: u32_t) -> u32_t;
    fn FSL_XCVR_EXT_CTRL_TX_FWM(v: u32_t) -> u32_t;
}

extern "C" {
    static FSL_XCVR_PHY_AI_CTRL_AI_RWB: u32_t;
    static FSL_XCVR_PHY_AI_CTRL_AI_RESETN: u32_t;
    static FSL_XCVR_PLL_BANDGAP: c_uint;
    static FSL_XCVR_PLL_BANDGAP_EN_VBG: u32_t;
    static FSL_XCVR_PLL_CTRL0: c_uint;
    static FSL_XCVR_PLL_NUM: c_uint;
    static FSL_XCVR_PLL_DEN: c_uint;
    static FSL_XCVR_PLL_PDIV: c_uint;
    static FSL_XCVR_PLL_CTRL0_HROFF: u32_t;
    static FSL_XCVR_PLL_CTRL0_PWP: u32_t;
    static FSL_XCVR_PLL_CTRL0_CM0_EN: u32_t;
    static FSL_XCVR_PLL_CTRL0_CM1_EN: u32_t;
    static FSL_XCVR_PLL_CTRL0_CM2_EN: u32_t;
    static FSL_XCVR_GP_PLL_DIV_MFI_SHIFT: c_uint;
    static FSL_XCVR_GP_PLL_NUMERATOR_MFN_SHIFT: c_uint;
    static FSL_XCVR_GP_PLL_DIV: c_uint;
    static FSL_XCVR_GP_PLL_NUMERATOR: c_uint;
    static FSL_XCVR_GP_PLL_DENOMINATOR: c_uint;
    static FSL_XCVR_GP_PLL_CTRL: c_uint;
    static FSL_XCVR_GP_PLL_CTRL_POWERUP: u32_t;
    static FSL_XCVR_GP_PLL_CTRL_CLKMUX_EN: u32_t;
    static FSL_XCVR_PHY_CTRL_TSDIFF_OE: u32_t;
    static FSL_XCVR_PHY_CTRL_PHY_EN: u32_t;
    static FSL_XCVR_PHY_CTRL2_EARC_TXMS: u32_t;
    static FSL_XCVR_PHY_CTRL_SPDIF_EN: u32_t;
    static FSL_XCVR_PHY_CTRL_RX_CM_EN: u32_t;
    static FSL_XCVR_PHY_CTRL_ARC_MODE_SE_EN: u32_t;
    static FSL_XCVR_PHY_CTRL_ARC_MODE_CM_EN: u32_t;
    static FSL_XCVR_PHY_CTRL_TX_CLK_AUD_SS: u32_t;
    static FSL_XCVR_TX_DPTH_CTRL_BYPASS_FEM: u32_t;
    static FSL_XCVR_TX_DPTH_CTRL_FRM_FMT: u32_t;
    static FSL_XCVR_TX_DPTH_CTRL_STRT_DATA_TX: u32_t;
    static FSL_XCVR_RX_DPTH_CTRL_STORE_FMT: u32_t;
    static FSL_XCVR_RX_DPTH_CTRL_CLR_RX_FIFO: u32_t;
    static FSL_XCVR_RX_DPTH_CTRL_COMP: u32_t;
    static FSL_XCVR_RX_DPTH_CTRL_LAYB_CTRL: u32_t;
    static FSL_XCVR_RX_DPTH_CTRL_CSA: u32_t;
    static FSL_XCVR_EXT_CTRL_SPDIF_MODE: u32_t;
    static FSL_XCVR_EXT_CTRL_TX_RX_MODE: u32_t;
    static FSL_XCVR_EXT_CTRL_RX_FWM_MASK: u32_t;
    static FSL_XCVR_EXT_CTRL_TX_FWM_MASK: u32_t;
    static FSL_XCVR_EXT_CTRL_DMA_RD_DIS: u32_t;
    static FSL_XCVR_EXT_CTRL_DMA_WR_DIS: u32_t;
    static FSL_XCVR_EXT_CTRL_PAGE_MASK: u32_t;
    static FSL_XCVR_EXT_CTRL_CORE_RESET: u32_t;
    static FSL_XCVR_EXT_CTRL_RX_DPTH_RESET: u32_t;
    static FSL_XCVR_FIFO_WMK_RX: u32_t;
    static FSL_XCVR_FIFO_WMK_TX: u32_t;
    static FSL_XCVR_REG_OFFSET: c_int;
    static FSL_XCVR_CAP_DATA_STR: usize;
    static FSL_XCVR_RX_CS_CTRL_0: usize;
    static FSL_XCVR_RX_CS_CTRL_1: usize;
    static FSL_XCVR_RX_CS_BUFF_0: usize;
    static FSL_XCVR_RX_CS_BUFF_1: usize;
    static FSL_XCVR_MAXBURST_RX: u32_t;
    static FSL_XCVR_MAXBURST_TX: u32_t;
    static FSL_XCVR_IRQ_EARC_ALL: u32_t;
    static FSL_XCVR_IRQ_NEW_CS: u32_t;
    static FSL_XCVR_IRQ_NEW_UD: u32_t;
    static FSL_XCVR_IRQ_MUTE: u32_t;
    static FSL_XCVR_IRQ_FIFO_UOFL_ERR: u32_t;
    static FSL_XCVR_IRQ_ARC_MODE: u32_t;
    static FSL_XCVR_IRQ_DMA_RD_REQ: u32_t;
    static FSL_XCVR_IRQ_DMA_WR_REQ: u32_t;
    static FSL_XCVR_IRQ_CMDC_STATUS_UPD: u32_t;
    static FSL_XCVR_IRQ_PREAMBLE_MISMATCH: u32_t;
    static FSL_XCVR_IRQ_UNEXP_PRE_REC: u32_t;
    static FSL_XCVR_IRQ_M_W_PRE_MISMATCH: u32_t;
    static FSL_XCVR_IRQ_B_PRE_MISMATCH: u32_t;
    static FSL_XCVR_ISR_CMDC_TX_EN: u32_t;
}

#[repr(C)]
struct device { _private: [u8; 0] }
#[repr(C)]
struct platform_device { dev: device, name: *const c_char }
#[repr(C)]
struct regmap { _private: [u8; 0] }
#[repr(C)]
struct clk { _private: [u8; 0] }
#[repr(C)]
struct reset_control { _private: [u8; 0] }
#[repr(C)]
struct firmware { size: c_int, data: *const u8_t }
#[repr(C)]
struct resource { start: phys_addr_t }
#[repr(C)]
struct work_struct { _private: [u8; 0] }
#[repr(C)]
struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
struct snd_card { controls_rwsem: rw_semaphore }
#[repr(C)]
struct rw_semaphore { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_card { snd_card: *mut snd_card, dai_link: *mut c_void }
#[repr(C)]
struct snd_soc_component { card: *mut snd_soc_card }
#[repr(C)]
struct snd_soc_dai { component: *mut snd_soc_component, dev: *mut device }
#[repr(C)]
struct snd_soc_pcm_runtime { pcm: *mut snd_pcm }
#[repr(C)]
struct snd_pcm { streams: [snd_pcm_str; 2] }
#[repr(C)]
struct snd_pcm_str { substream_count: c_int }
#[repr(C)]
struct snd_pcm_runtime { rate: u32_t, channels: u32_t }
#[repr(C)]
struct snd_pcm_substream { stream: c_int, runtime: *mut snd_pcm_runtime }
#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    chan_name: *const c_char,
    addr: phys_addr_t,
    maxburst: u32_t,
}
#[repr(C)]
struct snd_aes_iec958 { status: [u8_t; 24] }
#[repr(C)]
struct snd_ctl_elem_id { _private: [u8; 0] }
#[repr(C)]
struct snd_kcontrol_volatile { access: c_uint }
#[repr(C)]
struct snd_kcontrol {
    private_value: ulong_t,
    vd: [snd_kcontrol_volatile; 1],
    id: snd_ctl_elem_id,
}
#[repr(C)]
struct enumerated_value { item: [c_uint; 4] }
#[repr(C)]
struct bytes_value { data: [u8_t; FSL_XCVR_CAPDS_SIZE] }
#[repr(C)]
struct iec958_value { status: [u8_t; 24] }
#[repr(C)]
union snd_ctl_elem_value_union {
    enumerated: enumerated_value,
    bytes: bytes_value,
    iec958: iec958_value,
}
#[repr(C)]
struct snd_ctl_elem_value { value: snd_ctl_elem_value_union }
#[repr(C)]
struct snd_ctl_elem_info { type_: c_int, count: c_uint }
#[repr(C)]
struct soc_enum { _private: [u8; 0] }
#[repr(C)]
struct snd_kcontrol_new {
    iface: c_int,
    name: *const c_char,
    access: c_uint,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: ulong_t,
}
#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const u32_t,
    mask: u32_t,
}
#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}
#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    rates: u32_t,
    formats: u64,
}
#[repr(C)]
struct snd_soc_dai_driver {
    ops: *const snd_soc_dai_ops,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
}
#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    legacy_dai_naming: c_uint,
}
#[repr(C)]
struct reg_default { reg: c_uint, def: c_uint }
#[repr(C)]
struct regmap_config {
    name: *const c_char,
    reg_bits: c_uint,
    reg_stride: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    cache_type: c_int,
    reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
}
#[repr(C)]
struct of_device_id { compatible: *const c_char, data: *const c_void }
#[repr(C)]
struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: platform_driver_inner,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}
#[repr(C)]
struct platform_driver_inner {
    name: *const c_char,
    pm: *const dev_pm_ops,
    of_match_table: *const of_device_id,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum fsl_xcvr_pll_verison {
    PLL_MX8MP,
    PLL_MX95,
}

#[repr(C)]
struct fsl_xcvr_soc_data {
    fw_name: *const c_char,
    spdif_only: bool_t,
    use_edma: bool_t,
    use_phy: bool_t,
    pll_ver: fsl_xcvr_pll_verison,
}

#[repr(C)]
struct fsl_xcvr {
    soc_data: *const fsl_xcvr_soc_data,
    pdev: *mut platform_device,
    regmap: *mut regmap,
    regmap_phy: *mut regmap,
    regmap_pll: *mut regmap,
    ipg_clk: *mut clk,
    pll_ipg_clk: *mut clk,
    phy_clk: *mut clk,
    spba_clk: *mut clk,
    pll8k_clk: *mut clk,
    pll11k_clk: *mut clk,
    reset: *mut reset_control,
    streams: u8_t,
    mode: u32_t,
    arc_mode: u32_t,
    ram_addr: *mut c_void,
    dma_prms_rx: snd_dmaengine_dai_dma_data,
    dma_prms_tx: snd_dmaengine_dai_dma_data,
    rx_iec958: snd_aes_iec958,
    tx_iec958: snd_aes_iec958,
    cap_ds: [u8_t; FSL_XCVR_CAPDS_SIZE],
    work_rst: work_struct,
    lock: spinlock_t, /* Protect hw_reset and trigger */
    spdif_constr_rates: snd_pcm_hw_constraint_list,
    spdif_constr_rates_list: [u32_t; SPDIF_NUM_RATES],
}

#[repr(C)]
struct fsl_xcvr_pll_conf {
    mfi: u8_t,   /* min=0x18, max=0x38 */
    mfn: u32_t,  /* signed int, 2's compl., min=0x3FFF0000, max=0x00010000 */
    mfd: u32_t,  /* unsigned int */
    fout: u32_t, /* Fout = Fref*(MFI + MFN/MFD), Fref is 24MHz */
}

static inc_mode: [*const c_char; 2] = [
    b"On enabled and bitcount increment\0".as_ptr() as *const c_char,
    b"On enabled\0".as_ptr() as *const c_char,
];

// SOC_ENUM_SINGLE_DECL, FSL_ASOC_* and SOC_ENUM_EXT generated control metadata
// from the C source is represented here by the same named Rust statics.
static transmit_tstmp_enum: soc_enum = soc_enum { _private: [] };
static receive_tstmp_enum: soc_enum = soc_enum { _private: [] };
static fsl_xcvr_timestamp_ctrls: [snd_kcontrol_new; 16] = [snd_kcontrol_new {
    iface: 0, name: ptr::null(), access: 0, info: None, get: None, put: None, private_value: 0,
}; 16];

static fsl_xcvr_pll_cfg: [fsl_xcvr_pll_conf; 4] = [
    fsl_xcvr_pll_conf { mfi: 54, mfn: 1,  mfd: 6,   fout: 1300000000 }, /* 1.3 GHz */
    fsl_xcvr_pll_conf { mfi: 32, mfn: 96, mfd: 125, fout: 786432000 },  /* 8000 Hz */
    fsl_xcvr_pll_conf { mfi: 30, mfn: 66, mfd: 625, fout: 722534400 },  /* 11025 Hz */
    fsl_xcvr_pll_conf { mfi: 29, mfn: 1,  mfd: 6,   fout: 700000000 },  /* 700 MHz */
];

/*
 * HDMI2.1 spec defines 6- and 12-channels layout for one bit audio
 * stream. Todo: to check how this case can be considered below
 */
static fsl_xcvr_earc_channels: [u32_t; 5] = [1, 2, 8, 16, 32];
static fsl_xcvr_earc_channels_constr: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list { count: 5, list: fsl_xcvr_earc_channels.as_ptr(), mask: 0 };
static fsl_xcvr_earc_rates: [u32_t; 18] = [
    32000, 44100, 48000, 64000, 88200, 96000,
    128000, 176400, 192000, 256000, 352800, 384000,
    512000, 705600, 768000, 1024000, 1411200, 1536000,
];
static fsl_xcvr_earc_rates_constr: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list { count: 18, list: fsl_xcvr_earc_rates.as_ptr(), mask: 0 };
static fsl_xcvr_spdif_channels: [u32_t; 1] = [2];
static fsl_xcvr_spdif_channels_constr: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list { count: 1, list: fsl_xcvr_spdif_channels.as_ptr(), mask: 0 };
static fsl_xcvr_spdif_rates: [u32_t; 7] = [32000, 44100, 48000, 88200, 96000, 176400, 192000];
static fsl_xcvr_spdif_rates_constr: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list { count: 7, list: fsl_xcvr_spdif_rates.as_ptr(), mask: 0 };

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_int;
    fn snd_soc_card_get_kcontrol(card: *mut snd_soc_card, name: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *const snd_ctl_elem_id);
    fn snd_soc_get_pcm_runtime(card: *mut snd_soc_card, link: *mut c_void) -> *mut snd_soc_pcm_runtime;
    fn snd_pcm_hw_constraint_list(rt: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_constraint_step(rt: *mut snd_pcm_runtime, cond: c_uint, var: c_int, step: u32_t) -> c_int;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, tx: *mut snd_dmaengine_dai_dma_data, rx: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_add_dai_controls(dai: *mut snd_soc_dai, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn lockdep_assert_held(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn udelay(usec: c_uint);
    fn msleep(msec: c_uint);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, n: usize);
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, n: usize);
    fn memset_io(dst: *mut c_void, val: c_int, n: usize);
    fn writel_relaxed(val: u32_t, addr: *mut c_void);
    fn schedule_work(work: *mut work_struct) -> bool_t;
    fn cancel_work_sync(work: *mut work_struct) -> bool_t;
}

unsafe extern "C" fn fsl_xcvr_arc_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let val = snd_soc_enum_item_to_val(e, *item.add(0));
    if val < 0 || val > 1 { return -EINVAL; }
    let ret = ((*xcvr).arc_mode != val as u32_t) as c_int;
    (*xcvr).arc_mode = val as u32_t;
    ret
}

unsafe extern "C" fn fsl_xcvr_arc_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    (*ucontrol).value.enumerated.item[0] = (*xcvr).arc_mode;
    0
}

static fsl_xcvr_phy_arc_cfg: [u32_t; 2] = unsafe {
    [FSL_XCVR_PHY_CTRL_ARC_MODE_SE_EN, FSL_XCVR_PHY_CTRL_ARC_MODE_CM_EN]
};
static fsl_xcvr_arc_mode: [*const c_char; 2] = [
    b"Single Ended\0".as_ptr() as *const c_char,
    b"Common\0".as_ptr() as *const c_char,
];
static fsl_xcvr_arc_mode_enum: soc_enum = soc_enum { _private: [] };
static mut fsl_xcvr_arc_mode_kctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0, name: b"ARC Mode\0".as_ptr() as *const c_char, access: 0,
    info: None, get: Some(fsl_xcvr_arc_mode_get), put: Some(fsl_xcvr_arc_mode_put), private_value: 0,
};

/* Capabilities data structure, bytes */
unsafe extern "C" fn fsl_xcvr_type_capds_bytes_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = FSL_XCVR_CAPDS_SIZE as c_uint;
    0
}

unsafe extern "C" fn fsl_xcvr_capds_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    memcpy((*ucontrol).value.bytes.data.as_mut_ptr() as *mut c_void, (*xcvr).cap_ds.as_ptr() as *const c_void, FSL_XCVR_CAPDS_SIZE);
    0
}

unsafe extern "C" fn fsl_xcvr_capds_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    let changed = (memcmp((*xcvr).cap_ds.as_ptr() as *const c_void, (*ucontrol).value.bytes.data.as_ptr() as *const c_void, size_of::<[u8_t; FSL_XCVR_CAPDS_SIZE]>()) != 0) as c_int;
    memcpy((*xcvr).cap_ds.as_mut_ptr() as *mut c_void, (*ucontrol).value.bytes.data.as_ptr() as *const c_void, size_of::<[u8_t; FSL_XCVR_CAPDS_SIZE]>());
    changed
}

static mut fsl_xcvr_earc_capds_kctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Capabilities Data Structure\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(fsl_xcvr_type_capds_bytes_info),
    get: Some(fsl_xcvr_capds_get),
    put: Some(fsl_xcvr_capds_put),
    private_value: 0,
};

unsafe fn fsl_xcvr_activate_ctl(dai: *mut snd_soc_dai, name: *const c_char, active: bool_t) -> c_int {
    let card = (*(*dai).component).card;
    lockdep_assert_held(&mut (*(*card).snd_card).controls_rwsem);
    let kctl = snd_soc_card_get_kcontrol(card, name);
    if kctl.is_null() { return -ENOENT; }
    let enabled = ((*kctl).vd[0].access & SNDRV_CTL_ELEM_ACCESS_WRITE) != 0;
    if active == enabled { return 0; /* nothing to do */ }
    if active {
        (*kctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_WRITE;
    } else {
        (*kctl).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_WRITE;
    }
    snd_ctl_notify((*card).snd_card, SNDRV_CTL_EVENT_MASK_INFO, &(*kctl).id);
    1
}

unsafe extern "C" fn fsl_xcvr_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let val = snd_soc_enum_item_to_val(e, *item.add(0));
    let card = (*(*dai).component).card;
    if val < FSL_XCVR_MODE_SPDIF as c_int || val > FSL_XCVR_MODE_EARC as c_int { return -EINVAL; }
    let ret = ((*xcvr).mode != val as u32_t) as c_int;
    (*xcvr).mode = val as u32_t;
    fsl_xcvr_activate_ctl(dai, fsl_xcvr_arc_mode_kctl.name, (*xcvr).mode == FSL_XCVR_MODE_ARC);
    fsl_xcvr_activate_ctl(dai, fsl_xcvr_earc_capds_kctl.name, (*xcvr).mode == FSL_XCVR_MODE_EARC);
    /* Allow playback for SPDIF only */
    let rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link);
    (*(*rtd).pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream_count =
        if (*xcvr).mode == FSL_XCVR_MODE_SPDIF { 1 } else { 0 };
    ret
}

unsafe extern "C" fn fsl_xcvr_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    (*ucontrol).value.enumerated.item[0] = (*xcvr).mode;
    0
}

static fsl_xcvr_mode: [*const c_char; 3] = [
    b"SPDIF\0".as_ptr() as *const c_char,
    b"ARC RX\0".as_ptr() as *const c_char,
    b"eARC\0".as_ptr() as *const c_char,
];
static fsl_xcvr_mode_enum: soc_enum = soc_enum { _private: [] };
static mut fsl_xcvr_mode_kctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0, name: b"XCVR Mode\0".as_ptr() as *const c_char, access: 0,
    info: None, get: Some(fsl_xcvr_mode_get), put: Some(fsl_xcvr_mode_put), private_value: 0,
};

/** phy: true => phy, false => pll */
unsafe fn fsl_xcvr_ai_write(xcvr: *mut fsl_xcvr, reg: u8_t, data: u32_t, phy: bool_t) -> c_int {
    let dev = &mut (*(*xcvr).pdev).dev as *mut device;
    let mut val: u32_t = 0;
    let idx = BIT(if phy { 26 } else { 24 });
    let tidx = BIT(if phy { 27 } else { 25 });
    regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_CLR, 0xFF | FSL_XCVR_PHY_AI_CTRL_AI_RWB);
    regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_SET, reg as u32_t);
    regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_WDATA, data);
    regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_TOG, idx);
    let mut ret = 0;
    let mut tries = 1000;
    while tries > 0 {
        regmap_read((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL, &mut val);
        if (val & idx) == ((val & tidx) >> 1) { break; }
        tries -= 1;
    }
    if tries == 0 { ret = -EINVAL; }
    if ret != 0 {
        dev_err(dev, b"AI timeout: failed to set %s reg 0x%02x=0x%08x\n\0".as_ptr() as *const c_char,
                if phy { b"PHY\0".as_ptr() } else { b"PLL\0".as_ptr() }, reg as c_uint, data);
    }
    ret
}

unsafe fn fsl_xcvr_ai_read(xcvr: *mut fsl_xcvr, reg: u8_t, data: *mut u32_t, phy: bool_t) -> c_int {
    let dev = &mut (*(*xcvr).pdev).dev as *mut device;
    let mut val: u32_t = 0;
    let idx = BIT(if phy { 26 } else { 24 });
    let tidx = BIT(if phy { 27 } else { 25 });
    regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_CLR, 0xFF | FSL_XCVR_PHY_AI_CTRL_AI_RWB);
    regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_SET, reg as u32_t | FSL_XCVR_PHY_AI_CTRL_AI_RWB);
    regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_TOG, idx);
    let mut ret = 0;
    let mut tries = 1000;
    while tries > 0 {
        regmap_read((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL, &mut val);
        if (val & idx) == ((val & tidx) >> 1) { break; }
        tries -= 1;
    }
    if tries == 0 { ret = -EINVAL; }
    if ret != 0 {
        dev_err(dev, b"AI timeout: failed to read %s reg 0x%02x\n\0".as_ptr() as *const c_char,
                if phy { b"PHY\0".as_ptr() } else { b"PLL\0".as_ptr() }, reg as c_uint);
    }
    regmap_read((*xcvr).regmap, FSL_XCVR_PHY_AI_RDATA, data);
    ret
}

unsafe extern "C" fn fsl_xcvr_phy_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    fsl_xcvr_ai_read(context as *mut fsl_xcvr, reg as u8_t, val, true)
}
unsafe extern "C" fn fsl_xcvr_phy_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    fsl_xcvr_ai_write(context as *mut fsl_xcvr, reg as u8_t, val, true)
}
unsafe extern "C" fn fsl_xcvr_pll_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    fsl_xcvr_ai_read(context as *mut fsl_xcvr, reg as u8_t, val, false)
}
unsafe extern "C" fn fsl_xcvr_pll_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    fsl_xcvr_ai_write(context as *mut fsl_xcvr, reg as u8_t, val, false)
}

unsafe fn fsl_xcvr_en_phy_pll(xcvr: *mut fsl_xcvr, freq: u32_t, tx: bool_t) -> c_int {
    let dev = &mut (*(*xcvr).pdev).dev as *mut device;
    let mut i: usize = 0;
    let mut div: u32_t = 0;
    let mut val: u32_t;
    let mut ret: c_int;
    if !(*(*xcvr).soc_data).use_phy { return 0; }
    while i < fsl_xcvr_pll_cfg.len() {
        if fsl_xcvr_pll_cfg[i].fout % freq == 0 {
            div = fsl_xcvr_pll_cfg[i].fout / freq;
            break;
        }
        i += 1;
    }
    if div == 0 || i >= fsl_xcvr_pll_cfg.len() { return -EINVAL; }
    let log2 = ilog2(div);
    /* Release AI interface from reset */
    ret = regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_SET, FSL_XCVR_PHY_AI_CTRL_AI_RESETN);
    if ret < 0 { dev_err(dev, b"Error while setting IER0: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    match (*(*xcvr).soc_data).pll_ver {
        fsl_xcvr_pll_verison::PLL_MX8MP => {
            /* PLL: BANDGAP_SET: EN_VBG (enable bandgap) */
            regmap_set_bits((*xcvr).regmap_pll, FSL_XCVR_PLL_BANDGAP, FSL_XCVR_PLL_BANDGAP_EN_VBG);
            /* PLL: CTRL0: DIV_INTEGER */
            regmap_write((*xcvr).regmap_pll, FSL_XCVR_PLL_CTRL0, fsl_xcvr_pll_cfg[i].mfi as u32_t);
            /* PLL: NUMERATOR: MFN */
            regmap_write((*xcvr).regmap_pll, FSL_XCVR_PLL_NUM, fsl_xcvr_pll_cfg[i].mfn);
            /* PLL: DENOMINATOR: MFD */
            regmap_write((*xcvr).regmap_pll, FSL_XCVR_PLL_DEN, fsl_xcvr_pll_cfg[i].mfd);
            /* PLL: CTRL0_SET: HOLD_RING_OFF, POWER_UP */
            regmap_set_bits((*xcvr).regmap_pll, FSL_XCVR_PLL_CTRL0, FSL_XCVR_PLL_CTRL0_HROFF | FSL_XCVR_PLL_CTRL0_PWP);
            udelay(25);
            /* PLL: CTRL0: Clear Hold Ring Off */
            regmap_clear_bits((*xcvr).regmap_pll, FSL_XCVR_PLL_CTRL0, FSL_XCVR_PLL_CTRL0_HROFF);
            udelay(100);
            if tx { /* TX is enabled for SPDIF only */
                /* PLL: POSTDIV: PDIV0 */
                regmap_write((*xcvr).regmap_pll, FSL_XCVR_PLL_PDIV, FSL_XCVR_PLL_PDIVx(log2, 0));
                /* PLL: CTRL_SET: CLKMUX0_EN */
                regmap_set_bits((*xcvr).regmap_pll, FSL_XCVR_PLL_CTRL0, FSL_XCVR_PLL_CTRL0_CM0_EN);
            } else if (*xcvr).mode == FSL_XCVR_MODE_EARC { /* eARC RX */
                /* PLL: POSTDIV: PDIV1 */
                regmap_write((*xcvr).regmap_pll, FSL_XCVR_PLL_PDIV, FSL_XCVR_PLL_PDIVx(log2, 1));
                /* PLL: CTRL_SET: CLKMUX1_EN */
                regmap_set_bits((*xcvr).regmap_pll, FSL_XCVR_PLL_CTRL0, FSL_XCVR_PLL_CTRL0_CM1_EN);
            } else { /* SPDIF / ARC RX */
                /* PLL: POSTDIV: PDIV2 */
                regmap_write((*xcvr).regmap_pll, FSL_XCVR_PLL_PDIV, FSL_XCVR_PLL_PDIVx(log2, 2));
                /* PLL: CTRL_SET: CLKMUX2_EN */
                regmap_set_bits((*xcvr).regmap_pll, FSL_XCVR_PLL_CTRL0, FSL_XCVR_PLL_CTRL0_CM2_EN);
            }
        }
        fsl_xcvr_pll_verison::PLL_MX95 => {
            val = ((fsl_xcvr_pll_cfg[i].mfi as u32_t) << FSL_XCVR_GP_PLL_DIV_MFI_SHIFT) | div;
            regmap_write((*xcvr).regmap_pll, FSL_XCVR_GP_PLL_DIV, val);
            val = fsl_xcvr_pll_cfg[i].mfn << FSL_XCVR_GP_PLL_NUMERATOR_MFN_SHIFT;
            regmap_write((*xcvr).regmap_pll, FSL_XCVR_GP_PLL_NUMERATOR, val);
            regmap_write((*xcvr).regmap_pll, FSL_XCVR_GP_PLL_DENOMINATOR, fsl_xcvr_pll_cfg[i].mfd);
            val = FSL_XCVR_GP_PLL_CTRL_POWERUP | FSL_XCVR_GP_PLL_CTRL_CLKMUX_EN;
            regmap_write((*xcvr).regmap_pll, FSL_XCVR_GP_PLL_CTRL, val);
        }
    }
    if (*xcvr).mode == FSL_XCVR_MODE_EARC { /* eARC mode */
        /* PHY: CTRL_SET: TX_DIFF_OE, PHY_EN */
        regmap_set_bits((*xcvr).regmap_phy, FSL_XCVR_PHY_CTRL, FSL_XCVR_PHY_CTRL_TSDIFF_OE | FSL_XCVR_PHY_CTRL_PHY_EN);
        /* PHY: CTRL2_SET: EARC_TX_MODE */
        regmap_set_bits((*xcvr).regmap_phy, FSL_XCVR_PHY_CTRL2, FSL_XCVR_PHY_CTRL2_EARC_TXMS);
    } else if !tx { /* SPDIF / ARC RX mode */
        if (*xcvr).mode == FSL_XCVR_MODE_SPDIF {
            /* PHY: CTRL_SET: SPDIF_EN */
            regmap_set_bits((*xcvr).regmap_phy, FSL_XCVR_PHY_CTRL, FSL_XCVR_PHY_CTRL_SPDIF_EN);
        } else {
            /* PHY: CTRL_SET: ARC RX setup */
            regmap_set_bits((*xcvr).regmap_phy, FSL_XCVR_PHY_CTRL,
                FSL_XCVR_PHY_CTRL_PHY_EN | FSL_XCVR_PHY_CTRL_RX_CM_EN | fsl_xcvr_phy_arc_cfg[(*xcvr).arc_mode as usize]);
        }
    }
    dev_dbg(dev, b"PLL Fexp: %u, Fout: %u, mfi: %u, mfn: %u, mfd: %d, div: %u, pdiv0: %u\n\0".as_ptr() as *const c_char,
            freq, fsl_xcvr_pll_cfg[i].fout, fsl_xcvr_pll_cfg[i].mfi as c_uint,
            fsl_xcvr_pll_cfg[i].mfn, fsl_xcvr_pll_cfg[i].mfd as c_int, div, log2);
    0
}

extern "C" {
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: u32_t) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn fsl_asoc_reparent_pll_clocks(dev: *mut device, clk: *mut clk, pll8k: *mut clk, pll11k: *mut clk, freq: u32_t);
}

unsafe fn fsl_xcvr_en_aud_pll(xcvr: *mut fsl_xcvr, mut freq: u32_t) -> c_int {
    let dev = &mut (*(*xcvr).pdev).dev as *mut device;
    let mut ret: c_int;
    freq = if (*(*xcvr).soc_data).spdif_only { freq / 5 } else { freq };
    clk_disable_unprepare((*xcvr).phy_clk);
    fsl_asoc_reparent_pll_clocks(dev, (*xcvr).phy_clk, (*xcvr).pll8k_clk, (*xcvr).pll11k_clk, freq);
    ret = clk_set_rate((*xcvr).phy_clk, freq);
    if ret < 0 { dev_err(dev, b"Error while setting AUD PLL rate: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    ret = clk_prepare_enable((*xcvr).phy_clk);
    if ret != 0 { dev_err(dev, b"failed to start PHY clock: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    if !(*(*xcvr).soc_data).use_phy { return 0; }
    /* Release AI interface from reset */
    ret = regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_SET, FSL_XCVR_PHY_AI_CTRL_AI_RESETN);
    if ret < 0 { dev_err(dev, b"Error while setting IER0: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    if (*xcvr).mode == FSL_XCVR_MODE_EARC { /* eARC mode */
        /* PHY: CTRL_SET: TX_DIFF_OE, PHY_EN */
        regmap_set_bits((*xcvr).regmap_phy, FSL_XCVR_PHY_CTRL, FSL_XCVR_PHY_CTRL_TSDIFF_OE | FSL_XCVR_PHY_CTRL_PHY_EN);
        /* PHY: CTRL2_SET: EARC_TX_MODE */
        regmap_set_bits((*xcvr).regmap_phy, FSL_XCVR_PHY_CTRL2, FSL_XCVR_PHY_CTRL2_EARC_TXMS);
    } else { /* SPDIF mode */
        /* PHY: CTRL_SET: TX_CLK_AUD_SS | SPDIF_EN */
        regmap_set_bits((*xcvr).regmap_phy, FSL_XCVR_PHY_CTRL, FSL_XCVR_PHY_CTRL_TX_CLK_AUD_SS | FSL_XCVR_PHY_CTRL_SPDIF_EN);
    }
    dev_dbg(dev, b"PLL Fexp: %u\n\0".as_ptr() as *const c_char, freq);
    0
}

unsafe extern "C" fn fsl_xcvr_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut m_ctl: u32_t = 0;
    let mut v_ctl: u32_t = 0;
    let r = (*(*substream).runtime).rate;
    let ch = (*(*substream).runtime).channels;
    let fout = 32u32.wrapping_mul(r).wrapping_mul(ch).wrapping_mul(10);
    let mut ret: c_int = 0;
    match (*xcvr).mode {
        FSL_XCVR_MODE_SPDIF => {
            if (*(*xcvr).soc_data).spdif_only && tx {
                ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_TX_DPTH_CTRL, FSL_XCVR_TX_DPTH_CTRL_BYPASS_FEM, FSL_XCVR_TX_DPTH_CTRL_BYPASS_FEM);
                if ret < 0 { dev_err((*dai).dev, b"Failed to set bypass fem: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
            }
            // fallthrough to ARC case
            if tx {
                ret = fsl_xcvr_en_aud_pll(xcvr, fout);
                if ret < 0 { dev_err((*dai).dev, b"Failed to set TX freq %u: %d\n\0".as_ptr() as *const c_char, fout, ret); return ret; }
                ret = regmap_set_bits((*xcvr).regmap, FSL_XCVR_TX_DPTH_CTRL, FSL_XCVR_TX_DPTH_CTRL_FRM_FMT);
                if ret < 0 { dev_err((*dai).dev, b"Failed to set TX_DPTH: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
                /** set SPDIF MODE - this flag is used to gate SPDIF output, useless for SPDIF RX */
                m_ctl |= FSL_XCVR_EXT_CTRL_SPDIF_MODE;
                v_ctl |= FSL_XCVR_EXT_CTRL_SPDIF_MODE;
            } else {
                /** Clear RX FIFO, flip RX FIFO bits, disable eARC related HW mode detects */
                ret = regmap_set_bits((*xcvr).regmap, FSL_XCVR_RX_DPTH_CTRL,
                    FSL_XCVR_RX_DPTH_CTRL_STORE_FMT | FSL_XCVR_RX_DPTH_CTRL_CLR_RX_FIFO |
                    FSL_XCVR_RX_DPTH_CTRL_COMP | FSL_XCVR_RX_DPTH_CTRL_LAYB_CTRL);
                if ret < 0 { dev_err((*dai).dev, b"Failed to set RX_DPTH: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
                ret = fsl_xcvr_en_phy_pll(xcvr, FSL_XCVR_SPDIF_RX_FREQ, tx);
                if ret < 0 { dev_err((*dai).dev, b"Failed to set RX freq %u: %d\n\0".as_ptr() as *const c_char, FSL_XCVR_SPDIF_RX_FREQ, ret); return ret; }
            }
        }
        FSL_XCVR_MODE_ARC => {
            if tx {
                ret = fsl_xcvr_en_aud_pll(xcvr, fout);
                if ret < 0 { return ret; }
                ret = regmap_set_bits((*xcvr).regmap, FSL_XCVR_TX_DPTH_CTRL, FSL_XCVR_TX_DPTH_CTRL_FRM_FMT);
                if ret < 0 { return ret; }
                m_ctl |= FSL_XCVR_EXT_CTRL_SPDIF_MODE;
                v_ctl |= FSL_XCVR_EXT_CTRL_SPDIF_MODE;
            } else {
                ret = regmap_set_bits((*xcvr).regmap, FSL_XCVR_RX_DPTH_CTRL,
                    FSL_XCVR_RX_DPTH_CTRL_STORE_FMT | FSL_XCVR_RX_DPTH_CTRL_CLR_RX_FIFO |
                    FSL_XCVR_RX_DPTH_CTRL_COMP | FSL_XCVR_RX_DPTH_CTRL_LAYB_CTRL);
                if ret < 0 { return ret; }
                ret = fsl_xcvr_en_phy_pll(xcvr, FSL_XCVR_SPDIF_RX_FREQ, tx);
                if ret < 0 { return ret; }
            }
        }
        FSL_XCVR_MODE_EARC => {
            if !tx {
                /** Clear RX FIFO, flip RX FIFO bits */
                ret = regmap_set_bits((*xcvr).regmap, FSL_XCVR_RX_DPTH_CTRL,
                    FSL_XCVR_RX_DPTH_CTRL_STORE_FMT | FSL_XCVR_RX_DPTH_CTRL_CLR_RX_FIFO);
                if ret < 0 { dev_err((*dai).dev, b"Failed to set RX_DPTH: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
                /** Enable eARC related HW mode detects */
                ret = regmap_clear_bits((*xcvr).regmap, FSL_XCVR_RX_DPTH_CTRL,
                    FSL_XCVR_RX_DPTH_CTRL_COMP | FSL_XCVR_RX_DPTH_CTRL_LAYB_CTRL);
                if ret < 0 { dev_err((*dai).dev, b"Failed to clr TX_DPTH: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
            }
            /* clear CMDC RESET */
            m_ctl |= FSL_XCVR_EXT_CTRL_CMDC_RESET(tx);
            /* set TX_RX_MODE */
            m_ctl |= FSL_XCVR_EXT_CTRL_TX_RX_MODE;
            v_ctl |= if tx { FSL_XCVR_EXT_CTRL_TX_RX_MODE } else { 0 };
        }
        _ => {}
    }
    ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, m_ctl, v_ctl);
    if ret < 0 { dev_err((*dai).dev, b"Error while setting EXT_CTRL: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    0
}

unsafe fn fsl_xcvr_constr(substream: *const snd_pcm_substream, channels: *const snd_pcm_hw_constraint_list, rates: *const snd_pcm_hw_constraint_list) -> c_int {
    let rt = (*substream).runtime;
    let mut ret = snd_pcm_hw_constraint_list(rt, 0, SNDRV_PCM_HW_PARAM_CHANNELS, channels);
    if ret < 0 { return ret; }
    ret = snd_pcm_hw_constraint_list(rt, 0, SNDRV_PCM_HW_PARAM_RATE, rates);
    if ret < 0 { return ret; }
    0
}

unsafe extern "C" fn fsl_xcvr_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut ret: c_int = 0;
    if ((*xcvr).streams as u32_t & BIT((*substream).stream as c_uint)) != 0 {
        dev_err((*dai).dev, b"%sX busy\n\0".as_ptr() as *const c_char, if tx { b"T\0".as_ptr() } else { b"R\0".as_ptr() });
        return -EBUSY;
    }
    /* EDMA controller needs period size to be a multiple of tx/rx maxburst */
    if (*(*xcvr).soc_data).use_edma {
        snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
            if tx { (*xcvr).dma_prms_tx.maxburst } else { (*xcvr).dma_prms_rx.maxburst });
    }
    match (*xcvr).mode {
        FSL_XCVR_MODE_SPDIF | FSL_XCVR_MODE_ARC => {
            if (*(*xcvr).soc_data).spdif_only && tx {
                ret = fsl_xcvr_constr(substream, &fsl_xcvr_spdif_channels_constr, &(*xcvr).spdif_constr_rates);
            } else {
                ret = fsl_xcvr_constr(substream, &fsl_xcvr_spdif_channels_constr, &fsl_xcvr_spdif_rates_constr);
            }
        }
        FSL_XCVR_MODE_EARC => {
            ret = fsl_xcvr_constr(substream, &fsl_xcvr_earc_channels_constr, &fsl_xcvr_earc_rates_constr);
        }
        _ => {}
    }
    if ret < 0 { return ret; }
    (*xcvr).streams |= BIT((*substream).stream as c_uint) as u8_t;
    if !(*(*xcvr).soc_data).spdif_only {
        let card = (*(*dai).component).card;
        /* Disable XCVR controls if there is stream started */
        down_read(&mut (*(*card).snd_card).controls_rwsem);
        fsl_xcvr_activate_ctl(dai, fsl_xcvr_mode_kctl.name, false);
        fsl_xcvr_activate_ctl(dai, fsl_xcvr_arc_mode_kctl.name, false);
        fsl_xcvr_activate_ctl(dai, fsl_xcvr_earc_capds_kctl.name, false);
        up_read(&mut (*(*card).snd_card).controls_rwsem);
    }
    0
}

unsafe extern "C" fn fsl_xcvr_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut mask: u32_t = 0;
    let mut val: u32_t = 0;
    let mut ret: c_int;
    (*xcvr).streams &= !(BIT((*substream).stream as c_uint) as u8_t);
    /* Enable XCVR controls if there is no stream started */
    if (*xcvr).streams == 0 {
        if !(*(*xcvr).soc_data).spdif_only {
            let card = (*(*dai).component).card;
            down_read(&mut (*(*card).snd_card).controls_rwsem);
            fsl_xcvr_activate_ctl(dai, fsl_xcvr_mode_kctl.name, true);
            fsl_xcvr_activate_ctl(dai, fsl_xcvr_arc_mode_kctl.name, (*xcvr).mode == FSL_XCVR_MODE_ARC);
            fsl_xcvr_activate_ctl(dai, fsl_xcvr_earc_capds_kctl.name, (*xcvr).mode == FSL_XCVR_MODE_EARC);
            up_read(&mut (*(*card).snd_card).controls_rwsem);
        }
        ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_IER0, FSL_XCVR_IRQ_EARC_ALL, 0);
        if ret < 0 { dev_err((*dai).dev, b"Failed to set IER0: %d\n\0".as_ptr() as *const c_char, ret); return; }
        /* clear SPDIF MODE */
        if (*xcvr).mode == FSL_XCVR_MODE_SPDIF { mask |= FSL_XCVR_EXT_CTRL_SPDIF_MODE; }
    }
    if (*xcvr).mode == FSL_XCVR_MODE_EARC {
        /* set CMDC RESET */
        mask |= FSL_XCVR_EXT_CTRL_CMDC_RESET(tx);
        val |= FSL_XCVR_EXT_CTRL_CMDC_RESET(tx);
    }
    ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, mask, val);
    if ret < 0 { dev_err((*dai).dev, b"Err setting DPATH RESET: %d\n\0".as_ptr() as *const c_char, ret); }
}

unsafe extern "C" fn fsl_xcvr_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut ret: c_int = 0;
    // guard(spinlock_irqsave)(&xcvr->lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            /* set DPATH RESET */
            ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_DPTH_RESET(tx), FSL_XCVR_EXT_CTRL_DPTH_RESET(tx));
            if ret < 0 { dev_err((*dai).dev, b"Failed to set DPATH RESET: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
            if tx {
                match (*xcvr).mode {
                    FSL_XCVR_MODE_EARC => {
                        /* set isr_cmdc_tx_en, w1c */
                        ret = regmap_write((*xcvr).regmap, FSL_XCVR_ISR_SET, FSL_XCVR_ISR_CMDC_TX_EN);
                        if ret < 0 { dev_err((*dai).dev, b"err updating isr %d\n\0".as_ptr() as *const c_char, ret); return ret; }
                        // fallthrough
                        ret = regmap_set_bits((*xcvr).regmap, FSL_XCVR_TX_DPTH_CTRL, FSL_XCVR_TX_DPTH_CTRL_STRT_DATA_TX);
                        if ret < 0 { dev_err((*dai).dev, b"Failed to start DATA_TX: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
                    }
                    FSL_XCVR_MODE_SPDIF => {
                        ret = regmap_set_bits((*xcvr).regmap, FSL_XCVR_TX_DPTH_CTRL, FSL_XCVR_TX_DPTH_CTRL_STRT_DATA_TX);
                        if ret < 0 { dev_err((*dai).dev, b"Failed to start DATA_TX: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
                    }
                    _ => {}
                }
            }
            /* enable DMA RD/WR */
            ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_DMA_DIS(tx), 0);
            if ret < 0 { return ret; }
            ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_IER0, FSL_XCVR_IRQ_EARC_ALL, FSL_XCVR_IRQ_EARC_ALL);
            if ret < 0 { return ret; }
            /* clear DPATH RESET */
            ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_DPTH_RESET(tx), 0);
            if ret < 0 { return ret; }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /* disable DMA RD/WR */
            ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_DMA_DIS(tx), FSL_XCVR_EXT_CTRL_DMA_DIS(tx));
            if ret < 0 { return ret; }
            ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_IER0, FSL_XCVR_IRQ_EARC_ALL, 0);
            if ret < 0 { return ret; }
            if tx {
                match (*xcvr).mode {
                    FSL_XCVR_MODE_SPDIF => {
                        ret = regmap_clear_bits((*xcvr).regmap, FSL_XCVR_TX_DPTH_CTRL, FSL_XCVR_TX_DPTH_CTRL_STRT_DATA_TX);
                        if ret < 0 { return ret; }
                        if (*(*xcvr).soc_data).spdif_only { return ret; }
                        ret = regmap_write((*xcvr).regmap, FSL_XCVR_ISR_CLR, FSL_XCVR_ISR_CMDC_TX_EN);
                        if ret < 0 { return ret; }
                    }
                    FSL_XCVR_MODE_EARC => {
                        /* clear ISR_CMDC_TX_EN, W1C */
                        ret = regmap_write((*xcvr).regmap, FSL_XCVR_ISR_CLR, FSL_XCVR_ISR_CMDC_TX_EN);
                        if ret < 0 { return ret; }
                    }
                    _ => {}
                }
            }
        }
        _ => ret = -EINVAL,
    }
    ret
}

extern "C" {
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
}

unsafe fn fsl_xcvr_load_firmware(xcvr: *mut fsl_xcvr) -> c_int {
    let dev = &mut (*(*xcvr).pdev).dev as *mut device;
    let mut ret: c_int;
    let mut rem: c_int;
    let mut off: c_int;
    let mut out: c_int;
    let mut size = FSL_XCVR_REG_OFFSET;
    let mut mask: u32_t;
    let mut val: u32_t;
    let mut fw: *const firmware = ptr::null();
    ret = request_firmware(&mut fw, (*(*xcvr).soc_data).fw_name, dev);
    if ret != 0 { dev_err(dev, b"failed to request firmware.\n\0".as_ptr() as *const c_char); return ret; }
    rem = (*fw).size;
    /* RAM is 20KiB = 16KiB code + 4KiB data => max 10 pages 2KiB each */
    if rem > 16384 {
        dev_err(dev, b"FW size %d is bigger than 16KiB.\n\0".as_ptr() as *const c_char, rem);
        release_firmware(fw);
        return -ENOMEM;
    }
    for page in 0..10 {
        ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_PAGE_MASK, FSL_XCVR_EXT_CTRL_PAGE(page));
        if ret < 0 { release_firmware(fw); return ret; }
        off = page * size;
        out = min_i32(rem, size);
        /* IPG clock is assumed to be running, otherwise it will hang */
        if out > 0 {
            /* write firmware into code memory */
            memcpy_toio((*xcvr).ram_addr, (*fw).data.add(off as usize) as *const c_void, out as usize);
            rem -= out;
            if rem == 0 {
                /* last part of firmware written */
                /* clean remaining part of code memory page */
                memset_io(((*xcvr).ram_addr as *mut u8).add(out as usize) as *mut c_void, 0, (size - out) as usize);
            }
        } else {
            /* clean current page, including data memory */
            memset_io((*xcvr).ram_addr, 0, size as usize);
        }
    }
    /* configure watermarks */
    mask = FSL_XCVR_EXT_CTRL_RX_FWM_MASK | FSL_XCVR_EXT_CTRL_TX_FWM_MASK;
    val = FSL_XCVR_EXT_CTRL_RX_FWM(FSL_XCVR_FIFO_WMK_RX);
    val |= FSL_XCVR_EXT_CTRL_TX_FWM(FSL_XCVR_FIFO_WMK_TX);
    /* disable DMA RD/WR */
    mask |= FSL_XCVR_EXT_CTRL_DMA_RD_DIS | FSL_XCVR_EXT_CTRL_DMA_WR_DIS;
    val |= FSL_XCVR_EXT_CTRL_DMA_RD_DIS | FSL_XCVR_EXT_CTRL_DMA_WR_DIS;
    /* Data RAM is 4KiB, last two pages: 8 and 9. Select page 8. */
    mask |= FSL_XCVR_EXT_CTRL_PAGE_MASK;
    val |= FSL_XCVR_EXT_CTRL_PAGE(8);
    ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, mask, val);
    if ret < 0 { release_firmware(fw); return ret; }
    /* Store Capabilities Data Structure into Data RAM */
    memcpy_toio(((*xcvr).ram_addr as *mut u8).add(FSL_XCVR_CAP_DATA_STR) as *mut c_void,
                (*xcvr).cap_ds.as_ptr() as *const c_void, FSL_XCVR_CAPDS_SIZE);
    release_firmware(fw);
    0
}

unsafe extern "C" fn fsl_xcvr_type_iec958_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}
unsafe extern "C" fn fsl_xcvr_type_iec958_bytes_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = size_of::<[u8_t; 24]>() as c_uint;
    0
}
unsafe extern "C" fn fsl_xcvr_rx_cs_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    memcpy((*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void, (*xcvr).rx_iec958.status.as_ptr() as *const c_void, 24);
    0
}
unsafe extern "C" fn fsl_xcvr_tx_cs_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    memcpy((*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void, (*xcvr).tx_iec958.status.as_ptr() as *const c_void, 24);
    0
}
unsafe extern "C" fn fsl_xcvr_tx_cs_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    let changed = (memcmp((*xcvr).tx_iec958.status.as_ptr() as *const c_void, (*ucontrol).value.iec958.status.as_ptr() as *const c_void, size_of::<[u8_t; 24]>()) != 0) as c_int;
    memcpy((*xcvr).tx_iec958.status.as_mut_ptr() as *mut c_void, (*ucontrol).value.iec958.status.as_ptr() as *const c_void, size_of::<[u8_t; 24]>());
    changed
}

static mut fsl_xcvr_rx_ctls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Capture Default\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(fsl_xcvr_type_iec958_info),
        get: Some(fsl_xcvr_rx_cs_get),
        put: None,
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"Capture Channel Status\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(fsl_xcvr_type_iec958_bytes_info),
        get: Some(fsl_xcvr_rx_cs_get),
        put: None,
        private_value: 0,
    },
];
static mut fsl_xcvr_tx_ctls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Playback Default\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(fsl_xcvr_type_iec958_info),
        get: Some(fsl_xcvr_tx_cs_get),
        put: Some(fsl_xcvr_tx_cs_put),
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"Playback Channel Status\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(fsl_xcvr_type_iec958_bytes_info),
        get: Some(fsl_xcvr_tx_cs_get),
        put: Some(fsl_xcvr_tx_cs_put),
        private_value: 0,
    },
];

unsafe extern "C" fn fsl_xcvr_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let xcvr = snd_soc_dai_get_drvdata(dai) as *mut fsl_xcvr;
    snd_soc_dai_init_dma_data(dai, &mut (*xcvr).dma_prms_tx, &mut (*xcvr).dma_prms_rx);
    if (*(*xcvr).soc_data).spdif_only {
        (*xcvr).mode = FSL_XCVR_MODE_SPDIF;
    } else {
        snd_soc_add_dai_controls(dai, &fsl_xcvr_mode_kctl, 1);
        snd_soc_add_dai_controls(dai, &fsl_xcvr_arc_mode_kctl, 1);
        snd_soc_add_dai_controls(dai, &fsl_xcvr_earc_capds_kctl, 1);
    }
    snd_soc_add_dai_controls(dai, fsl_xcvr_tx_ctls.as_ptr(), fsl_xcvr_tx_ctls.len() as c_uint);
    snd_soc_add_dai_controls(dai, fsl_xcvr_rx_ctls.as_ptr(), fsl_xcvr_rx_ctls.len() as c_uint);
    0
}

static fsl_xcvr_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_xcvr_dai_probe),
    prepare: Some(fsl_xcvr_prepare),
    startup: Some(fsl_xcvr_startup),
    shutdown: Some(fsl_xcvr_shutdown),
    trigger: Some(fsl_xcvr_trigger),
};

static mut fsl_xcvr_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    ops: &fsl_xcvr_dai_ops,
    playback: snd_soc_pcm_stream {
        stream_name: b"CPU-Playback\0".as_ptr() as *const c_char,
        channels_min: 1, channels_max: 32, rate_min: 32000, rate_max: 1536000,
        rates: SNDRV_PCM_RATE_KNOT, formats: SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"CPU-Capture\0".as_ptr() as *const c_char,
        channels_min: 1, channels_max: 32, rate_min: 32000, rate_max: 1536000,
        rates: SNDRV_PCM_RATE_KNOT, formats: SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE,
    },
};

unsafe extern "C" fn fsl_xcvr_component_probe(component: *mut snd_soc_component) -> c_int {
    let xcvr = snd_soc_component_get_drvdata(component) as *mut fsl_xcvr;
    snd_soc_component_init_regmap(component, (*xcvr).regmap);
    0
}

static fsl_xcvr_comp: snd_soc_component_driver = snd_soc_component_driver {
    name: b"fsl-xcvr-dai\0".as_ptr() as *const c_char,
    probe: Some(fsl_xcvr_component_probe),
    controls: fsl_xcvr_timestamp_ctrls.as_ptr(),
    num_controls: 16,
    legacy_dai_naming: 1,
};

static fsl_xcvr_reg_defaults: [reg_default; 50] = unsafe { [
    reg_default { reg: FSL_XCVR_VERSION, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_EXT_CTRL, def: 0xF8204040 },
    reg_default { reg: FSL_XCVR_EXT_STATUS, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_EXT_IER0, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_EXT_IER1, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_EXT_ISR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_EXT_ISR_SET, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_EXT_ISR_CLR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_EXT_ISR_TOG, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_IER, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_ISR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_ISR_SET, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_ISR_CLR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_ISR_TOG, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_CLK_CTRL, def: 0x0000018F },
    reg_default { reg: FSL_XCVR_RX_DPTH_CTRL, def: 0x00040CC1 },
    reg_default { reg: FSL_XCVR_RX_DPTH_CTRL_SET, def: 0x00040CC1 },
    reg_default { reg: FSL_XCVR_RX_DPTH_CTRL_CLR, def: 0x00040CC1 },
    reg_default { reg: FSL_XCVR_RX_DPTH_CTRL_TOG, def: 0x00040CC1 },
    reg_default { reg: FSL_XCVR_RX_DPTH_CNTR_CTRL, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_RX_DPTH_CNTR_CTRL_SET, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_RX_DPTH_CNTR_CTRL_CLR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_RX_DPTH_CNTR_CTRL_TOG, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_RX_DPTH_TSCR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_RX_DPTH_BCR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_RX_DPTH_BCTR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_RX_DPTH_BCRR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_CTRL, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_CTRL_SET, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_CTRL_CLR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_CTRL_TOG, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_CS_DATA_0, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_CS_DATA_1, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_CS_DATA_2, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_CS_DATA_3, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_CS_DATA_4, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_CS_DATA_5, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_CNTR_CTRL, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_CNTR_CTRL_SET, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_CNTR_CTRL_CLR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_CNTR_CTRL_TOG, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_TSCR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_BCR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_BCTR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_TX_DPTH_BCRR, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_DEBUG_REG_0, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_DEBUG_REG_1, def: 0x00000000 },
] };

unsafe extern "C" fn fsl_xcvr_readable_reg(dev: *mut device, reg: c_uint) -> bool_t {
    let xcvr = dev_get_drvdata(dev) as *mut fsl_xcvr;
    if !(*(*xcvr).soc_data).use_phy {
        if (reg >= FSL_XCVR_IER && reg <= FSL_XCVR_PHY_AI_RDATA) || reg > FSL_XCVR_TX_DPTH_BCRR { return false; }
    }
    matches!(reg,
        FSL_XCVR_VERSION | FSL_XCVR_EXT_CTRL | FSL_XCVR_EXT_STATUS | FSL_XCVR_EXT_IER0 |
        FSL_XCVR_EXT_IER1 | FSL_XCVR_EXT_ISR | FSL_XCVR_EXT_ISR_SET | FSL_XCVR_EXT_ISR_CLR |
        FSL_XCVR_EXT_ISR_TOG | FSL_XCVR_IER | FSL_XCVR_ISR | FSL_XCVR_ISR_SET |
        FSL_XCVR_ISR_CLR | FSL_XCVR_ISR_TOG | FSL_XCVR_PHY_AI_CTRL | FSL_XCVR_PHY_AI_CTRL_SET |
        FSL_XCVR_PHY_AI_CTRL_CLR | FSL_XCVR_PHY_AI_CTRL_TOG | FSL_XCVR_PHY_AI_RDATA |
        FSL_XCVR_CLK_CTRL | FSL_XCVR_RX_DPTH_CTRL | FSL_XCVR_RX_DPTH_CTRL_SET |
        FSL_XCVR_RX_DPTH_CTRL_CLR | FSL_XCVR_RX_DPTH_CTRL_TOG | FSL_XCVR_RX_CS_DATA_0 |
        FSL_XCVR_RX_CS_DATA_1 | FSL_XCVR_RX_CS_DATA_2 | FSL_XCVR_RX_CS_DATA_3 |
        FSL_XCVR_RX_CS_DATA_4 | FSL_XCVR_RX_CS_DATA_5 | FSL_XCVR_RX_DPTH_CNTR_CTRL |
        FSL_XCVR_RX_DPTH_CNTR_CTRL_SET | FSL_XCVR_RX_DPTH_CNTR_CTRL_CLR |
        FSL_XCVR_RX_DPTH_CNTR_CTRL_TOG | FSL_XCVR_RX_DPTH_TSCR | FSL_XCVR_RX_DPTH_BCR |
        FSL_XCVR_RX_DPTH_BCTR | FSL_XCVR_RX_DPTH_BCRR | FSL_XCVR_TX_DPTH_CTRL |
        FSL_XCVR_TX_DPTH_CTRL_SET | FSL_XCVR_TX_DPTH_CTRL_CLR | FSL_XCVR_TX_DPTH_CTRL_TOG |
        FSL_XCVR_TX_CS_DATA_0 | FSL_XCVR_TX_CS_DATA_1 | FSL_XCVR_TX_CS_DATA_2 |
        FSL_XCVR_TX_CS_DATA_3 | FSL_XCVR_TX_CS_DATA_4 | FSL_XCVR_TX_CS_DATA_5 |
        FSL_XCVR_TX_DPTH_CNTR_CTRL | FSL_XCVR_TX_DPTH_CNTR_CTRL_SET |
        FSL_XCVR_TX_DPTH_CNTR_CTRL_CLR | FSL_XCVR_TX_DPTH_CNTR_CTRL_TOG |
        FSL_XCVR_TX_DPTH_TSCR | FSL_XCVR_TX_DPTH_BCR | FSL_XCVR_TX_DPTH_BCTR |
        FSL_XCVR_TX_DPTH_BCRR | FSL_XCVR_DEBUG_REG_0 | FSL_XCVR_DEBUG_REG_1)
}

unsafe extern "C" fn fsl_xcvr_writeable_reg(dev: *mut device, reg: c_uint) -> bool_t {
    let xcvr = dev_get_drvdata(dev) as *mut fsl_xcvr;
    if !(*(*xcvr).soc_data).use_phy && reg >= FSL_XCVR_IER && reg <= FSL_XCVR_PHY_AI_RDATA { return false; }
    matches!(reg,
        FSL_XCVR_EXT_CTRL | FSL_XCVR_EXT_IER0 | FSL_XCVR_EXT_IER1 | FSL_XCVR_EXT_ISR |
        FSL_XCVR_EXT_ISR_SET | FSL_XCVR_EXT_ISR_CLR | FSL_XCVR_EXT_ISR_TOG |
        FSL_XCVR_IER | FSL_XCVR_ISR_SET | FSL_XCVR_ISR_CLR | FSL_XCVR_ISR_TOG |
        FSL_XCVR_PHY_AI_CTRL | FSL_XCVR_PHY_AI_CTRL_SET | FSL_XCVR_PHY_AI_CTRL_CLR |
        FSL_XCVR_PHY_AI_CTRL_TOG | FSL_XCVR_PHY_AI_WDATA | FSL_XCVR_CLK_CTRL |
        FSL_XCVR_RX_DPTH_CTRL | FSL_XCVR_RX_DPTH_CTRL_SET | FSL_XCVR_RX_DPTH_CTRL_CLR |
        FSL_XCVR_RX_DPTH_CTRL_TOG | FSL_XCVR_RX_DPTH_CNTR_CTRL |
        FSL_XCVR_RX_DPTH_CNTR_CTRL_SET | FSL_XCVR_RX_DPTH_CNTR_CTRL_CLR |
        FSL_XCVR_RX_DPTH_CNTR_CTRL_TOG | FSL_XCVR_TX_DPTH_CTRL |
        FSL_XCVR_TX_DPTH_CTRL_SET | FSL_XCVR_TX_DPTH_CTRL_CLR | FSL_XCVR_TX_DPTH_CTRL_TOG |
        FSL_XCVR_TX_CS_DATA_0 | FSL_XCVR_TX_CS_DATA_1 | FSL_XCVR_TX_CS_DATA_2 |
        FSL_XCVR_TX_CS_DATA_3 | FSL_XCVR_TX_CS_DATA_4 | FSL_XCVR_TX_CS_DATA_5 |
        FSL_XCVR_TX_DPTH_CNTR_CTRL | FSL_XCVR_TX_DPTH_CNTR_CTRL_SET |
        FSL_XCVR_TX_DPTH_CNTR_CTRL_CLR | FSL_XCVR_TX_DPTH_CNTR_CTRL_TOG)
}

unsafe extern "C" fn fsl_xcvr_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    matches!(reg,
        FSL_XCVR_EXT_STATUS | FSL_XCVR_EXT_ISR | FSL_XCVR_EXT_ISR_SET | FSL_XCVR_EXT_ISR_CLR |
        FSL_XCVR_EXT_ISR_TOG | FSL_XCVR_ISR | FSL_XCVR_ISR_SET | FSL_XCVR_ISR_CLR |
        FSL_XCVR_ISR_TOG | FSL_XCVR_PHY_AI_CTRL | FSL_XCVR_PHY_AI_CTRL_SET |
        FSL_XCVR_PHY_AI_CTRL_CLR | FSL_XCVR_PHY_AI_CTRL_TOG | FSL_XCVR_PHY_AI_RDATA |
        FSL_XCVR_RX_CS_DATA_0 | FSL_XCVR_RX_CS_DATA_1 | FSL_XCVR_RX_CS_DATA_2 |
        FSL_XCVR_RX_CS_DATA_3 | FSL_XCVR_RX_CS_DATA_4 | FSL_XCVR_RX_CS_DATA_5 |
        FSL_XCVR_RX_DPTH_CNTR_CTRL | FSL_XCVR_RX_DPTH_CNTR_CTRL_SET |
        FSL_XCVR_RX_DPTH_CNTR_CTRL_CLR | FSL_XCVR_RX_DPTH_CNTR_CTRL_TOG |
        FSL_XCVR_RX_DPTH_TSCR | FSL_XCVR_RX_DPTH_BCR | FSL_XCVR_RX_DPTH_BCTR |
        FSL_XCVR_RX_DPTH_BCRR | FSL_XCVR_TX_DPTH_CNTR_CTRL |
        FSL_XCVR_TX_DPTH_CNTR_CTRL_SET | FSL_XCVR_TX_DPTH_CNTR_CTRL_CLR |
        FSL_XCVR_TX_DPTH_CNTR_CTRL_TOG | FSL_XCVR_TX_DPTH_TSCR | FSL_XCVR_TX_DPTH_BCR |
        FSL_XCVR_TX_DPTH_BCTR | FSL_XCVR_TX_DPTH_BCRR | FSL_XCVR_DEBUG_REG_0 |
        FSL_XCVR_DEBUG_REG_1)
}

static fsl_xcvr_regmap_cfg: regmap_config = unsafe { regmap_config {
    name: ptr::null(), reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: FSL_XCVR_MAX_REG,
    reg_defaults: fsl_xcvr_reg_defaults.as_ptr(), num_reg_defaults: fsl_xcvr_reg_defaults.len() as c_uint,
    readable_reg: Some(fsl_xcvr_readable_reg), volatile_reg: Some(fsl_xcvr_volatile_reg),
    writeable_reg: Some(fsl_xcvr_writeable_reg), cache_type: REGCACHE_FLAT, reg_read: None, reg_write: None,
} };

static fsl_xcvr_phy_reg_defaults: [reg_default; 8] = unsafe { [
    reg_default { reg: FSL_XCVR_PHY_CTRL, def: 0x58200804 },
    reg_default { reg: FSL_XCVR_PHY_STATUS, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_PHY_ANALOG_TRIM, def: 0x00260F13 },
    reg_default { reg: FSL_XCVR_PHY_SLEW_RATE_TRIM, def: 0x00000411 },
    reg_default { reg: FSL_XCVR_PHY_DATA_TEST_DELAY, def: 0x00990000 },
    reg_default { reg: FSL_XCVR_PHY_TEST_CTRL, def: 0x00000000 },
    reg_default { reg: FSL_XCVR_PHY_DIFF_CDR_CTRL, def: 0x016D0009 },
    reg_default { reg: FSL_XCVR_PHY_CTRL2, def: 0x80000000 },
] };

static fsl_xcvr_regmap_phy_cfg: regmap_config = unsafe { regmap_config {
    name: b"phy\0".as_ptr() as *const c_char, reg_bits: 8, reg_stride: 4, val_bits: 32,
    max_register: FSL_XCVR_PHY_CTRL2_TOG, reg_defaults: fsl_xcvr_phy_reg_defaults.as_ptr(),
    num_reg_defaults: fsl_xcvr_phy_reg_defaults.len() as c_uint, readable_reg: None, volatile_reg: None,
    writeable_reg: None, cache_type: REGCACHE_FLAT, reg_read: Some(fsl_xcvr_phy_reg_read),
    reg_write: Some(fsl_xcvr_phy_reg_write),
} };
static fsl_xcvr_regmap_pllv0_cfg: regmap_config = unsafe { regmap_config {
    name: b"pllv0\0".as_ptr() as *const c_char, reg_bits: 8, reg_stride: 4, val_bits: 32,
    max_register: FSL_XCVR_PLL_STAT0_TOG, reg_defaults: ptr::null(), num_reg_defaults: 0,
    readable_reg: None, volatile_reg: None, writeable_reg: None, cache_type: REGCACHE_FLAT,
    reg_read: Some(fsl_xcvr_pll_reg_read), reg_write: Some(fsl_xcvr_pll_reg_write),
} };
static fsl_xcvr_regmap_pllv1_cfg: regmap_config = unsafe { regmap_config {
    name: b"pllv1\0".as_ptr() as *const c_char, reg_bits: 8, reg_stride: 4, val_bits: 32,
    max_register: FSL_XCVR_GP_PLL_STATUS_TOG, reg_defaults: ptr::null(), num_reg_defaults: 0,
    readable_reg: None, volatile_reg: None, writeable_reg: None, cache_type: REGCACHE_FLAT,
    reg_read: Some(fsl_xcvr_pll_reg_read), reg_write: Some(fsl_xcvr_pll_reg_write),
} };

extern "C" {
    fn container_of_work_rst(work: *mut work_struct) -> *mut fsl_xcvr;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn fsl_asoc_get_pll_clocks(dev: *mut device, pll8k: *mut *mut clk, pll11k: *mut *mut clk);
    fn fsl_asoc_constrain_rates(dst: *mut snd_pcm_hw_constraint_list, src: *const snd_pcm_hw_constraint_list, pll8k: *mut clk, pll11k: *mut clk, arg: *mut c_void, list: *mut u32_t);
    fn devm_platform_ioremap_resource_byname(pdev: *mut platform_device, name: *const c_char) -> *mut c_void;
    fn devm_regmap_init_mmio_clk(dev: *mut device, clk_id: *const c_char, regs: *mut c_void, cfg: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init(dev: *mut device, bus: *const c_void, context: *mut c_void, cfg: *const regmap_config) -> *mut regmap;
    fn devm_reset_control_get_optional_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn platform_get_resource_byname(pdev: *mut platform_device, ty: c_uint, name: *const c_char) -> *mut resource;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, cfg: *const c_void, flags: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, comp: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn spin_lock_init(lock: *mut spinlock_t);
    fn reset_control_assert(reset: *mut reset_control) -> c_int;
    fn reset_control_deassert(reset: *mut reset_control) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

unsafe extern "C" fn reset_rx_work(work: *mut work_struct) {
    let xcvr = container_of_work_rst(work);
    let dev = &mut (*(*xcvr).pdev).dev as *mut device;
    let mut ext_ctrl: u32_t = 0;
    dev_dbg(dev, b"reset rx path\n\0".as_ptr() as *const c_char);
    // guard(spinlock_irqsave)(&xcvr->lock);
    regmap_read((*xcvr).regmap, FSL_XCVR_EXT_CTRL, &mut ext_ctrl);
    if (ext_ctrl & FSL_XCVR_EXT_CTRL_DMA_RD_DIS) == 0 {
        regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_DMA_RD_DIS, FSL_XCVR_EXT_CTRL_DMA_RD_DIS);
        regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_RX_DPTH_RESET, FSL_XCVR_EXT_CTRL_RX_DPTH_RESET);
        regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_DMA_RD_DIS, 0);
        regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_RX_DPTH_RESET, 0);
    }
}

unsafe extern "C" fn irq0_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let xcvr = devid as *mut fsl_xcvr;
    let dev = &mut (*(*xcvr).pdev).dev as *mut device;
    let regmap = (*xcvr).regmap;
    let mut reg_ctrl: *mut c_void;
    let mut reg_buff: *mut c_void;
    let mut isr: u32_t = 0;
    let mut isr_clr: u32_t = 0;
    let mut val: u32_t = 0;
    regmap_read(regmap, FSL_XCVR_EXT_ISR, &mut isr);
    if (isr & FSL_XCVR_IRQ_NEW_CS) != 0 {
        dev_dbg(dev, b"Received new CS block\n\0".as_ptr() as *const c_char);
        isr_clr |= FSL_XCVR_IRQ_NEW_CS;
        if !(*(*xcvr).soc_data).fw_name.is_null() {
            /* Data RAM is 4KiB, last two pages: 8 and 9. Select page 8. */
            regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_PAGE_MASK, FSL_XCVR_EXT_CTRL_PAGE(8));
            /* Find updated CS buffer */
            reg_ctrl = ((*xcvr).ram_addr as *mut u8).add(FSL_XCVR_RX_CS_CTRL_0) as *mut c_void;
            reg_buff = ((*xcvr).ram_addr as *mut u8).add(FSL_XCVR_RX_CS_BUFF_0) as *mut c_void;
            memcpy_fromio(&mut val as *mut _ as *mut c_void, reg_ctrl, size_of::<u32_t>());
            if val == 0 {
                reg_ctrl = ((*xcvr).ram_addr as *mut u8).add(FSL_XCVR_RX_CS_CTRL_1) as *mut c_void;
                reg_buff = ((*xcvr).ram_addr as *mut u8).add(FSL_XCVR_RX_CS_BUFF_1) as *mut c_void;
                memcpy_fromio(&mut val as *mut _ as *mut c_void, reg_ctrl, size_of::<u32_t>());
            }
            if val != 0 {
                /* copy CS buffer */
                memcpy_fromio((*xcvr).rx_iec958.status.as_mut_ptr() as *mut c_void, reg_buff, size_of::<[u8_t; 24]>());
                for i in 0..6 {
                    let p = (*xcvr).rx_iec958.status.as_mut_ptr().add(i * 4) as *mut u32_t;
                    val = *p;
                    *p = bitrev32(val);
                }
                /* clear CS control register */
                writel_relaxed(0, reg_ctrl);
            }
        } else {
            regmap_read((*xcvr).regmap, FSL_XCVR_RX_CS_DATA_0, (*xcvr).rx_iec958.status.as_mut_ptr().add(0) as *mut u32_t);
            regmap_read((*xcvr).regmap, FSL_XCVR_RX_CS_DATA_1, (*xcvr).rx_iec958.status.as_mut_ptr().add(4) as *mut u32_t);
            regmap_read((*xcvr).regmap, FSL_XCVR_RX_CS_DATA_2, (*xcvr).rx_iec958.status.as_mut_ptr().add(8) as *mut u32_t);
            regmap_read((*xcvr).regmap, FSL_XCVR_RX_CS_DATA_3, (*xcvr).rx_iec958.status.as_mut_ptr().add(12) as *mut u32_t);
            regmap_read((*xcvr).regmap, FSL_XCVR_RX_CS_DATA_4, (*xcvr).rx_iec958.status.as_mut_ptr().add(16) as *mut u32_t);
            regmap_read((*xcvr).regmap, FSL_XCVR_RX_CS_DATA_5, (*xcvr).rx_iec958.status.as_mut_ptr().add(20) as *mut u32_t);
            for i in 0..6 {
                let p = (*xcvr).rx_iec958.status.as_mut_ptr().add(i * 4) as *mut u32_t;
                val = *p;
                *p = bitrev32(val);
            }
            regmap_set_bits((*xcvr).regmap, FSL_XCVR_RX_DPTH_CTRL, FSL_XCVR_RX_DPTH_CTRL_CSA);
        }
    }
    if (isr & FSL_XCVR_IRQ_NEW_UD) != 0 { dev_dbg(dev, b"Received new UD block\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_NEW_UD; }
    if (isr & FSL_XCVR_IRQ_MUTE) != 0 { dev_dbg(dev, b"HW mute bit detected\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_MUTE; }
    if (isr & FSL_XCVR_IRQ_FIFO_UOFL_ERR) != 0 { dev_dbg(dev, b"RX/TX FIFO full/empty\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_FIFO_UOFL_ERR; }
    if (isr & FSL_XCVR_IRQ_ARC_MODE) != 0 { dev_dbg(dev, b"CMDC SM falls out of eARC mode\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_ARC_MODE; }
    if (isr & FSL_XCVR_IRQ_DMA_RD_REQ) != 0 { dev_dbg(dev, b"DMA read request\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_DMA_RD_REQ; }
    if (isr & FSL_XCVR_IRQ_DMA_WR_REQ) != 0 { dev_dbg(dev, b"DMA write request\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_DMA_WR_REQ; }
    if (isr & FSL_XCVR_IRQ_CMDC_STATUS_UPD) != 0 { dev_dbg(dev, b"CMDC status update\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_CMDC_STATUS_UPD; }
    if (isr & FSL_XCVR_IRQ_PREAMBLE_MISMATCH) != 0 { dev_dbg(dev, b"Preamble mismatch\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_PREAMBLE_MISMATCH; }
    if (isr & FSL_XCVR_IRQ_UNEXP_PRE_REC) != 0 { dev_dbg(dev, b"Unexpected preamble received\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_UNEXP_PRE_REC; }
    if (isr & FSL_XCVR_IRQ_M_W_PRE_MISMATCH) != 0 { dev_dbg(dev, b"M/W preamble mismatch\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_M_W_PRE_MISMATCH; }
    if (isr & FSL_XCVR_IRQ_B_PRE_MISMATCH) != 0 { dev_dbg(dev, b"B preamble mismatch\n\0".as_ptr() as *const c_char); isr_clr |= FSL_XCVR_IRQ_B_PRE_MISMATCH; }
    if (isr & (FSL_XCVR_IRQ_PREAMBLE_MISMATCH | FSL_XCVR_IRQ_UNEXP_PRE_REC | FSL_XCVR_IRQ_M_W_PRE_MISMATCH | FSL_XCVR_IRQ_B_PRE_MISMATCH)) != 0 {
        schedule_work(&mut (*xcvr).work_rst);
    }
    if isr_clr != 0 {
        regmap_write(regmap, FSL_XCVR_EXT_ISR_CLR, isr_clr);
        return IRQ_HANDLED;
    }
    IRQ_NONE
}

static fsl_xcvr_imx8mp_data: fsl_xcvr_soc_data = fsl_xcvr_soc_data {
    fw_name: b"imx/xcvr/xcvr-imx8mp.bin\0".as_ptr() as *const c_char,
    spdif_only: false, use_edma: false, use_phy: true, pll_ver: fsl_xcvr_pll_verison::PLL_MX8MP,
};
static fsl_xcvr_imx93_data: fsl_xcvr_soc_data = fsl_xcvr_soc_data {
    fw_name: ptr::null(), spdif_only: true, use_edma: true, use_phy: false, pll_ver: fsl_xcvr_pll_verison::PLL_MX8MP,
};
static fsl_xcvr_imx95_data: fsl_xcvr_soc_data = fsl_xcvr_soc_data {
    fw_name: b"imx/xcvr/xcvr-imx95.bin\0".as_ptr() as *const c_char,
    spdif_only: true, use_edma: true, use_phy: true, pll_ver: fsl_xcvr_pll_verison::PLL_MX95,
};

static fsl_xcvr_dt_ids: [of_device_id; 4] = [
    of_device_id { compatible: b"fsl,imx8mp-xcvr\0".as_ptr() as *const c_char, data: &fsl_xcvr_imx8mp_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx93-xcvr\0".as_ptr() as *const c_char, data: &fsl_xcvr_imx93_data as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx95-xcvr\0".as_ptr() as *const c_char, data: &fsl_xcvr_imx95_data as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() }, /* sentinel */
];
// MODULE_DEVICE_TABLE(of, fsl_xcvr_dt_ids);

unsafe extern "C" fn fsl_xcvr_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut xcvr: *mut fsl_xcvr;
    let mut rx_res: *mut resource;
    let mut tx_res: *mut resource;
    let mut regs: *mut c_void;
    let mut ret: c_int;
    let mut irq: c_int;
    xcvr = devm_kzalloc(dev, size_of::<fsl_xcvr>(), GFP_KERNEL) as *mut fsl_xcvr;
    if xcvr.is_null() { return -ENOMEM; }
    (*xcvr).pdev = pdev;
    (*xcvr).soc_data = of_device_get_match_data(&mut (*pdev).dev) as *const fsl_xcvr_soc_data;
    (*xcvr).ipg_clk = devm_clk_get(dev, b"ipg\0".as_ptr() as *const c_char);
    if IS_ERR((*xcvr).ipg_clk as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).ipg_clk as *const c_void), b"failed to get ipg clock\n\0".as_ptr() as *const c_char); }
    (*xcvr).phy_clk = devm_clk_get(dev, b"phy\0".as_ptr() as *const c_char);
    if IS_ERR((*xcvr).phy_clk as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).phy_clk as *const c_void), b"failed to get phy clock\n\0".as_ptr() as *const c_char); }
    (*xcvr).spba_clk = devm_clk_get(dev, b"spba\0".as_ptr() as *const c_char);
    if IS_ERR((*xcvr).spba_clk as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).spba_clk as *const c_void), b"failed to get spba clock\n\0".as_ptr() as *const c_char); }
    (*xcvr).pll_ipg_clk = devm_clk_get(dev, b"pll_ipg\0".as_ptr() as *const c_char);
    if IS_ERR((*xcvr).pll_ipg_clk as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).pll_ipg_clk as *const c_void), b"failed to get pll_ipg clock\n\0".as_ptr() as *const c_char); }
    fsl_asoc_get_pll_clocks(dev, &mut (*xcvr).pll8k_clk, &mut (*xcvr).pll11k_clk);
    if (*(*xcvr).soc_data).spdif_only {
        if (*xcvr).pll8k_clk.is_null() && (*xcvr).pll11k_clk.is_null() { (*xcvr).pll8k_clk = (*xcvr).phy_clk; }
        fsl_asoc_constrain_rates(&mut (*xcvr).spdif_constr_rates, &fsl_xcvr_spdif_rates_constr,
            (*xcvr).pll8k_clk, (*xcvr).pll11k_clk, ptr::null_mut(), (*xcvr).spdif_constr_rates_list.as_mut_ptr());
    }
    (*xcvr).ram_addr = devm_platform_ioremap_resource_byname(pdev, b"ram\0".as_ptr() as *const c_char);
    if IS_ERR((*xcvr).ram_addr as *const c_void) { return PTR_ERR((*xcvr).ram_addr as *const c_void); }
    regs = devm_platform_ioremap_resource_byname(pdev, b"regs\0".as_ptr() as *const c_char);
    if IS_ERR(regs as *const c_void) { return PTR_ERR(regs as *const c_void); }
    (*xcvr).regmap = devm_regmap_init_mmio_clk(dev, ptr::null(), regs, &fsl_xcvr_regmap_cfg);
    if IS_ERR((*xcvr).regmap as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).regmap as *const c_void), b"failed to init XCVR regmap\n\0".as_ptr() as *const c_char); }
    if (*(*xcvr).soc_data).use_phy {
        (*xcvr).regmap_phy = devm_regmap_init(dev, ptr::null(), xcvr as *mut c_void, &fsl_xcvr_regmap_phy_cfg);
        if IS_ERR((*xcvr).regmap_phy as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).regmap_phy as *const c_void), b"failed to init XCVR PHY regmap\n\0".as_ptr() as *const c_char); }
        match (*(*xcvr).soc_data).pll_ver {
            fsl_xcvr_pll_verison::PLL_MX8MP => {
                (*xcvr).regmap_pll = devm_regmap_init(dev, ptr::null(), xcvr as *mut c_void, &fsl_xcvr_regmap_pllv0_cfg);
                if IS_ERR((*xcvr).regmap_pll as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).regmap_pll as *const c_void), b"failed to init XCVR PLL regmap\n\0".as_ptr() as *const c_char); }
            }
            fsl_xcvr_pll_verison::PLL_MX95 => {
                (*xcvr).regmap_pll = devm_regmap_init(dev, ptr::null(), xcvr as *mut c_void, &fsl_xcvr_regmap_pllv1_cfg);
                if IS_ERR((*xcvr).regmap_pll as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).regmap_pll as *const c_void), b"failed to init XCVR PLL regmap\n\0".as_ptr() as *const c_char); }
            }
        }
    }
    (*xcvr).reset = devm_reset_control_get_optional_exclusive(dev, ptr::null());
    if IS_ERR((*xcvr).reset as *const c_void) { return dev_err_probe(dev, PTR_ERR((*xcvr).reset as *const c_void), b"failed to get XCVR reset control\n\0".as_ptr() as *const c_char); }
    /* get IRQs */
    irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    ret = devm_request_irq(dev, irq, irq0_isr, 0, (*pdev).name, xcvr as *mut c_void);
    if ret != 0 { return dev_err_probe(dev, ret, b"failed to claim IRQ0\n\0".as_ptr() as *const c_char); }
    rx_res = platform_get_resource_byname(pdev, IORESOURCE_MEM, b"rxfifo\0".as_ptr() as *const c_char);
    tx_res = platform_get_resource_byname(pdev, IORESOURCE_MEM, b"txfifo\0".as_ptr() as *const c_char);
    if rx_res.is_null() || tx_res.is_null() { return dev_err_probe(dev, -EINVAL, b"could not find rxfifo or txfifo resource\n\0".as_ptr() as *const c_char); }
    (*xcvr).dma_prms_rx.chan_name = b"rx\0".as_ptr() as *const c_char;
    (*xcvr).dma_prms_tx.chan_name = b"tx\0".as_ptr() as *const c_char;
    (*xcvr).dma_prms_rx.addr = (*rx_res).start;
    (*xcvr).dma_prms_tx.addr = (*tx_res).start;
    (*xcvr).dma_prms_rx.maxburst = FSL_XCVR_MAXBURST_RX;
    (*xcvr).dma_prms_tx.maxburst = FSL_XCVR_MAXBURST_TX;
    platform_set_drvdata(pdev, xcvr as *mut c_void);
    pm_runtime_enable(dev);
    regcache_cache_only((*xcvr).regmap, true);
    if (*(*xcvr).soc_data).use_phy {
        regcache_cache_only((*xcvr).regmap_phy, true);
        regcache_cache_only((*xcvr).regmap_pll, true);
    }
    /*
     * Register platform component before registering cpu dai for there
     * is not defer probe for platform component in snd_soc_add_pcm_runtime().
     */
    ret = devm_snd_dmaengine_pcm_register(dev, ptr::null(), 0);
    if ret != 0 { pm_runtime_disable(dev); return dev_err_probe(dev, ret, b"failed to pcm register\n\0".as_ptr() as *const c_char); }
    ret = devm_snd_soc_register_component(dev, &fsl_xcvr_comp, &mut fsl_xcvr_dai, 1);
    if ret != 0 {
        pm_runtime_disable(dev);
        dev_err(dev, b"failed to register component %s\n\0".as_ptr() as *const c_char, fsl_xcvr_comp.name);
    }
    INIT_WORK(&mut (*xcvr).work_rst, reset_rx_work);
    spin_lock_init(&mut (*xcvr).lock);
    ret
}

unsafe extern "C" fn fsl_xcvr_remove(pdev: *mut platform_device) {
    let xcvr = dev_get_drvdata(&mut (*pdev).dev) as *mut fsl_xcvr;
    cancel_work_sync(&mut (*xcvr).work_rst);
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn fsl_xcvr_runtime_suspend(dev: *mut device) -> c_int {
    let xcvr = dev_get_drvdata(dev) as *mut fsl_xcvr;
    let mut ret: c_int;
    if !(*(*xcvr).soc_data).spdif_only && (*xcvr).mode == FSL_XCVR_MODE_EARC {
        /* Assert M0+ reset */
        ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_CORE_RESET, FSL_XCVR_EXT_CTRL_CORE_RESET);
        if ret < 0 { dev_err(dev, b"Failed to assert M0+ core: %d\n\0".as_ptr() as *const c_char, ret); }
    }
    regcache_cache_only((*xcvr).regmap, true);
    if (*(*xcvr).soc_data).use_phy {
        regcache_cache_only((*xcvr).regmap_phy, true);
        regcache_cache_only((*xcvr).regmap_pll, true);
    }
    clk_disable_unprepare((*xcvr).spba_clk);
    clk_disable_unprepare((*xcvr).phy_clk);
    clk_disable_unprepare((*xcvr).pll_ipg_clk);
    clk_disable_unprepare((*xcvr).ipg_clk);
    0
}

unsafe extern "C" fn fsl_xcvr_runtime_resume(dev: *mut device) -> c_int {
    let xcvr = dev_get_drvdata(dev) as *mut fsl_xcvr;
    let mut ret: c_int;
    ret = reset_control_assert((*xcvr).reset);
    if ret < 0 { dev_err(dev, b"Failed to assert M0+ reset: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    ret = clk_prepare_enable((*xcvr).ipg_clk);
    if ret != 0 { dev_err(dev, b"failed to start IPG clock.\n\0".as_ptr() as *const c_char); return ret; }
    ret = clk_prepare_enable((*xcvr).pll_ipg_clk);
    if ret != 0 { dev_err(dev, b"failed to start PLL IPG clock.\n\0".as_ptr() as *const c_char); goto_stop_ipg_clk(xcvr); return ret; }
    ret = clk_prepare_enable((*xcvr).phy_clk);
    if ret != 0 { dev_err(dev, b"failed to start PHY clock: %d\n\0".as_ptr() as *const c_char, ret); goto_stop_pll_ipg_clk(xcvr); return ret; }
    ret = clk_prepare_enable((*xcvr).spba_clk);
    if ret != 0 { dev_err(dev, b"failed to start SPBA clock.\n\0".as_ptr() as *const c_char); goto_stop_phy_clk(xcvr); return ret; }
    ret = reset_control_deassert((*xcvr).reset);
    if ret != 0 { dev_err(dev, b"failed to deassert M0+ reset.\n\0".as_ptr() as *const c_char); goto_stop_spba_clk(xcvr); return ret; }
    regcache_cache_only((*xcvr).regmap, false);
    regcache_mark_dirty((*xcvr).regmap);
    ret = regcache_sync((*xcvr).regmap);
    if ret != 0 { dev_err(dev, b"failed to sync regcache.\n\0".as_ptr() as *const c_char); goto_stop_spba_clk(xcvr); return ret; }
    if (*(*xcvr).soc_data).use_phy {
        ret = regmap_write((*xcvr).regmap, FSL_XCVR_PHY_AI_CTRL_SET, FSL_XCVR_PHY_AI_CTRL_AI_RESETN);
        if ret < 0 { dev_err(dev, b"Error while release PHY reset: %d\n\0".as_ptr() as *const c_char, ret); goto_stop_spba_clk(xcvr); return ret; }
        regcache_cache_only((*xcvr).regmap_phy, false);
        regcache_mark_dirty((*xcvr).regmap_phy);
        ret = regcache_sync((*xcvr).regmap_phy);
        if ret != 0 { dev_err(dev, b"failed to sync phy regcache.\n\0".as_ptr() as *const c_char); goto_stop_spba_clk(xcvr); return ret; }
        regcache_cache_only((*xcvr).regmap_pll, false);
        regcache_mark_dirty((*xcvr).regmap_pll);
        ret = regcache_sync((*xcvr).regmap_pll);
        if ret != 0 { dev_err(dev, b"failed to sync pll regcache.\n\0".as_ptr() as *const c_char); goto_stop_spba_clk(xcvr); return ret; }
    }
    if !(*(*xcvr).soc_data).fw_name.is_null() {
        ret = fsl_xcvr_load_firmware(xcvr);
        if ret != 0 { dev_err(dev, b"failed to load firmware.\n\0".as_ptr() as *const c_char); goto_stop_spba_clk(xcvr); return ret; }
        /* Release M0+ reset */
        ret = regmap_update_bits((*xcvr).regmap, FSL_XCVR_EXT_CTRL, FSL_XCVR_EXT_CTRL_CORE_RESET, 0);
        if ret < 0 { dev_err(dev, b"M0+ core release failed: %d\n\0".as_ptr() as *const c_char, ret); goto_stop_spba_clk(xcvr); return ret; }
        /* Let M0+ core complete firmware initialization */
        msleep(50);
    }
    0
}

unsafe fn goto_stop_spba_clk(xcvr: *mut fsl_xcvr) {
    clk_disable_unprepare((*xcvr).spba_clk);
    goto_stop_phy_clk(xcvr);
}
unsafe fn goto_stop_phy_clk(xcvr: *mut fsl_xcvr) {
    clk_disable_unprepare((*xcvr).phy_clk);
    goto_stop_pll_ipg_clk(xcvr);
}
unsafe fn goto_stop_pll_ipg_clk(xcvr: *mut fsl_xcvr) {
    clk_disable_unprepare((*xcvr).pll_ipg_clk);
    goto_stop_ipg_clk(xcvr);
}
unsafe fn goto_stop_ipg_clk(xcvr: *mut fsl_xcvr) {
    clk_disable_unprepare((*xcvr).ipg_clk);
}

static fsl_xcvr_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut fsl_xcvr_driver: platform_driver = platform_driver {
    probe: Some(fsl_xcvr_probe),
    driver: platform_driver_inner {
        name: b"fsl-xcvr\0".as_ptr() as *const c_char,
        pm: &fsl_xcvr_pm_ops,
        of_match_table: fsl_xcvr_dt_ids.as_ptr(),
    },
    remove: Some(fsl_xcvr_remove),
};
// module_platform_driver(fsl_xcvr_driver);
// MODULE_AUTHOR("Viorel Suman <viorel.suman@nxp.com>");
// MODULE_DESCRIPTION("NXP Audio Transceiver (XCVR) driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
