// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car SSIU/SSI support
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// Based on fsi.c
// Kuninori Morimoto <morimoto.kuninori@renesas.com>

/*
 * you can enable below define if you don't need
 * SSI interrupt status debug message when debugging
 * see rsnd_print_irq_status()
 *
 * #define RSND_DEBUG_NO_IRQ_STATUS 1
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = c_uint;
type bool_t = bool;
type snd_pcm_uframes_t = c_uint;
type irqreturn_t = c_int;

const FORCE: u32 = 1u32 << 31; /* Fixed */
const DMEN: u32 = 1u32 << 28; /* DMA Enable */
const UIEN: u32 = 1u32 << 27; /* Underflow Interrupt Enable */
const OIEN: u32 = 1u32 << 26; /* Overflow Interrupt Enable */
const IIEN: u32 = 1u32 << 25; /* Idle Mode Interrupt Enable */
const DIEN: u32 = 1u32 << 24; /* Data Interrupt Enable */
const CHNL_4: u32 = 1u32 << 22; /* Channels */
const CHNL_6: u32 = 2u32 << 22; /* Channels */
const CHNL_8: u32 = 3u32 << 22; /* Channels */
const DWL_MASK: u32 = 7u32 << 19; /* Data Word Length mask */
const DWL_8: u32 = 0u32 << 19; /* Data Word Length */
const DWL_16: u32 = 1u32 << 19; /* Data Word Length */
const DWL_18: u32 = 2u32 << 19; /* Data Word Length */
const DWL_20: u32 = 3u32 << 19; /* Data Word Length */
const DWL_22: u32 = 4u32 << 19; /* Data Word Length */
const DWL_24: u32 = 5u32 << 19; /* Data Word Length */
const DWL_32: u32 = 6u32 << 19; /* Data Word Length */

/*
 * System word length
 */
const SWL_16: u32 = 1 << 16; /* R/W System Word Length */
const SWL_24: u32 = 2 << 16; /* R/W System Word Length */
const SWL_32: u32 = 3 << 16; /* R/W System Word Length */

const SCKD: u32 = 1 << 15; /* Serial Bit Clock Direction */
const SWSD: u32 = 1 << 14; /* Serial WS Direction */
const SCKP: u32 = 1 << 13; /* Serial Bit Clock Polarity */
const SWSP: u32 = 1 << 12; /* Serial WS Polarity */
const SDTA: u32 = 1 << 10; /* Serial Data Alignment */
const PDTA: u32 = 1 << 9; /* Parallel Data Alignment */
const DEL: u32 = 1 << 8; /* Serial Data Delay */
#[inline]
const fn CKDV(v: c_int) -> u32 {
    (v as u32) << 4
}
const TRMD: u32 = 1 << 1; /* Transmit/Receive Mode Select */
const EN: u32 = 1 << 0; /* SSI Module Enable */

const UIRQ: u32 = 1 << 27; /* Underflow Error Interrupt Status */
const OIRQ: u32 = 1 << 26; /* Overflow Error Interrupt Status */
const IIRQ: u32 = 1 << 25; /* Idle Mode Interrupt Status */
const DIRQ: u32 = 1 << 24; /* Data Interrupt Status Flag */

const CONT: u32 = 1 << 8; /* WS Continue Function */
const WS_MODE: u32 = 1 << 0; /* WS Mode */

const SSI_NAME: *const c_char = b"ssi\0".as_ptr() as *const c_char;

const RSND_SSI_CLK_PIN_SHARE: u32 = 1 << 0;
const RSND_SSI_NO_BUSIF: u32 = 1 << 1; /* SSI+DMA without BUSIF */
const RSND_SSI_PROBED: u32 = 1 << 2;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const IRQF_SHARED: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;

#[repr(C)]
pub struct rsnd_mod {
    pub ops: *mut rsnd_mod_ops,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
}

#[repr(C)]
pub struct rsnd_ssi {
    pub mod_: rsnd_mod,
    pub flags: u32,
    pub cr_own: u32,
    pub cr_clk: u32,
    pub cr_mode: u32,
    pub cr_en: u32,
    pub wsr: u32,
    pub chan: c_int,
    pub rate: c_int,
    pub irq: c_int,
    pub usrcnt: c_uint,
    /* for PIO */
    pub byte_pos: c_int,
    pub byte_per_period: c_int,
    pub next_period_byte: c_int,
}

#[repr(C)]
pub struct rsnd_priv {
    pub ssi: *mut rsnd_ssi,
    pub ssi_nr: c_int,
    pub lock: c_int,
}

#[repr(C)]
pub struct rsnd_dai_stream {
    pub substream: *mut snd_pcm_substream,
    pub dma: *mut c_void,
    pub parent_ssi_status: u32,
}

#[repr(C)]
pub struct rsnd_dai {
    pub playback: rsnd_dai_stream,
    pub capture: rsnd_dai_stream,
    pub bit_clk_inv: c_int,
    pub frm_clk_inv: c_int,
    pub data_alignment: c_int,
    pub sys_delay: c_int,
    pub chan_width: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut u8,
    pub format: c_int,
    pub period_size: c_int,
    pub channels: c_int,
    pub periods: c_int,
}

