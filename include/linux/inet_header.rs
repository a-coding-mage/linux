/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Swansea University Computer Society NET3
 *
 * This work is derived from NET2Debugged, which is in turn derived
 * from NET2D, and from Ross Biro's work for the LINUX operating system.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, net/net_namespace.h, and linux/socket.h.

/*
 * These mimic similar macros defined in user-space for inet_ntop(3).
 * See /usr/include/netinet/in.h.
 */
pub const INET_ADDRSTRLEN: usize = 16;
pub const INET6_ADDRSTRLEN: usize = 48;

extern "C" {
    pub fn in_aton(str_: *const ::core::ffi::c_char) -> __be32;
    pub fn in4_pton(
        src: *const ::core::ffi::c_char,
        srclen: ::core::ffi::c_int,
        dst: *mut u8,
        delim: ::core::ffi::c_int,
        end: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn in6_pton(
        src: *const ::core::ffi::c_char,
        srclen: ::core::ffi::c_int,
        dst: *mut u8,
        delim: ::core::ffi::c_int,
        end: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn inet_pton_with_scope(
        net: *mut net,
        af: u16,
        src: *const ::core::ffi::c_char,
        port: *const ::core::ffi::c_char,
        addr: *mut sockaddr_storage,
    ) -> ::core::ffi::c_int;
    pub fn inet_addr_is_any(addr: *mut sockaddr_storage) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
