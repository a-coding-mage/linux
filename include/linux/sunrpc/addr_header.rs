/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of linux/include/linux/sunrpc/addr.h.
 * C includes and build-time configuration are supplied by surrounding code.
 */

extern "C" {
    pub fn rpc_ntop(
        sap: *const sockaddr,
        buf: *mut core::ffi::c_char,
        buflen: usize,
    ) -> usize;
    pub fn rpc_pton(
        net: *mut net,
        buf: *const core::ffi::c_char,
        buflen: usize,
        sap: *mut sockaddr,
        salen: usize,
    ) -> usize;
    pub fn rpc_sockaddr2uaddr(
        sap: *const sockaddr,
        gfp: gfp_t,
    ) -> *mut core::ffi::c_char;
    pub fn rpc_uaddr2sockaddr(
        net: *mut net,
        uaddr: *const core::ffi::c_char,
        uaddr_len: usize,
        sap: *mut sockaddr,
        salen: usize,
    ) -> usize;
}

pub const IPV6_SCOPE_DELIMITER: u8 = b'%';
pub const IPV6_SCOPE_ID_LEN: usize = core::mem::size_of::<[u8; 12]>();

#[inline]
pub unsafe fn rpc_get_port(sap: *const sockaddr) -> u16 {
    match (*sap).sa_family {
        AF_INET => u16::from_be((*((sap as *const sockaddr_in))).sin_port),
        AF_INET6 => u16::from_be((*((sap as *const sockaddr_in6))).sin6_port),
        _ => 0,
    }
}

#[inline]
pub unsafe fn rpc_set_port(sap: *mut sockaddr, port: u16) {
    match (*sap).sa_family {
        AF_INET => (*((sap as *mut sockaddr_in))).sin_port = port.to_be(),
        AF_INET6 => (*((sap as *mut sockaddr_in6))).sin6_port = port.to_be(),
        _ => {}
    }
}

#[inline]
pub unsafe fn rpc_cmp_addr4(sap1: *const sockaddr, sap2: *const sockaddr) -> bool {
    (*(sap1 as *const sockaddr_in)).sin_addr.s_addr
        == (*(sap2 as *const sockaddr_in)).sin_addr.s_addr
}

#[inline]
pub unsafe fn __rpc_copy_addr4(dst: *mut sockaddr, src: *const sockaddr) -> bool {
    let ssin = src as *const sockaddr_in;
    let dsin = dst as *mut sockaddr_in;
    (*dsin).sin_family = (*ssin).sin_family;
    (*dsin).sin_addr.s_addr = (*ssin).sin_addr.s_addr;
    true
}

/* Equivalent of #if IS_ENABLED(CONFIG_IPV6); IPv6 support is supplied by the build. */
#[inline]
pub unsafe fn rpc_cmp_addr6(sap1: *const sockaddr, sap2: *const sockaddr) -> bool {
    let sin1 = sap1 as *const sockaddr_in6;
    let sin2 = sap2 as *const sockaddr_in6;
    if !ipv6_addr_equal(&(*sin1).sin6_addr, &(*sin2).sin6_addr) {
        false
    } else if ipv6_addr_type(&(*sin1).sin6_addr) & IPV6_ADDR_LINKLOCAL != 0 {
        (*sin1).sin6_scope_id == (*sin2).sin6_scope_id
    } else {
        true
    }
}

#[inline]
pub unsafe fn __rpc_copy_addr6(dst: *mut sockaddr, src: *const sockaddr) -> bool {
    let ssin6 = src as *const sockaddr_in6;
    let dsin6 = dst as *mut sockaddr_in6;
    (*dsin6).sin6_family = (*ssin6).sin6_family;
    (*dsin6).sin6_addr = (*ssin6).sin6_addr;
    (*dsin6).sin6_scope_id = (*ssin6).sin6_scope_id;
    true
}

#[inline]
pub unsafe fn rpc_cmp_addr(sap1: *const sockaddr, sap2: *const sockaddr) -> bool {
    if (*sap1).sa_family == (*sap2).sa_family {
        match (*sap1).sa_family {
            AF_INET => rpc_cmp_addr4(sap1, sap2),
            AF_INET6 => rpc_cmp_addr6(sap1, sap2),
            _ => false,
        }
    } else {
        false
    }
}

#[inline]
pub unsafe fn rpc_cmp_addr_port(sap1: *const sockaddr, sap2: *const sockaddr) -> bool {
    rpc_cmp_addr(sap1, sap2) && rpc_get_port(sap1) == rpc_get_port(sap2)
}

#[inline]
pub unsafe fn rpc_copy_addr(dst: *mut sockaddr, src: *const sockaddr) -> bool {
    match (*src).sa_family {
        AF_INET => __rpc_copy_addr4(dst, src),
        AF_INET6 => __rpc_copy_addr6(dst, src),
        _ => false,
    }
}

#[inline]
pub unsafe fn rpc_get_scope_id(sa: *const sockaddr) -> u32 {
    if (*sa).sa_family != AF_INET6 {
        return 0;
    }
    (*(sa as *const sockaddr_in6)).sin6_scope_id
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
