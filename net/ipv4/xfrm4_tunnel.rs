// SPDX-License-Identifier: GPL-2.0-only
/* xfrm4_tunnel.c: Generic IP tunnel transformer.
 *
 * Copyright (C) 2003 David S. Miller (davem@redhat.com)
 */

// pr_fmt(fmt) expands to "IPsec: " followed by fmt.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static THIS_MODULE: *mut c_void;

    fn skb_push(skb: *mut sk_buff, len: isize) -> *mut c_void;
    fn skb_network_offset(skb: *const sk_buff) -> isize;
    fn ip_hdr(skb: *const sk_buff) -> *mut iphdr;
    fn xfrm4_rcv_spi(skb: *mut sk_buff, proto: u8, saddr: u32) -> c_int;
    fn xfrm_register_type(ty: *const xfrm_type, family: c_int) -> c_int;
    fn xfrm_unregister_type(ty: *const xfrm_type, family: c_int);
    fn xfrm4_tunnel_register(tunnel: *const xfrm_tunnel, family: c_int) -> c_int;
    fn xfrm4_tunnel_deregister(tunnel: *const xfrm_tunnel, family: c_int) -> c_int;
    fn pr_info(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iphdr {
    pub protocol: u8,
    pub saddr: u32,
}

#[repr(C)]
pub struct xfrm_state {
    pub props: xfrm_props,
    pub encap: *mut c_void,
}

#[repr(C)]
pub struct xfrm_props {
    pub mode: c_int,
    pub header_len: usize,
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_type {
    pub owner: *mut c_void,
    pub proto: u8,
    pub init_state: Option<unsafe extern "C" fn(*mut xfrm_state, *mut netlink_ext_ack) -> c_int>,
    pub destructor: Option<unsafe extern "C" fn(*mut xfrm_state)>,
    pub input: Option<unsafe extern "C" fn(*mut xfrm_state, *mut sk_buff) -> c_int>,
    pub output: Option<unsafe extern "C" fn(*mut xfrm_state, *mut sk_buff) -> c_int>,
}

#[repr(C)]
pub struct xfrm_tunnel {
    pub handler: Option<unsafe extern "C" fn(*mut sk_buff) -> c_int>,
    pub err_handler: Option<unsafe extern "C" fn(*mut sk_buff, c_uint) -> c_int>,
    pub priority: c_int,
}

const XFRM_MODE_TUNNEL: c_int = 1;
const IPPROTO_IPIP: u8 = 4;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EAGAIN: c_int = 11;

unsafe fn nl_set_err_msg(_extack: *mut netlink_ext_ack, _msg: *const c_char) {
    // NL_SET_ERR_MSG is supplied by the kernel dependency.
}

unsafe extern "C" fn ipip_output(_x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    skb_push(skb, -skb_network_offset(skb));
    0
}

unsafe extern "C" fn ipip_xfrm_rcv(_x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    (*ip_hdr(skb)).protocol as c_int
}

unsafe extern "C" fn ipip_init_state(
    x: *mut xfrm_state,
    extack: *mut netlink_ext_ack,
) -> c_int {
    if (*x).props.mode != XFRM_MODE_TUNNEL {
        nl_set_err_msg(extack, c"IPv4 tunnel can only be used with tunnel mode".as_ptr());
        return -EINVAL;
    }

    if !(*x).encap.is_null() {
        nl_set_err_msg(extack, c"IPv4 tunnel is not compatible with encapsulation".as_ptr());
        return -EINVAL;
    }

    (*x).props.header_len = core::mem::size_of::<iphdr>();
    0
}

unsafe extern "C" fn ipip_destroy(_x: *mut xfrm_state) {}

static IPIP_TYPE: xfrm_type = xfrm_type {
    owner: unsafe { THIS_MODULE },
    proto: IPPROTO_IPIP,
    init_state: Some(ipip_init_state),
    destructor: Some(ipip_destroy),
    input: Some(ipip_xfrm_rcv),
    output: Some(ipip_output),
};

unsafe extern "C" fn xfrm_tunnel_rcv(skb: *mut sk_buff) -> c_int {
    xfrm4_rcv_spi(skb, IPPROTO_IPIP, (*ip_hdr(skb)).saddr)
}

unsafe extern "C" fn xfrm_tunnel_err(_skb: *mut sk_buff, _info: c_uint) -> c_int {
    -ENOENT
}

static XFRM_TUNNEL_HANDLER: xfrm_tunnel = xfrm_tunnel {
    handler: Some(xfrm_tunnel_rcv),
    err_handler: Some(xfrm_tunnel_err),
    priority: 4,
};

// Preserved from #if IS_ENABLED(CONFIG_IPV6).
#[cfg(feature = "CONFIG_IPV6")]
static XFRM64_TUNNEL_HANDLER: xfrm_tunnel = xfrm_tunnel {
    handler: Some(xfrm_tunnel_rcv),
    err_handler: Some(xfrm_tunnel_err),
    priority: 3,
};

unsafe extern "C" fn ipip_init() -> c_int {
    if xfrm_register_type(&IPIP_TYPE, AF_INET) < 0 {
        pr_info(c"%s: can't add xfrm type\n".as_ptr(), c"ipip_init".as_ptr());
        return -EAGAIN;
    }

    if xfrm4_tunnel_register(&XFRM_TUNNEL_HANDLER, AF_INET) != 0 {
        pr_info(c"%s: can't add xfrm handler for AF_INET\n".as_ptr(), c"ipip_init".as_ptr());
        xfrm_unregister_type(&IPIP_TYPE, AF_INET);
        return -EAGAIN;
    }

    // Preserved from #if IS_ENABLED(CONFIG_IPV6).
    #[cfg(feature = "CONFIG_IPV6")]
    if xfrm4_tunnel_register(&XFRM64_TUNNEL_HANDLER, AF_INET6) != 0 {
        pr_info(c"%s: can't add xfrm handler for AF_INET6\n".as_ptr(), c"ipip_init".as_ptr());
        xfrm4_tunnel_deregister(&XFRM_TUNNEL_HANDLER, AF_INET);
        xfrm_unregister_type(&IPIP_TYPE, AF_INET);
        return -EAGAIN;
    }
    0
}

unsafe extern "C" fn ipip_fini() {
    // Preserved from #if IS_ENABLED(CONFIG_IPV6).
    #[cfg(feature = "CONFIG_IPV6")]
    if xfrm4_tunnel_deregister(&XFRM64_TUNNEL_HANDLER, AF_INET6) != 0 {
        pr_info(c"%s: can't remove xfrm handler for AF_INET6\n".as_ptr(), c"ipip_fini".as_ptr());
    }
    if xfrm4_tunnel_deregister(&XFRM_TUNNEL_HANDLER, AF_INET) != 0 {
        pr_info(c"%s: can't remove xfrm handler for AF_INET\n".as_ptr(), c"ipip_fini".as_ptr());
    }
    xfrm_unregister_type(&IPIP_TYPE, AF_INET);
}

// module_init(ipip_init);
// module_exit(ipip_fini);
// MODULE_DESCRIPTION("IPv4 XFRM tunnel driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_XFRM_TYPE(AF_INET, XFRM_PROTO_IPIP);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
