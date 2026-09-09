// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2007 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel dependencies supplied by other translation units.

extern "C" {
    fn gen_estimator_read(est: *const gen_estimator, sample: *mut gnet_stats_rate_est64);
    fn xt_rateest_lookup(net: *mut net, name: *const u8) -> *mut xt_rateest;
    fn xt_rateest_put(net: *mut net, est: *mut xt_rateest);
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct gen_estimator { _private: [u8; 0] }
#[repr(C)]
pub struct xt_rateest { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnet_stats_rate_est64 {
    pub bps: u64,
    pub pps: u64,
}

#[repr(C)]
pub struct xt_action_param { pub matchinfo: *mut core::ffi::c_void }
#[repr(C)]
pub struct xt_mtchk_param { pub net: *mut net, pub matchinfo: *mut core::ffi::c_void }
#[repr(C)]
pub struct xt_mtdtor_param { pub net: *mut net, pub matchinfo: *mut core::ffi::c_void }

#[repr(C)]
pub struct xt_rateest_match_info {
    pub name1: [u8; 32],
    pub name2: [u8; 32],
    pub flags: u32,
    pub mode: u32,
    pub bps1: u64,
    pub pps1: u64,
    pub bps2: u64,
    pub pps2: u64,
    pub est1: *mut xt_rateest,
    pub est2: *mut xt_rateest,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub revision: u8,
    pub family: u16,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    pub matchsize: usize,
    pub usersize: usize,
    pub me: *mut core::ffi::c_void,
}

const XT_RATEEST_MATCH_DELTA: u32 = 1 << 0;
const XT_RATEEST_MATCH_ABS: u32 = 1 << 1;
const XT_RATEEST_MATCH_REL: u32 = 1 << 2;
const XT_RATEEST_MATCH_BPS: u32 = 1 << 3;
const XT_RATEEST_MATCH_PPS: u32 = 1 << 4;
const XT_RATEEST_MATCH_INVERT: u32 = 1 << 5;
const XT_RATEEST_MATCH_LT: u32 = 0;
const XT_RATEEST_MATCH_GT: u32 = 1;
const XT_RATEEST_MATCH_EQ: u32 = 2;
const NFPROTO_UNSPEC: u16 = 0;

unsafe extern "C" fn xt_rateest_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let _ = skb;
    let info = &*((*par).matchinfo as *const xt_rateest_match_info);
    let mut sample = gnet_stats_rate_est64 { bps: 0, pps: 0 };
    let (mut bps1, mut bps2, mut pps1, mut pps2);
    let mut ret = true;

    gen_estimator_read(info.est1 as *const gen_estimator, &mut sample);
    if info.flags & XT_RATEEST_MATCH_DELTA != 0 {
        bps1 = if info.bps1 >= sample.bps { info.bps1 - sample.bps } else { 0 };
        pps1 = if info.pps1 >= sample.pps { info.pps1 - sample.pps } else { 0 };
    } else { bps1 = sample.bps; pps1 = sample.pps; }

    if info.flags & XT_RATEEST_MATCH_ABS != 0 {
        bps2 = info.bps2; pps2 = info.pps2;
    } else {
        gen_estimator_read(info.est2 as *const gen_estimator, &mut sample);
        if info.flags & XT_RATEEST_MATCH_DELTA != 0 {
            bps2 = if info.bps2 >= sample.bps { info.bps2 - sample.bps } else { 0 };
            pps2 = if info.pps2 >= sample.pps { info.pps2 - sample.pps } else { 0 };
        } else { bps2 = sample.bps; pps2 = sample.pps; }
    }

    match info.mode {
        XT_RATEEST_MATCH_LT => {
            if info.flags & XT_RATEEST_MATCH_BPS != 0 { ret &= bps1 < bps2; }
            if info.flags & XT_RATEEST_MATCH_PPS != 0 { ret &= pps1 < pps2; }
        },
        XT_RATEEST_MATCH_GT => {
            if info.flags & XT_RATEEST_MATCH_BPS != 0 { ret &= bps1 > bps2; }
            if info.flags & XT_RATEEST_MATCH_PPS != 0 { ret &= pps1 > pps2; }
        },
        XT_RATEEST_MATCH_EQ => {
            if info.flags & XT_RATEEST_MATCH_BPS != 0 { ret &= bps1 == bps2; }
            if info.flags & XT_RATEEST_MATCH_PPS != 0 { ret &= pps1 == pps2; }
        },
        _ => {}
    }
    ret ^= (info.flags & XT_RATEEST_MATCH_INVERT != 0);
    ret
}

unsafe extern "C" fn xt_rateest_mt_checkentry(par: *const xt_mtchk_param) -> i32 {
    let info = &mut *((*par).matchinfo as *mut xt_rateest_match_info);
    let mut ret = -22;
    let est1;
    let mut est2 = core::ptr::null_mut();
    let valid_names = |n: &[u8; 32]| n.iter().position(|&c| c == 0).unwrap_or(32) < 32;
    if (info.flags & (XT_RATEEST_MATCH_ABS | XT_RATEEST_MATCH_REL)).count_ones() != 1 ||
       info.flags & (XT_RATEEST_MATCH_BPS | XT_RATEEST_MATCH_PPS) == 0 { return ret; }
    if info.mode != XT_RATEEST_MATCH_EQ && info.mode != XT_RATEEST_MATCH_LT && info.mode != XT_RATEEST_MATCH_GT { return ret; }
    if !valid_names(&info.name1) || !valid_names(&info.name2) { return -36; }
    ret = -2;
    est1 = xt_rateest_lookup((*par).net, info.name1.as_ptr());
    if est1.is_null() { return ret; }
    if info.flags & XT_RATEEST_MATCH_REL != 0 {
        est2 = xt_rateest_lookup((*par).net, info.name2.as_ptr());
        if est2.is_null() { xt_rateest_put((*par).net, est1); return ret; }
    }
    info.est1 = est1; info.est2 = est2; 0
}

unsafe extern "C" fn xt_rateest_mt_destroy(par: *const xt_mtdtor_param) {
    let info = &*((*par).matchinfo as *const xt_rateest_match_info);
    xt_rateest_put((*par).net, info.est1);
    if !info.est2.is_null() { xt_rateest_put((*par).net, info.est2); }
}

#[no_mangle]
pub static mut xt_rateest_mt_reg: xt_match = xt_match {
    name: b"rateest\0".as_ptr(),
    revision: 0,
    family: NFPROTO_UNSPEC,
    match_: Some(xt_rateest_mt),
    checkentry: Some(xt_rateest_mt_checkentry),
    destroy: Some(xt_rateest_mt_destroy),
    matchsize: core::mem::size_of::<xt_rateest_match_info>(),
    usersize: core::mem::offset_of!(xt_rateest_match_info, est1),
    me: core::ptr::null_mut(),
};

#[no_mangle]
pub unsafe extern "C" fn xt_rateest_mt_init() -> i32 {
    xt_register_match(&mut xt_rateest_mt_reg)
}

#[no_mangle]
pub unsafe extern "C" fn xt_rateest_mt_fini() {
    xt_unregister_match(&mut xt_rateest_mt_reg);
}

// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("xtables rate estimator match");
// MODULE_ALIAS("ipt_rateest");
// MODULE_ALIAS("ip6t_rateest");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
