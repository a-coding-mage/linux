// SPDX-License-Identifier: GPL-2.0-only
/*
 * Handle the TLS Alert protocol
 *
 * Author: Chuck Lever <chuck.lever@oracle.com>
 *
 * Copyright (c) 2023, Oracle and/or its affiliates.
 */

// Dependencies supplied by the surrounding kernel/networking translation.

/**
 * tls_alert_send - send a TLS Alert on a kTLS socket
 * @sock: open kTLS socket to send on
 * @level: TLS Alert level
 * @description: TLS Alert description
 *
 * Returns zero on success or a negative errno.
 */
pub unsafe fn tls_alert_send(sock: *mut socket, level: u8, description: u8) -> i32 {
    let record_type: u8 = TLS_RECORD_TYPE_ALERT as u8;
    let mut buf: [u8; CMSG_SPACE(core::mem::size_of::<u8>()) as usize] = [0; CMSG_SPACE(core::mem::size_of::<u8>()) as usize];
    let mut msg: msghdr = core::mem::zeroed();
    let cmsg: *mut cmsghdr;
    let mut iov: kvec;
    let mut alert: [u8; 2] = [0; 2];
    let ret: i32;

    trace_tls_alert_send((*sock).sk, level, description);

    alert[0] = level;
    alert[1] = description;
    iov.iov_base = alert.as_mut_ptr() as *mut core::ffi::c_void;
    iov.iov_len = core::mem::size_of::<[u8; 2]>();

    buf.fill(0);
    msg.msg_control = buf.as_mut_ptr() as *mut core::ffi::c_void;
    msg.msg_controllen = buf.len();
    msg.msg_flags = MSG_DONTWAIT;

    cmsg = CMSG_FIRSTHDR(&mut msg);
    (*cmsg).cmsg_level = SOL_TLS;
    (*cmsg).cmsg_type = TLS_SET_RECORD_TYPE;
    (*cmsg).cmsg_len = CMSG_LEN(core::mem::size_of::<u8>());
    core::ptr::copy_nonoverlapping(
        &record_type as *const u8,
        CMSG_DATA(cmsg) as *mut u8,
        core::mem::size_of::<u8>(),
    );

    iov_iter_kvec(&mut msg.msg_iter, ITER_SOURCE, &mut iov, 1, iov.iov_len);
    ret = sock_sendmsg(sock, &mut msg);
    if ret < 0 { ret } else { 0 }
}

/**
 * tls_get_record_type - Look for TLS RECORD_TYPE information
 * @sk: socket (for IP address information)
 * @cmsg: incoming message to be parsed
 *
 * Returns zero or a TLS_RECORD_TYPE value.
 */
pub unsafe fn tls_get_record_type(sk: *const sock, cmsg: *const cmsghdr) -> u8 {
    let record_type: u8;

    if (*cmsg).cmsg_level != SOL_TLS {
        return 0;
    }
    if (*cmsg).cmsg_type != TLS_GET_RECORD_TYPE {
        return 0;
    }

    record_type = *(CMSG_DATA(cmsg) as *const u8);
    trace_tls_contenttype(sk, record_type);
    record_type
}

/**
 * tls_alert_recv - Parse TLS Alert messages
 * @sk: socket (for IP address information)
 * @msg: incoming message to be parsed
 * @level: OUT - TLS AlertLevel value
 * @description: OUT - TLS AlertDescription value
 *
 */
pub unsafe fn tls_alert_recv(
    sk: *const sock,
    msg: *const msghdr,
    level: *mut u8,
    description: *mut u8,
) {
    let iov: *const kvec;
    let data: *mut u8;

    iov = (*msg).msg_iter.kvec;
    data = (*iov).iov_base as *mut u8;
    *level = *data.add(0);
    *description = *data.add(1);

    trace_tls_alert_recv(sk, *level, *description);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
