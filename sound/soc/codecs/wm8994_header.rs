/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8994.h  --  WM8994 Soc Audio driver
 */

/* C dependencies omitted from executable Rust:
 * <linux/clk.h>
 * <sound/soc.h>
 * <linux/firmware.h>
 * <linux/completion.h>
 * <linux/workqueue.h>
 * <linux/mutex.h>
 * "wm_hubs.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const WM8994_MCLK1: c_int = 0;
pub const WM8994_MCLK2: c_int = 1;
pub const WM8994_NUM_MCLK: c_int = 2;

/* Sources for AIF1/2 SYSCLK - use with set_dai_sysclk() */
pub const WM8994_SYSCLK_MCLK1: c_int = 1;
pub const WM8994_SYSCLK_MCLK2: c_int = 2;
pub const WM8994_SYSCLK_FLL1: c_int = 3;
pub const WM8994_SYSCLK_FLL2: c_int = 4;

/* OPCLK is also configured with set_dai_sysclk, specify division*10 as rate. */
pub const WM8994_SYSCLK_OPCLK: c_int = 5;

pub const WM8994_FLL1: c_int = 1;
pub const WM8994_FLL2: c_int = 2;

pub const WM8994_FLL_SRC_MCLK1: c_int = 1;
pub const WM8994_FLL_SRC_MCLK2: c_int = 2;
pub const WM8994_FLL_SRC_LRCLK: c_int = 3;
pub const WM8994_FLL_SRC_BCLK: c_int = 4;
pub const WM8994_FLL_SRC_INTERNAL: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wm8994_vmid_mode {
    WM8994_VMID_NORMAL = 0,
    WM8994_VMID_FORCE = 1,
}

pub type wm1811_micdet_cb = Option<unsafe extern "C" fn(data: *mut c_void)>;
pub type wm1811_mic_id_cb = Option<unsafe extern "C" fn(data: *mut c_void, status: u16)>;

unsafe extern "C" {
    pub fn wm8994_mic_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        micbias: c_int,
    ) -> c_int;

    pub fn wm8958_mic_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        det_cb: wm1811_micdet_cb,
        det_cb_data: *mut c_void,
        id_cb: wm1811_mic_id_cb,
        id_cb_data: *mut c_void,
    ) -> c_int;

    pub fn wm8994_vmid_mode(
        component: *mut snd_soc_component,
        mode: wm8994_vmid_mode,
    ) -> c_int;

    pub fn wm8958_aif_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;

    pub fn wm8958_dsp2_init(component: *mut snd_soc_component);
}

#[repr(C)]
pub struct wm8994_micdet {
    pub jack: *mut snd_soc_jack,
    pub detecting: bool,
}

/* codec private data */
#[repr(C)]
pub struct wm8994_fll_config {
    pub src: c_int,
    pub in_: c_int,
    pub out: c_int,
}

pub const WM8994_NUM_DRC: usize = 3;
pub const WM8994_NUM_EQ: usize = 3;

#[repr(C)]
pub struct wm8994 {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wm8994_priv {
    pub hubs: wm_hubs_data,
    pub wm8994: *mut wm8994,
    pub mclk: [clk_bulk_data; WM8994_NUM_MCLK as usize],
    pub sysclk: [c_int; 2],
    pub sysclk_rate: [c_int; 2],
    pub mclk_rate: [c_int; 2],
    pub aifclk: [c_int; 2],
    pub aifdiv: [c_int; 2],
    pub channels: [c_int; 2],
    pub fll: [wm8994_fll_config; 2],
    pub fll_suspend: [wm8994_fll_config; 2],
    pub fll_locked: [completion; 2],
    pub fll_locked_irq: bool,
    pub fll_byp: bool,
    pub clk_has_run: bool,

    pub vmid_refcount: c_int,
    pub active_refcount: c_int,
    pub vmid_mode: wm8994_vmid_mode,

    pub dac_rates: [c_int; 2],
    pub lrclk_shared: [c_int; 2],

    pub mbc_ena: [c_int; 3],
    pub hpf1_ena: [c_int; 3],
    pub hpf2_ena: [c_int; 3],
    pub vss_ena: [c_int; 3],
    pub enh_eq_ena: [c_int; 3],

    /* Platform dependent DRC configuration */
    pub drc_texts: *const *const c_char,
    pub drc_cfg: [c_int; WM8994_NUM_DRC],
    pub drc_enum: soc_enum,

    /* Platform dependent ReTune mobile configuration */
    pub num_retune_mobile_texts: c_int,
    pub retune_mobile_texts: *const *const c_char,
    pub retune_mobile_cfg: [c_int; WM8994_NUM_EQ],
    pub retune_mobile_enum: soc_enum,

    /* Platform dependent MBC configuration */
    pub mbc_cfg: c_int,
    pub mbc_texts: *const *const c_char,
    pub mbc_enum: soc_enum,

    /* Platform dependent VSS configuration */
    pub vss_cfg: c_int,
    pub vss_texts: *const *const c_char,
    pub vss_enum: soc_enum,

    /* Platform dependent VSS HPF configuration */
    pub vss_hpf_cfg: c_int,
    pub vss_hpf_texts: *const *const c_char,
    pub vss_hpf_enum: soc_enum,

    /* Platform dependent enhanced EQ configuration */
    pub enh_eq_cfg: c_int,
    pub enh_eq_texts: *const *const c_char,
    pub enh_eq_enum: soc_enum,

    pub accdet_lock: mutex,
    pub micdet: [wm8994_micdet; 2],
    pub mic_work: delayed_work,
    pub open_circuit_work: delayed_work,
    pub mic_complete_work: delayed_work,
    pub mic_status: u16,
    pub mic_detecting: bool,
    pub jack_mic: bool,
    pub btn_mask: c_int,
    pub jackdet: bool,
    pub jackdet_mode: c_int,
    pub jackdet_bootstrap: delayed_work,

    pub micdet_irq: c_int,
    pub micd_cb: wm1811_micdet_cb,
    pub micd_cb_data: *mut c_void,
    pub mic_id_cb: wm1811_mic_id_cb,
    pub mic_id_cb_data: *mut c_void,

    /* C bitfields: unsigned int aif1clk_enable:1; unsigned int aif2clk_enable:1; */
    pub aif1clk_enable: c_uint,
    pub aif2clk_enable: c_uint,

    /* C bitfields: unsigned int aif1clk_disable:1; unsigned int aif2clk_disable:1; */
    pub aif1clk_disable: c_uint,
    pub aif2clk_disable: c_uint,

    pub fw_lock: mutex,
    pub dsp_active: c_int,
    pub cur_fw: *const firmware,
    pub mbc: *const firmware,
    pub mbc_vss: *const firmware,
    pub enh_eq: *const firmware,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
