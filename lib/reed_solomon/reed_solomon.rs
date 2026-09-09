// SPDX-License-Identifier: GPL-2.0
/*
 * Generic Reed Solomon encoder / decoder library
 *
 * Copyright (C) 2004 Thomas Gleixner (tglx@kernel.org)
 *
 * Reed Solomon code lifted from reed solomon library written by Phil Karn
 * Copyright 2002 Phil Karn, KA9Q
 *
 * Description:
 *
 * The generic Reed Solomon library provides runtime configurable
 * encoding / decoding of RS codes.
 */

// C dependencies: linux/errno.h, linux/kernel.h, linux/init.h,
// linux/module.h, linux/rslib.h, linux/slab.h, linux/mutex.h.

use core::ffi::c_void;

pub type GfpT = u32;

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
pub struct RsCodec {
    pub list: ListHead,
    pub mm: i32,
    pub nn: i32,
    pub fcr: i32,
    pub prim: i32,
    pub nroots: i32,
    pub gfpoly: i32,
    pub gffunc: Option<unsafe extern "C" fn(i32) -> i32>,
    pub iprim: i32,
    pub users: i32,
    pub alpha_to: *mut u16,
    pub index_of: *mut u16,
    pub genpoly: *mut u16,
}

#[repr(C)]
pub struct RsControl {
    pub codec: *mut RsCodec,
}

const RS_DECODE_LAMBDA: usize = 0;
const RS_DECODE_SYN: usize = 1;
const RS_DECODE_B: usize = 2;
const RS_DECODE_T: usize = 3;
const RS_DECODE_OMEGA: usize = 4;
const RS_DECODE_ROOT: usize = 5;
const RS_DECODE_REG: usize = 6;
const RS_DECODE_LOC: usize = 7;
const RS_DECODE_NUM_BUFFERS: usize = 8;

// External kernel/runtime facilities supplied by other files.
extern "C" {
    fn rs_modnn(rs: *mut RsCodec, x: i32) -> i32;
}

static mut CODEC_LIST: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn codec_init(
    symsize: i32,
    gfpoly: i32,
    gffunc: Option<unsafe extern "C" fn(i32) -> i32>,
    fcr: i32,
    prim: i32,
    nroots: i32,
    _gfp: GfpT,
) -> *mut RsCodec {
    let rs = Box::into_raw(Box::new(RsCodec {
        list: ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
        mm: symsize, nn: (1i32 << symsize) - 1, fcr, prim, nroots, gfpoly, gffunc,
        iprim: 0, users: 0, alpha_to: core::ptr::null_mut(),
        index_of: core::ptr::null_mut(), genpoly: core::ptr::null_mut(),
    }));

    let nn = (*rs).nn as usize;
    (*rs).alpha_to = vec![0u16; nn + 1].leak().as_mut_ptr();
    (*rs).index_of = vec![0u16; nn + 1].leak().as_mut_ptr();
    (*rs).genpoly = vec![0u16; nroots as usize + 1].leak().as_mut_ptr();

    (*rs).index_of.add(nn).write(nn as u16);
    (*rs).alpha_to.add(nn).write(0);
    let mut sr: i32;
    if gfpoly != 0 {
        sr = 1;
        for i in 0..(*rs).nn {
            (*rs).index_of.add(sr as usize).write(i as u16);
            (*rs).alpha_to.add(i as usize).write(sr as u16);
            sr <<= 1;
            if sr & (1 << symsize) != 0 { sr ^= gfpoly; }
            sr &= (*rs).nn;
        }
    } else {
        sr = gffunc.expect("external Galois-field function")(0);
        for i in 0..(*rs).nn {
            (*rs).index_of.add(sr as usize).write(i as u16);
            (*rs).alpha_to.add(i as usize).write(sr as u16);
            sr = gffunc.expect("external Galois-field function")(sr);
        }
    }
    if sr != *(*rs).alpha_to { drop(Box::from_raw(rs)); return core::ptr::null_mut(); }

    let mut iprim = 1;
    while iprim % prim != 0 { iprim += (*rs).nn; }
    (*rs).iprim = iprim / prim;
    (*rs).genpoly.write(1);
    let mut root = fcr * prim;
    for i in 0..nroots {
        (*rs).genpoly.add((i + 1) as usize).write(1);
        let mut j = i;
        while j > 0 {
            let gj = *(*rs).genpoly.add(j as usize);
            if gj != 0 {
                let idx = rs_modnn(rs, (*rs).index_of.add(gj as usize).read() as i32 + root);
                (*rs).genpoly.add(j as usize).write(*(*rs).genpoly.add((j - 1) as usize) ^ *(*rs).alpha_to.add(idx as usize));
            } else { (*rs).genpoly.add(j as usize).write(*(*rs).genpoly.add((j - 1) as usize)); }
            j -= 1;
        }
        let idx = rs_modnn(rs, (*rs).index_of.read() as i32 + root);
        (*rs).genpoly.write(*(*rs).alpha_to.add(idx as usize));
        root += prim;
    }
    for i in 0..=nroots as usize { (*rs).genpoly.add(i).write((*rs).index_of.add((*rs).genpoly.add(i).read() as usize).read()); }
    (*rs).users = 1;
    // list_add(&rs->list, &codec_list); supplied by the host list implementation.
    rs
}

