// SPDX-License-Identifier: GPL-2.0
//
// mix.c
//
// Copyright (c) 2015 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

/*
 *		    CTUn	MIXn
 *		    +------+	+------+
 * [SRC3 / SRC6] -> |CTU n0| ->	[MIX n0| ->
 * [SRC4 / SRC9] -> |CTU n1| ->	[MIX n1| ->
 * [SRC0 / SRC1] -> |CTU n2| ->	[MIX n2| ->
 * [SRC2 / SRC5] -> |CTU n3| ->	[MIX n3| ->
 *		    +------+	+------+
 *
 * ex)
 *	DAI0 : playback = <&src0 &ctu02 &mix0 &dvc0 &ssi0>;
 *	DAI1 : playback = <&src2 &ctu03 &mix0 &dvc0 &ssi0>;
 *
 * MIX Volume
 *	amixer set "MIX",0  100%  // DAI0 Volume
 *	amixer set "MIX",1  100%  // DAI1 Volume
 *
 * Volume Ramp
 *	amixer set "MIX Ramp Up Rate"   "0.125 dB/1 step"
 *	amixer set "MIX Ramp Down Rate" "4 dB/1 step"
 *	amixer set "MIX Ramp" on
 *	aplay xxx.wav &
 *	amixer set "MIX",0  80%  // DAI0 Volume Down
 *	amixer set "MIX",1 100%  // DAI1 Volume Up
 */

// Dependency intent from C: #include "rsnd.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type u32 = c_uint;

const MIX_NAME: &[u8] = b"mix\0";

#[repr(C)]
pub struct rsnd_mix {
    pub mod_: rsnd_mod,
    pub volumeA: rsnd_kctrl_cfg_s, /* MDBAR */
    pub volumeB: rsnd_kctrl_cfg_s, /* MDBBR */
    pub volumeC: rsnd_kctrl_cfg_s, /* MDBCR */
    pub volumeD: rsnd_kctrl_cfg_s, /* MDBDR */
    pub ren: rsnd_kctrl_cfg_s, /* Ramp Enable */
    pub rup: rsnd_kctrl_cfg_s, /* Ramp Rate Up */
    pub rdw: rsnd_kctrl_cfg_s, /* Ramp Rate Down */
    pub flags: u32,
}

const ONCE_KCTRL_INITIALIZED: u32 = 1 << 0;
const HAS_VOLA: u32 = 1 << 1;
const HAS_VOLB: u32 = 1 << 2;
const HAS_VOLC: u32 = 1 << 3;
const HAS_VOLD: u32 = 1 << 4;

const VOL_MAX: u32 = 0x3ff;

unsafe fn rsnd_mod_to_mix(mod_: *mut rsnd_mod) -> *mut rsnd_mix {
    mod_ as *mut rsnd_mix
}

unsafe fn rsnd_mix_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_mix {
    ((*priv_).mix as *mut rsnd_mix).add(id as usize)
}

unsafe fn rsnd_mix_nr(priv_: *mut rsnd_priv) -> c_int {
    (*priv_).mix_nr
}

unsafe fn rsnd_mix_activation(mod_: *mut rsnd_mod) {
    rsnd_mod_write(mod_, MIX_SWRSR, 0);
    rsnd_mod_write(mod_, MIX_SWRSR, 1);
}

unsafe fn rsnd_mix_halt(mod_: *mut rsnd_mod) {
    rsnd_mod_write(mod_, MIX_MIXIR, 1);
    rsnd_mod_write(mod_, MIX_SWRSR, 0);
}

unsafe fn rsnd_mix_get_vol(mix: *mut rsnd_mix, has_vol: u32, volume: *mut rsnd_kctrl_cfg_s) -> u32 {
    if rsnd_flags_has(mix, has_vol) != 0 {
        VOL_MAX.wrapping_sub(rsnd_kctrl_vals(volume) as u32)
    } else {
        0
    }
}

