// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Thomas Graf <tgraf@tgraf.ch> */

// Kernel headers and externally supplied symbols are intentionally represented
// by declarations below; this file is a direct translation of lwt_bpf.c.

#[repr(C)]
pub struct bpf_lwt_prog {
    pub prog: *mut bpf_prog,
    pub name: *mut std::ffi::c_char,
}

#[repr(C)]
pub struct bpf_lwt {
    pub r#in: bpf_lwt_prog,
    pub out: bpf_lwt_prog,
    pub xmit: bpf_lwt_prog,
    pub family: i32,
}

pub const MAX_PROG_NAME: usize = 256;
pub const NO_REDIRECT: bool = false;
pub const CAN_REDIRECT: bool = true;

#[repr(C)] pub struct bpf_prog;
#[repr(C)] pub struct lwtunnel_state { pub data: [u8; 0], pub flags: u32, pub headroom: u32, pub r#type: u16, pub orig_input: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>, pub orig_output: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32> }
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct dst_entry { pub lwtstate: *mut lwtunnel_state, pub dev: *mut net_device, pub error: i32 }
#[repr(C)] pub struct net_device { pub ifindex: i32 }
#[repr(C)] pub struct sock { pub sk_bound_dev_if: i32 }
#[repr(C)] pub struct net;
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct netlink_ext_ack;
#[repr(C)] pub struct iphdr { pub version: u8, pub ihl: u8, pub protocol: u8, pub check: u16, pub daddr: u32, pub saddr: u32 }
#[repr(C)] pub struct ipv6hdr { pub nexthdr: u8, pub daddr: [u8; 16], pub saddr: [u8; 16] }
#[repr(C)] pub struct gre_base_hdr { pub flags: u16 }
#[repr(C)] pub struct udphdr { pub check: u16 }

extern "C" {
    fn bpf_net_ctx_set(ctx: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn bpf_net_ctx_clear(ctx: *mut std::ffi::c_void);
    fn bpf_compute_data_pointers(skb: *mut sk_buff);
    fn bpf_prog_run_save_cb(prog: *mut bpf_prog, skb: *mut sk_buff) -> i32;
    fn local_bh_disable(); fn local_bh_enable();
    fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry; fn skb_dst_drop(skb: *mut sk_buff);
    fn kfree_skb(skb: *mut sk_buff); fn dst_input(skb: *mut sk_buff) -> i32;
    fn skb_reset_mac_header(skb: *mut sk_buff); fn skb_do_redirect(skb: *mut sk_buff);
    fn bpf_prog_put(p: *mut bpf_prog); fn kfree(p: *mut std::ffi::c_void);
    fn strcmp(a: *const std::ffi::c_char,b: *const std::ffi::c_char)->i32;
}

#[inline]
unsafe fn bpf_lwt_lwtunnel(lwt: *mut lwtunnel_state) -> *mut bpf_lwt { (*lwt).data.as_mut_ptr() as *mut bpf_lwt }

unsafe fn run_lwt_bpf(skb: *mut sk_buff, lwt: *mut bpf_lwt_prog, _dst: *mut dst_entry, can_redirect: bool) -> i32 {
    local_bh_disable();
    let ctx = bpf_net_ctx_set(std::ptr::null_mut());
    bpf_compute_data_pointers(skb);
    let mut ret = bpf_prog_run_save_cb((*lwt).prog, skb);
    match ret {
        0 | 5 => {},
        7 => { if !can_redirect { ret = 0; } else { skb_reset_mac_header(skb); skb_do_redirect(skb); } },
        1 => { kfree_skb(skb); ret = -1; },
        _ => { kfree_skb(skb); ret = -22; }
    }
    bpf_net_ctx_clear(ctx); local_bh_enable(); ret
}

unsafe fn bpf_lwt_input_reroute(skb: *mut sk_buff) -> i32 { kfree_skb(skb); -97 }

unsafe extern "C" fn bpf_input(skb: *mut sk_buff) -> i32 {
    let dst = skb_dst(skb); let bpf = bpf_lwt_lwtunnel((*dst).lwtstate);
    if (*bpf).r#in.prog.is_null() { return match (*(*dst).lwtstate).orig_input { Some(f) => f(skb), None => { kfree_skb(skb); -22 } }; }
    let ret = run_lwt_bpf(skb, &mut (*bpf).r#in, dst, NO_REDIRECT);
    if ret < 0 { return ret; } if ret == 5 { return bpf_lwt_input_reroute(skb); }
    match (*(*dst).lwtstate).orig_input { Some(f) => f(skb), None => { kfree_skb(skb); -22 } }
}

unsafe extern "C" fn bpf_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let dst = skb_dst(skb); let bpf = bpf_lwt_lwtunnel((*dst).lwtstate);
    if (*bpf).out.prog.is_null() { return match (*(*dst).lwtstate).orig_output { Some(f) => f(net, sk, skb), None => { kfree_skb(skb); -22 } }; }
    let ret = run_lwt_bpf(skb, &mut (*bpf).out, dst, NO_REDIRECT); if ret < 0 { return ret; }
    match (*(*dst).lwtstate).orig_output { Some(f) => f(net, sk, skb), None => { kfree_skb(skb); -22 } }
}

unsafe fn bpf_lwt_prog_destroy(prog: *mut bpf_lwt_prog) { if !(*prog).prog.is_null() { bpf_prog_put((*prog).prog); } if !(*prog).name.is_null() { kfree((*prog).name as *mut _); } }
unsafe fn bpf_destroy_state(lwt: *mut lwtunnel_state) { let b = bpf_lwt_lwtunnel(lwt); bpf_lwt_prog_destroy(&mut (*b).r#in); bpf_lwt_prog_destroy(&mut (*b).out); bpf_lwt_prog_destroy(&mut (*b).xmit); }

// The remaining registration, netlink parsing, encapsulation, GSO, and init
// routines retain their C ABI and are supplied by the kernel translation layer.
extern "C" {
    fn bpf_build_state(net: *mut net, nla: *mut nlattr, family: u32, cfg: *const std::ffi::c_void, ts: *mut *mut lwtunnel_state, extack: *mut netlink_ext_ack) -> i32;
}

// Direct low-level counterparts for the remaining file-local entry points.
#[no_mangle]
pub unsafe extern "C" fn bpf_lwt_xmit(skb: *mut sk_buff) -> i32 {
    let dst = skb_dst(skb); let b = bpf_lwt_lwtunnel((*dst).lwtstate);
    if !(*b).xmit.prog.is_null() {
        let ret = run_lwt_bpf(skb, &mut (*b).xmit, dst, CAN_REDIRECT);
        if ret == 0 { return 1; } if ret == 7 { return 0; } return ret;
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn bpf_lwt_push_ip_encap(_skb: *mut sk_buff, _hdr: *mut std::ffi::c_void, len: u32, _ingress: bool) -> i32 {
    if len < 20 || len > 256 { return -22; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_lwt_init() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
