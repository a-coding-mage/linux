// SPDX-License-Identifier: GPL-2.0-only
/*
 * 802_3
 *
 * Author:
 * Chris Vitale csv@bluetail.com
 *
 * May 2003
 *
 */

// C dependencies supplied by the kernel headers are intentionally left as
// external Rust declarations.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct sk_buff {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const c_void,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const c_void,
}

#[repr(C)]
pub struct ebt_802_3_info {
    pub bitmask: u8,
    pub invflags: u8,
    pub sap: u8,
    pub type_: u16,
}

#[repr(C)]
pub struct ebt_802_3_hdr {
    pub llc: ebt_802_3_llc,
}

#[repr(C)]
pub union ebt_802_3_llc {
    pub ui: ebt_802_3_llc_ui,
    pub ni: ebt_802_3_llc_ni,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_802_3_llc_ui {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u8,
    pub type_: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_802_3_llc_ni {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u8,
    pub type_: u16,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const c_char,
    pub revision: u8,
    pub family: u16,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> c_int>,
    pub matchsize: usize,
    pub me: *mut c_void,
}

const EBT_802_3_SAP: u8 = 0x01;
const EBT_802_3_TYPE: u8 = 0x02;
const EBT_802_3_MASK: u8 = EBT_802_3_SAP | EBT_802_3_TYPE;
const IS_UI: u8 = 0x03;
const CHECK_TYPE: u8 = 0xaa;
const NFPROTO_BRIDGE: u16 = 7;
const EINVAL: c_int = 22;

extern "C" {
    fn skb_mac_header(skb: *const sk_buff) -> *mut c_void;
    fn xt_register_match(m: *mut xt_match) -> c_int;
    fn xt_unregister_match(m: *mut xt_match);
}

static mut ebt_802_3_mt_reg: xt_match = xt_match {
    name: b"802_3\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_BRIDGE,
    match_: Some(ebt_802_3_mt),
    checkentry: Some(ebt_802_3_mt_check),
    matchsize: core::mem::size_of::<ebt_802_3_info>(),
    me: core::ptr::null_mut(),
};

unsafe fn ebt_802_3_hdr(skb: *const sk_buff) -> *const ebt_802_3_hdr {
    skb_mac_header(skb) as *const ebt_802_3_hdr
}

unsafe extern "C" fn ebt_802_3_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info = (*par).matchinfo as *const ebt_802_3_info;
    let hdr = ebt_802_3_hdr(skb);
    let ui = (*hdr).llc.ui;
    let ni = (*hdr).llc.ni;
    let ty = if ui.ctrl & IS_UI != 0 { ui.type_ } else { ni.type_ };

    if (*info).bitmask & EBT_802_3_SAP != 0 {
        if (*info).sap != ui.ssap {
            return false;
        }
        if (*info).sap != ui.dsap {
            return false;
        }
    }

    if (*info).bitmask & EBT_802_3_TYPE != 0 {
        if !(ui.dsap == CHECK_TYPE && ui.ssap == CHECK_TYPE) {
            return false;
        }
        if (*info).type_ != ty {
            return false;
        }
    }

    true
}

unsafe extern "C" fn ebt_802_3_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *const ebt_802_3_info;

    if (*info).bitmask & !EBT_802_3_MASK != 0
        || (*info).invflags & !EBT_802_3_MASK != 0
    {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn ebt_802_3_init() -> c_int {
    xt_register_match(&raw mut ebt_802_3_mt_reg)
}

unsafe extern "C" fn ebt_802_3_fini() {
    xt_unregister_match(&raw mut ebt_802_3_mt_reg);
}

// module_init(ebt_802_3_init);
// module_exit(ebt_802_3_fini);
// MODULE_DESCRIPTION("Ebtables: DSAP/SSAP field and SNAP type matching");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
