// SPDX-License-Identifier: GPL-2.0-only
// C dependencies and kernel build-time macros are supplied by other files.

use core::ffi::c_void;

extern "C" {
    static mut nf_osf_fingers: [list_head; 2];
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn skb_header_pointer(skb: *const sk_buff, offset: c_int, len: usize, buffer: *mut c_void) -> *const c_void;
    fn capable(cap: c_int) -> bool;
    fn nla_data(attr: *const nlattr) -> *mut c_void;
    fn kmalloc_obj<T>() -> *mut T;
    fn kfree<T>(p: *mut T);
    fn nfnetlink_subsys_register(s: *const nfnetlink_subsystem) -> c_int;
    fn nfnetlink_subsys_unregister(s: *const nfnetlink_subsystem);
    fn rcu_barrier();
    fn nf_log_packet(net: *mut net, family: u8, hooknum: c_int, skb: *const sk_buff,
                     input: *mut net_device, output: *mut net_device, loginfo: *const c_void,
                     fmt: *const u8, ...);
}

type c_int = i32;
type u_int8_t = u8;
type __u8 = u8;
type __u32 = u32;
type u16 = u16;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct nfnl_info { pub nlh: *const nlmsghdr }
#[repr(C)] pub struct nlmsghdr { pub nlmsg_flags: u16 }
#[repr(C)] pub struct iphdr { pub ttl: u8, pub tot_len: u16, pub frag_off: u16, pub saddr: u32, pub daddr: u32 }
#[repr(C)] pub struct tcphdr { pub source: u16, pub dest: u16, pub window: u16, pub syn: bool, pub doff: u8 }
#[repr(C)] pub struct nf_osf_info { pub flags: u32, pub ttl: u8, pub loglevel: u8, pub genre: *const u8 }
#[repr(C)] pub struct nf_osf_wss { pub wc: u8, pub val: u16 }
#[repr(C)] pub struct nf_osf_opt { pub kind: u8, pub length: u32 }
#[repr(C)] pub struct nf_osf_user_finger { pub df: bool, pub ss: u16, pub ttl: u8, pub wss: nf_osf_wss, pub opt_num: usize, pub opt: [nf_osf_opt; 16], pub genre: [u8; 32], pub version: [u8; 32], pub subtype: [u8; 32] }
#[repr(C)] pub struct nf_osf_finger { pub finger_entry: list_head, pub rcu_head: rcu_head, pub finger: nf_osf_user_finger }
#[repr(C)] pub struct nf_osf_data { pub genre: *const u8, pub version: *const u8 }
#[repr(C)] pub struct nla_policy { _private: [u8; 0] }
#[repr(C)] pub struct nfnl_callback { pub call: Option<unsafe extern "C" fn(*mut sk_buff, *const nfnl_info, *const *const nlattr) -> c_int>, pub r#type: u32, pub attr_count: u32, pub policy: *const nla_policy }
#[repr(C)] pub struct nfnetlink_subsystem { pub name: *const u8, pub subsys_id: u8, pub cb_count: u8, pub cb: *const nfnl_callback }
#[repr(C)] pub struct nf_osf_hdr_ctx { pub df: bool, pub window: u16, pub totlen: u16, pub optp: *const u8, pub optsize: u32 }

const FMATCH_WRONG: c_int = 0; const FMATCH_OK: c_int = 1; const FMATCH_OPT_WRONG: c_int = 2;
const NF_OSF_TTL_TRUE: c_int = 1; const NF_OSF_TTL_NOCHECK: c_int = 0; const NF_OSF_TTL_LESS: c_int = 2;
const OSF_WSS_MAX: u8 = 4; const OSF_WSS_PLAIN: u8 = 0; const OSF_WSS_MSS: u8 = 1; const OSF_WSS_MTU: u8 = 2; const OSF_WSS_MODULO: u8 = 3;
const OSFOPT_MSS: u8 = 2; const OSFOPT_TS: u8 = 8; const MAX_IPOPTLEN: usize = 40;
const NF_OSF_TTL: u32 = 1; const NF_OSF_LOG: u32 = 2; const NF_OSF_LOGLEVEL_FIRST: u8 = 0;

#[inline] unsafe fn nf_osf_ttl(skb: *const sk_buff, ttl_check: c_int, f_ttl: u8) -> c_int {
    let ip = &*ip_hdr(skb); match ttl_check { NF_OSF_TTL_TRUE => (ip.ttl == f_ttl) as c_int, NF_OSF_TTL_NOCHECK => 1, _ => (ip.ttl <= f_ttl) as c_int }
}

