/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
 * Copyright (C) 2020, Oracle.
 */

/* Translated from the C header guard: _NET_SUNRPC_SOCKLIB_H_. */

extern "C" {
    pub fn csum_partial_copy_to_xdr(
        xdr: *mut xdr_buf,
        skb: *mut sk_buff,
    ) -> ::std::os::raw::c_int;

    pub fn xprt_sock_sendmsg(
        sock: *mut socket,
        msg: *mut msghdr,
        xdr: *mut xdr_buf,
        base: ::std::os::raw::c_uint,
        marker: rpc_fraghdr,
        sent_p: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
