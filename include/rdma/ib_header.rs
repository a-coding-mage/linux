/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2010 Intel Corporation.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to the original Linux includes.

#[repr(C)]
pub union ib_addr_union {
    pub uib_addr8: [__u8; 16],
    pub uib_addr16: [__be16; 8],
    pub uib_addr32: [__be32; 4],
    pub uib_addr64: [__be64; 2],
}

#[repr(C)]
pub struct ib_addr {
    pub ib_u: ib_addr_union,
}

impl ib_addr {
    #[inline]
    pub unsafe fn sib_addr8(&self) -> &[__u8; 16] { &self.ib_u.uib_addr8 }
    #[inline]
    pub unsafe fn sib_addr16(&self) -> &[__be16; 8] { &self.ib_u.uib_addr16 }
    #[inline]
    pub unsafe fn sib_addr32(&self) -> &[__be32; 4] { &self.ib_u.uib_addr32 }
    #[inline]
    pub unsafe fn sib_addr64(&self) -> &[__be64; 2] { &self.ib_u.uib_addr64 }
    #[inline]
    pub unsafe fn sib_raw(&self) -> &[__u8; 16] { &self.ib_u.uib_addr8 }
    #[inline]
    pub unsafe fn sib_subnet_prefix(&self) -> __be64 { self.ib_u.uib_addr64[0] }
    #[inline]
    pub unsafe fn sib_interface_id(&self) -> __be64 { self.ib_u.uib_addr64[1] }
}

#[inline]
pub unsafe fn ib_addr_any(a: *const ib_addr) -> bool {
    ((*a).ib_u.uib_addr64[0] | (*a).ib_u.uib_addr64[1]) == 0
}

#[inline]
pub unsafe fn ib_addr_loopback(a: *const ib_addr) -> bool {
    ((*a).ib_u.uib_addr32[0] |
     (*a).ib_u.uib_addr32[1] |
     (*a).ib_u.uib_addr32[2] |
     ((*a).ib_u.uib_addr32[3] ^ htonl(1))) == 0
}

#[inline]
pub unsafe fn ib_addr_set(addr: *mut ib_addr,
                          w1: __be32, w2: __be32, w3: __be32, w4: __be32) {
    (*addr).ib_u.uib_addr32[0] = w1;
    (*addr).ib_u.uib_addr32[1] = w2;
    (*addr).ib_u.uib_addr32[2] = w3;
    (*addr).ib_u.uib_addr32[3] = w4;
}

#[inline]
pub unsafe fn ib_addr_cmp(a1: *const ib_addr, a2: *const ib_addr) -> core::ffi::c_int {
    memcmp(a1 as *const core::ffi::c_void,
           a2 as *const core::ffi::c_void,
           core::mem::size_of::<ib_addr>())
}

#[repr(C)]
pub struct sockaddr_ib {
    pub sib_family: core::ffi::c_ushort, /* AF_IB */
    pub sib_pkey: __be16,
    pub sib_flowinfo: __be32,
    pub sib_addr: ib_addr,
    pub sib_sid: __be64,
    pub sib_sid_mask: __be64,
    pub sib_scope_id: __u64,
}

/*
 * The IB interfaces that use write() as bi-directional ioctl() are
 * fundamentally unsafe, since there are lots of ways to trigger "write()"
 * calls from various contexts with elevated privileges. That includes the
 * traditional suid executable error message writes, but also various kernel
 * interfaces that can write to file descriptors.
 *
 * This function provides protection for the legacy API by restricting the
 * calling context.
 */
#[inline]
pub unsafe fn ib_safe_file_access(filp: *const file) -> bool {
    (*filp).f_cred == current_cred()
}

extern "C" {
    fn htonl(hostlong: core::ffi::c_uint) -> __be32;
    fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> core::ffi::c_int;
    fn current_cred() -> *const cred;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