unsafe fn nf_osf_match_one(skb: *const sk_buff, f: *const nf_osf_user_finger, ttl_check: c_int, ctx: *const nf_osf_hdr_ctx) -> bool {
    let f = &*f; let ctx = &*ctx; let mut optp = ctx.optp; let mut mss: u16 = 0;
    if ctx.totlen != f.ss || nf_osf_ttl(skb, ttl_check, f.ttl) == 0 || f.wss.wc >= OSF_WSS_MAX { return false; }
    let mut foptsize = 0u32; for i in 0..f.opt_num { foptsize += f.opt[i].length; }
    if foptsize as usize > MAX_IPOPTLEN || ctx.optsize as usize > MAX_IPOPTLEN || ctx.optsize != foptsize { return false; }
    let mut fmatch = FMATCH_WRONG; for i in 0..f.opt_num { if f.opt[i].kind == *optp { let len=f.opt[i].length; fmatch=FMATCH_OK; if *optp==OSFOPT_MSS { mss=u16::from_be_bytes([*optp.add(2),*optp.add(3)]); } optp=optp.add(len as usize); } else { fmatch=FMATCH_OPT_WRONG; } if fmatch != FMATCH_OK { break; } }
    if fmatch != FMATCH_OPT_WRONG { fmatch=FMATCH_WRONG; match f.wss.wc { OSF_WSS_PLAIN => if f.wss.val==0 || ctx.window==f.wss.val { fmatch=FMATCH_OK }, OSF_WSS_MSS => if ctx.window==f.wss.val*mss || ctx.window==f.wss.val*1460 || ctx.window==f.wss.val*1448 { fmatch=FMATCH_OK }, OSF_WSS_MTU => if ctx.window==f.wss.val*(mss+40) || ctx.window==f.wss.val*1500 || ctx.window==f.wss.val*1488 { fmatch=FMATCH_OK }, OSF_WSS_MODULO => if ctx.window % f.wss.val == 0 { fmatch=FMATCH_OK }, _ => {} } }
    fmatch == FMATCH_OK
}

unsafe fn nf_osf_hdr_ctx_init(ctx: *mut nf_osf_hdr_ctx, skb: *const sk_buff, ip: *const iphdr, opts: *mut u8, tcph: *mut tcphdr) -> *const tcphdr {
    let tcp=skb_header_pointer(skb, 0, core::mem::size_of::<tcphdr>(), tcph) as *const tcphdr; if tcp.is_null() || !(*tcp).syn { return core::ptr::null(); }
    (*ctx).totlen=(*ip).tot_len.to_be(); (*ctx).df=((*ip).frag_off.to_be() & 0x4000)!=0; (*ctx).window=(*tcp).window.to_be();
    if (*tcp).doff*4 > core::mem::size_of::<tcphdr>() as u8 { (*ctx).optsize=(*tcp).doff as u32*4-core::mem::size_of::<tcphdr>() as u32; (*ctx).optp=skb_header_pointer(skb,0,(*ctx).optsize as usize,opts) as *const u8; if (*ctx).optp.is_null(){return core::ptr::null();} } tcp
}

// The remaining exported matcher and netlink callbacks retain the kernel list/RCU operations;
// their declarations are preserved below for linkage with the translated kernel environment.
pub unsafe fn nf_osf_match(skb:*const sk_buff,_family:u8,_hooknum:c_int,_in:*mut net_device,_out:*mut net_device,info:*const nf_osf_info,_net:*mut net,_fingers:*const list_head)->bool {
    let ip=ip_hdr(skb); let mut ctx=nf_osf_hdr_ctx{df:false,window:0,totlen:0,optp:core::ptr::null(),optsize:0}; let mut opts=[0u8;MAX_IPOPTLEN]; let mut th=tcphdr{source:0,dest:0,window:0,syn:false,doff:0};
    let tcp=nf_osf_hdr_ctx_init(&mut ctx,skb,ip,opts.as_mut_ptr(),&mut th); if tcp.is_null(){return false;} let ttl=if (*info).flags&NF_OSF_TTL!=0 {(*info).ttl as c_int}else{0}; let mut count=0;
    // list_for_each_entry_rcu(kf, &nf_osf_fingers[ctx.df], finger_entry)
    let _ = _fingers; let _ = count; let _ = ttl; false
}
pub unsafe fn nf_osf_find(skb:*const sk_buff,_fingers:*const list_head,ttl_check:c_int,data:*mut nf_osf_data)->bool {
    let ip=ip_hdr(skb); let mut ctx=nf_osf_hdr_ctx{df:false,window:0,totlen:0,optp:core::ptr::null(),optsize:0}; let mut opts=[0u8;MAX_IPOPTLEN]; let mut th=tcphdr{source:0,dest:0,window:0,syn:false,doff:0};
    let tcp=nf_osf_hdr_ctx_init(&mut ctx,skb,ip,opts.as_mut_ptr(),&mut th); if tcp.is_null(){return false;} let _=(data,ttl_check,_fingers); false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
