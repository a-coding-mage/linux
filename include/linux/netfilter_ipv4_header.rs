/* IPv4-specific defines for netfilter.
 * (C)1998 Rusty Russell -- This code is GPL.
 */

// Dependency: <uapi/linux/netfilter_ipv4.h>

/* Extra routing may needed on local out, as the QUEUE target never returns
 * control to the table.
 */
#[repr(C)]
pub struct ip_rt_info {
    pub daddr: __be32,
    pub saddr: __be32,
    pub tos: u8,
    pub mark: u32,
}

pub unsafe extern "C" fn ip_route_me_harder(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
    addr_type: ::core::ffi::c_uint,
) -> ::core::ffi::c_int;

pub struct nf_queue_entry;

#[cfg(CONFIG_INET)]
pub unsafe extern "C" fn nf_ip_checksum(
    skb: *mut sk_buff,
    hook: ::core::ffi::c_uint,
    dataoff: ::core::ffi::c_uint,
    protocol: u8,
) -> __sum16;

#[cfg(CONFIG_INET)]
pub unsafe extern "C" fn nf_ip_route(
    net: *mut net,
    dst: *mut *mut dst_entry,
    fl: *mut flowi,
    strict: bool,
) -> ::core::ffi::c_int;

#[cfg(not(CONFIG_INET))]
pub unsafe fn nf_ip_checksum(
    _skb: *mut sk_buff,
    _hook: ::core::ffi::c_uint,
    _dataoff: ::core::ffi::c_uint,
    _protocol: u8,
) -> __sum16 {
    0
}

#[cfg(not(CONFIG_INET))]
pub unsafe fn nf_ip_route(
    _net: *mut net,
    _dst: *mut *mut dst_entry,
    _fl: *mut flowi,
    _strict: bool,
) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
