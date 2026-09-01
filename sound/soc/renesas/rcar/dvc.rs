// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car DVC support
//
// Copyright (C) 2014 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

/*
 * Playback Volume
 *	amixer set "DVC Out" 100%
 *
 * Capture Volume
 *	amixer set "DVC In" 100%
 *
 * Playback Mute
 *	amixer set "DVC Out Mute" on
 *
 * Capture Mute
 *	amixer set "DVC In Mute" on
 *
 * Volume Ramp
 *	amixer set "DVC Out Ramp Up Rate"   "0.125 dB/64 steps"
 *	amixer set "DVC Out Ramp Down Rate" "0.125 dB/512 steps"
 *	amixer set "DVC Out Ramp" on
 *	aplay xxx.wav &
 *	amixer set "DVC Out"  80%  // Volume Down
 *	amixer set "DVC Out" 100%  // Volume Up
 */

// C dependency: "rsnd.h"

pub type u32 = u32;

pub const DVC_NAME: *const ::std::os::raw::c_char = b"dvc\0".as_ptr() as *const ::std::os::raw::c_char;

#[repr(C)]
pub struct rsnd_mod {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_kctrl_cfg_m {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_kctrl_cfg_s {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_dai_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_priv {
    pub dvc: *mut ::std::os::raw::c_void,
    pub dvc_nr: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_dvc {
    pub mod_: rsnd_mod,
    pub volume: rsnd_kctrl_cfg_m,
    pub mute: rsnd_kctrl_cfg_m,
    pub ren: rsnd_kctrl_cfg_s,   /* Ramp Enable */
    pub rup: rsnd_kctrl_cfg_s,   /* Ramp Rate Up */
    pub rdown: rsnd_kctrl_cfg_s, /* Ramp Rate Down */
}

#[repr(C)]
pub struct rsnd_mod_ops {
    pub name: *const ::std::os::raw::c_char,
    pub dma_req: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod) -> *mut dma_chan>,
    pub probe: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> ::std::os::raw::c_int>,
    pub init: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> ::std::os::raw::c_int>,
    pub quit: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> ::std::os::raw::c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_soc_pcm_runtime) -> ::std::os::raw::c_int>,
    pub get_status: Option<unsafe extern "C" fn()>,
    // CONFIG_DEBUG_FS: .debug_info = rsnd_dvc_debug_info
}

unsafe extern "C" {
    static mut RSND_MAX_CHANNELS: ::std::os::raw::c_int;
    static mut VOLUME_RAMP_MAX_DVC: ::std::os::raw::c_uint;
    static mut volume_ramp_rate: *const ::std::os::raw::c_char;
    static mut GFP_KERNEL: ::std::os::raw::c_uint;
    static mut EINVAL: ::std::os::raw::c_int;
    static mut ENOMEM: ::std::os::raw::c_int;
    static mut RSND_MOD_DVC: ::std::os::raw::c_int;
    static mut RSND_BASE_SCU: ::std::os::raw::c_int;
    static mut rsnd_mod_get_status: Option<unsafe extern "C" fn()>;
    static mut rsnd_kctrl_accept_anytime: Option<unsafe extern "C" fn()>;

    fn rsnd_mod_write(mod_: *mut rsnd_mod, reg: ::std::os::raw::c_uint, data: u32);
    fn rsnd_kctrl_vals(cfg: rsnd_kctrl_cfg_s) -> u32;
    fn rsnd_kctrl_valm(cfg: rsnd_kctrl_cfg_m, i: ::std::os::raw::c_int) -> u32;
    fn rsnd_kctrl_max(cfg: rsnd_kctrl_cfg_m) -> u32;
    fn rsnd_kctrl_size(cfg: rsnd_kctrl_cfg_m) -> ::std::os::raw::c_int;
    fn rsnd_get_adinr_bit(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_runtime_channel_after_ctu(io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_cmd_attach(io: *mut rsnd_dai_stream, id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn rsnd_mod_id(mod_: *mut rsnd_mod) -> ::std::os::raw::c_int;
    fn rsnd_mod_power_on(mod_: *mut rsnd_mod) -> ::std::os::raw::c_int;
    fn rsnd_mod_power_off(mod_: *mut rsnd_mod);
    fn rsnd_io_to_rdai(io: *mut rsnd_dai_stream) -> *mut rsnd_dai;
    fn rsnd_io_is_play(io: *mut rsnd_dai_stream) -> ::std::os::raw::c_int;
    fn rsnd_rdai_channels_get(rdai: *mut rsnd_dai) -> ::std::os::raw::c_int;
    fn rsnd_kctrl_new_m(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, rtd: *mut snd_soc_pcm_runtime, name: *const ::std::os::raw::c_char, accept: Option<unsafe extern "C" fn()>, update: unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod), cfg: *mut rsnd_kctrl_cfg_m, size: ::std::os::raw::c_int, max: u32) -> ::std::os::raw::c_int;
    fn rsnd_kctrl_new_s(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, rtd: *mut snd_soc_pcm_runtime, name: *const ::std::os::raw::c_char, accept: Option<unsafe extern "C" fn()>, update: unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod), cfg: *mut rsnd_kctrl_cfg_s, max: u32) -> ::std::os::raw::c_int;
    fn rsnd_kctrl_new_e(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, rtd: *mut snd_soc_pcm_runtime, name: *const ::std::os::raw::c_char, accept: Option<unsafe extern "C" fn()>, update: unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod), cfg: *mut rsnd_kctrl_cfg_s, texts: *const ::std::os::raw::c_char, max: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv;
    fn rsnd_dma_request_channel(node: *mut device_node, name: *const ::std::os::raw::c_char, mod_: *mut rsnd_mod, dir: *const ::std::os::raw::c_char) -> *mut dma_chan;
    fn rsnd_dvc_of_node(priv_: *mut rsnd_priv) -> *mut device_node;
    fn rsnd_debugfs_mod_reg_show(m: *mut seq_file, mod_: *mut rsnd_mod, base: ::std::os::raw::c_int, offset: ::std::os::raw::c_int, size: ::std::os::raw::c_int);
    fn rsnd_mod_get(dvc: *mut rsnd_dvc) -> *mut rsnd_mod;
    fn WARN_ON(condition: bool) -> bool;
    fn rsnd_priv_to_dev(priv_: *mut rsnd_priv) -> *mut device;
    fn of_get_child_count(node: *mut device_node) -> ::std::os::raw::c_int;
    fn devm_kcalloc(dev: *mut device, n: ::std::os::raw::c_int, size: usize, flags: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_void;
    fn rsnd_devm_clk_get_indexed(dev: *mut device, name: *const ::std::os::raw::c_char, i: ::std::os::raw::c_int) -> *mut clk;
    fn IS_ERR(ptr: *mut clk) -> bool;
    fn PTR_ERR(ptr: *mut clk) -> ::std::os::raw::c_int;
    fn rsnd_mod_init(priv_: *mut rsnd_priv, mod_: *mut rsnd_mod, ops: *mut rsnd_mod_ops, clk: *mut clk, rstc: *mut ::std::os::raw::c_void, type_: ::std::os::raw::c_int, id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn of_node_put(node: *mut device_node);
    fn rsnd_mod_quit(mod_: *mut rsnd_mod);
    fn rsnd_suspend_clk_reset(clk: *mut clk, rstc: *mut ::std::os::raw::c_void);
    fn rsnd_resume_clk_reset(clk: *mut clk, rstc: *mut ::std::os::raw::c_void);
    fn DVC_SWRSR() -> ::std::os::raw::c_uint;
    fn DVC_DVUIR() -> ::std::os::raw::c_uint;
    fn DVC_VOLxR(i: ::std::os::raw::c_int) -> ::std::os::raw::c_uint;
    fn DVC_ADINR() -> ::std::os::raw::c_uint;
    fn DVC_DVUCR() -> ::std::os::raw::c_uint;
    fn DVC_VRCTR() -> ::std::os::raw::c_uint;
    fn DVC_VRPDR() -> ::std::os::raw::c_uint;
    fn DVC_VRDBR() -> ::std::os::raw::c_uint;
    fn DVC_DVUER() -> ::std::os::raw::c_uint;
    fn DVC_ZCMCR() -> ::std::os::raw::c_uint;
}

unsafe fn rsnd_dvc_get(priv_: *mut rsnd_priv, id: ::std::os::raw::c_int) -> *mut rsnd_dvc {
    ((*priv_).dvc as *mut rsnd_dvc).offset(id as isize)
}

unsafe fn rsnd_dvc_nr(priv_: *mut rsnd_priv) -> ::std::os::raw::c_int {
    (*priv_).dvc_nr
}

unsafe fn rsnd_mod_to_dvc(mod_: *mut rsnd_mod) -> *mut rsnd_dvc {
    mod_ as *mut rsnd_dvc
}

unsafe extern "C" fn rsnd_dvc_activation(mod_: *mut rsnd_mod) {
    rsnd_mod_write(mod_, DVC_SWRSR(), 0);
    rsnd_mod_write(mod_, DVC_SWRSR(), 1);
}

unsafe extern "C" fn rsnd_dvc_halt(mod_: *mut rsnd_mod) {
    rsnd_mod_write(mod_, DVC_DVUIR(), 1);
    rsnd_mod_write(mod_, DVC_SWRSR(), 0);
}

unsafe fn rsnd_dvc_get_vrpdr(dvc: *mut rsnd_dvc) -> u32 {
    (rsnd_kctrl_vals((*dvc).rup) << 8) | rsnd_kctrl_vals((*dvc).rdown)
}

unsafe fn rsnd_dvc_get_vrdbr(dvc: *mut rsnd_dvc) -> u32 {
    0x3ffu32.wrapping_sub(rsnd_kctrl_valm((*dvc).volume, 0) >> 13)
}

unsafe extern "C" fn rsnd_dvc_volume_parameter(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let dvc: *mut rsnd_dvc = rsnd_mod_to_dvc(mod_);
    let mut val: [u32; 32] = [0; 32];
    let mut i: ::std::os::raw::c_int;

    /* Enable Ramp */
    if rsnd_kctrl_vals((*dvc).ren) != 0 {
        i = 0;
        while i < RSND_MAX_CHANNELS {
            val[i as usize] = rsnd_kctrl_max((*dvc).volume);
            i += 1;
        }
    } else {
        i = 0;
        while i < RSND_MAX_CHANNELS {
            val[i as usize] = rsnd_kctrl_valm((*dvc).volume, i);
            i += 1;
        }
    }

    /* Enable Digital Volume */
    i = 0;
    while i < RSND_MAX_CHANNELS {
        rsnd_mod_write(mod_, DVC_VOLxR(i), val[i as usize]);
        i += 1;
    }
}

unsafe extern "C" fn rsnd_dvc_volume_init(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let dvc: *mut rsnd_dvc = rsnd_mod_to_dvc(mod_);
    let mut adinr: u32 = 0;
    let mut dvucr: u32 = 0;
    let mut vrctr: u32 = 0;
    let mut vrpdr: u32 = 0;
    let mut vrdbr: u32 = 0;

    adinr = rsnd_get_adinr_bit(mod_, io) | rsnd_runtime_channel_after_ctu(io);

    /* Enable Digital Volume, Zero Cross Mute Mode */
    dvucr |= 0x101;

    /* Enable Ramp */
    if rsnd_kctrl_vals((*dvc).ren) != 0 {
        dvucr |= 0x10;

        /*
         * FIXME !!
         * use scale-downed Digital Volume
         * as Volume Ramp
         * 7F FFFF -> 3FF
         */
        vrctr = 0xff;
        vrpdr = rsnd_dvc_get_vrpdr(dvc);
        vrdbr = rsnd_dvc_get_vrdbr(dvc);
    }

    /* Initialize operation */
    rsnd_mod_write(mod_, DVC_DVUIR(), 1);

    /* General Information */
    rsnd_mod_write(mod_, DVC_ADINR(), adinr);
    rsnd_mod_write(mod_, DVC_DVUCR(), dvucr);

    /* Volume Ramp Parameter */
    rsnd_mod_write(mod_, DVC_VRCTR(), vrctr);
    rsnd_mod_write(mod_, DVC_VRPDR(), vrpdr);
    rsnd_mod_write(mod_, DVC_VRDBR(), vrdbr);

    /* Digital Volume Function Parameter */
    rsnd_dvc_volume_parameter(io, mod_);

    /* cancel operation */
    rsnd_mod_write(mod_, DVC_DVUIR(), 0);
}

unsafe extern "C" fn rsnd_dvc_volume_update(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let dvc: *mut rsnd_dvc = rsnd_mod_to_dvc(mod_);
    let mut zcmcr: u32 = 0;
    let mut vrpdr: u32 = 0;
    let mut vrdbr: u32 = 0;
    let mut i: ::std::os::raw::c_int;

    i = 0;
    while i < rsnd_kctrl_size((*dvc).mute) {
        zcmcr |= ((rsnd_kctrl_valm((*dvc).mute, i) != 0) as u32) << i;
        i += 1;
    }

    if rsnd_kctrl_vals((*dvc).ren) != 0 {
        vrpdr = rsnd_dvc_get_vrpdr(dvc);
        vrdbr = rsnd_dvc_get_vrdbr(dvc);
    }

    /* Disable DVC Register access */
    rsnd_mod_write(mod_, DVC_DVUER(), 0);

    /* Zero Cross Mute Function */
    rsnd_mod_write(mod_, DVC_ZCMCR(), zcmcr);

    /* Volume Ramp Function */
    rsnd_mod_write(mod_, DVC_VRPDR(), vrpdr);
    rsnd_mod_write(mod_, DVC_VRDBR(), vrdbr);
    /* add DVC_VRWTR here */

    /* Digital Volume Function Parameter */
    rsnd_dvc_volume_parameter(io, mod_);

    /* Enable DVC Register access */
    rsnd_mod_write(mod_, DVC_DVUER(), 1);
}

unsafe extern "C" fn rsnd_dvc_probe_(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> ::std::os::raw::c_int {
    rsnd_cmd_attach(io, rsnd_mod_id(mod_))
}

unsafe extern "C" fn rsnd_dvc_init(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> ::std::os::raw::c_int {
    let mut ret: ::std::os::raw::c_int;

    ret = rsnd_mod_power_on(mod_);
    if ret < 0 {
        return ret;
    }

    rsnd_dvc_activation(mod_);

    rsnd_dvc_volume_init(io, mod_);

    rsnd_dvc_volume_update(io, mod_);

    0
}

unsafe extern "C" fn rsnd_dvc_quit(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> ::std::os::raw::c_int {
    rsnd_dvc_halt(mod_);

    rsnd_mod_power_off(mod_);

    0
}

unsafe extern "C" fn rsnd_dvc_pcm_new(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    rtd: *mut snd_soc_pcm_runtime,
) -> ::std::os::raw::c_int {
    let dvc: *mut rsnd_dvc = rsnd_mod_to_dvc(mod_);
    let rdai: *mut rsnd_dai = rsnd_io_to_rdai(io);
    let is_play: ::std::os::raw::c_int = rsnd_io_is_play(io);
    let channels: ::std::os::raw::c_int = rsnd_rdai_channels_get(rdai);
    let mut ret: ::std::os::raw::c_int;

    /* Volume */
    ret = rsnd_kctrl_new_m(
        mod_,
        io,
        rtd,
        if is_play != 0 { b"DVC Out Playback Volume\0".as_ptr() } else { b"DVC In Capture Volume\0".as_ptr() } as *const ::std::os::raw::c_char,
        rsnd_kctrl_accept_anytime,
        rsnd_dvc_volume_update,
        &mut (*dvc).volume,
        channels,
        0x00800000 - 1,
    );
    if ret < 0 {
        return ret;
    }

    /* Mute */
    ret = rsnd_kctrl_new_m(
        mod_,
        io,
        rtd,
        if is_play != 0 { b"DVC Out Mute Switch\0".as_ptr() } else { b"DVC In Mute Switch\0".as_ptr() } as *const ::std::os::raw::c_char,
        rsnd_kctrl_accept_anytime,
        rsnd_dvc_volume_update,
        &mut (*dvc).mute,
        channels,
        1,
    );
    if ret < 0 {
        return ret;
    }

    /* Ramp */
    ret = rsnd_kctrl_new_s(
        mod_,
        io,
        rtd,
        if is_play != 0 { b"DVC Out Ramp Switch\0".as_ptr() } else { b"DVC In Ramp Switch\0".as_ptr() } as *const ::std::os::raw::c_char,
        rsnd_kctrl_accept_anytime,
        rsnd_dvc_volume_update,
        &mut (*dvc).ren,
        1,
    );
    if ret < 0 {
        return ret;
    }

    ret = rsnd_kctrl_new_e(
        mod_,
        io,
        rtd,
        if is_play != 0 { b"DVC Out Ramp Up Rate\0".as_ptr() } else { b"DVC In Ramp Up Rate\0".as_ptr() } as *const ::std::os::raw::c_char,
        rsnd_kctrl_accept_anytime,
        rsnd_dvc_volume_update,
        &mut (*dvc).rup,
        volume_ramp_rate,
        VOLUME_RAMP_MAX_DVC,
    );
    if ret < 0 {
        return ret;
    }

    ret = rsnd_kctrl_new_e(
        mod_,
        io,
        rtd,
        if is_play != 0 { b"DVC Out Ramp Down Rate\0".as_ptr() } else { b"DVC In Ramp Down Rate\0".as_ptr() } as *const ::std::os::raw::c_char,
        rsnd_kctrl_accept_anytime,
        rsnd_dvc_volume_update,
        &mut (*dvc).rdown,
        volume_ramp_rate,
        VOLUME_RAMP_MAX_DVC,
    );

    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn rsnd_dvc_dma_req(
    io: *mut rsnd_dai_stream,
    mod_: *mut rsnd_mod,
) -> *mut dma_chan {
    let priv_: *mut rsnd_priv = rsnd_mod_to_priv(mod_);

    rsnd_dma_request_channel(
        rsnd_dvc_of_node(priv_),
        DVC_NAME,
        mod_,
        b"tx\0".as_ptr() as *const ::std::os::raw::c_char,
    )
}

// CONFIG_DEBUG_FS
unsafe extern "C" fn rsnd_dvc_debug_info(
    m: *mut seq_file,
    io: *mut rsnd_dai_stream,
    mod_: *mut rsnd_mod,
) {
    rsnd_debugfs_mod_reg_show(
        m,
        mod_,
        RSND_BASE_SCU,
        0xe00 + rsnd_mod_id(mod_) * 0x100,
        0x60,
    );
}

static mut rsnd_dvc_ops: rsnd_mod_ops = rsnd_mod_ops {
    name: DVC_NAME,
    dma_req: Some(rsnd_dvc_dma_req),
    probe: Some(rsnd_dvc_probe_),
    init: Some(rsnd_dvc_init),
    quit: Some(rsnd_dvc_quit),
    pcm_new: Some(rsnd_dvc_pcm_new),
    get_status: unsafe { rsnd_mod_get_status },
};

pub unsafe extern "C" fn rsnd_dvc_mod_get(
    priv_: *mut rsnd_priv,
    mut id: ::std::os::raw::c_int,
) -> *mut rsnd_mod {
    if WARN_ON(id < 0 || id >= rsnd_dvc_nr(priv_)) {
        id = 0;
    }

    rsnd_mod_get(rsnd_dvc_get(priv_, id))
}

pub unsafe extern "C" fn rsnd_dvc_probe(priv_: *mut rsnd_priv) -> ::std::os::raw::c_int {
    let mut node: *mut device_node;
    let dev: *mut device = rsnd_priv_to_dev(priv_);
    let mut dvc: *mut rsnd_dvc;
    let mut clk: *mut clk;
    let mut i: ::std::os::raw::c_int;
    let mut nr: ::std::os::raw::c_int;
    let mut ret: ::std::os::raw::c_int;

    node = rsnd_dvc_of_node(priv_);
    if node.is_null() {
        return 0; /* not used is not error */
    }

    nr = of_get_child_count(node);
    if nr == 0 {
        ret = -EINVAL;
        of_node_put(node);
        return ret;
    }

    dvc = devm_kcalloc(dev, nr, ::std::mem::size_of::<rsnd_dvc>(), GFP_KERNEL) as *mut rsnd_dvc;
    if dvc.is_null() {
        ret = -ENOMEM;
        of_node_put(node);
        return ret;
    }

    (*priv_).dvc_nr = nr;
    (*priv_).dvc = dvc as *mut ::std::os::raw::c_void;

    i = 0;
    ret = 0;
    // for_each_child_of_node_scoped(node, np)
    while i < nr {
        dvc = rsnd_dvc_get(priv_, i);

        clk = rsnd_devm_clk_get_indexed(dev, DVC_NAME, i);
        if IS_ERR(clk) {
            ret = PTR_ERR(clk);
            of_node_put(node);
            return ret;
        }

        ret = rsnd_mod_init(
            priv_,
            rsnd_mod_get(dvc),
            &mut rsnd_dvc_ops,
            clk,
            ::std::ptr::null_mut(),
            RSND_MOD_DVC,
            i,
        );
        if ret != 0 {
            of_node_put(node);
            return ret;
        }

        i += 1;
    }

    of_node_put(node);

    ret
}

pub unsafe extern "C" fn rsnd_dvc_remove(priv_: *mut rsnd_priv) {
    let mut dvc: *mut rsnd_dvc;
    let mut i: ::std::os::raw::c_int;

    i = 0;
    while i < rsnd_dvc_nr(priv_) {
        dvc = rsnd_dvc_get(priv_, i);
        rsnd_mod_quit(rsnd_mod_get(dvc));
        i += 1;
    }
}

pub unsafe extern "C" fn rsnd_dvc_suspend(priv_: *mut rsnd_priv) {
    let mut dvc: *mut rsnd_dvc;
    let mut i: ::std::os::raw::c_int;

    i = 0;
    while i < rsnd_dvc_nr(priv_) {
        dvc = rsnd_dvc_get(priv_, i);
        // Access to rsnd_mod fields clk/rstc depends on the external rsnd_mod layout.
        rsnd_suspend_clk_reset(::std::ptr::null_mut(), ::std::ptr::null_mut());
        i += 1;
    }
}

pub unsafe extern "C" fn rsnd_dvc_resume(priv_: *mut rsnd_priv) {
    let mut dvc: *mut rsnd_dvc;
    let mut i: ::std::os::raw::c_int;

    i = 0;
    while i < rsnd_dvc_nr(priv_) {
        dvc = rsnd_dvc_get(priv_, i);
        // Access to rsnd_mod fields clk/rstc depends on the external rsnd_mod layout.
        rsnd_resume_clk_reset(::std::ptr::null_mut(), ::std::ptr::null_mut());
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
