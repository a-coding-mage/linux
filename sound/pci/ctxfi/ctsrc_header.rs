/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctsrc.h
 *
 * @Brief
 * This file contains the definition of the Sample Rate Convertor
 * resource management object.
 *
 * @Author	Liu Chun
 * @Date 	May 13 2008
 */

/* Dependencies from the original header:
 * "ctresource.h", "ctimap.h", <linux/spinlock.h>, <linux/list.h>,
 * and <sound/core.h>.
 */

pub const SRC_STATE_OFF: u32 = 0x0;
pub const SRC_STATE_INIT: u32 = 0x4;
pub const SRC_STATE_RUN: u32 = 0x5;

pub const SRC_SF_U8: u32 = 0x0;
pub const SRC_SF_S16: u32 = 0x1;
pub const SRC_SF_S24: u32 = 0x2;
pub const SRC_SF_S32: u32 = 0x3;
pub const SRC_SF_F32: u32 = 0x4;

/* External dependency types supplied by translated headers. */
pub type rsc = crate::rsc;
pub type rsc_mgr = crate::rsc_mgr;
pub type imapper = crate::imapper;
pub type snd_card = crate::snd_card;
pub type spinlock_t = crate::spinlock_t;
pub type list_head = crate::list_head;
pub type hw = crate::hw;

/* Define the descriptor of a src resource */
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCMODE {
    MEMRD = 0, /* Read data from host memory */
    MEMWR = 1, /* Write data to host memory */
    ARCRW = 2, /* Read from and write to audio ring channel */
    NUM_SRCMODES = 3,
}

#[repr(C)]
pub struct src {
    pub rsc: rsc,                         /* Basic resource info */
    pub intlv: *mut src,                  /* Pointer to next interleaved SRC in a series */
    pub ops: *const src_rsc_ops,          /* SRC specific operations */
    pub multi: ::core::ffi::c_uchar,      /* Number of contiguous srcs for interleaved usage */
    pub mode: ::core::ffi::c_uchar,       /* Working mode of this SRC resource */
}

