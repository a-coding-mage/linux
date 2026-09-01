// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car SRC support
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

/*
 * You can use Synchronous Sampling Rate Convert (if no DVC)
 *
 *	amixer set "SRC Out Rate" on
 *	aplay xxx.wav &
 *	amixer set "SRC Out Rate" 96000 // convert rate to 96000Hz
 *	amixer set "SRC Out Rate" 22050 // convert rate to 22050Hz
 */

/*
 * you can enable below define if you don't need
 * SSI interrupt status debug message when debugging
 * see rsnd_print_irq_status()
 *
 * #define RSND_DEBUG_NO_IRQ_STATUS 1
 */

// C dependencies: <linux/of_irq.h>, "rsnd.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type uint = c_uint;
type bool_t = bool;
type irqreturn_t = c_uint;

const SRC_NAME: &[u8] = b"src\0";

/* SCU_SYSTEM_STATUS0/1 */
const fn OUF_SRC(id: c_int) -> u32 {
    ((1u32 << ((id + 16) as u32)) | (1u32 << (id as u32))) as u32
}

#[repr(C)]
pub struct rsnd_mod {
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
}

#[repr(C)]
pub struct rsnd_dai_stream {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct rsnd_priv {
    pub src: *mut c_void,
    pub src_nr: c_int,
    pub src_ctrl: *mut c_void,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct rsnd_kctrl_cfg_s {
    pub val: u32,
}

#[repr(C)]
pub struct rsnd_src {
    pub mod_: rsnd_mod,
    pub dma: *mut rsnd_mod,
    pub sen: rsnd_kctrl_cfg_s,  /* sync convert enable */
    pub sync: rsnd_kctrl_cfg_s, /* sync convert */
    pub current_sync_rate: u32,
    pub irq: c_int,
}

#[repr(C)]
pub struct rsnd_src_ctrl {
    pub scu: *mut clk,
    pub scu_x2: *mut clk,
    pub scu_supply: *mut clk,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
}

#[repr(C)]
pub struct rsnd_mod_ops {
    pub name: *const c_char,
    pub dma_req: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod) -> *mut dma_chan>,
    pub probe: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub quit: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub start: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub irq: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv, c_int) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_soc_pcm_runtime) -> c_int>,
    pub get_status: Option<unsafe extern "C" fn()>,
    /* CONFIG_DEBUG_FS: .debug_info = rsnd_src_debug_info */
}

#[repr(C)] pub struct dma_chan { _private: [u8; 0] }
#[repr(C)] pub struct rsnd_mod_status { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct reset_control { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

const SRC_SWRSR: c_int = 0;
const SRC_SRCIR: c_int = 0;
const SRC_IFSVR: c_int = 0;
const SRC_ADINR: c_int = 0;
const SRC_IFSCR: c_int = 0;
const SRC_SRCCR: c_int = 0;
const SRC_BSDSR: c_int = 0;
const SRC_BSISR: c_int = 0;
const SRC_I_BUSIF_MODE: c_int = 0;
const SRC_O_BUSIF_MODE: c_int = 0;
const SRC_BUSIF_DALIGN: c_int = 0;
const SRC_ROUTE_MODE0: c_int = 0;
const SRC_INT_ENABLE0: c_int = 0;
const SRC_CTRL: c_int = 0;
const SCU_SYS_INT_EN0: c_int = 0;
const SCU_SYS_INT_EN1: c_int = 0;
const SCU_SYS_STATUS0: c_int = 0;
const SCU_SYS_STATUS1: c_int = 0;
const RSND_BASE_SCU: c_int = 0;
const IRQF_SHARED: c_ulong = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const GFP_KERNEL: c_uint = 0;
const RSND_MOD_SRC: c_int = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

type c_ulong = core::ffi::c_ulong;