unsafe fn rsnd_mix_volume_parameter(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let mix = rsnd_mod_to_mix(mod_);
    let volA = rsnd_mix_get_vol(mix, HAS_VOLA, ptr::addr_of_mut!((*mix).volumeA));
    let volB = rsnd_mix_get_vol(mix, HAS_VOLB, ptr::addr_of_mut!((*mix).volumeB));
    let volC = rsnd_mix_get_vol(mix, HAS_VOLC, ptr::addr_of_mut!((*mix).volumeC));
    let volD = rsnd_mix_get_vol(mix, HAS_VOLD, ptr::addr_of_mut!((*mix).volumeD));

    dev_dbg(
        dev,
        b"MIX A/B/C/D = %02x/%02x/%02x/%02x\n\0".as_ptr() as *const c_char,
        volA,
        volB,
        volC,
        volD,
    );

    rsnd_mod_write(mod_, MIX_MDBAR, volA);
    rsnd_mod_write(mod_, MIX_MDBBR, volB);
    rsnd_mod_write(mod_, MIX_MDBCR, volC);
    rsnd_mod_write(mod_, MIX_MDBDR, volD);
}

unsafe fn rsnd_mix_volume_init(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    let mix = rsnd_mod_to_mix(mod_);

    rsnd_mod_write(mod_, MIX_MIXIR, 1);

    /* General Information */
    rsnd_mod_write(mod_, MIX_ADINR, rsnd_runtime_channel_after_ctu(io) as u32);

    /* volume step */
    rsnd_mod_write(mod_, MIX_MIXMR, rsnd_kctrl_vals(ptr::addr_of_mut!((*mix).ren)) as u32);
    rsnd_mod_write(
        mod_,
        MIX_MVPDR,
        ((rsnd_kctrl_vals(ptr::addr_of_mut!((*mix).rup)) as u32) << 8)
            | (rsnd_kctrl_vals(ptr::addr_of_mut!((*mix).rdw)) as u32),
    );

    /* common volume parameter */
    rsnd_mix_volume_parameter(io, mod_);

    rsnd_mod_write(mod_, MIX_MIXIR, 0);
}

unsafe extern "C" fn rsnd_mix_volume_update(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
    /* Disable MIX dB setting */
    rsnd_mod_write(mod_, MIX_MDBER, 0);

    /* common volume parameter */
    rsnd_mix_volume_parameter(io, mod_);

    /* Enable MIX dB setting */
    rsnd_mod_write(mod_, MIX_MDBER, 1);
}

