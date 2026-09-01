// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car CMD support
//
// Copyright (C) 2015 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

// C source included "rsnd.h"; the declarations referenced here are expected
// to be provided by the surrounding translated repository.

#[repr(C)]
pub struct rsnd_cmd {
    pub mod_: rsnd_mod,
}

pub const CMD_NAME: *const ::core::ffi::c_char = b"cmd\0".as_ptr() as *const ::core::ffi::c_char;

#[inline]
unsafe fn rsnd_cmd_nr(priv_: *mut rsnd_priv) -> i32 {
    unsafe { (*priv_).cmd_nr }
}

macro_rules! for_each_rsnd_cmd {
    ($pos:ident, $priv:expr, $i:ident, $body:block) => {{
        $i = 0;
        while $i < unsafe { rsnd_cmd_nr($priv) } {
            $pos = unsafe { ((*$priv).cmd as *mut rsnd_cmd).add($i as usize) };
            $body
            $i += 1;
        }
    }};
}

unsafe extern "C" fn rsnd_cmd_init(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> ::core::ffi::c_int {
    let dvc: *mut rsnd_mod = unsafe { rsnd_io_to_mod_dvc(io) };
    let mix: *mut rsnd_mod = unsafe { rsnd_io_to_mod_mix(io) };
    let dev: *mut device = unsafe { rsnd_priv_to_dev(priv_) };
    let mut data: u32;
    static PATH: [u32; 10] = [
        0,
        1 << 0,
        0,
        0,
        0,
        1 << 8,
        1 << 12,
        0,
        0,
        1 << 15,
    ];

    if mix.is_null() && dvc.is_null() {
        return 0;
    }

    if PATH.len() < (unsafe { rsnd_mod_id(mod_) } + 1) as usize {
        return -ENXIO;
    }

    if !mix.is_null() {
        let mut rdai: *mut rsnd_dai = ::core::ptr::null_mut();
        let mut i: ::core::ffi::c_int = 0;

        /*
         * it is assuming that integrater is well understanding about
         * data path. Here doesn't check impossible connection,
         * like src2 + src5
         */
        data = 0;
        for_each_rsnd_dai!(rdai, priv_, i, {
            let mut tio: *mut rsnd_dai_stream = unsafe { &mut (*rdai).playback };
            let mut src: *mut rsnd_mod = unsafe { rsnd_io_to_mod_src(tio) };

            if mix == unsafe { rsnd_io_to_mod_mix(tio) } {
                data |= PATH[unsafe { rsnd_mod_id(src) } as usize];
            }

            tio = unsafe { &mut (*rdai).capture };
            src = unsafe { rsnd_io_to_mod_src(tio) };
            if mix == unsafe { rsnd_io_to_mod_mix(tio) } {
                data |= PATH[unsafe { rsnd_mod_id(src) } as usize];
            }
        });
    } else {
        let src: *mut rsnd_mod = unsafe { rsnd_io_to_mod_src(io) };

        static CMD_CASE: [u8; 10] = [0x3, 0x3, 0x4, 0x1, 0x2, 0x4, 0x1, 0, 0, 0x2];

        if unsafe { unlikely(src.is_null()) } {
            return -EIO;
        }

        data = PATH[unsafe { rsnd_mod_id(src) } as usize]
            | ((CMD_CASE[unsafe { rsnd_mod_id(src) } as usize] as u32) << 16);
    }

    unsafe {
        dev_dbg(dev, b"ctu/mix path = 0x%08x\n\0".as_ptr() as *const ::core::ffi::c_char, data);

        rsnd_mod_write(mod_, CMD_ROUTE_SLCT, data);
        rsnd_mod_write(mod_, CMD_BUSIF_MODE, rsnd_get_busif_shift(io, mod_) | 1);
        rsnd_mod_write(mod_, CMD_BUSIF_DALIGN, rsnd_get_dalign(mod_, io));

        rsnd_adg_set_cmd_timsel_gen2(mod_, io);
    }

    0
}

unsafe extern "C" fn rsnd_cmd_start(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> ::core::ffi::c_int {
    unsafe {
        rsnd_mod_write(mod_, CMD_CTRL, 0x10);
    }

    0
}

unsafe extern "C" fn rsnd_cmd_stop(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> ::core::ffi::c_int {
    unsafe {
        rsnd_mod_write(mod_, CMD_CTRL, 0);
    }

    0
}

// CONFIG_DEBUG_FS:
// When enabled in the original C build, rsnd_cmd_debug_info is assigned to
// rsnd_cmd_ops.debug_info.
#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" fn rsnd_cmd_debug_info(
    m: *mut seq_file,
    io: *mut rsnd_dai_stream,
    mod_: *mut rsnd_mod,
) {
    unsafe {
        rsnd_debugfs_mod_reg_show(
            m,
            mod_,
            RSND_BASE_SCU,
            0x180 + rsnd_mod_id_raw(mod_) * 0x20,
            0x30,
        );
    }
}

static mut rsnd_cmd_ops: rsnd_mod_ops = rsnd_mod_ops {
    name: CMD_NAME,
    init: Some(rsnd_cmd_init),
    start: Some(rsnd_cmd_start),
    stop: Some(rsnd_cmd_stop),
    get_status: Some(rsnd_mod_get_status),
    #[cfg(CONFIG_DEBUG_FS)]
    debug_info: Some(rsnd_cmd_debug_info),
};

unsafe fn rsnd_cmd_mod_get(priv_: *mut rsnd_priv, mut id: ::core::ffi::c_int) -> *mut rsnd_mod {
    if unsafe { WARN_ON(id < 0 || id >= rsnd_cmd_nr(priv_)) } {
        id = 0;
    }

    unsafe { rsnd_mod_get(((*priv_).cmd as *mut rsnd_cmd).add(id as usize)) }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_cmd_attach(
    io: *mut rsnd_dai_stream,
    id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let priv_: *mut rsnd_priv = unsafe { rsnd_io_to_priv(io) };
    let mod_: *mut rsnd_mod = unsafe { rsnd_cmd_mod_get(priv_, id) };

    unsafe { rsnd_dai_connect(mod_, io, (*mod_).type_) }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_cmd_probe(priv_: *mut rsnd_priv) -> ::core::ffi::c_int {
    let dev: *mut device = unsafe { rsnd_priv_to_dev(priv_) };
    let mut cmd: *mut rsnd_cmd;
    let mut i: ::core::ffi::c_int = 0;
    let nr: ::core::ffi::c_int;

    /* same number as DVC */
    nr = unsafe { (*priv_).dvc_nr };
    if nr == 0 {
        return 0;
    }

    cmd = unsafe {
        devm_kcalloc(
            dev,
            nr as usize,
            ::core::mem::size_of::<rsnd_cmd>(),
            GFP_KERNEL,
        ) as *mut rsnd_cmd
    };
    if cmd.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).cmd_nr = nr;
        (*priv_).cmd = cmd as *mut ::core::ffi::c_void;
    }

    for_each_rsnd_cmd!(cmd, priv_, i, {
        let ret: ::core::ffi::c_int = unsafe {
            rsnd_mod_init(
                priv_,
                rsnd_mod_get(cmd),
                &raw mut rsnd_cmd_ops,
                ::core::ptr::null_mut(),
                ::core::ptr::null_mut(),
                RSND_MOD_CMD,
                i,
            )
        };
        if ret != 0 {
            return ret;
        }
    });

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_cmd_remove(priv_: *mut rsnd_priv) {
    let mut cmd: *mut rsnd_cmd = ::core::ptr::null_mut();
    let mut i: ::core::ffi::c_int = 0;

    for_each_rsnd_cmd!(cmd, priv_, i, {
        unsafe {
            rsnd_mod_quit(rsnd_mod_get(cmd));
        }
    });
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