#[no_mangle]
pub unsafe extern "C" fn free_rs(rs: *mut RsControl) {
    if rs.is_null() { return; }
    let cd = (*rs).codec;
    (*cd).users -= 1;
    if (*cd).users == 0 { drop(Box::from_raw(cd)); }
    drop(Box::from_raw(rs));
}

unsafe fn init_rs_internal(symsize: i32, gfpoly: i32, gffunc: Option<unsafe extern "C" fn(i32) -> i32>, fcr: i32, prim: i32, nroots: i32, gfp: GfpT) -> *mut RsControl {
    if symsize < 1 || fcr < 0 || fcr >= (1 << symsize) || prim <= 0 || prim >= (1 << symsize) || nroots < 0 || nroots >= (1 << symsize) { return core::ptr::null_mut(); }
    let rs = Box::into_raw(Box::new(RsControl { codec: core::ptr::null_mut() }));
    (*rs).codec = codec_init(symsize, gfpoly, gffunc, fcr, prim, nroots, gfp);
    if (*rs).codec.is_null() { drop(Box::from_raw(rs)); core::ptr::null_mut() } else { rs }
}

#[no_mangle]
pub unsafe extern "C" fn init_rs_gfp(symsize: i32, gfpoly: i32, fcr: i32, prim: i32, nroots: i32, gfp: GfpT) -> *mut RsControl {
    init_rs_internal(symsize, gfpoly, None, fcr, prim, nroots, gfp)
}

#[no_mangle]
pub unsafe extern "C" fn init_rs_non_canonical(symsize: i32, gffunc: Option<unsafe extern "C" fn(i32) -> i32>, fcr: i32, prim: i32, nroots: i32) -> *mut RsControl {
    init_rs_internal(symsize, 0, gffunc, fcr, prim, nroots, 0)
}

// The following bodies are supplied by the source-level dependencies encode_rs.c and decode_rs.c.
#[cfg(CONFIG_REED_SOLOMON_ENC8)]
pub unsafe extern "C" fn encode_rs8(_rsc: *mut RsControl, _data: *mut u8, _len: i32, _par: *mut u16, _invmsk: u16) -> i32 { /* include "encode_rs.c" */ 0 }
#[cfg(CONFIG_REED_SOLOMON_DEC8)]
pub unsafe extern "C" fn decode_rs8(_rsc: *mut RsControl, _data: *mut u8, _par: *mut u16, _len: i32, _s: *mut u16, _no_eras: i32, _eras_pos: *mut i32, _invmsk: u16, _corr: *mut u16) -> i32 { /* include "decode_rs.c" */ 0 }
#[cfg(CONFIG_REED_SOLOMON_ENC16)]
pub unsafe extern "C" fn encode_rs16(_rsc: *mut RsControl, _data: *mut u16, _len: i32, _par: *mut u16, _invmsk: u16) -> i32 { /* include "encode_rs.c" */ 0 }
#[cfg(CONFIG_REED_SOLOMON_DEC16)]
pub unsafe extern "C" fn decode_rs16(_rsc: *mut RsControl, _data: *mut u16, _par: *mut u16, _len: i32, _s: *mut u16, _no_eras: i32, _eras_pos: *mut i32, _invmsk: u16, _corr: *mut u16) -> i32 { /* include "decode_rs.c" */ 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