unsafe extern "C" fn rsnd_mix_probe_(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> c_int {
    rsnd_cmd_attach(io, rsnd_mod_id(mod_))
}

unsafe extern "C" fn rsnd_mix_init(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> c_int {
    let mut ret: c_int;

    ret = rsnd_mod_power_on(mod_);
    if ret < 0 {
        return ret;
    }

    rsnd_mix_activation(mod_);

    rsnd_mix_volume_init(io, mod_);

    rsnd_mix_volume_update(io, mod_);

    0
}

unsafe extern "C" fn rsnd_mix_quit(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> c_int {
    rsnd_mix_halt(mod_);

    rsnd_mod_power_off(mod_);

    0
}

unsafe extern "C" fn rsnd_mix_pcm_new(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let priv_ = rsnd_mod_to_priv(mod_);
    let dev = rsnd_priv_to_dev(priv_);
    let mix = rsnd_mod_to_mix(mod_);
    let src_mod = rsnd_io_to_mod_src(io);
    let volume: *mut rsnd_kctrl_cfg_s;
    let mut ret: c_int;

    match rsnd_mod_id(src_mod) {
        3 | 6 => {
            /* MDBAR */
            volume = ptr::addr_of_mut!((*mix).volumeA);
            rsnd_flags_set(mix, HAS_VOLA);
        }
        4 | 9 => {
            /* MDBBR */
            volume = ptr::addr_of_mut!((*mix).volumeB);
            rsnd_flags_set(mix, HAS_VOLB);
        }
        0 | 1 => {
            /* MDBCR */
            volume = ptr::addr_of_mut!((*mix).volumeC);
            rsnd_flags_set(mix, HAS_VOLC);
        }
        2 | 5 => {
            /* MDBDR */
            volume = ptr::addr_of_mut!((*mix).volumeD);
            rsnd_flags_set(mix, HAS_VOLD);
        }
        _ => {
            dev_err(
                dev,
                b"unknown SRC is connected\n\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
    }

    /* Volume */
    ret = rsnd_kctrl_new_s(
        mod_,
        io,
        rtd,
        b"MIX Playback Volume\0".as_ptr() as *const c_char,
        rsnd_kctrl_accept_anytime,
        Some(rsnd_mix_volume_update),
        volume,
        VOL_MAX,
    );
    if ret < 0 {
        return ret;
    }
    rsnd_kctrl_vals_set(volume, VOL_MAX as c_int);

    if rsnd_flags_has(mix, ONCE_KCTRL_INITIALIZED) != 0 {
        return ret;
    }

    /* Ramp */
    ret = rsnd_kctrl_new_s(
        mod_,
        io,
        rtd,
        b"MIX Ramp Switch\0".as_ptr() as *const c_char,
        rsnd_kctrl_accept_anytime,
        Some(rsnd_mix_volume_update),
        ptr::addr_of_mut!((*mix).ren),
        1,
    );
    if ret < 0 {
        return ret;
    }

    ret = rsnd_kctrl_new_e(
        mod_,
        io,
        rtd,
        b"MIX Ramp Up Rate\0".as_ptr() as *const c_char,
        rsnd_kctrl_accept_anytime,
        Some(rsnd_mix_volume_update),
        ptr::addr_of_mut!((*mix).rup),
        volume_ramp_rate.as_ptr(),
        VOLUME_RAMP_MAX_MIX,
    );
    if ret < 0 {
        return ret;
    }

    ret = rsnd_kctrl_new_e(
        mod_,
        io,
        rtd,
        b"MIX Ramp Down Rate\0".as_ptr() as *const c_char,
        rsnd_kctrl_accept_anytime,
        Some(rsnd_mix_volume_update),
        ptr::addr_of_mut!((*mix).rdw),
        volume_ramp_rate.as_ptr(),
        VOLUME_RAMP_MAX_MIX,
    );

    rsnd_flags_set(mix, ONCE_KCTRL_INITIALIZED);

    ret
}

// C conditional intent: this debug callback is present only with CONFIG_DEBUG_FS.
#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" fn rsnd_mix_debug_info(
    m: *mut seq_file,
    io: *mut rsnd_dai_stream,
    mod_: *mut rsnd_mod,
) {
    rsnd_debugfs_mod_reg_show(
        m,
        mod_,
        RSND_BASE_SCU,
        0xd00 + rsnd_mod_id(mod_) * 0x40,
        0x30,
    );
}

static mut rsnd_mix_ops: rsnd_mod_ops = rsnd_mod_ops {
    name: MIX_NAME.as_ptr() as *const c_char,
    probe: Some(rsnd_mix_probe_),
    init: Some(rsnd_mix_init),
    quit: Some(rsnd_mix_quit),
    pcm_new: Some(rsnd_mix_pcm_new),
    get_status: Some(rsnd_mod_get_status),
    #[cfg(CONFIG_DEBUG_FS)]
    debug_info: Some(rsnd_mix_debug_info),
};

#[no_mangle]
pub unsafe extern "C" fn rsnd_mix_mod_get(priv_: *mut rsnd_priv, mut id: c_int) -> *mut rsnd_mod {
    if WARN_ON((id < 0 || id >= rsnd_mix_nr(priv_)) as c_int) != 0 {
        id = 0;
    }

    rsnd_mod_get(rsnd_mix_get(priv_, id))
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_mix_probe(priv_: *mut rsnd_priv) -> c_int {
    let node: *mut device_node;
    let dev = rsnd_priv_to_dev(priv_);
    let mut mix: *mut rsnd_mix;
    let mut clk: *mut clk;
    let mut i: c_int;
    let nr: c_int;
    let mut ret: c_int;

    node = rsnd_mix_of_node(priv_);
    if node.is_null() {
        return 0; /* not used is not error */
    }

    nr = of_get_child_count(node);
    if nr == 0 {
        ret = -EINVAL;
        goto_rsnd_mix_probe_done(node, ret)
    } else {
        mix = devm_kcalloc(
            dev,
            nr as usize,
            core::mem::size_of::<rsnd_mix>(),
            GFP_KERNEL,
        ) as *mut rsnd_mix;
        if mix.is_null() {
            ret = -ENOMEM;
            goto_rsnd_mix_probe_done(node, ret)
        } else {
            (*priv_).mix_nr = nr;
            (*priv_).mix = mix as *mut c_void;

            i = 0;
            ret = 0;
            let mut np = MaybeUninit::<*mut device_node>::uninit();
            let mut iter = for_each_child_of_node_scoped_start(node, np.as_mut_ptr());
            while iter != 0 {
                mix = rsnd_mix_get(priv_, i);

                clk = rsnd_devm_clk_get_indexed(dev, MIX_NAME.as_ptr() as *const c_char, i);
                if IS_ERR(clk as *const c_void) != 0 {
                    ret = PTR_ERR(clk as *const c_void);
                    break;
                }

                ret = rsnd_mod_init(
                    priv_,
                    rsnd_mod_get(mix),
                    ptr::addr_of_mut!(rsnd_mix_ops),
                    clk,
                    ptr::null_mut(),
                    RSND_MOD_MIX,
                    i,
                );
                if ret != 0 {
                    break;
                }

                i += 1;
                iter = for_each_child_of_node_scoped_next(node, np.as_mut_ptr());
            }

            of_node_put(node);

            ret
        }
    }
}

unsafe fn goto_rsnd_mix_probe_done(node: *mut device_node, ret: c_int) -> c_int {
    of_node_put(node);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_mix_remove(priv_: *mut rsnd_priv) {
    let mut mix: *mut rsnd_mix;
    let mut i: c_int = 0;

    while i < rsnd_mix_nr(priv_) {
        mix = ((*priv_).mix as *mut rsnd_mix).add(i as usize);
        if mix.is_null() {
            break;
        }
        rsnd_mod_quit(rsnd_mod_get(mix));
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_mix_suspend(priv_: *mut rsnd_priv) {
    let mut mix: *mut rsnd_mix;
    let mut i: c_int = 0;

    while i < rsnd_mix_nr(priv_) {
        mix = ((*priv_).mix as *mut rsnd_mix).add(i as usize);
        if mix.is_null() {
            break;
        }
        rsnd_suspend_clk_reset((*rsnd_mod_get(mix)).clk, (*rsnd_mod_get(mix)).rstc);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_mix_resume(priv_: *mut rsnd_priv) {
    let mut mix: *mut rsnd_mix;
    let mut i: c_int = 0;

    while i < rsnd_mix_nr(priv_) {
        mix = ((*priv_).mix as *mut rsnd_mix).add(i as usize);
        if mix.is_null() {
            break;
        }
        rsnd_resume_clk_reset((*rsnd_mod_get(mix)).clk, (*rsnd_mod_get(mix)).rstc);
        i += 1;
    }
}

#[repr(C)]
pub struct rsnd_mod {
    pub clk: *mut clk,
    pub rstc: *mut c_void,
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
    pub mix_nr: c_int,
    pub mix: *mut c_void,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
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
pub struct rsnd_mod_ops {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub quit: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_soc_pcm_runtime) -> c_int>,
    pub get_status: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debug_info: Option<unsafe extern "C" fn(*mut seq_file, *mut rsnd_dai_stream, *mut rsnd_mod)>,
}

unsafe extern "C" {
    static volume_ramp_rate: [*const c_char; 0];

    static MIX_SWRSR: c_int;
    static MIX_MIXIR: c_int;
    static MIX_MDBAR: c_int;
    static MIX_MDBBR: c_int;
    static MIX_MDBCR: c_int;
    static MIX_MDBDR: c_int;
    static MIX_ADINR: c_int;
    static MIX_MIXMR: c_int;
    static MIX_MVPDR: c_int;
    static MIX_MDBER: c_int;
    static VOLUME_RAMP_MAX_MIX: u32;
    static RSND_BASE_SCU: c_int;
    static GFP_KERNEL: c_uint;
    static RSND_MOD_MIX: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;

    fn rsnd_mod_write(mod_: *mut rsnd_mod, reg: c_int, data: u32);
    fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv;
    fn rsnd_priv_to_dev(priv_: *mut rsnd_priv) -> *mut device;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn rsnd_runtime_channel_after_ctu(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_kctrl_vals(cfg: *mut rsnd_kctrl_cfg_s) -> c_int;
    fn rsnd_kctrl_vals_set(cfg: *mut rsnd_kctrl_cfg_s, val: c_int);
    fn rsnd_cmd_attach(io: *mut rsnd_dai_stream, id: c_int) -> c_int;
    fn rsnd_mod_id(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_power_on(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_power_off(mod_: *mut rsnd_mod);
    fn rsnd_io_to_mod_src(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_flags_has(mix: *mut rsnd_mix, flag: u32) -> c_int;
    fn rsnd_flags_set(mix: *mut rsnd_mix, flag: u32);
    fn rsnd_kctrl_accept_anytime() -> c_int;
    fn rsnd_kctrl_new_s(
        mod_: *mut rsnd_mod,
        io: *mut rsnd_dai_stream,
        rtd: *mut snd_soc_pcm_runtime,
        name: *const c_char,
        accept: unsafe extern "C" fn() -> c_int,
        update: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod)>,
        cfg: *mut rsnd_kctrl_cfg_s,
        max: u32,
    ) -> c_int;
    fn rsnd_kctrl_new_e(
        mod_: *mut rsnd_mod,
        io: *mut rsnd_dai_stream,
        rtd: *mut snd_soc_pcm_runtime,
        name: *const c_char,
        accept: unsafe extern "C" fn() -> c_int,
        update: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod)>,
        cfg: *mut rsnd_kctrl_cfg_s,
        texts: *const *const c_char,
        max: u32,
    ) -> c_int;
    fn rsnd_mod_get_status(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_debugfs_mod_reg_show(m: *mut seq_file, mod_: *mut rsnd_mod, base: c_int, offset: c_int, size: c_int);
    fn WARN_ON(condition: c_int) -> c_int;
    fn rsnd_mod_get(mix: *mut rsnd_mix) -> *mut rsnd_mod;
    fn rsnd_mix_of_node(priv_: *mut rsnd_priv) -> *mut device_node;
    fn of_get_child_count(node: *mut device_node) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn rsnd_devm_clk_get_indexed(dev: *mut device, name: *const c_char, index: c_int) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn rsnd_mod_init(
        priv_: *mut rsnd_priv,
        mod_: *mut rsnd_mod,
        ops: *mut rsnd_mod_ops,
        clk: *mut clk,
        type_: *mut c_void,
        id: c_int,
        index: c_int,
    ) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn rsnd_mod_quit(mod_: *mut rsnd_mod);
    fn rsnd_suspend_clk_reset(clk: *mut clk, rstc: *mut c_void);
    fn rsnd_resume_clk_reset(clk: *mut clk, rstc: *mut c_void);

    // Rust stand-ins for the C for_each_child_of_node_scoped(node, np) iterator macro.
    fn for_each_child_of_node_scoped_start(node: *mut device_node, np: *mut *mut device_node) -> c_int;
    fn for_each_child_of_node_scoped_next(node: *mut device_node, np: *mut *mut device_node) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