unsafe extern "C" {
    fn rsnd_mod_write(mod_: *mut rsnd_mod, reg: c_int, data: u32);
    fn rsnd_mod_read(mod_: *mut rsnd_mod, reg: c_int) -> u32;
    fn rsnd_mod_bset(mod_: *mut rsnd_mod, reg: c_int, mask: u32, data: u32);
    fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv;
    fn rsnd_mod_id(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_name(mod_: *mut rsnd_mod) -> *const c_char;
    fn rsnd_mod_get(src: *mut rsnd_src) -> *mut rsnd_mod;
    fn rsnd_mod_power_on(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_power_off(mod_: *mut rsnd_mod);
    fn rsnd_mod_interrupt(mod_: *mut rsnd_mod, handler: unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream));
    fn rsnd_mod_get_status();
    fn rsnd_mod_init(priv_: *mut rsnd_priv, mod_: *mut rsnd_mod, ops: *mut rsnd_mod_ops, clk: *mut clk, rstc: *mut reset_control, type_: c_int, id: c_int) -> c_int;
    fn rsnd_mod_quit(mod_: *mut rsnd_mod);

    fn rsnd_io_is_play(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_io_to_runtime(io: *mut rsnd_dai_stream) -> *mut snd_pcm_runtime;
    fn rsnd_io_converted_rate(io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_io_to_mod_src(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_dvc(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_cmd(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_is_working(io: *mut rsnd_dai_stream) -> bool_t;
    fn rsnd_io_to_priv(io: *mut rsnd_dai_stream) -> *mut rsnd_priv;

    fn rsnd_dma_request_channel(np: *mut device_node, name: *const c_char, mod_: *mut rsnd_mod, dir: *const c_char) -> *mut dma_chan;
    fn rsnd_dma_attach(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod, dma: *mut *mut rsnd_mod) -> c_int;
    fn rsnd_src_of_node(priv_: *mut rsnd_priv) -> *mut device_node;
    fn rsnd_src_get_in_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_src_get_out_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_runtime_channel_original(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_get_adinr_bit(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_get_busif_shift(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> u32;
    fn rsnd_get_dalign(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_adg_set_src_timesel_gen2(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, fin: u32, fout: u32);
    fn rsnd_is_gen3_e3(priv_: *mut rsnd_priv) -> bool_t;

    fn rsnd_priv_to_dev(priv_: *mut rsnd_priv) -> *mut device;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get_optional_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_reset_control_get_optional_shared(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn rsnd_devm_clk_get_indexed(dev: *mut device, name: *const u8, idx: c_int) -> *mut clk;

    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn WARN_ON(condition: bool_t) -> bool_t;

    fn rsnd_node_count(priv_: *mut rsnd_priv, node: *mut device_node, name: *const u8) -> c_int;
    fn rsnd_node_fixed_index(dev: *mut device, np: *mut device_node, name: *const u8, i: c_int) -> c_int;
    fn of_device_is_available(np: *mut device_node) -> bool_t;
    fn irq_of_parse_and_map(dev: *mut device_node, index: c_int) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn first_child_of_node(node: *mut device_node) -> *mut device_node;
    fn next_child_of_node(node: *mut device_node, prev: *mut device_node) -> *mut device_node;

    fn rsnd_print_irq_status(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);

    fn rsnd_kctrl_accept_anytime(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_kctrl_new_s(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, rtd: *mut snd_soc_pcm_runtime, name: *const c_char, accept: unsafe extern "C" fn(*mut rsnd_dai_stream) -> c_int, update: unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod), cfg: *mut rsnd_kctrl_cfg_s, max: u32) -> c_int;

    fn rsnd_debugfs_mod_reg_show(m: *mut seq_file, mod_: *mut rsnd_mod, base: c_int, offset: c_int, size: c_int);
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn rsnd_suspend_clk_reset(clk: *mut clk, rstc: *mut reset_control);
    fn rsnd_resume_clk_reset(clk: *mut clk, rstc: *mut reset_control);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
}

type c_long = core::ffi::c_long;

unsafe fn rsnd_src_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_src {
    ((*priv_).src as *mut rsnd_src).add(id as usize)
}

unsafe fn rsnd_src_nr(priv_: *mut rsnd_priv) -> c_int {
    (*priv_).src_nr
}

unsafe fn rsnd_mod_to_src(mod_: *mut rsnd_mod) -> *mut rsnd_src {
    mod_ as *mut rsnd_src
}

unsafe fn rsnd_src_sync_is_enabled(mod_: *mut rsnd_mod) -> u32 {
    (*rsnd_mod_to_src(mod_)).sen.val
}

unsafe fn rsnd_priv_to_src_ctrl(priv_: *mut rsnd_priv) -> *mut rsnd_src_ctrl {
    (*priv_).src_ctrl as *mut rsnd_src_ctrl
}

unsafe extern "C" fn rsnd_src_activation(mod_: *mut rsnd_mod) {
    rsnd_mod_write(mod_, SRC_SWRSR, 0);
    rsnd_mod_write(mod_, SRC_SWRSR, 1);
}

unsafe extern "C" fn rsnd_src_halt(mod_: *mut rsnd_mod) {
    rsnd_mod_write(mod_, SRC_SRCIR, 1);
    rsnd_mod_write(mod_, SRC_SWRSR, 0);
}

unsafe extern "C" fn rsnd_src_dma_req(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> *mut dma_chan {
    let priv_ = rsnd_mod_to_priv(mod_);
    let is_play = rsnd_io_is_play(io);

    rsnd_dma_request_channel(
        rsnd_src_of_node(priv_),
        SRC_NAME.as_ptr() as *const c_char,
        mod_,
        if is_play != 0 { b"rx\0".as_ptr() } else { b"tx\0".as_ptr() } as *const c_char,
    )
}

unsafe extern "C" fn rsnd_src_convert_rate(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> u32 {
    let runtime = rsnd_io_to_runtime(io);
    let src = rsnd_mod_to_src(mod_);
    let mut convert_rate: u32;

    if runtime.is_null() {
        return 0;
    }

    if rsnd_src_sync_is_enabled(mod_) == 0 {
        return rsnd_io_converted_rate(io);
    }

    convert_rate = (*src).current_sync_rate;

    if convert_rate == 0 {
        convert_rate = rsnd_io_converted_rate(io);
    }

    if convert_rate == 0 {
        convert_rate = (*runtime).rate;
    }

    convert_rate
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_src_get_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream, is_in: c_int) -> c_uint {
    let src_mod = rsnd_io_to_mod_src(io);
    let runtime = rsnd_io_to_runtime(io);
    let mut rate: c_uint = 0;
    let is_play = rsnd_io_is_play(io);

    /*
     * Playback
     * runtime_rate -> [SRC] -> convert_rate
     *
     * Capture
     * convert_rate -> [SRC] -> runtime_rate
     */

    if is_play == is_in {
        return (*runtime).rate;
    }

    /*
     * return convert rate if SRC is used,
     * otherwise, return runtime->rate as usual
     */
    if !src_mod.is_null() {
        rate = rsnd_src_convert_rate(io, src_mod);
    }

    if rate == 0 {
        rate = (*runtime).rate;
    }

    rate
}

static bsdsr_table_pattern1: [u32; 6] = [
    0x01800000, /* 6 - 1/6 */
    0x01000000, /* 6 - 1/4 */
    0x00c00000, /* 6 - 1/3 */
    0x00800000, /* 6 - 1/2 */
    0x00600000, /* 6 - 2/3 */
    0x00400000, /* 6 - 1   */
];

static bsdsr_table_pattern2: [u32; 6] = [
    0x02400000, /* 6 - 1/6 */
    0x01800000, /* 6 - 1/4 */
    0x01200000, /* 6 - 1/3 */
    0x00c00000, /* 6 - 1/2 */
    0x00900000, /* 6 - 2/3 */
    0x00600000, /* 6 - 1   */
];

static bsisr_table: [u32; 6] = [
    0x00100060, /* 6 - 1/6 */
    0x00100040, /* 6 - 1/4 */
    0x00100030, /* 6 - 1/3 */
    0x00100020, /* 6 - 1/2 */
    0x00100020, /* 6 - 2/3 */
    0x00100020, /* 6 - 1   */
];

static chan288888: [u32; 6] = [
    0x00000006, /* 1 to 2 */
    0x000001fe, /* 1 to 8 */
    0x000001fe, /* 1 to 8 */
    0x000001fe, /* 1 to 8 */
    0x000001fe, /* 1 to 8 */
    0x000001fe, /* 1 to 8 */
];

static chan244888: [u32; 6] = [
    0x00000006, /* 1 to 2 */
    0x0000001e, /* 1 to 4 */
    0x0000001e, /* 1 to 4 */
    0x000001fe, /* 1 to 8 */
    0x000001fe, /* 1 to 8 */
    0x000001fe, /* 1 to 8 */
];

static chan222222: [u32; 6] = [
    0x00000006, /* 1 to 2 */
    0x00000006, /* 1 to 2 */
    0x00000006, /* 1 to 2 */
    0x00000006, /* 1 to 2 */
    0x00000006, /* 1 to 2 */
    0x00000006, /* 1 to 2 */
];

unsafe extern "C" fn rsnd_src_set_convert_rate(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let runtime = rsnd_io_to_runtime(io);
    let priv_ = rsnd_mod_to_priv(mod_);
    let src = rsnd_mod_to_src(mod_);
    let fin: u32;
    let fout: u32;
    let new_rate: u32;
    let mut inc: c_int;
    let cnt: c_int;
    let mut rate: c_int;
    let base: u64;
    let mut val: u64;

    if runtime.is_null() {
        return;
    }

    if rsnd_src_sync_is_enabled(mod_) == 0 {
        return;
    }

    fin = rsnd_src_get_in_rate(priv_, io);
    fout = rsnd_src_get_out_rate(priv_, io);

    new_rate = if (*src).sync.val != 0 { (*src).sync.val } else { fout };

    /* Do nothing if no diff */
    if new_rate == (*src).current_sync_rate {
        return;
    }

    /*
     * SRCm_IFSVR::INTIFS can change within 1%
     * see
     *	SRCm_IFSVR::INTIFS Note
     */
    inc = (fout / 100) as c_int;
    cnt = ((new_rate as c_int - fout as c_int).abs()) / inc;
    if fout > new_rate {
        inc *= -1;
    }

    /*
     * After start running SRC, we can update only SRC_IFSVR
     * for Synchronous Mode
     */
    base = 0x0400000u64 * fin as u64;
    rate = fout as c_int;
    for _i in 0..cnt {
        val = base;
        rate += inc;
        val /= rate as u64;

        rsnd_mod_write(mod_, SRC_IFSVR, val as u32);
    }
    val = base;
    val /= new_rate as u64;

    rsnd_mod_write(mod_, SRC_IFSVR, val as u32);

    /* update current_sync_rate */
    (*src).current_sync_rate = new_rate;
}

unsafe extern "C" fn rsnd_src_init_convert_rate(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let runtime = rsnd_io_to_runtime(io);
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let is_play = rsnd_io_is_play(io);
    let mut use_src: c_int = 0;
    let fin: u32;
    let fout: u32;
    let mut ifscr: u32;
    let adinr: u32;
    let mut cr: u32;
    let mut route: u32;
    let i_busif: u32;
    let o_busif: u32;
    let tmp: u32;
    let mut bsdsr_table: *const u32;
    let mut chptn: *const u32;
    let ratio: uint;
    let chan: c_int;
    let mut idx: usize;

    if runtime.is_null() {
        return;
    }

    fin = rsnd_src_get_in_rate(priv_, io);
    fout = rsnd_src_get_out_rate(priv_, io);

    chan = rsnd_runtime_channel_original(io);

    /* 6 - 1/6 are very enough ratio for SRC_BSDSR */
    ratio = if fin == fout {
        0
    } else if fin > fout {
        100 * fin / fout
    } else {
        100 * fout / fin
    };

    if ratio > 600 {
        dev_err(dev, b"FSO/FSI ratio error\n\0".as_ptr() as *const c_char);
        return;
    }

    use_src = ((fin != fout) as c_int) | ((rsnd_src_sync_is_enabled(mod_) != 0) as c_int);

    /*
     * SRC_ADINR
     */
    adinr = rsnd_get_adinr_bit(mod_, io) | chan as u32;

    /*
     * SRC_IFSCR
     * SRC_SRCCR / SRC_ROUTE_MODE0
     */
    ifscr = 0;
    cr = 0x00011110;
    route = 0x0;
    if use_src != 0 {
        route = 0x1;
        ifscr = 0x1;

        if rsnd_src_sync_is_enabled(mod_) != 0 {
            cr |= 0x1;
            route |= if rsnd_io_is_play(io) != 0 { 0x1 << 24 } else { 0x1 << 25 };
        }
    }

    /*
     * SRC_BSDSR / SRC_BSISR
     *
     * see
     *	Combination of Register Setting Related to
     *	FSO/FSI Ratio and Channel, Latency
     */
    match rsnd_mod_id(mod_) {
        0 => {
            chptn = chan288888.as_ptr();
            bsdsr_table = bsdsr_table_pattern1.as_ptr();
        }
        1 | 3 | 4 => {
            chptn = chan244888.as_ptr();
            bsdsr_table = bsdsr_table_pattern1.as_ptr();
        }
        2 | 9 => {
            chptn = chan222222.as_ptr();
            bsdsr_table = bsdsr_table_pattern1.as_ptr();
        }
        5 | 6 | 7 | 8 => {
            chptn = chan222222.as_ptr();
            bsdsr_table = bsdsr_table_pattern2.as_ptr();
        }
        _ => {
            dev_err(dev, b"unknown BSDSR/BSDIR settings\n\0".as_ptr() as *const c_char);
            return;
        }
    }

    /*
     * E3 need to overwrite
     */
    if rsnd_is_gen3_e3(priv_) {
        match rsnd_mod_id(mod_) {
            0 | 4 => {
                chptn = chan222222.as_ptr();
            }
            _ => {}
        }
    }

    idx = 0;
    while idx < chan222222.len() {
        if *chptn.add(idx) & (1u32 << chan as u32) != 0 {
            break;
        }
        idx += 1;
    }

    if chan > 8 || idx >= chan222222.len() {
        dev_err(dev, b"unknown BSDSR/BSDIR settings\n\0".as_ptr() as *const c_char);
        return;
    }

    /* BUSIF_MODE */
    tmp = rsnd_get_busif_shift(io, mod_);
    i_busif = (if is_play != 0 { tmp } else { 0 }) | 1;
    o_busif = (if is_play == 0 { tmp } else { 0 }) | 1;

    rsnd_mod_write(mod_, SRC_ROUTE_MODE0, route);

    rsnd_mod_write(mod_, SRC_SRCIR, 1);	/* initialize */
    rsnd_mod_write(mod_, SRC_ADINR, adinr);
    rsnd_mod_write(mod_, SRC_IFSCR, ifscr);
    rsnd_mod_write(mod_, SRC_SRCCR, cr);
    rsnd_mod_write(mod_, SRC_BSDSR, *bsdsr_table.add(idx));
    rsnd_mod_write(mod_, SRC_BSISR, bsisr_table[idx]);
    rsnd_mod_write(mod_, SRC_SRCIR, 0);	/* cancel initialize */

    rsnd_mod_write(mod_, SRC_I_BUSIF_MODE, i_busif);
    rsnd_mod_write(mod_, SRC_O_BUSIF_MODE, o_busif);

    rsnd_mod_write(mod_, SRC_BUSIF_DALIGN, rsnd_get_dalign(mod_, io));

    rsnd_adg_set_src_timesel_gen2(mod_, io, fin, fout);

    /* update SRC_IFSVR */
    rsnd_src_set_convert_rate(io, mod_);
}

unsafe extern "C" fn rsnd_src_irq(mod_: *mut rsnd_mod, _io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv, enable: c_int) -> c_int {
    let src = rsnd_mod_to_src(mod_);
    let mut sys_int_val: u32;
    let int_val: u32;
    let sys_int_mask: u32;
    let irq = (*src).irq;
    let id = rsnd_mod_id(mod_);

    sys_int_val = OUF_SRC(id);
    sys_int_mask = sys_int_val;
    let mut local_int_val = 0x3300;

    /*
     * IRQ is not supported on non-DT
     * see
     *	rsnd_src_probe_()
     */
    if irq <= 0 || enable == 0 {
        sys_int_val = 0;
        local_int_val = 0;
    }

    /*
     * WORKAROUND
     *
     * ignore over flow error when rsnd_src_sync_is_enabled()
     */
    if rsnd_src_sync_is_enabled(mod_) != 0 {
        sys_int_val = sys_int_val & 0xffff;
    }

    rsnd_mod_write(mod_, SRC_INT_ENABLE0, local_int_val);
    rsnd_mod_bset(mod_, SCU_SYS_INT_EN0, sys_int_mask, sys_int_val);
    rsnd_mod_bset(mod_, SCU_SYS_INT_EN1, sys_int_mask, sys_int_val);

    0
}

unsafe extern "C" fn rsnd_src_status_clear(mod_: *mut rsnd_mod) {
    let val = OUF_SRC(rsnd_mod_id(mod_));

    rsnd_mod_write(mod_, SCU_SYS_STATUS0, val);
    rsnd_mod_write(mod_, SCU_SYS_STATUS1, val);
}

unsafe extern "C" fn rsnd_src_error_occurred(mod_: *mut rsnd_mod) -> bool_t {
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let mut val0: u32;
    let val1: u32;
    let status0: u32;
    let status1: u32;
    let mut ret = false;

    val0 = OUF_SRC(rsnd_mod_id(mod_));
    val1 = val0;

    /*
     * WORKAROUND
     *
     * ignore over flow error when rsnd_src_sync_is_enabled()
     */
    if rsnd_src_sync_is_enabled(mod_) != 0 {
        val0 = val0 & 0xffff;
    }

    status0 = rsnd_mod_read(mod_, SCU_SYS_STATUS0);
    status1 = rsnd_mod_read(mod_, SCU_SYS_STATUS1);
    if (status0 & val0) != 0 || (status1 & val1) != 0 {
        rsnd_print_irq_status(
            dev,
            b"%s err status : 0x%08x, 0x%08x\n\0".as_ptr() as *const c_char,
            rsnd_mod_name(mod_),
            status0,
            status1,
        );

        ret = true;
    }

    ret
}

unsafe extern "C" fn rsnd_src_start(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv) -> c_int {
    let val: u32;

    /*
     * WORKAROUND
     *
     * Enable SRC output if you want to use sync convert together with DVC
     */
    val = if !rsnd_io_to_mod_dvc(io).is_null() && rsnd_src_sync_is_enabled(mod_) == 0 {
        0x01
    } else {
        0x11
    };

    rsnd_mod_write(mod_, SRC_CTRL, val);

    0
}

unsafe extern "C" fn rsnd_src_stop(mod_: *mut rsnd_mod, _io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv) -> c_int {
    rsnd_mod_write(mod_, SRC_CTRL, 0);

    0
}

unsafe extern "C" fn rsnd_src_init(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv) -> c_int {
    let src = rsnd_mod_to_src(mod_);
    let ret: c_int;

    /* reset sync convert_rate */
    (*src).current_sync_rate = 0;
    (*src).sync.val = (*src).current_sync_rate;

    ret = rsnd_mod_power_on(mod_);
    if ret < 0 {
        return ret;
    }

    rsnd_src_activation(mod_);

    rsnd_src_init_convert_rate(io, mod_);

    rsnd_src_status_clear(mod_);

    0
}

unsafe extern "C" fn rsnd_src_quit(mod_: *mut rsnd_mod, _io: *mut rsnd_dai_stream, _priv: *mut rsnd_priv) -> c_int {
    let src = rsnd_mod_to_src(mod_);

    rsnd_src_halt(mod_);

    rsnd_mod_power_off(mod_);

    /* reset sync convert_rate */
    (*src).current_sync_rate = 0;
    (*src).sync.val = (*src).current_sync_rate;

    0
}

unsafe extern "C" fn __rsnd_src_interrupt(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) {
    let priv_ = rsnd_mod_to_priv(mod_);
    let mut stop = false;

    spin_lock(&mut (*priv_).lock);
    loop {
        /* ignore all cases if not working */
        if !rsnd_io_is_working(io) {
            break;
        }

        if rsnd_src_error_occurred(mod_) {
            stop = true;
        }

        rsnd_src_status_clear(mod_);
        break;
    }
    spin_unlock(&mut (*priv_).lock);

    if stop {
        snd_pcm_stop_xrun((*io).substream);
    }
}

unsafe extern "C" fn rsnd_src_interrupt(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let mod_ = data as *mut rsnd_mod;

    rsnd_mod_interrupt(mod_, __rsnd_src_interrupt);

    IRQ_HANDLED
}

unsafe extern "C" fn rsnd_src_kctrl_accept_runtime(io: *mut rsnd_dai_stream) -> c_int {
    let runtime = rsnd_io_to_runtime(io);

    if runtime.is_null() {
        let priv_ = rsnd_io_to_priv(io);
        let dev = rsnd_priv_to_dev(priv_);

        dev_warn(dev, b"\"SRC Out Rate\" can use during running\n\0".as_ptr() as *const c_char);

        return 0;
    }

    1
}

unsafe extern "C" fn rsnd_src_probe_(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, priv_: *mut rsnd_priv) -> c_int {
    let src = rsnd_mod_to_src(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let irq = (*src).irq;
    let mut ret: c_int;

    if irq > 0 {
        /*
         * IRQ is not supported on non-DT
         * see
         *	rsnd_src_irq()
         */
        ret = devm_request_irq(
            dev,
            irq,
            rsnd_src_interrupt,
            IRQF_SHARED,
            dev_name(dev),
            mod_ as *mut c_void,
        );
        if ret != 0 {
            return ret;
        }
    }

    ret = rsnd_dma_attach(io, mod_, &mut (*src).dma);

    ret
}

unsafe extern "C" fn rsnd_src_pcm_new(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let src = rsnd_mod_to_src(mod_);
    let mut ret: c_int;

    /*
     * enable SRC sync convert if possible
     */

    /*
     * It can't use SRC Synchronous convert
     * when Capture if it uses CMD
     */
    if !rsnd_io_to_mod_cmd(io).is_null() && rsnd_io_is_play(io) == 0 {
        return 0;
    }

    /*
     * enable sync convert
     */
    ret = rsnd_kctrl_new_s(
        mod_,
        io,
        rtd,
        if rsnd_io_is_play(io) != 0 {
            b"SRC Out Rate Switch\0".as_ptr()
        } else {
            b"SRC In Rate Switch\0".as_ptr()
        } as *const c_char,
        rsnd_kctrl_accept_anytime,
        rsnd_src_init_convert_rate,
        &mut (*src).sen,
        1,
    );
    if ret < 0 {
        return ret;
    }

    ret = rsnd_kctrl_new_s(
        mod_,
        io,
        rtd,
        if rsnd_io_is_play(io) != 0 {
            b"SRC Out Rate\0".as_ptr()
        } else {
            b"SRC In Rate\0".as_ptr()
        } as *const c_char,
        rsnd_src_kctrl_accept_runtime,
        rsnd_src_set_convert_rate,
        &mut (*src).sync,
        192000,
    );

    ret
}

/* CONFIG_DEBUG_FS */
unsafe extern "C" fn rsnd_src_debug_info(m: *mut seq_file, io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let _ = io;
    rsnd_debugfs_mod_reg_show(m, mod_, RSND_BASE_SCU, rsnd_mod_id(mod_) * 0x20, 0x20);
    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    rsnd_debugfs_mod_reg_show(m, mod_, RSND_BASE_SCU, 0x1c0, 0x20);
    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    rsnd_debugfs_mod_reg_show(m, mod_, RSND_BASE_SCU, 0x200 + rsnd_mod_id(mod_) * 0x40, 0x40);
}

static mut rsnd_src_ops: rsnd_mod_ops = rsnd_mod_ops {
    name: SRC_NAME.as_ptr() as *const c_char,
    dma_req: Some(rsnd_src_dma_req),
    probe: Some(rsnd_src_probe_),
    init: Some(rsnd_src_init),
    quit: Some(rsnd_src_quit),
    start: Some(rsnd_src_start),
    stop: Some(rsnd_src_stop),
    irq: Some(rsnd_src_irq),
    pcm_new: Some(rsnd_src_pcm_new),
    get_status: Some(rsnd_mod_get_status),
    /* DEBUG_INFO */
};

#[no_mangle]
pub unsafe extern "C" fn rsnd_src_mod_get(priv_: *mut rsnd_priv, mut id: c_int) -> *mut rsnd_mod {
    if WARN_ON(id < 0 || id >= rsnd_src_nr(priv_)) {
        id = 0;
    }

    rsnd_mod_get(rsnd_src_get(priv_, id))
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_src_probe(priv_: *mut rsnd_priv) -> c_int {
    let node: *mut device_node;
    let dev = rsnd_priv_to_dev(priv_);
    let rstc: *mut reset_control;
    let src_ctrl: *mut rsnd_src_ctrl;
    let mut src: *mut rsnd_src;
    let clk: *mut clk;
    let mut i: c_int;
    let nr: c_int;
    let mut ret: c_int;

    node = rsnd_src_of_node(priv_);
    if node.is_null() {
        return 0; /* not used is not error */
    }

    nr = rsnd_node_count(priv_, node, SRC_NAME.as_ptr());
    if nr == 0 {
        ret = -EINVAL;
        of_node_put(node);
        return ret;
    }

    src_ctrl = devm_kzalloc(dev, size_of::<rsnd_src_ctrl>(), GFP_KERNEL) as *mut rsnd_src_ctrl;
    if src_ctrl.is_null() {
        ret = -ENOMEM;
        of_node_put(node);
        return ret;
    }

    src = devm_kcalloc(dev, nr as usize, size_of::<rsnd_src>(), GFP_KERNEL) as *mut rsnd_src;
    if src.is_null() {
        ret = -ENOMEM;
        of_node_put(node);
        return ret;
    }

    (*priv_).src_nr = nr;
    (*priv_).src = src as *mut c_void;
    (*priv_).src_ctrl = src_ctrl as *mut c_void;

    (*src_ctrl).scu = devm_clk_get_optional_enabled(dev, b"scu\0".as_ptr() as *const c_char);
    if IS_ERR((*src_ctrl).scu as *const c_void) {
        ret = dev_err_probe(dev, PTR_ERR((*src_ctrl).scu as *const c_void), b"failed to get scu clock\n\0".as_ptr() as *const c_char);
        of_node_put(node);
        return ret;
    }

    (*src_ctrl).scu_x2 = devm_clk_get_optional_enabled(dev, b"scu_x2\0".as_ptr() as *const c_char);
    if IS_ERR((*src_ctrl).scu_x2 as *const c_void) {
        ret = dev_err_probe(dev, PTR_ERR((*src_ctrl).scu_x2 as *const c_void), b"failed to get scu_x2 clock\n\0".as_ptr() as *const c_char);
        of_node_put(node);
        return ret;
    }

    (*src_ctrl).scu_supply = devm_clk_get_optional_enabled(dev, b"scu_supply\0".as_ptr() as *const c_char);
    if IS_ERR((*src_ctrl).scu_supply as *const c_void) {
        ret = dev_err_probe(dev, PTR_ERR((*src_ctrl).scu_supply as *const c_void), b"failed to get scu_supply clock\n\0".as_ptr() as *const c_char);
        of_node_put(node);
        return ret;
    }

    /*
     * Shared SCU reset for every SRC module; acquire once.
     * R-Car platforms typically don't have SRC reset controls.
     */
    rstc = devm_reset_control_get_optional_shared(dev, b"scu\0".as_ptr() as *const c_char);
    if IS_ERR(rstc as *const c_void) {
        ret = PTR_ERR(rstc as *const c_void) as c_int;
        of_node_put(node);
        return ret;
    }

    i = 0;
    let mut np = first_child_of_node(node);
    while !np.is_null() {
        if !of_device_is_available(np) {
            i += 1;
            np = next_child_of_node(node, np);
            continue;
        }

        i = rsnd_node_fixed_index(dev, np, SRC_NAME.as_ptr(), i);
        if i < 0 {
            ret = -EINVAL;
            of_node_put(node);
            return ret;
        }

        src = rsnd_src_get(priv_, i);

        (*src).irq = irq_of_parse_and_map(np, 0);
        if (*src).irq == 0 {
            ret = -EINVAL;
            of_node_put(node);
            return ret;
        }

        clk = rsnd_devm_clk_get_indexed(dev, SRC_NAME.as_ptr(), i);
        if IS_ERR(clk as *const c_void) {
            ret = PTR_ERR(clk as *const c_void) as c_int;
            of_node_put(node);
            return ret;
        }

        ret = rsnd_mod_init(priv_, rsnd_mod_get(src), &mut rsnd_src_ops, clk, rstc, RSND_MOD_SRC, i);
        if ret != 0 {
            of_node_put(node);
            return ret;
        }

        i += 1;
        np = next_child_of_node(node, np);
    }

    ret = 0;

    of_node_put(node);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_src_remove(priv_: *mut rsnd_priv) {
    let mut i: c_int = 0;

    while i < rsnd_src_nr(priv_) {
        let src = rsnd_src_get(priv_, i);
        rsnd_mod_quit(rsnd_mod_get(src));
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_src_suspend(priv_: *mut rsnd_priv) {
    let src_ctrl = rsnd_priv_to_src_ctrl(priv_);
    let mut i: c_int = 0;

    if src_ctrl.is_null() {
        return;
    }

    while i < rsnd_src_nr(priv_) {
        let src = rsnd_src_get(priv_, i);
        let mod_ = rsnd_mod_get(src);
        rsnd_suspend_clk_reset((*mod_).clk, (*mod_).rstc);
        i += 1;
    }

    clk_disable_unprepare((*src_ctrl).scu_x2);
    clk_disable_unprepare((*src_ctrl).scu);
    clk_disable_unprepare((*src_ctrl).scu_supply);
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_src_resume(priv_: *mut rsnd_priv) {
    let src_ctrl = rsnd_priv_to_src_ctrl(priv_);
    let mut i: c_int = 0;

    if src_ctrl.is_null() {
        return;
    }

    clk_prepare_enable((*src_ctrl).scu_supply);
    clk_prepare_enable((*src_ctrl).scu);
    clk_prepare_enable((*src_ctrl).scu_x2);

    while i < rsnd_src_nr(priv_) {
        let src = rsnd_src_get(priv_, i);
        let mod_ = rsnd_mod_get(src);
        rsnd_resume_clk_reset((*mod_).clk, (*mod_).rstc);
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
