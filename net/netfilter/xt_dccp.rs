// SPDX-License-Identifier: GPL-2.0-only
/*
 * iptables module for DCCP protocol header matching
 *
 * (C) 2005 by Harald Welte <laforge@netfilter.org>
 */

// Kernel dependencies supplied by other translation units.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Harald Welte <laforge@netfilter.org>");
// MODULE_DESCRIPTION("Xtables: DCCP protocol packet match");
// MODULE_ALIAS("ipt_dccp");
// MODULE_ALIAS("ip6t_dccp");

#[inline]
fn dccheck(cond: bool, option: u32, flag: u32, invflag: u32) -> bool {
    !((flag & option) != 0) || (((invflag & option) != 0) ^ cond)
}

static mut dccp_optbuf: *mut u8 = core::ptr::null_mut();
// DEFINE_SPINLOCK(dccp_buflock);
static mut dccp_buflock: Spinlock = Spinlock;

// External kernel types and functions are supplied by other files.
extern "C" {
    fn __dccp_hdr_len(dh: *const dccp_hdr) -> u32;
    fn spin_lock_bh(lock: *mut Spinlock);
    fn spin_unlock_bh(lock: *mut Spinlock);
    fn skb_header_pointer(
        skb: *const sk_buff,
        offset: u32,
        length: u32,
        buffer: *mut core::ffi::c_void,
    ) -> *const u8;
    fn ntohs(value: u16) -> u16;
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn xt_register_matches(matches: *mut xt_match, count: usize) -> i32;
    fn xt_unregister_matches(matches: *mut xt_match, count: usize);
    fn pr_warn_once(format: *const u8, ...);
}

#[repr(C)]
struct Spinlock;
#[repr(C)]
struct sk_buff;
#[repr(C)]
struct dccp_hdr {
    dccph_sport: u16,
    dccph_dport: u16,
    dccph_doff: u8,
    dccph_type: u8,
}
#[repr(C)]
struct xt_dccp_info {
    spts: [u16; 2],
    dpts: [u16; 2],
    typemask: u16,
    option: u8,
    flags: u8,
    invflags: u8,
}
#[repr(C)]
struct xt_action_param {
    matchinfo: *const core::ffi::c_void,
    fragoff: u16,
    thoff: u32,
    hotdrop: bool,
}
#[repr(C)]
struct xt_mtchk_param {
    matchinfo: *const core::ffi::c_void,
}
#[repr(C)]
struct xt_match {
    name: *const u8,
    family: u16,
    checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    r#match: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    matchsize: usize,
    proto: u8,
    me: *mut core::ffi::c_void,
}

const XT_DCCP_SRC_PORTS: u32 = 1;
const XT_DCCP_DEST_PORTS: u32 = 2;
const XT_DCCP_TYPE: u32 = 4;
const XT_DCCP_OPTION: u32 = 8;
const XT_DCCP_VALID_FLAGS: u32 = XT_DCCP_SRC_PORTS | XT_DCCP_DEST_PORTS | XT_DCCP_TYPE | XT_DCCP_OPTION;
const NFPROTO_IPV4: u16 = 2;
const NFPROTO_IPV6: u16 = 10;
const IPPROTO_DCCP: u8 = 33;
const GFP_KERNEL: u32 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

#[inline]
unsafe fn dccp_find_option(option: u8, skb: *const sk_buff, protoff: u32,
                            dh: *const dccp_hdr, hotdrop: *mut bool) -> bool {
    let optoff = __dccp_hdr_len(dh);
    let optlen = ((*dh).dccph_doff as u32).wrapping_mul(4).wrapping_sub(optoff);
    if ((*dh).dccph_doff as u32).wrapping_mul(4) < optoff { *hotdrop = true; return false; }
    if optlen == 0 { return false; }
    spin_lock_bh(&raw mut dccp_buflock);
    let op = skb_header_pointer(skb, protoff + optoff, optlen, dccp_optbuf.cast());
    if op.is_null() { spin_unlock_bh(&raw mut dccp_buflock); *hotdrop = true; return false; }
    let mut i = 0;
    while i < optlen {
        if *op.add(i as usize) == option { spin_unlock_bh(&raw mut dccp_buflock); return true; }
        if *op.add(i as usize) < 2 || i == optlen - 1 { i += 1; }
        else { i += if *op.add((i + 1) as usize) != 0 { *op.add((i + 1) as usize) as u32 } else { 1 }; }
    }
    spin_unlock_bh(&raw mut dccp_buflock); false
}