#[repr(C)]
pub struct rsnd_mod_ops {
    pub name: *const c_char,
    pub dma_req: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod) -> *mut dma_chan>,
    pub probe: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub quit: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub start: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub irq: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_pcm_uframes_t) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_soc_pcm_runtime) -> c_int>,
    pub fallback: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub get_status: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, rsnd_mod_type) -> *mut u32>,
    /* CONFIG_DEBUG_FS: .debug_info = rsnd_ssi_debug_info */
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct dma_chan { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct reset_control { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsnd_mod_type {
    RSND_MOD_SSI,
    RSND_MOD_SSIM1,
    RSND_MOD_SSIM2,
    RSND_MOD_SSIM3,
    RSND_MOD_SSIP,
}

extern "C" {
    static SSISR: c_int;
    static SSIWSR: c_int;
    static SSICR: c_int;
    static SSI_INT_ENABLE: c_int;
    static SSITDR: c_int;
    static SSIRDR: c_int;
    static RSND_BASE_SSI: c_int;

    fn rsnd_io_to_mod_ssi(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_ssip(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_src(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod(io: *mut rsnd_dai_stream, type_: rsnd_mod_type) -> *mut rsnd_mod;
    fn rsnd_mod_id(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv;
    fn rsnd_priv_to_dev(priv_: *mut rsnd_priv) -> *mut device;
    fn rsnd_rdai_to_priv(rdai: *mut rsnd_dai) -> *mut rsnd_priv;
    fn rsnd_io_to_priv(io: *mut rsnd_dai_stream) -> *mut rsnd_priv;
    fn rsnd_io_to_rdai(io: *mut rsnd_dai_stream) -> *mut rsnd_dai;
    fn rsnd_io_to_runtime(io: *mut rsnd_dai_stream) -> *mut snd_pcm_runtime;
    fn rsnd_mod_write(mod_: *mut rsnd_mod, reg: c_int, data: u32);
    fn rsnd_mod_read(mod_: *mut rsnd_mod, reg: c_int) -> u32;
    fn udelay(usecs: c_uint);
    fn rsnd_mod_name(mod_: *mut rsnd_mod) -> *const c_char;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn rsnd_runtime_is_multi_ssi(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_rdai_width_get(rdai: *mut rsnd_dai) -> c_int;
    fn rsnd_adg_clk_query(priv_: *mut rsnd_priv, rate: c_uint) -> c_int;
    fn rsnd_rdai_is_clk_master(rdai: *mut rsnd_dai) -> c_int;
    fn rsnd_runtime_channel_for_ssi(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_runtime_is_tdm_split(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_io_converted_chan(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_channel_normalization(chan: c_int) -> c_int;
    fn rsnd_adg_ssi_clk_try_start(mod_: *mut rsnd_mod, rate: c_uint) -> c_int;
    fn rsnd_adg_ssi_clk_stop(mod_: *mut rsnd_mod);
    fn rsnd_src_get_out_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> c_uint;
    fn rsnd_src_get_in_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> c_uint;
    fn rsnd_runtime_is_tdm(io: *mut rsnd_dai_stream) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn rsnd_ssi_is_dma_mode(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_power_on(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_power_off(mod_: *mut rsnd_mod);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn rsnd_io_is_play(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_is_gen1(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_io_is_working(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_print_irq_status(dev: *mut device, fmt: *const c_char, ...);
    fn rsnd_ssiu_busif_err_status_clear(mod_: *mut rsnd_mod) -> bool_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn rsnd_mod_interrupt(mod_: *mut rsnd_mod, cb: unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream));
    fn rsnd_mod_get_status(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, type_: rsnd_mod_type) -> *mut u32;
    fn rsnd_dai_connect(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, type_: rsnd_mod_type);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn samples_to_bytes(runtime: *mut snd_pcm_runtime, samples: c_int) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_int) -> snd_pcm_uframes_t;
    fn rsnd_dma_attach(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod, dma: *mut *mut c_void) -> c_int;
    fn rsnd_dma_request_channel(node: *mut device_node, name: *const c_char, mod_: *mut rsnd_mod, dma_name: *const c_char) -> *mut dma_chan;
    fn rsnd_ssi_of_node(priv_: *mut rsnd_priv) -> *mut device_node;
    fn rsnd_rdai_channels_set(rdai: *mut rsnd_dai, channels: c_int);
    fn rsnd_rdai_ssi_lane_set(rdai: *mut rsnd_dai, lane: c_int);
    fn rsnd_node_fixed_index(dev: *mut device, node: *mut device_node, name: *const c_char, i: c_int) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn rsnd_node_count(priv_: *mut rsnd_priv, node: *mut device_node, name: *const c_char) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: c_int, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_is_available(node: *mut device_node) -> c_int;
    fn rsnd_devm_clk_get_indexed(dev: *mut device, name: *const c_char, i: c_int) -> *mut clk;
    fn rsnd_devm_reset_control_get_optional_indexed(dev: *mut device, name: *const c_char, i: c_int) -> *mut reset_control;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_property_read_bool(node: *mut device_node, name: *const c_char) -> c_int;
    fn irq_of_parse_and_map(node: *mut device_node, index: c_int) -> c_int;
    fn rsnd_mod_init(priv_: *mut rsnd_priv, mod_: *mut rsnd_mod, ops: *mut rsnd_mod_ops, clk: *mut clk, rstc: *mut reset_control, type_: rsnd_mod_type, id: c_int) -> c_int;
    fn rsnd_mod_get(ssi: *mut rsnd_ssi) -> *mut rsnd_mod;
    fn rsnd_mod_quit(mod_: *mut rsnd_mod);
    fn rsnd_suspend_clk_reset(clk: *mut clk, rstc: *mut reset_control);
    fn rsnd_resume_clk_reset(clk: *mut clk, rstc: *mut reset_control);
    fn WARN_ON(condition: c_int) -> c_int;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn rsnd_debugfs_mod_reg_show(m: *mut seq_file, mod_: *mut rsnd_mod, base: c_int, offset: c_int, size: c_int);
}

#[inline]
unsafe fn rsnd_mod_to_ssi(mod_: *mut rsnd_mod) -> *mut rsnd_ssi {
    mod_ as *mut rsnd_ssi
}

#[inline]
unsafe fn rsnd_ssi_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_ssi {
    (*priv_).ssi.add(id as usize)
}

#[inline]
unsafe fn rsnd_ssi_nr(priv_: *mut rsnd_priv) -> c_int {
    (*priv_).ssi_nr
}

#[inline]
unsafe fn rsnd_flags_has(ssi: *mut rsnd_ssi, flag: u32) -> bool {
    ((*ssi).flags & flag) != 0
}

#[inline]
unsafe fn rsnd_flags_set(ssi: *mut rsnd_ssi, flag: u32) {
    (*ssi).flags |= flag;
}

#[inline]
unsafe fn rsnd_flags_del(ssi: *mut rsnd_ssi, flag: u32) {
    (*ssi).flags &= !flag;
}

#[inline]
unsafe fn rsnd_ssi_is_parent(ssi: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> bool {
    ssi == rsnd_io_to_mod_ssip(io)
}

#[inline]
unsafe fn rsnd_ssi_is_multi_secondary(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> bool {
    (rsnd_ssi_multi_secondaries(io) & (1u32 << rsnd_mod_id(mod_))) != 0
}

#[inline]
unsafe fn rsnd_ssi_is_run_mods(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> bool {
    (rsnd_ssi_run_mods(io) & (1u32 << rsnd_mod_id(mod_))) != 0
}

#[inline]
unsafe fn rsnd_ssi_can_output_clk(mod_: *mut rsnd_mod) -> bool {
    __rsnd_ssi_is_pin_sharing(mod_) == 0
}

pub unsafe extern "C" fn rsnd_ssi_use_busif(io: *mut rsnd_dai_stream) -> c_int {
    let mod_ = rsnd_io_to_mod_ssi(io);
    let ssi = rsnd_mod_to_ssi(mod_);
    let mut use_busif = 0;

    if rsnd_ssi_is_dma_mode(mod_) == 0 {
        return 0;
    }

    if !rsnd_flags_has(ssi, RSND_SSI_NO_BUSIF) {
        use_busif = 1;
    }
    if !rsnd_io_to_mod_src(io).is_null() {
        use_busif = 1;
    }

    use_busif
}

unsafe extern "C" fn rsnd_ssi_status_clear(mod_: *mut rsnd_mod) {
    rsnd_mod_write(mod_, SSISR, 0);
}

unsafe extern "C" fn rsnd_ssi_status_get(mod_: *mut rsnd_mod) -> u32 {
    rsnd_mod_read(mod_, SSISR)
}

unsafe extern "C" fn rsnd_ssi_status_check(mod_: *mut rsnd_mod, bit: u32) {
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let mut i = 0;

    while i < 1024 {
        let status = rsnd_ssi_status_get(mod_);
        if (status & bit) != 0 {
            return;
        }

        udelay(5);
        i += 1;
    }

    dev_warn(dev, b"%s status check failed\n\0".as_ptr() as *const c_char, rsnd_mod_name(mod_));
}

unsafe extern "C" fn rsnd_ssi_multi_secondaries(io: *mut rsnd_dai_stream) -> u32 {
    static TYPES: [rsnd_mod_type; 3] = [
        rsnd_mod_type::RSND_MOD_SSIM1,
        rsnd_mod_type::RSND_MOD_SSIM2,
        rsnd_mod_type::RSND_MOD_SSIM3,
    ];
    let mut mask: c_int = 0;

    for type_ in TYPES {
        let mod_ = rsnd_io_to_mod(io, type_);
        if mod_.is_null() {
            continue;
        }

        mask |= 1 << rsnd_mod_id(mod_);
    }

    mask as u32
}

unsafe extern "C" fn rsnd_ssi_run_mods(io: *mut rsnd_dai_stream) -> u32 {
    let ssi_mod = rsnd_io_to_mod_ssi(io);
    let ssi_parent_mod = rsnd_io_to_mod_ssip(io);
    let mut mods = rsnd_ssi_multi_secondaries_runtime(io) | (1u32 << rsnd_mod_id(ssi_mod));

    if !ssi_parent_mod.is_null() {
        mods |= 1u32 << rsnd_mod_id(ssi_parent_mod);
    }

    mods
}

pub unsafe extern "C" fn rsnd_ssi_multi_secondaries_runtime(io: *mut rsnd_dai_stream) -> u32 {
    if rsnd_runtime_is_multi_ssi(io) != 0 {
        return rsnd_ssi_multi_secondaries(io);
    }

    0
}

unsafe extern "C" fn rsnd_rdai_width_to_swl(rdai: *mut rsnd_dai) -> u32 {
    let priv_ = rsnd_rdai_to_priv(rdai);
    let dev = rsnd_priv_to_dev(priv_);
    let width = rsnd_rdai_width_get(rdai);

    match width {
        32 => SWL_32,
        24 => SWL_24,
        16 => SWL_16,
        _ => {
            dev_err(dev, b"unsupported slot width value: %d\n\0".as_ptr() as *const c_char, width);
            0
        }
    }
}

pub unsafe extern "C" fn rsnd_ssi_clk_query(
    rdai: *mut rsnd_dai,
    param1: c_int,
    param2: c_int,
    idx: *mut c_int,
) -> c_uint {
    let priv_ = rsnd_rdai_to_priv(rdai);
    static SSI_CLK_MUL_TABLE: [c_int; 7] = [1, 2, 4, 8, 16, 6, 12];
    let width = rsnd_rdai_width_get(rdai);

    for j in 0..SSI_CLK_MUL_TABLE.len() {
        /*
         * It will set SSIWSR.CONT here, but SSICR.CKDV = 000
         * with it is not allowed. (SSIWSR.WS_MODE with
         * SSICR.CKDV = 000 is not allowed either).
         * Skip it. See SSICR.CKDV
         */
        if j == 0 {
            continue;
        }

        let main_rate = (width * param1 * param2 * SSI_CLK_MUL_TABLE[j]) as c_uint;

        let ret = rsnd_adg_clk_query(priv_, main_rate);
        if ret < 0 {
            continue;
        }

        if !idx.is_null() {
            *idx = j as c_int;
        }

        return main_rate;
    }

    0
}

unsafe extern "C" fn rsnd_ssi_master_clk_start(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> c_int {
    let priv_ = rsnd_io_to_priv(io);
    let dev = rsnd_priv_to_dev(priv_);
    let rdai = rsnd_io_to_rdai(io);
    let ssi = rsnd_mod_to_ssi(mod_);
    let mut chan = rsnd_runtime_channel_for_ssi(io);
    let mut idx: c_int = 0;
    let rate = if rsnd_io_is_play(io) != 0 {
        rsnd_src_get_out_rate(priv_, io)
    } else {
        rsnd_src_get_in_rate(priv_, io)
    };

    if rsnd_rdai_is_clk_master(rdai) == 0 || !rsnd_ssi_can_output_clk(mod_) || rsnd_ssi_is_multi_secondary(mod_, io) {
        return 0;
    }

    if rsnd_runtime_is_tdm_split(io) != 0 {
        chan = rsnd_io_converted_chan(io);
    }

    chan = rsnd_channel_normalization(chan);

    if (*ssi).usrcnt > 0 {
        if (*ssi).rate != rate as c_int {
            dev_err(dev, b"SSI parent/child should use same rate\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        if (*ssi).chan != chan {
            dev_err(dev, b"SSI parent/child should use same chan\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        return 0;
    }

    let mut ret = -EIO;
    let main_rate = rsnd_ssi_clk_query(rdai, rate as c_int, chan, &mut idx);
    if main_rate == 0 {
        dev_err(dev, b"unsupported clock rate (%d)\n\0".as_ptr() as *const c_char, rate);
        return ret;
    }

    ret = rsnd_adg_ssi_clk_try_start(mod_, main_rate);
    if ret < 0 {
        dev_err(dev, b"unsupported clock rate (%d)\n\0".as_ptr() as *const c_char, rate);
        return ret;
    }

    /*
     * SSI clock will be output contiguously
     * by below settings.
     * This means, rsnd_ssi_master_clk_start()
     * and rsnd_ssi_register_setup() are necessary
     * for SSI parent
     *
     * SSICR  : FORCE, SCKD, SWSD
     * SSIWSR : CONT
     */
    (*ssi).cr_clk = FORCE | rsnd_rdai_width_to_swl(rdai) | SCKD | SWSD | CKDV(idx);
    (*ssi).wsr = CONT;
    (*ssi).rate = rate as c_int;
    (*ssi).chan = chan;

    dev_dbg(dev, b"%s outputs %d chan %u Hz\n\0".as_ptr() as *const c_char, rsnd_mod_name(mod_), chan, rate);

    0
}

unsafe extern "C" fn rsnd_ssi_master_clk_stop(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) {
    let rdai = rsnd_io_to_rdai(io);
    let ssi = rsnd_mod_to_ssi(mod_);

    if rsnd_rdai_is_clk_master(rdai) == 0 || !rsnd_ssi_can_output_clk(mod_) || (*ssi).usrcnt > 1 {
        return;
    }

    (*ssi).cr_clk = 0;
    (*ssi).rate = 0;
    (*ssi).chan = 0;

    rsnd_adg_ssi_clk_stop(mod_);
}

unsafe extern "C" fn rsnd_ssi_config_init(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) {
    let rdai = rsnd_io_to_rdai(io);
    let priv_ = rsnd_rdai_to_priv(rdai);
    let dev = rsnd_priv_to_dev(priv_);
    let runtime = rsnd_io_to_runtime(io);
    let ssi = rsnd_mod_to_ssi(mod_);
    let mut cr_own = (*ssi).cr_own;
    let mut cr_mode = (*ssi).cr_mode;
    let mut wsr = (*ssi).wsr;
    let is_tdm = rsnd_runtime_is_tdm(io);
    let is_tdm_split = rsnd_runtime_is_tdm_split(io);

    if is_tdm != 0 {
        dev_dbg(dev, b"TDM mode\n\0".as_ptr() as *const c_char);
    }
    if is_tdm_split != 0 {
        dev_dbg(dev, b"TDM Split mode\n\0".as_ptr() as *const c_char);
    }

    cr_own |= FORCE | rsnd_rdai_width_to_swl(rdai);

    if (*rdai).bit_clk_inv != 0 { cr_own |= SCKP; }
    if (*rdai).frm_clk_inv != 0 && is_tdm == 0 { cr_own |= SWSP; }
    if (*rdai).data_alignment != 0 { cr_own |= SDTA; }
    if (*rdai).sys_delay != 0 { cr_own |= DEL; }

    /*
     * TDM Mode
     * see
     *      rsnd_ssiu_init_gen2()
     */
    if is_tdm != 0 || is_tdm_split != 0 {
        wsr |= WS_MODE;
        cr_own |= CHNL_8;
    }

    /*
     * We shouldn't exchange SWSP after running.
     * This means, parent needs to care it.
     */
    if !rsnd_ssi_is_parent(mod_, io) {
        if rsnd_io_is_play(io) != 0 {
            cr_own |= TRMD;
        }

        cr_own &= !DWL_MASK;
        let mut width = snd_pcm_format_width((*runtime).format);
        if is_tdm_split != 0 {
            /*
             * The SWL and DWL bits in SSICR should be fixed at 32-bit
             * setting when TDM split mode.
             * see datasheet
             *      Operation :: TDM Format Split Function (TDM Split Mode)
             */
            width = 32;
        }

        match width {
            8 => cr_own |= DWL_8,
            16 => cr_own |= DWL_16,
            24 => cr_own |= DWL_24,
            32 => cr_own |= DWL_32,
            _ => {}
        }

        if rsnd_ssi_is_dma_mode(mod_) != 0 {
            cr_mode = UIEN | OIEN | DMEN;
        } else {
            cr_mode = DIEN;
        }
    }

    (*ssi).cr_own = cr_own;
    (*ssi).cr_mode = cr_mode;
    (*ssi).wsr = wsr;
}

unsafe extern "C" fn rsnd_ssi_register_setup(mod_: *mut rsnd_mod) {
    let ssi = rsnd_mod_to_ssi(mod_);

    rsnd_mod_write(mod_, SSIWSR, (*ssi).wsr);
    rsnd_mod_write(mod_, SSICR, (*ssi).cr_own | (*ssi).cr_clk | (*ssi).cr_mode | (*ssi).cr_en);
}

unsafe extern "C" fn rsnd_ssi_init(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv) -> c_int {
    let ssi = rsnd_mod_to_ssi(mod_);

    if !rsnd_ssi_is_run_mods(mod_, io) {
        return 0;
    }

    let mut ret = rsnd_ssi_master_clk_start(mod_, io);
    if ret < 0 {
        return ret;
    }

    (*ssi).usrcnt += 1;

    ret = rsnd_mod_power_on(mod_);
    if ret < 0 {
        return ret;
    }

    rsnd_ssi_config_init(mod_, io);
    rsnd_ssi_register_setup(mod_);
    rsnd_ssi_status_clear(mod_);

    0
}

unsafe extern "C" fn rsnd_ssi_quit(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, priv_: *mut rsnd_priv) -> c_int {
    let ssi = rsnd_mod_to_ssi(mod_);
    let dev = rsnd_priv_to_dev(priv_);

    if !rsnd_ssi_is_run_mods(mod_, io) {
        return 0;
    }

    if (*ssi).usrcnt == 0 {
        dev_err(dev, b"%s usrcnt error\n\0".as_ptr() as *const c_char, rsnd_mod_name(mod_));
        return -EIO;
    }

    rsnd_ssi_master_clk_stop(mod_, io);
    rsnd_mod_power_off(mod_);
    (*ssi).usrcnt -= 1;

    if (*ssi).usrcnt == 0 {
        (*ssi).cr_own = 0;
        (*ssi).cr_mode = 0;
        (*ssi).wsr = 0;
    }

    0
}

unsafe extern "C" fn rsnd_ssi_hw_params(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, _substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rdai = rsnd_io_to_rdai(io);
    let fmt_width = snd_pcm_format_width(params_format(params)) as c_uint;

    if fmt_width > (*rdai).chan_width {
        let priv_ = rsnd_io_to_priv(io);
        let dev = rsnd_priv_to_dev(priv_);

        dev_err(dev, b"invalid combination of slot-width and format-data-width\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn rsnd_ssi_start(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv) -> c_int {
    let ssi = rsnd_mod_to_ssi(mod_);

    if !rsnd_ssi_is_run_mods(mod_, io) || rsnd_ssi_multi_secondaries_runtime(io) != 0 || rsnd_ssi_is_parent(mod_, io) {
        return 0;
    }

    (*ssi).cr_en = EN;
    rsnd_mod_write(mod_, SSICR, (*ssi).cr_own | (*ssi).cr_clk | (*ssi).cr_mode | (*ssi).cr_en);

    0
}

unsafe extern "C" fn rsnd_ssi_stop(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv) -> c_int {
    let ssi = rsnd_mod_to_ssi(mod_);

    if !rsnd_ssi_is_run_mods(mod_, io) || rsnd_ssi_is_parent(mod_, io) {
        return 0;
    }

    let cr = (*ssi).cr_own | (*ssi).cr_clk;

    /*
     * disable all IRQ,
     * Playback: Wait all data was sent
     * Capture:  It might not receave data. Do nothing
     */
    if rsnd_io_is_play(io) != 0 {
        rsnd_mod_write(mod_, SSICR, cr | (*ssi).cr_en);
        rsnd_ssi_status_check(mod_, DIRQ);
    }

    /* In multi-SSI mode, stop is performed by setting ssi0129 in
     * SSI_CONTROL to 0 (in rsnd_ssio_stop_gen2). Do nothing here.
     */
    if rsnd_ssi_multi_secondaries_runtime(io) != 0 {
        return 0;
    }

    rsnd_mod_write(mod_, SSICR, cr);
    rsnd_ssi_status_check(mod_, IIRQ);
    (*ssi).cr_en = 0;

    0
}

unsafe extern "C" fn rsnd_ssi_irq(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, priv_: *mut rsnd_priv, enable: c_int) -> c_int {
    let mut val: u32 = 0;
    let is_tdm = rsnd_runtime_is_tdm(io);
    let is_tdm_split = rsnd_runtime_is_tdm_split(io);
    let id = rsnd_mod_id(mod_);

    if rsnd_is_gen1(priv_) != 0 || rsnd_ssi_is_parent(mod_, io) || !rsnd_ssi_is_run_mods(mod_, io) {
        return 0;
    }

    if enable != 0 {
        val = if rsnd_ssi_is_dma_mode(mod_) != 0 { 0x0e000000 } else { 0x0f000000 };
    }

    if is_tdm != 0 || is_tdm_split != 0 {
        match id {
            0 | 1 | 2 | 3 | 4 | 9 => val |= 0x0000ff00,
            _ => {}
        }
    }

    rsnd_mod_write(mod_, SSI_INT_ENABLE, val);

    0
}

unsafe extern "C" fn rsnd_ssi_pio_interrupt(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> bool_t {
    let runtime = rsnd_io_to_runtime(io);
    let ssi = rsnd_mod_to_ssi(mod_);
    let buf = (*runtime).dma_area.add((*ssi).byte_pos as usize) as *mut u32;
    let mut shift = 0;
    let mut elapsed = false;

    if snd_pcm_format_width((*runtime).format) == 24 {
        shift = 8;
    }

    /*
     * 8/16/32 data can be assesse to TDR/RDR register
     * directly as 32bit data
     * see rsnd_ssi_init()
     */
    if rsnd_io_is_play(io) != 0 {
        rsnd_mod_write(mod_, SSITDR, (*buf) << shift);
    } else {
        *buf = rsnd_mod_read(mod_, SSIRDR) >> shift;
    }

    let mut byte_pos = (*ssi).byte_pos + core::mem::size_of::<u32>() as c_int;

    if byte_pos >= (*ssi).next_period_byte {
        let mut period_pos = byte_pos / (*ssi).byte_per_period;

        if period_pos >= (*runtime).periods {
            byte_pos = 0;
            period_pos = 0;
        }

        (*ssi).next_period_byte = (period_pos + 1) * (*ssi).byte_per_period;
        elapsed = true;
    }

    core::ptr::write_volatile(&mut (*ssi).byte_pos, byte_pos);

    elapsed
}

unsafe extern "C" fn __rsnd_ssi_interrupt(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) {
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let is_dma = rsnd_ssi_is_dma_mode(mod_);
    let mut elapsed = false;
    let mut stop = false;

    /*
     * C used scoped_guard(spinlock, &priv->lock). The translated body keeps
     * the same ordering; locking itself is supplied by the kernel-side helper.
     */
    if rsnd_io_is_working(io) != 0 {
        let status = rsnd_ssi_status_get(mod_);

        if is_dma == 0 && (status & DIRQ) != 0 {
            elapsed = rsnd_ssi_pio_interrupt(mod_, io);
        }

        if is_dma != 0 && (status & (UIRQ | OIRQ)) != 0 {
            rsnd_print_irq_status(dev, b"%s err status : 0x%08x\n\0".as_ptr() as *const c_char, rsnd_mod_name(mod_), status);
            stop = true;
        }

        stop |= rsnd_ssiu_busif_err_status_clear(mod_);
        rsnd_ssi_status_clear(mod_);
    }

    if elapsed {
        snd_pcm_period_elapsed((*io).substream);
    }

    if stop {
        snd_pcm_stop_xrun((*io).substream);
    }
}

unsafe extern "C" fn rsnd_ssi_interrupt(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let mod_ = data as *mut rsnd_mod;

    rsnd_mod_interrupt(mod_, __rsnd_ssi_interrupt);

    IRQ_HANDLED
}

unsafe extern "C" fn rsnd_ssi_get_status(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, type_: rsnd_mod_type) -> *mut u32 {
    /*
     * SSIP (= SSI parent) needs to be special, otherwise,
     * 2nd SSI might doesn't start. see also rsnd_mod_call()
     */
    if type_ == rsnd_mod_type::RSND_MOD_SSIP {
        return &mut (*io).parent_ssi_status;
    }

    rsnd_mod_get_status(mod_, io, type_)
}

unsafe extern "C" fn rsnd_ssi_parent_attach(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) {
    let rdai = rsnd_io_to_rdai(io);
    let priv_ = rsnd_mod_to_priv(mod_);

    if __rsnd_ssi_is_pin_sharing(mod_) == 0 || rsnd_rdai_is_clk_master(rdai) == 0 || rsnd_ssi_is_multi_secondary(mod_, io) {
        return;
    }

    match rsnd_mod_id(mod_) {
        1 | 2 | 9 => rsnd_dai_connect(rsnd_ssi_mod_get(priv_, 0), io, rsnd_mod_type::RSND_MOD_SSIP),
        4 => rsnd_dai_connect(rsnd_ssi_mod_get(priv_, 3), io, rsnd_mod_type::RSND_MOD_SSIP),
        8 => rsnd_dai_connect(rsnd_ssi_mod_get(priv_, 7), io, rsnd_mod_type::RSND_MOD_SSIP),
        _ => {}
    }
}

unsafe extern "C" fn rsnd_ssi_pcm_new(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, _rtd: *mut snd_soc_pcm_runtime) -> c_int {
    /*
     * rsnd_rdai_is_clk_master() will be enabled after set_fmt,
     * and, pcm_new will be called after it.
     * This function reuse pcm_new at this point.
     */
    rsnd_ssi_parent_attach(mod_, io);

    0
}

unsafe extern "C" fn rsnd_ssi_common_probe(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, priv_: *mut rsnd_priv) -> c_int {
    let dev = rsnd_priv_to_dev(priv_);
    let ssi = rsnd_mod_to_ssi(mod_);
    let mut ret = 0;

    if rsnd_ssi_is_multi_secondary(mod_, io) {
        return 0;
    }

    if !rsnd_flags_has(ssi, RSND_SSI_PROBED) {
        ret = request_irq((*ssi).irq, rsnd_ssi_interrupt, IRQF_SHARED, dev_name(dev), mod_ as *mut c_void);
        rsnd_flags_set(ssi, RSND_SSI_PROBED);
    }

    ret
}

unsafe extern "C" fn rsnd_ssi_common_remove(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv) -> c_int {
    let ssi = rsnd_mod_to_ssi(mod_);
    let pure_ssi_mod = rsnd_io_to_mod_ssi(io);

    if pure_ssi_mod != mod_ {
        return 0;
    }

    if rsnd_flags_has(ssi, RSND_SSI_PROBED) {
        free_irq((*ssi).irq, mod_ as *mut c_void);
        rsnd_flags_del(ssi, RSND_SSI_PROBED);
    }

    0
}

unsafe extern "C" fn rsnd_ssi_pio_init(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, priv_: *mut rsnd_priv) -> c_int {
    let runtime = rsnd_io_to_runtime(io);
    let ssi = rsnd_mod_to_ssi(mod_);

    if !rsnd_ssi_is_parent(mod_, io) {
        (*ssi).byte_pos = 0;
        (*ssi).byte_per_period = (*runtime).period_size * (*runtime).channels * samples_to_bytes(runtime, 1);
        (*ssi).next_period_byte = (*ssi).byte_per_period;
    }

    rsnd_ssi_init(mod_, io, priv_)
}

unsafe extern "C" fn rsnd_ssi_pio_pointer(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, pointer: *mut snd_pcm_uframes_t) -> c_int {
    let ssi = rsnd_mod_to_ssi(mod_);
    let runtime = rsnd_io_to_runtime(io);

    *pointer = bytes_to_frames(runtime, core::ptr::read_volatile(&(*ssi).byte_pos));

    0
}

static mut RSND_SSI_PIO_OPS: rsnd_mod_ops = rsnd_mod_ops {
    name: SSI_NAME,
    dma_req: None,
    probe: Some(rsnd_ssi_common_probe),
    remove: Some(rsnd_ssi_common_remove),
    init: Some(rsnd_ssi_pio_init),
    quit: Some(rsnd_ssi_quit),
    start: Some(rsnd_ssi_start),
    stop: Some(rsnd_ssi_stop),
    irq: Some(rsnd_ssi_irq),
    pointer: Some(rsnd_ssi_pio_pointer),
    pcm_new: Some(rsnd_ssi_pcm_new),
    fallback: None,
    hw_params: Some(rsnd_ssi_hw_params),
    get_status: Some(rsnd_ssi_get_status),
};

unsafe extern "C" fn rsnd_ssi_dma_probe(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, priv_: *mut rsnd_priv) -> c_int {
    if rsnd_ssi_is_multi_secondary(mod_, io) {
        return 0;
    }

    let mut ret = rsnd_ssi_common_probe(mod_, io, priv_);
    if ret != 0 {
        return ret;
    }

    ret = rsnd_dma_attach(io, mod_, &mut (*io).dma);

    ret
}

unsafe extern "C" fn rsnd_ssi_fallback(mod_: *mut rsnd_mod, _io: *mut rsnd_dai_stream, priv_: *mut rsnd_priv) -> c_int {
    let dev = rsnd_priv_to_dev(priv_);

    /*
     * fallback to PIO
     *
     * SSI .probe might be called again.
     * see
     *      rsnd_rdai_continuance_probe()
     */
    (*mod_).ops = &mut RSND_SSI_PIO_OPS;

    dev_info(dev, b"%s fallback to PIO mode\n\0".as_ptr() as *const c_char, rsnd_mod_name(mod_));

    0
}

unsafe extern "C" fn rsnd_ssi_dma_req(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> *mut dma_chan {
    let priv_ = rsnd_mod_to_priv(mod_);
    let is_play = rsnd_io_is_play(io);
    let name: *const c_char;

    /*
     * It should use "rcar_sound,ssiu" (R-Car) or "ssiu" (RZ/G3E) on DT.
     * We need to keep compatibility for old version.
     */
    if rsnd_ssi_use_busif(io) != 0 {
        name = if is_play != 0 { b"rxu\0".as_ptr() } else { b"txu\0".as_ptr() } as *const c_char;
    } else {
        name = if is_play != 0 { b"rx\0".as_ptr() } else { b"tx\0".as_ptr() } as *const c_char;
    }

    rsnd_dma_request_channel(rsnd_ssi_of_node(priv_), SSI_NAME, mod_, name)
}

/* CONFIG_DEBUG_FS */
unsafe extern "C" fn rsnd_ssi_debug_info(m: *mut seq_file, io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let rdai = rsnd_io_to_rdai(io);
    let ssi = rsnd_mod_to_ssi(mod_);

    seq_printf(m, b"clock:           %s\n\0".as_ptr() as *const c_char, if rsnd_rdai_is_clk_master(rdai) != 0 { b"provider\0".as_ptr() } else { b"consumer\0".as_ptr() });
    seq_printf(m, b"bit_clk_inv:     %d\n\0".as_ptr() as *const c_char, (*rdai).bit_clk_inv);
    seq_printf(m, b"frm_clk_inv:     %d\n\0".as_ptr() as *const c_char, (*rdai).frm_clk_inv);
    seq_printf(m, b"pin share:       %d\n\0".as_ptr() as *const c_char, __rsnd_ssi_is_pin_sharing(mod_));
    seq_printf(m, b"can out clk:     %d\n\0".as_ptr() as *const c_char, rsnd_ssi_can_output_clk(mod_) as c_int);
    seq_printf(m, b"multi secondary: %d\n\0".as_ptr() as *const c_char, rsnd_ssi_is_multi_secondary(mod_, io) as c_int);
    seq_printf(m, b"tdm:             %d, %d\n\0".as_ptr() as *const c_char, rsnd_runtime_is_tdm(io), rsnd_runtime_is_tdm_split(io));
    seq_printf(m, b"chan:            %d\n\0".as_ptr() as *const c_char, (*ssi).chan);
    seq_printf(m, b"user:            %d\n\0".as_ptr() as *const c_char, (*ssi).usrcnt);

    rsnd_debugfs_mod_reg_show(m, mod_, RSND_BASE_SSI, rsnd_mod_id(mod_) * 0x40, 0x40);
}

static mut RSND_SSI_DMA_OPS: rsnd_mod_ops = rsnd_mod_ops {
    name: SSI_NAME,
    dma_req: Some(rsnd_ssi_dma_req),
    probe: Some(rsnd_ssi_dma_probe),
    remove: Some(rsnd_ssi_common_remove),
    init: Some(rsnd_ssi_init),
    quit: Some(rsnd_ssi_quit),
    start: Some(rsnd_ssi_start),
    stop: Some(rsnd_ssi_stop),
    irq: Some(rsnd_ssi_irq),
    pointer: None,
    pcm_new: Some(rsnd_ssi_pcm_new),
    fallback: Some(rsnd_ssi_fallback),
    hw_params: Some(rsnd_ssi_hw_params),
    get_status: Some(rsnd_ssi_get_status),
};

pub unsafe extern "C" fn rsnd_ssi_is_dma_mode(mod_: *mut rsnd_mod) -> c_int {
    ((*mod_).ops == &mut RSND_SSI_DMA_OPS) as c_int
}

unsafe extern "C" fn rsnd_ssi_connect(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) {
    let rdai = rsnd_io_to_rdai(io);
    static TYPES: [rsnd_mod_type; 4] = [
        rsnd_mod_type::RSND_MOD_SSI,
        rsnd_mod_type::RSND_MOD_SSIM1,
        rsnd_mod_type::RSND_MOD_SSIM2,
        rsnd_mod_type::RSND_MOD_SSIM3,
    ];

    /* try SSI -> SSIM1 -> SSIM2 -> SSIM3 */
    for i in 0..TYPES.len() {
        let type_ = TYPES[i];
        if rsnd_io_to_mod(io, type_).is_null() {
            rsnd_dai_connect(mod_, io, type_);
            rsnd_rdai_channels_set(rdai, ((i + 1) * 2) as c_int);
            rsnd_rdai_ssi_lane_set(rdai, (i + 1) as c_int);
            return;
        }
    }
}

pub unsafe extern "C" fn rsnd_parse_connect_ssi(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node) {
    let priv_ = rsnd_rdai_to_priv(rdai);
    let dev = rsnd_priv_to_dev(priv_);
    let node = rsnd_ssi_of_node(priv_);
    let mut i: c_int;

    if node.is_null() {
        return;
    }

    /*
     * for_each_child_of_node_scoped(node, np) is a kernel iterator supplied by
     * the surrounding tree; this isolated translation preserves the loop intent.
     */
    i = 0;
    let np: *mut device_node = core::ptr::null_mut();
    while !np.is_null() {
        let mut mod_: *mut rsnd_mod;

        i = rsnd_node_fixed_index(dev, np, SSI_NAME, i);
        if i < 0 {
            break;
        }

        mod_ = rsnd_ssi_mod_get(priv_, i);

        if np == playback {
            rsnd_ssi_connect(mod_, &mut (*rdai).playback);
        }
        if np == capture {
            rsnd_ssi_connect(mod_, &mut (*rdai).capture);
        }
        i += 1;
    }

    of_node_put(node);
}

pub unsafe extern "C" fn rsnd_ssi_mod_get(priv_: *mut rsnd_priv, mut id: c_int) -> *mut rsnd_mod {
    if WARN_ON((id < 0 || id >= rsnd_ssi_nr(priv_)) as c_int) != 0 {
        id = 0;
    }

    rsnd_mod_get(rsnd_ssi_get(priv_, id))
}

pub unsafe extern "C" fn __rsnd_ssi_is_pin_sharing(mod_: *mut rsnd_mod) -> c_int {
    if mod_.is_null() {
        return 0;
    }

    rsnd_flags_has(rsnd_mod_to_ssi(mod_), RSND_SSI_CLK_PIN_SHARE) as c_int
}

pub unsafe extern "C" fn rsnd_ssi_probe(priv_: *mut rsnd_priv) -> c_int {
    let mut rstc: *mut reset_control;
    let node = rsnd_ssi_of_node(priv_);
    let dev = rsnd_priv_to_dev(priv_);
    let mut ops: *mut rsnd_mod_ops;
    let mut clk: *mut clk;
    let mut ssi: *mut rsnd_ssi;
    let mut ret: c_int;

    if node.is_null() {
        return -EINVAL;
    }

    let nr = rsnd_node_count(priv_, node, SSI_NAME);
    if nr == 0 {
        ret = -EINVAL;
        of_node_put(node);
        return ret;
    }

    ssi = devm_kcalloc(dev, nr, core::mem::size_of::<rsnd_ssi>(), GFP_KERNEL) as *mut rsnd_ssi;
    if ssi.is_null() {
        ret = -ENOMEM;
        of_node_put(node);
        return ret;
    }

    (*priv_).ssi = ssi;
    (*priv_).ssi_nr = nr;

    /*
     * for_each_child_of_node_scoped(node, np) is supplied externally.
     * The body below is the direct translation of one iteration.
     */
    let mut i = 0;
    let np: *mut device_node = core::ptr::null_mut();
    while !np.is_null() {
        if of_device_is_available(np) == 0 {
            i += 1;
            continue;
        }

        i = rsnd_node_fixed_index(dev, np, SSI_NAME, i);
        if i < 0 {
            ret = -EINVAL;
            of_node_put(node);
            return ret;
        }

        ssi = rsnd_ssi_get(priv_, i);

        clk = rsnd_devm_clk_get_indexed(dev, SSI_NAME, i);
        if IS_ERR(clk as *const c_void) != 0 {
            ret = PTR_ERR(clk as *const c_void);
            of_node_put(node);
            return ret;
        }

        /*
         * RZ/G3E uses per-SSI reset controllers.
         * R-Car platforms typically don't have SSI reset controls.
         */
        rstc = rsnd_devm_reset_control_get_optional_indexed(dev, SSI_NAME, i);
        if IS_ERR(rstc as *const c_void) != 0 {
            ret = PTR_ERR(rstc as *const c_void);
            of_node_put(node);
            return ret;
        }

        if of_property_read_bool(np, b"shared-pin\0".as_ptr() as *const c_char) != 0 {
            rsnd_flags_set(ssi, RSND_SSI_CLK_PIN_SHARE);
        }

        if of_property_read_bool(np, b"no-busif\0".as_ptr() as *const c_char) != 0 {
            rsnd_flags_set(ssi, RSND_SSI_NO_BUSIF);
        }

        (*ssi).irq = irq_of_parse_and_map(np, 0);
        if (*ssi).irq == 0 {
            ret = -EINVAL;
            of_node_put(node);
            return ret;
        }

        ops = if of_property_read_bool(np, b"pio-transfer\0".as_ptr() as *const c_char) != 0 {
            &mut RSND_SSI_PIO_OPS
        } else {
            &mut RSND_SSI_DMA_OPS
        };

        ret = rsnd_mod_init(priv_, rsnd_mod_get(ssi), ops, clk, rstc, rsnd_mod_type::RSND_MOD_SSI, i);
        if ret != 0 {
            of_node_put(node);
            return ret;
        }

        i += 1;
    }

    ret = 0;
    of_node_put(node);

    ret
}

pub unsafe extern "C" fn rsnd_ssi_remove(priv_: *mut rsnd_priv) {
    let mut i = 0;

    while i < rsnd_ssi_nr(priv_) {
        let ssi = rsnd_ssi_get(priv_, i);
        rsnd_mod_quit(rsnd_mod_get(ssi));
        i += 1;
    }
}

pub unsafe extern "C" fn rsnd_ssi_suspend(priv_: *mut rsnd_priv) {
    let mut i = 0;

    while i < rsnd_ssi_nr(priv_) {
        let ssi = rsnd_ssi_get(priv_, i);
        let mod_ = rsnd_mod_get(ssi);
        rsnd_suspend_clk_reset((*mod_).clk, (*mod_).rstc);
        i += 1;
    }
}

pub unsafe extern "C" fn rsnd_ssi_resume(priv_: *mut rsnd_priv) {
    let mut i = 0;

    while i < rsnd_ssi_nr(priv_) {
        let ssi = rsnd_ssi_get(priv_, i);
        let mod_ = rsnd_mod_get(ssi);
        rsnd_resume_clk_reset((*mod_).clk, (*mod_).rstc);
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
