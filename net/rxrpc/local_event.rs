// SPDX-License-Identifier: GPL-2.0-or-later
/* AF_RXRPC local endpoint management
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency intent preserved from the C implementation:
// Linux module, networking, socket, AF_RXRPC, generated release, and
// ar-internal declarations are supplied by other translation units.

static mut RXRPC_VERSION_STRING: [core::ffi::c_char; 65] = [0; 65];

/*
 * Generate the VERSION packet string.
 */
pub unsafe fn rxrpc_gen_version_string() {
    snprintf(
        RXRPC_VERSION_STRING.as_mut_ptr(),
        core::mem::size_of::<[core::ffi::c_char; 65]>(),
        c"linux-%.49s AF_RXRPC".as_ptr(),
        UTS_RELEASE,
    );
}

/*
 * Reply to a version request
 */
pub unsafe fn rxrpc_send_version_request(
    local: *mut rxrpc_local,
    hdr: *mut rxrpc_host_header,
    skb: *mut sk_buff,
) {
    let mut whdr: rxrpc_wire_header = core::mem::zeroed();
    let sp: *mut rxrpc_skb_priv = rxrpc_skb(skb);
    let mut srx: sockaddr_rxrpc = core::mem::zeroed();
    let mut msg: msghdr = core::mem::zeroed();
    let mut iov: [kvec; 2] = core::mem::zeroed();
    let len: usize;
    let ret: i32;

    _enter(c"");

    if rxrpc_extract_addr_from_skb(&mut srx, skb) < 0 {
        return;
    }

    msg.msg_name = (&mut srx.transport) as *mut _ as *mut core::ffi::c_void;
    msg.msg_namelen = srx.transport_len;
    msg.msg_control = core::ptr::null_mut();
    msg.msg_controllen = 0;
    msg.msg_flags = 0;

    whdr.epoch = htonl((*sp).hdr.epoch);
    whdr.cid = htonl((*sp).hdr.cid);
    whdr.callNumber = htonl((*sp).hdr.callNumber);
    whdr.seq = 0;
    whdr.serial = 0;
    whdr.type_ = RXRPC_PACKET_TYPE_VERSION;
    whdr.flags = RXRPC_LAST_PACKET | (!(*hdr).flags & RXRPC_CLIENT_INITIATED);
    whdr.userStatus = 0;
    whdr.securityIndex = 0;
    whdr._rsvd = 0;
    whdr.serviceId = htons((*sp).hdr.serviceId);

    iov[0].iov_base = &mut whdr as *mut _ as *mut core::ffi::c_void;
    iov[0].iov_len = core::mem::size_of::<rxrpc_wire_header>();
    iov[1].iov_base = RXRPC_VERSION_STRING.as_mut_ptr() as *mut core::ffi::c_void;
    iov[1].iov_len = core::mem::size_of::<[core::ffi::c_char; 65]>();

    len = iov[0].iov_len + iov[1].iov_len;

    ret = kernel_sendmsg((*local).socket, &mut msg, iov.as_mut_ptr(), 2, len);
    if ret < 0 {
        trace_rxrpc_tx_fail(
            (*local).debug_id,
            0,
            ret,
            rxrpc_tx_point_version_reply,
        );
    } else {
        trace_rxrpc_tx_packet(
            (*local).debug_id,
            &whdr,
            rxrpc_tx_point_version_reply,
        );
    }

    _leave(c"");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