#[inline]
unsafe fn match_types(dh: *const dccp_hdr, typemask: u16) -> bool { (typemask & (1u16 << (*dh).dccph_type)) != 0 }

#[inline]
unsafe fn match_option(option: u8, skb: *const sk_buff, protoff: u32, dh: *const dccp_hdr, hotdrop: *mut bool) -> bool {
    dccp_find_option(option, skb, protoff, dh, hotdrop)
}

unsafe extern "C" fn dccp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    if (*par).fragoff != 0 { return false; }
    let info = (*par).matchinfo as *const xt_dccp_info;
    let mut _dh = core::mem::MaybeUninit::<dccp_hdr>::uninit();
    let dh = skb_header_pointer(skb, (*par).thoff, core::mem::size_of::<dccp_hdr>() as u32, _dh.as_mut_ptr().cast()) as *const dccp_hdr;
    if dh.is_null() { (*par).hotdrop = true; return false; }
    dccheck(ntohs((*dh).dccph_sport) >= (*info).spts[0] && ntohs((*dh).dccph_sport) <= (*info).spts[1], XT_DCCP_SRC_PORTS, (*info).flags as u32, (*info).invflags as u32)
        && dccheck(ntohs((*dh).dccph_dport) >= (*info).dpts[0] && ntohs((*dh).dccph_dport) <= (*info).dpts[1], XT_DCCP_DEST_PORTS, (*info).flags as u32, (*info).invflags as u32)
        && dccheck(match_types(dh, (*info).typemask), XT_DCCP_TYPE, (*info).flags as u32, (*info).invflags as u32)
        && dccheck(match_option((*info).option, skb, (*par).thoff, dh, &raw mut (*par).hotdrop), XT_DCCP_OPTION, (*info).flags as u32, (*info).invflags as u32)
}

unsafe extern "C" fn dccp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_dccp_info;
    if ((*info).flags as u32 & !XT_DCCP_VALID_FLAGS) != 0 || ((*info).invflags as u32 & !XT_DCCP_VALID_FLAGS) != 0 || ((*info).invflags & !(*info).flags) != 0 { return -EINVAL; }
    0
}

static mut dccp_mt_reg: [xt_match; 2] = [
    xt_match { name: b"dccp\0".as_ptr(), family: NFPROTO_IPV4, checkentry: Some(dccp_mt_check), r#match: Some(dccp_mt), matchsize: core::mem::size_of::<xt_dccp_info>(), proto: IPPROTO_DCCP, me: core::ptr::null_mut() },
    xt_match { name: b"dccp\0".as_ptr(), family: NFPROTO_IPV6, checkentry: Some(dccp_mt_check), r#match: Some(dccp_mt), matchsize: core::mem::size_of::<xt_dccp_info>(), proto: IPPROTO_DCCP, me: core::ptr::null_mut() },
];

unsafe extern "C" fn dccp_mt_init() -> i32 {
    dccp_optbuf = kmalloc(256 * 4, GFP_KERNEL);
    if dccp_optbuf.is_null() { return -ENOMEM; }
    let ret = xt_register_matches(dccp_mt_reg.as_mut_ptr(), dccp_mt_reg.len());
    if ret != 0 { kfree(dccp_optbuf); }
    ret
}

unsafe extern "C" fn dccp_mt_exit() {
    xt_unregister_matches(dccp_mt_reg.as_mut_ptr(), dccp_mt_reg.len());
    kfree(dccp_optbuf);
}

// module_init(dccp_mt_init);
// module_exit(dccp_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