#[repr(C)]
pub struct src_rsc_ops {
    pub set_state: Option<unsafe extern "C" fn(src: *mut src, state: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_bm: Option<unsafe extern "C" fn(src: *mut src, bm: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_sf: Option<unsafe extern "C" fn(src: *mut src, sf: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_pm: Option<unsafe extern "C" fn(src: *mut src, pm: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_rom: Option<unsafe extern "C" fn(src: *mut src, rom: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_vo: Option<unsafe extern "C" fn(src: *mut src, vo: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_st: Option<unsafe extern "C" fn(src: *mut src, st: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_bp: Option<unsafe extern "C" fn(src: *mut src, bp: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_cisz: Option<unsafe extern "C" fn(src: *mut src, cisz: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_ca: Option<unsafe extern "C" fn(src: *mut src, ca: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_sa: Option<unsafe extern "C" fn(src: *mut src, sa: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_la: Option<unsafe extern "C" fn(src: *mut src, la: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_pitch: Option<unsafe extern "C" fn(src: *mut src, pitch: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_clr_zbufs: Option<unsafe extern "C" fn(src: *mut src) -> ::core::ffi::c_int>,
    pub commit_write: Option<unsafe extern "C" fn(src: *mut src) -> ::core::ffi::c_int>,
    pub get_ca: Option<unsafe extern "C" fn(src: *mut src) -> ::core::ffi::c_int>,
    pub init: Option<unsafe extern "C" fn(src: *mut src) -> ::core::ffi::c_int>,
    pub next_interleave: Option<unsafe extern "C" fn(src: *mut src) -> *mut src>,
}

/* Define src resource request description info */
#[repr(C)]
pub struct src_desc {
    pub multi: ::core::ffi::c_uchar, /* Number of contiguous master srcs for interleaved usage */
    pub msr: ::core::ffi::c_uchar,
    pub mode: ::core::ffi::c_uchar, /* Working mode of the requested srcs */
}

/* Define src manager object */
#[repr(C)]
pub struct src_mgr {
    pub mgr: rsc_mgr,              /* Basic resource manager info */
    pub card: *mut snd_card,       /* pointer to this card */
    pub mgr_lock: spinlock_t,
    pub get_src: Option<
        unsafe extern "C" fn(
            mgr: *mut src_mgr,
            desc: *const src_desc,
            rsrc: *mut *mut src,
        ) -> ::core::ffi::c_int,
    >, /* request src resource */
    pub put_src: Option<unsafe extern "C" fn(mgr: *mut src_mgr, src: *mut src) -> ::core::ffi::c_int>, /* return src resource */
    pub src_enable_s: Option<unsafe extern "C" fn(mgr: *mut src_mgr, src: *mut src) -> ::core::ffi::c_int>,
    pub src_enable: Option<unsafe extern "C" fn(mgr: *mut src_mgr, src: *mut src) -> ::core::ffi::c_int>,
    pub src_disable: Option<unsafe extern "C" fn(mgr: *mut src_mgr, src: *mut src) -> ::core::ffi::c_int>,
    pub commit_write: Option<unsafe extern "C" fn(mgr: *mut src_mgr) -> ::core::ffi::c_int>,
}

/* Define the descriptor of a SRC Input Mapper resource */
#[repr(C)]
pub struct srcimp {
    pub rsc: rsc,
    pub idx: [::core::ffi::c_uchar; 8],
    pub mapped: ::core::ffi::c_uint, /* A bit-map indicating which conj rsc is mapped */
    pub mgr: *mut srcimp_mgr,
    pub ops: *const srcimp_rsc_ops,
    pub imappers: [imapper; 0],
}

#[repr(C)]
pub struct srcimp_rsc_ops {
    pub map: Option<
        unsafe extern "C" fn(
            srcimp: *mut srcimp,
            user: *mut src,
            input: *mut rsc,
        ) -> ::core::ffi::c_int,
    >,
    pub unmap: Option<unsafe extern "C" fn(srcimp: *mut srcimp) -> ::core::ffi::c_int>,
}

/* Define SRCIMP resource request description info */
#[repr(C)]
pub struct srcimp_desc {
    pub msr: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct srcimp_mgr {
    pub mgr: rsc_mgr,        /* Basic resource manager info */
    pub card: *mut snd_card, /* pointer to this card */
    pub mgr_lock: spinlock_t,
    pub imap_lock: spinlock_t,
    pub imappers: list_head,
    pub init_imap: *mut imapper,
    pub init_imap_added: ::core::ffi::c_uint,
    pub get_srcimp: Option<
        unsafe extern "C" fn(
            mgr: *mut srcimp_mgr,
            desc: *const srcimp_desc,
            rsrcimp: *mut *mut srcimp,
        ) -> ::core::ffi::c_int,
    >, /* request srcimp resource */
    pub put_srcimp: Option<unsafe extern "C" fn(mgr: *mut srcimp_mgr, srcimp: *mut srcimp) -> ::core::ffi::c_int>, /* return srcimp resource */
    pub imap_add: Option<unsafe extern "C" fn(mgr: *mut srcimp_mgr, entry: *mut imapper) -> ::core::ffi::c_int>,
    pub imap_delete: Option<unsafe extern "C" fn(mgr: *mut srcimp_mgr, entry: *mut imapper) -> ::core::ffi::c_int>,
}

unsafe extern "C" {
    /* Constructor and destructor of SRC resource manager */
    pub fn src_mgr_create(hw: *mut hw, ptr: *mut *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn src_mgr_destroy(ptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;

    /* Constructor and destructor of SRCIMP resource manager */
    pub fn srcimp_mgr_create(hw: *mut hw, ptr: *mut *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn srcimp_mgr_destroy(ptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
