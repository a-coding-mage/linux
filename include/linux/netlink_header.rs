/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct net;

extern "C" {
    pub fn do_trace_netlink_extack(msg: *const ::core::ffi::c_char);
}

#[inline]
pub unsafe fn nlmsg_hdr(skb: *const sk_buff) -> *mut nlmsghdr {
    (*skb).data as *mut nlmsghdr
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum netlink_skb_flags {
    NETLINK_SKB_DST = 0x8,
}

#[repr(C)]
pub struct netlink_skb_parms {
    pub creds: scm_creds,
    pub portid: __u32,
    pub dst_group: __u32,
    pub flags: __u32,
    pub sk: *mut sock,
    pub nsid_is_set: bool,
    pub nsid: ::core::ffi::c_int,
}

#[macro_export]
macro_rules! NETLINK_CB {
    ($skb:expr) => {
        &mut *(($skb).cb.as_mut_ptr() as *mut netlink_skb_parms)
    };
}

#[macro_export]
macro_rules! NETLINK_CREDS {
    ($skb:expr) => {
        &mut $crate::NETLINK_CB!($skb).creds
    };
}

pub const NETLINK_CTX_SIZE: usize = 48;

extern "C" {
    pub fn netlink_table_grab();
    pub fn netlink_table_ungrab();
}

pub const NL_CFG_F_NONROOT_RECV: ::core::ffi::c_uint = 1 << 0;
pub const NL_CFG_F_NONROOT_SEND: ::core::ffi::c_uint = 1 << 1;

#[repr(C)]
pub struct netlink_kernel_cfg {
    pub groups: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub input: Option<unsafe extern "C" fn(skb: *mut sk_buff)>,
    pub bind: Option<unsafe extern "C" fn(net: *mut net, group: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub unbind: Option<unsafe extern "C" fn(net: *mut net, group: ::core::ffi::c_int)>,
    pub release: Option<unsafe extern "C" fn(sk: *mut sock, groups: *mut ::core::ffi::c_ulong)>,
}

extern "C" {
    pub fn __netlink_kernel_create(net: *mut net, unit: ::core::ffi::c_int, module: *mut module, cfg: *mut netlink_kernel_cfg) -> *mut sock;
}

#[inline]
pub unsafe fn netlink_kernel_create(net: *mut net, unit: ::core::ffi::c_int, cfg: *mut netlink_kernel_cfg) -> *mut sock {
    __netlink_kernel_create(net, unit, THIS_MODULE, cfg)
}

pub const NETLINK_MAX_COOKIE_LEN: usize = 8;
pub const NETLINK_MAX_FMTMSG_LEN: usize = 80;

#[repr(C)]
pub struct netlink_ext_ack {
    pub _msg: *const ::core::ffi::c_char,
    pub bad_attr: *const nlattr,
    pub policy: *const nla_policy,
    pub miss_nest: *const nlattr,
    pub miss_type: u16,
    pub cookie: [u8; NETLINK_MAX_COOKIE_LEN],
    pub cookie_len: u8,
    pub _msg_buf: [::core::ffi::c_char; NETLINK_MAX_FMTMSG_LEN],
}

#[macro_export]
macro_rules! NL_SET_ERR_MSG {
    ($extack:expr, $msg:expr) => {{
        static __MSG: &[u8] = concat!($msg, "\0").as_bytes();
        let __extack: *mut netlink_ext_ack = $extack;
        unsafe {
            do_trace_netlink_extack(__MSG.as_ptr() as *const ::core::ffi::c_char);
            if !__extack.is_null() { (*__extack)._msg = __MSG.as_ptr() as *const _; }
        }
    }};
}

#[macro_export]
macro_rules! NL_SET_ERR_MSG_WEAK {
    ($extack:expr, $msg:expr) => {{ unsafe { if !$extack.is_null() && (*$extack)._msg.is_null() { $crate::NL_SET_ERR_MSG!($extack, $msg); } } }};
}

#[macro_export]
macro_rules! NL_SET_BAD_ATTR_POLICY {
    ($extack:expr, $attr:expr, $pol:expr) => {{ unsafe { if !$extack.is_null() { (*$extack).bad_attr = $attr; (*$extack).policy = $pol; } } }};
}

#[macro_export]
macro_rules! NL_SET_BAD_ATTR {
    ($extack:expr, $attr:expr) => { $crate::NL_SET_BAD_ATTR_POLICY!($extack, $attr, core::ptr::null()) };
}

#[macro_export]
macro_rules! NL_SET_ERR_ATTR_POL {
    ($extack:expr, $attr:expr, $pol:expr, $msg:expr) => {{ $crate::NL_SET_ERR_MSG!($extack, $msg); unsafe { if !$extack.is_null() { (*$extack).bad_attr = $attr; (*$extack).policy = $pol; } } }};
}

#[macro_export]
macro_rules! NL_SET_ERR_ATTR {
    ($extack:expr, $attr:expr, $msg:expr) => { $crate::NL_SET_ERR_ATTR_POL!($extack, $attr, core::ptr::null(), $msg) };
}

#[macro_export]
macro_rules! NL_SET_ERR_ATTR_MISS {
    ($extack:expr, $nest:expr, $type_:expr) => {{ unsafe { if !$extack.is_null() { (*$extack).miss_nest = $nest; (*$extack).miss_type = $type_; } } }};
}

#[macro_export]
macro_rules! NL_REQ_ATTR_CHECK {
    ($extack:expr, $nest:expr, $tb:expr, $type_:expr) => {{ let __attr = $type_ as usize; let __retval = ($tb)[__attr].is_null(); if __retval { $crate::NL_SET_ERR_ATTR_MISS!($extack, $nest, $type_); } __retval }};
}

#[inline]
pub unsafe fn nl_set_extack_cookie_u64(extack: *mut netlink_ext_ack, cookie: u64) {
    if extack.is_null() { return; }
    core::ptr::copy_nonoverlapping((&cookie as *const u64).cast::<u8>(), (*extack).cookie.as_mut_ptr(), core::mem::size_of::<u64>());
    (*extack).cookie_len = core::mem::size_of::<u64>() as u8;
}

extern "C" {
    pub fn netlink_kernel_release(sk: *mut sock);
    pub fn __netlink_change_ngroups(sk: *mut sock, groups: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn netlink_change_ngroups(sk: *mut sock, groups: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn __netlink_clear_multicast_users(sk: *mut sock, group: ::core::ffi::c_uint);
    pub fn netlink_ack(in_skb: *mut sk_buff, nlh: *mut nlmsghdr, err: ::core::ffi::c_int, extack: *const netlink_ext_ack);
    pub fn netlink_has_listeners(sk: *mut sock, group: ::core::ffi::c_uint) -> bool;
    pub fn netlink_strict_get_check(skb: *mut sk_buff) -> bool;
    pub fn netlink_unicast(ssk: *mut sock, skb: *mut sk_buff, portid: __u32, nonblock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn netlink_broadcast(ssk: *mut sock, skb: *mut sk_buff, portid: __u32, group: __u32, allocation: gfp_t) -> ::core::ffi::c_int;
}

pub type netlink_filter_fn = Option<unsafe extern "C" fn(dsk: *mut sock, skb: *mut sk_buff, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int>;

extern "C" {
    pub fn netlink_broadcast_filtered(ssk: *mut sock, skb: *mut sk_buff, portid: __u32, group: __u32, allocation: gfp_t, filter: netlink_filter_fn, filter_data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn netlink_set_err(ssk: *mut sock, portid: __u32, group: __u32, code: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn netlink_register_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn netlink_unregister_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn netlink_getsockbyfd(fd: ::core::ffi::c_int) -> *mut sock;
    pub fn netlink_attachskb(sk: *mut sock, skb: *mut sk_buff, timeo: *mut ::core::ffi::c_long, ssk: *mut sock) -> ::core::ffi::c_int;
    pub fn netlink_detachskb(sk: *mut sock, skb: *mut sk_buff);
    pub fn netlink_sendskb(sk: *mut sock, skb: *mut sk_buff) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn netlink_skb_clone(skb: *mut sk_buff, gfp_mask: gfp_t) -> *mut sk_buff {
    let nskb = skb_clone(skb, gfp_mask);
    if nskb.is_null() { return core::ptr::null_mut(); }
    if is_vmalloc_addr((*skb).head) { (*nskb).destructor = (*skb).destructor; }
    nskb
}

// skb should fit one page; cap the value at 8K for userspace recvmsg buffers.
#[cfg(target_pointer_width = "64")]
pub const NLMSG_GOODSIZE: usize = if PAGE_SIZE < 8192 { SKB_WITH_OVERHEAD(PAGE_SIZE) } else { SKB_WITH_OVERHEAD(8192) };
pub const NLMSG_DEFAULT_SIZE: usize = NLMSG_GOODSIZE - NLMSG_HDRLEN;

#[repr(C)]
pub struct netlink_callback {
    pub skb: *mut sk_buff,
    pub nlh: *const nlmsghdr,
    pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut netlink_callback) -> ::core::ffi::c_int>,
    pub done: Option<unsafe extern "C" fn(*mut netlink_callback) -> ::core::ffi::c_int>,
    pub data: *mut ::core::ffi::c_void,
    pub module: *mut module,
    pub extack: *mut netlink_ext_ack,
    pub family: u16,
    pub answer_flags: u16,
    pub min_dump_alloc: u32,
    pub prev_seq: ::core::ffi::c_uint,
    pub seq: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_int,
    pub strict_check: bool,
    pub ctx_or_args: [u8; NETLINK_CTX_SIZE],
}

#[repr(C)]
pub struct netlink_notify { pub net: *mut net, pub portid: u32, pub protocol: ::core::ffi::c_int }

extern "C" {
    pub fn __nlmsg_put(skb: *mut sk_buff, portid: u32, seq: u32, type_: ::core::ffi::c_int, len: ::core::ffi::c_int, flags: ::core::ffi::c_int) -> *mut nlmsghdr;
}

#[repr(C)]
pub struct netlink_dump_control {
    pub start: Option<unsafe extern "C" fn(*mut netlink_callback) -> ::core::ffi::c_int>,
    pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut netlink_callback) -> ::core::ffi::c_int>,
    pub done: Option<unsafe extern "C" fn(*mut netlink_callback) -> ::core::ffi::c_int>,
    pub extack: *mut netlink_ext_ack,
    pub data: *mut ::core::ffi::c_void,
    pub module: *mut module,
    pub min_dump_alloc: u32,
    pub flags: ::core::ffi::c_int,
}

extern "C" {
    pub fn __netlink_dump_start(ssk: *mut sock, skb: *mut sk_buff, nlh: *const nlmsghdr, control: *mut netlink_dump_control) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn netlink_dump_start(ssk: *mut sock, skb: *mut sk_buff, nlh: *const nlmsghdr, control: *mut netlink_dump_control) -> ::core::ffi::c_int {
    if (*control).module.is_null() { (*control).module = THIS_MODULE; }
    __netlink_dump_start(ssk, skb, nlh, control)
}

#[repr(C)]
pub struct netlink_tap { pub dev: *mut net_device, pub module: *mut module, pub list: list_head }

extern "C" {
    pub fn netlink_add_tap(nt: *mut netlink_tap) -> ::core::ffi::c_int;
    pub fn netlink_remove_tap(nt: *mut netlink_tap) -> ::core::ffi::c_int;
    pub fn __netlink_ns_capable(nsp: *const netlink_skb_parms, ns: *mut user_namespace, cap: ::core::ffi::c_int) -> bool;
    pub fn netlink_ns_capable(skb: *const sk_buff, ns: *mut user_namespace, cap: ::core::ffi::c_int) -> bool;
    pub fn netlink_capable(skb: *const sk_buff, cap: ::core::ffi::c_int) -> bool;
    pub fn netlink_net_capable(skb: *const sk_buff, cap: ::core::ffi::c_int) -> bool;
    pub fn netlink_alloc_large_skb(size: ::core::ffi::c_uint, broadcast: ::core::ffi::c_int) -> *mut sk_buff;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
