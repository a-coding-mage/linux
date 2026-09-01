/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctdaio.h
 *
 * @Brief
 * This file contains the definition of Digital Audio Input Output
 * resource management object.
 *
 * @Author	Liu Chun
 * @Date 	May 23 2008
 */

/* Dependencies from the original header:
 * "ctresource.h", "ctimap.h", <linux/spinlock.h>, <linux/list.h>,
 * and <sound/core.h>.
 */

/* Define the descriptor of a daio resource */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DAIOTYP {
    LINEO1 = 0,
    LINEO2 = 1,
    LINEO3 = 2,
    LINEO4 = 3,
    SPDIFOO = 4, /* S/PDIF Out (Flexijack/Optical) */
    LINEIM = 5,
    SPDIFIO = 6, /* S/PDIF In (Flexijack/Optical) on the card */
    MIC = 7, /* Dedicated mic on Titanium HD */
    RCA = 8, /* Dedicated RCA on SE-300PCIE */
    SPDIFI_BAY = 9, /* S/PDIF In on internal drive bay */
    NUM_DAIOTYP = 10,
}

#[repr(C)]
pub struct daio {
    pub rscl: rsc, /* Basic resource info for left TX/RX */
    pub rscr: rsc, /* Basic resource info for right TX/RX */
    pub type_: DAIOTYP,
    pub output: ::core::ffi::c_uchar,
}

#[repr(C)]
pub struct dao {
    pub daio: daio,
    pub ops: *const dao_rsc_ops, /* DAO specific operations */
    pub imappers: *mut *mut imapper,
    pub mgr: *mut daio_mgr,
    pub hw: *mut hw,
    pub ctrl_blk: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct dai {
    pub daio: daio,
    pub ops: *const dai_rsc_ops, /* DAI specific operations */
    pub hw: *mut hw,
    pub ctrl_blk: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct dao_desc {
    /* C bitfields: msr:4, passthru:1 */
    pub bitfields: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dao_rsc_ops {
    pub set_spos: Option<unsafe extern "C" fn(dao: *mut dao, spos: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub commit_write: Option<unsafe extern "C" fn(dao: *mut dao) -> ::core::ffi::c_int>,
    pub get_spos: Option<unsafe extern "C" fn(dao: *mut dao, spos: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub reinit: Option<unsafe extern "C" fn(dao: *mut dao, desc: *const dao_desc) -> ::core::ffi::c_int>,
    pub set_left_input: Option<unsafe extern "C" fn(dao: *mut dao, input: *mut rsc) -> ::core::ffi::c_int>,
    pub set_right_input: Option<unsafe extern "C" fn(dao: *mut dao, input: *mut rsc) -> ::core::ffi::c_int>,
    pub clear_left_input: Option<unsafe extern "C" fn(dao: *mut dao) -> ::core::ffi::c_int>,
    pub clear_right_input: Option<unsafe extern "C" fn(dao: *mut dao) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct dai_rsc_ops {
    pub set_srt_srcl: Option<unsafe extern "C" fn(dai: *mut dai, src: *mut rsc) -> ::core::ffi::c_int>,
    pub set_srt_srcr: Option<unsafe extern "C" fn(dai: *mut dai, src: *mut rsc) -> ::core::ffi::c_int>,
    pub set_srt_msr: Option<unsafe extern "C" fn(dai: *mut dai, msr: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_enb_src: Option<unsafe extern "C" fn(dai: *mut dai, enb: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub set_enb_srt: Option<unsafe extern "C" fn(dai: *mut dai, enb: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub commit_write: Option<unsafe extern "C" fn(dai: *mut dai) -> ::core::ffi::c_int>,
}

/* Define daio resource request description info */
#[repr(C)]
pub struct daio_desc {
    /* C bitfields: type:4, msr:4, passthru:1, output:1 */
    pub bitfields: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct daio_mgr {
    pub mgr: rsc_mgr, /* Basic resource manager info */
    pub card: *mut snd_card, /* pointer to this card */
    pub mgr_lock: spinlock_t,
    pub imap_lock: spinlock_t,
    pub imappers: list_head,
    pub init_imap: *mut imapper,
    pub init_imap_added: ::core::ffi::c_uint,

    /* request one daio resource */
    pub get_daio: Option<
        unsafe extern "C" fn(
            mgr: *mut daio_mgr,
            desc: *const daio_desc,
            rdaio: *mut *mut daio,
        ) -> ::core::ffi::c_int,
    >,
    /* return one daio resource */
    pub put_daio: Option<unsafe extern "C" fn(mgr: *mut daio_mgr, daio: *mut daio) -> ::core::ffi::c_int>,
    pub daio_enable: Option<unsafe extern "C" fn(mgr: *mut daio_mgr, daio: *mut daio) -> ::core::ffi::c_int>,
    pub daio_disable: Option<unsafe extern "C" fn(mgr: *mut daio_mgr, daio: *mut daio) -> ::core::ffi::c_int>,
    pub imap_add: Option<unsafe extern "C" fn(mgr: *mut daio_mgr, entry: *mut imapper) -> ::core::ffi::c_int>,
    pub imap_delete: Option<unsafe extern "C" fn(mgr: *mut daio_mgr, entry: *mut imapper) -> ::core::ffi::c_int>,
    pub commit_write: Option<unsafe extern "C" fn(mgr: *mut daio_mgr) -> ::core::ffi::c_int>,
}

/* Constructor and destructor of daio resource manager */
extern "C" {
    pub fn daio_mgr_create(hw: *mut hw, ptr: *mut *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn daio_mgr_destroy(ptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
