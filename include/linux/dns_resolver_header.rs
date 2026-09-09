/*
 *   DNS Resolver upcall management for CIFS DFS and AFS
 *   Handles host name to IP address resolution and DNS query for AFSDB RR.
 *
 *   Copyright (c) International Business Machines  Corp., 2008
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *              Wang Lei (wang840925@gmail.com)
 *
 *   This library is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Lesser General Public License as published by
 *   the Free Software Foundation; either version 2.1 of the License, or
 *   (at your option) any later version.
 *
 *   This library is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
 *   the GNU Lesser General Public License for more details.
 *
 *   You should have received a copy of the GNU Lesser General Public License
 *   along with this library; if not, write to the Free Software
 *   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
 */

// Dependency intent preserved from: <uapi/linux/dns_resolver.h>

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

pub type size_t = usize;
pub type time64_t = i64;

extern "C" {
    pub fn dns_query(
        net: *mut net,
        type_: *const core::ffi::c_char,
        name: *const core::ffi::c_char,
        namelen: size_t,
        options: *const core::ffi::c_char,
        _result: *mut *mut core::ffi::c_char,
        _expiry: *mut time64_t,
        invalidate: bool,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
