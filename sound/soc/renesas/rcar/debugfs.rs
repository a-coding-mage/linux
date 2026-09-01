// SPDX-License-Identifier: GPL-2.0
//
// // Renesas R-Car debugfs support
//
// Copyright (c) 2021 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
//	> mount -t debugfs none /sys/kernel/debug
//	> cd /sys/kernel/debug/asoc/rcar-sound/ec500000.sound/rdai{N}/
//	> cat playback/xxx
//	> cat capture/xxx
//
// C source condition: CONFIG_DEBUG_FS

#![cfg(CONFIG_DEBUG_FS)]

use core::ffi::{c_char, c_int, c_uint, c_void};

unsafe extern "C" {
    static rsnd_debugfs_fops: file_operations;

    fn rsnd_io_to_mod_ssi(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv;
    fn rsnd_adg_clk_dbg_info(priv_: *mut rsnd_priv, m: *mut seq_file);
    fn rsnd_mod_name(mod_: *mut rsnd_mod) -> *const c_char;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn seq_puts(m: *mut seq_file, s: *const c_char) -> c_int;
    fn __raw_readl(addr: *const c_void) -> u32;
    fn rsnd_gen_get_phy_addr(priv_: *mut rsnd_priv, reg_id: c_int) -> phys_addr_t;
    fn rsnd_gen_get_base_addr(priv_: *mut rsnd_priv, reg_id: c_int) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn rsnd_is_gen1(priv_: *mut rsnd_priv) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
}

#[repr(C)]
pub struct seq_file {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct rsnd_dai_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_mod {
    pub ops: *mut rsnd_mod_ops,
    pub type_: c_int,
}

#[repr(C)]
pub struct rsnd_mod_ops {
    pub get_status:
        Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, c_int) -> *mut u32>,
    pub debug_info:
        Option<unsafe extern "C" fn(*mut seq_file, *mut rsnd_dai_stream, *mut rsnd_mod)>,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub debugfs_root: *mut dentry,
}

#[repr(C)]
pub struct rsnd_dai {
    pub playback: rsnd_dai_stream,
    pub capture: rsnd_dai_stream,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

pub type phys_addr_t = usize;

// C macro dependencies supplied by the original build:
// DEFINE_SHOW_ATTRIBUTE(rsnd_debugfs)
// for_each_rsnd_mod(i, mod, io)
// for_each_rsnd_dai(rdai, priv, i)
unsafe extern "C" {
    fn for_each_rsnd_mod(
        i: *mut c_int,
        mod_: *mut *mut rsnd_mod,
        io: *mut rsnd_dai_stream,
        body: unsafe extern "C" fn(*mut c_int, *mut *mut rsnd_mod, *mut rsnd_dai_stream),
    );
    fn for_each_rsnd_dai(
        rdai: *mut *mut rsnd_dai,
        priv_: *mut rsnd_priv,
        i: *mut c_int,
        body: unsafe extern "C" fn(*mut *mut rsnd_dai, *mut rsnd_priv, *mut c_int, *mut snd_soc_component),
        component: *mut snd_soc_component,
    );
}

unsafe extern "C" fn rsnd_debugfs_show_mod_body(
    i: *mut c_int,
    mod_: *mut *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
) {
    let _ = i;
    let status: *mut u32 =
        ((*(**mod_).ops).get_status).unwrap()(*mod_, io, (**mod_).type_);

    seq_printf(missing_seq_file(), c"name: %s\n".as_ptr(), rsnd_mod_name(*mod_));
    seq_printf(missing_seq_file(), c"status: %08x\n".as_ptr(), *status);

    if ((*(**mod_).ops).debug_info).is_some() {
        ((*(**mod_).ops).debug_info).unwrap()(missing_seq_file(), io, *mod_);
    }
}

static mut RSND_DEBUGFS_SHOW_SEQ_FILE: *mut seq_file = core::ptr::null_mut();

unsafe fn missing_seq_file() -> *mut seq_file {
    RSND_DEBUGFS_SHOW_SEQ_FILE
}

unsafe extern "C" fn rsnd_debugfs_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let _ = v;
    let io: *mut rsnd_dai_stream = (*m).private as *mut rsnd_dai_stream;
    let mut mod_: *mut rsnd_mod = rsnd_io_to_mod_ssi(io);
    let priv_: *mut rsnd_priv = rsnd_mod_to_priv(mod_);
    let mut i: c_int = 0;

    /* adg is out of mods */
    rsnd_adg_clk_dbg_info(priv_, m);

    RSND_DEBUGFS_SHOW_SEQ_FILE = m;
    for_each_rsnd_mod(&mut i, &mut mod_, io, rsnd_debugfs_show_mod_body);
    RSND_DEBUGFS_SHOW_SEQ_FILE = core::ptr::null_mut();

    return 0;
}

// DEFINE_SHOW_ATTRIBUTE(rsnd_debugfs);

#[no_mangle]
pub unsafe extern "C" fn rsnd_debugfs_reg_show(
    m: *mut seq_file,
    _addr: phys_addr_t,
    base: *mut c_void,
    offset: c_int,
    size: c_int,
) {
    let mut i: c_int = 0;
    let mut j: c_int;

    while i < size {
        let addr: phys_addr_t = _addr
            .wrapping_add(offset as phys_addr_t)
            .wrapping_add(i as phys_addr_t);

        seq_printf(m, c"%pa:".as_ptr(), &addr as *const phys_addr_t);
        j = 0;
        while j < 0x10 {
            seq_printf(
                m,
                c" %08x".as_ptr(),
                __raw_readl((base as *mut u8).offset((offset + i + j) as isize) as *const c_void),
            );
            j += 0x4;
        }
        seq_puts(m, c"\n".as_ptr());
        i += 0x10;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_debugfs_mod_reg_show(
    m: *mut seq_file,
    mod_: *mut rsnd_mod,
    reg_id: c_int,
    offset: c_int,
    size: c_int,
) {
    let priv_: *mut rsnd_priv = rsnd_mod_to_priv(mod_);

    rsnd_debugfs_reg_show(
        m,
        rsnd_gen_get_phy_addr(priv_, reg_id),
        rsnd_gen_get_base_addr(priv_, reg_id),
        offset,
        size,
    );
}

unsafe extern "C" fn rsnd_debugfs_probe_dai_body(
    rdai: *mut *mut rsnd_dai,
    _priv: *mut rsnd_priv,
    i: *mut c_int,
    component: *mut snd_soc_component,
) {
    let mut dir: *mut dentry;
    let mut name: [c_char; 64] = [0; 64];

    /*
     * created debugfs will be automatically
     * removed, nothing to do for _remove.
     * see
     *	soc_cleanup_component_debugfs()
     */
    snprintf(
        name.as_mut_ptr(),
        core::mem::size_of_val(&name),
        c"rdai%d".as_ptr(),
        *i,
    );
    dir = debugfs_create_dir(name.as_ptr(), (*component).debugfs_root);

    debugfs_create_file(
        c"playback".as_ptr(),
        0o444,
        dir,
        &mut (**rdai).playback as *mut rsnd_dai_stream as *mut c_void,
        &rsnd_debugfs_fops,
    );
    debugfs_create_file(
        c"capture".as_ptr(),
        0o444,
        dir,
        &mut (**rdai).capture as *mut rsnd_dai_stream as *mut c_void,
        &rsnd_debugfs_fops,
    );
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_debugfs_probe(component: *mut snd_soc_component) -> c_int {
    let priv_: *mut rsnd_priv = dev_get_drvdata((*component).dev) as *mut rsnd_priv;
    let mut rdai: *mut rsnd_dai = core::ptr::null_mut();
    let mut i: c_int = 0;

    /* Gen1 is not supported */
    if rsnd_is_gen1(priv_) != 0 {
        return 0;
    }

    for_each_rsnd_dai(
        &mut rdai,
        priv_,
        &mut i,
        rsnd_debugfs_probe_dai_body,
        component,
    );

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
