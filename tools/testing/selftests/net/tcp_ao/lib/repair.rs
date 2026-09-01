// SPDX-License-Identifier: GPL-2.0
/* This is over-simplified TCP_REPAIR for TCP_ESTABLISHED sockets
 * It tests that TCP-AO enabled connection can be restored.
 * For the proper socket repair see:
 * https://github.com/checkpoint-restore/criu/blob/criu-dev/soccr/soccr.h
 */
/* C dependencies removed from executable Rust:
 * <fcntl.h>, <linux/sockios.h>, <sys/ioctl.h>, "aolib.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;

pub const TCPOPT_MAXSEG: c_int = 2;
pub const TCPOPT_WINDOW: c_int = 3;
pub const TCPOPT_SACK_PERMITTED: c_int = 4;
pub const TCPOPT_TIMESTAMP: c_int = 8;

pub const TCP_ESTABLISHED: c_int = 1;
pub const TCP_SYN_SENT: c_int = 2;
pub const TCP_SYN_RECV: c_int = 3;
pub const TCP_FIN_WAIT1: c_int = 4;
pub const TCP_FIN_WAIT2: c_int = 5;
pub const TCP_TIME_WAIT: c_int = 6;
pub const TCP_CLOSE: c_int = 7;
pub const TCP_CLOSE_WAIT: c_int = 8;
pub const TCP_LAST_ACK: c_int = 9;
pub const TCP_LISTEN: c_int = 10;
pub const TCP_CLOSING: c_int = 11; /* Now a valid state */
pub const TCP_NEW_SYN_RECV: c_int = 12;

pub const TCP_MAX_STATES: c_int = 13; /* Leave at the end! */

pub type socklen_t = u32;
pub type size_t = usize;
pub type uint32_t = u32;

/* External constants, structs, and functions are supplied by translated
 * headers/libraries corresponding to the original C includes.
 */
unsafe extern "C" {
    static SOL_TCP: c_int;
    static TCP_REPAIR_QUEUE: c_int;
    static TCP_QUEUE_SEQ: c_int;
    static MSG_PEEK: c_int;
    static MSG_DONTWAIT: c_int;
    static TCP_INFO: c_int;
    static TCP_REPAIR_WINDOW: c_int;
    static SIOCOUTQ: c_long;
    static SIOCOUTQNSD: c_long;
    static TCP_SEND_QUEUE: c_int;
    static SIOCINQ: c_long;
    static TCP_RECV_QUEUE: c_int;
    static TCP_MAXSEG: c_int;
    static TCP_TIMESTAMP: c_int;
    static TCP_AO_REPAIR: c_int;
    static F_GETFL: c_int;
    static F_SETFL: c_int;
    static O_NONBLOCK: c_int;
    static SOL_SOCKET: c_int;
    static SO_BINDTODEVICE: c_int;
    static TCPI_OPT_SACK: u8;
    static TCPI_OPT_WSCALE: u8;
    static TCPI_OPT_TIMESTAMPS: u8;
    static TCP_REPAIR_OPTIONS: c_int;
    static TCP_REPAIR_ON: c_int;
    static TCP_REPAIR: c_int;
    static TCP_REPAIR_OFF_NO_WP: c_int;

    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn ioctl(fd: c_int, request: c_long, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn getsockname(socket: c_int, address: *mut c_void, address_len: *mut socklen_t) -> c_int;
    fn send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> isize;
    fn bind(socket: c_int, address: *const c_void, address_len: socklen_t) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn connect(socket: c_int, address: *const c_void, address_len: socklen_t) -> c_int;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;

    fn test_error(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct tcp_info {
    pub tcpi_state: u8,
    pub tcpi_options: u8,
    pub tcpi_snd_wscale: u8,
    pub tcpi_rcv_wscale: u8,
}

#[repr(C)]
pub struct tcp_repair_window {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tcp_sock_queue {
    pub seq: uint32_t,
    pub buf: *mut c_void,
}

#[repr(C)]
pub struct tcp_sock_state {
    pub info: tcp_info,
    pub trw: tcp_repair_window,
    pub outq_len: c_int,
    pub outq_nsd_len: c_int,
    pub out: tcp_sock_queue,
    pub inq_len: c_int,
    pub in_: tcp_sock_queue,
    pub mss: c_int,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct tcp_ao_repair {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tcp_repair_opt {
    pub opt_code: u32,
    pub opt_val: u32,
}

unsafe fn test_sock_checkpoint_queue(
    sk: c_int,
    queue: c_int,
    qlen: c_int,
    q: *mut tcp_sock_queue,
) {
    let mut len: socklen_t;
    let mut ret: c_int;

    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR_QUEUE,
        &queue as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR_QUEUE)".as_ptr());
    }

    len = size_of::<uint32_t>() as socklen_t;
    ret = getsockopt(
        sk,
        SOL_TCP,
        TCP_QUEUE_SEQ,
        &mut (*q).seq as *mut uint32_t as *mut c_void,
        &mut len,
    );
    if ret != 0 || len as usize != size_of::<uint32_t>() {
        test_error(c"getsockopt(TCP_QUEUE_SEQ): %d".as_ptr(), len as c_int);
    }

    if qlen == 0 {
        (*q).buf = core::ptr::null_mut();
        return;
    }

    (*q).buf = malloc(qlen as size_t);
    if (*q).buf.is_null() {
        test_error(c"malloc()".as_ptr());
    }
    ret = recv(sk, (*q).buf, qlen as size_t, MSG_PEEK | MSG_DONTWAIT) as c_int;
    if ret != qlen {
        test_error(c"recv(%d): %d".as_ptr(), qlen, ret);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __test_sock_checkpoint(
    sk: c_int,
    state: *mut tcp_sock_state,
    addr: *mut c_void,
    addr_size: size_t,
) {
    let mut len: socklen_t = size_of::<tcp_info>() as socklen_t;
    let mut ret: c_int;

    memset(
        state as *mut c_void,
        0,
        size_of::<tcp_sock_state>() as size_t,
    );

    ret = getsockopt(
        sk,
        SOL_TCP,
        TCP_INFO,
        &mut (*state).info as *mut tcp_info as *mut c_void,
        &mut len,
    );
    if ret != 0 || len as usize != size_of::<tcp_info>() {
        test_error(c"getsockopt(TCP_INFO): %d".as_ptr(), len as c_int);
    }

    len = addr_size as socklen_t;
    if getsockname(sk, addr, &mut len) != 0 || len as size_t != addr_size {
        test_error(c"getsockname(): %d".as_ptr(), len as c_int);
    }

    len = size_of::<tcp_repair_window>() as socklen_t;
    ret = getsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR_WINDOW,
        &mut (*state).trw as *mut tcp_repair_window as *mut c_void,
        &mut len,
    );
    if ret != 0 || len as usize != size_of::<tcp_repair_window>() {
        test_error(c"getsockopt(TCP_REPAIR_WINDOW): %d".as_ptr(), len as c_int);
    }

    if ioctl(sk, SIOCOUTQ, &mut (*state).outq_len) != 0 {
        test_error(c"ioctl(SIOCOUTQ)".as_ptr());
    }

    if ioctl(sk, SIOCOUTQNSD, &mut (*state).outq_nsd_len) != 0 {
        test_error(c"ioctl(SIOCOUTQNSD)".as_ptr());
    }
    test_sock_checkpoint_queue(sk, TCP_SEND_QUEUE, (*state).outq_len, &mut (*state).out);

    if ioctl(sk, SIOCINQ, &mut (*state).inq_len) != 0 {
        test_error(c"ioctl(SIOCINQ)".as_ptr());
    }
    test_sock_checkpoint_queue(sk, TCP_RECV_QUEUE, (*state).inq_len, &mut (*state).in_);

    if (*state).info.tcpi_state as c_int == TCP_CLOSE {
        (*state).outq_nsd_len = 0;
        (*state).outq_len = (*state).outq_nsd_len;
    }

    len = size_of::<c_int>() as socklen_t;
    ret = getsockopt(
        sk,
        SOL_TCP,
        TCP_MAXSEG,
        &mut (*state).mss as *mut c_int as *mut c_void,
        &mut len,
    );
    if ret != 0 || len as usize != size_of::<c_int>() {
        test_error(c"getsockopt(TCP_MAXSEG): %d".as_ptr(), len as c_int);
    }

    len = size_of::<c_int>() as socklen_t;
    ret = getsockopt(
        sk,
        SOL_TCP,
        TCP_TIMESTAMP,
        &mut (*state).timestamp as *mut c_int as *mut c_void,
        &mut len,
    );
    if ret != 0 || len as usize != size_of::<c_int>() {
        test_error(c"getsockopt(TCP_TIMESTAMP): %d".as_ptr(), len as c_int);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ao_checkpoint(sk: c_int, state: *mut tcp_ao_repair) {
    let mut len: socklen_t = size_of::<tcp_ao_repair>() as socklen_t;
    let mut ret: c_int;

    memset(state as *mut c_void, 0, size_of::<tcp_ao_repair>() as size_t);

    ret = getsockopt(sk, SOL_TCP, TCP_AO_REPAIR, state as *mut c_void, &mut len);
    if ret != 0 || len as usize != size_of::<tcp_ao_repair>() {
        test_error(c"getsockopt(TCP_AO_REPAIR): %d".as_ptr(), len as c_int);
    }
}

unsafe fn test_sock_restore_seq(sk: c_int, queue: c_int, seq: uint32_t) {
    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR_QUEUE,
        &queue as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR_QUEUE)".as_ptr());
    }

    if setsockopt(
        sk,
        SOL_TCP,
        TCP_QUEUE_SEQ,
        &seq as *const uint32_t as *const c_void,
        size_of::<uint32_t>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_QUEUE_SEQ)".as_ptr());
    }
}

unsafe fn test_sock_restore_queue(sk: c_int, queue: c_int, buf: *mut c_void, mut len: c_int) {
    let mut chunk: c_int = len;
    let mut off: size_t = 0;

    if len == 0 {
        return;
    }

    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR_QUEUE,
        &queue as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR_QUEUE)".as_ptr());
    }

    loop {
        let ret: c_int;

        ret = send(sk, (buf as *const u8).add(off) as *const c_void, chunk as size_t, 0) as c_int;
        if ret <= 0 {
            if chunk > 1024 {
                chunk >>= 1;
                continue;
            }
            test_error(c"send()".as_ptr());
        }
        off += ret as size_t;
        len -= ret;
        if len <= 0 {
            break;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __test_sock_restore(
    sk: c_int,
    device: *const c_char,
    state: *mut tcp_sock_state,
    saddr: *mut c_void,
    daddr: *mut c_void,
    addr_size: size_t,
) {
    let mut opts: [tcp_repair_opt; 4] = core::mem::zeroed();
    let mut opt_nr: c_uint = 0;
    let flags: c_long;

    if bind(sk, saddr as *const c_void, addr_size as socklen_t) != 0 {
        test_error(c"bind()".as_ptr());
    }

    flags = fcntl(sk, F_GETFL) as c_long;
    if flags < 0 || fcntl(sk, F_SETFL, (flags | O_NONBLOCK as c_long) as c_int) < 0 {
        test_error(c"fcntl()".as_ptr());
    }

    test_sock_restore_seq(
        sk,
        TCP_RECV_QUEUE,
        (*state).in_.seq.wrapping_sub((*state).inq_len as uint32_t),
    );
    test_sock_restore_seq(
        sk,
        TCP_SEND_QUEUE,
        (*state).out.seq.wrapping_sub((*state).outq_len as uint32_t),
    );

    if !device.is_null()
        && setsockopt(
            sk,
            SOL_SOCKET,
            SO_BINDTODEVICE,
            device as *const c_void,
            (strlen(device) + 1) as socklen_t,
        ) != 0
    {
        test_error(c"setsockopt(SO_BINDTODEVICE, %s)".as_ptr(), device);
    }

    if connect(sk, daddr as *const c_void, addr_size as socklen_t) != 0 {
        test_error(c"connect()".as_ptr());
    }

    if ((*state).info.tcpi_options & TCPI_OPT_SACK) != 0 {
        opts[opt_nr as usize].opt_code = TCPOPT_SACK_PERMITTED as u32;
        opts[opt_nr as usize].opt_val = 0;
        opt_nr += 1;
    }
    if ((*state).info.tcpi_options & TCPI_OPT_WSCALE) != 0 {
        opts[opt_nr as usize].opt_code = TCPOPT_WINDOW as u32;
        opts[opt_nr as usize].opt_val = (*state).info.tcpi_snd_wscale as u32
            + (((*state).info.tcpi_rcv_wscale as u32) << 16);
        opt_nr += 1;
    }
    if ((*state).info.tcpi_options & TCPI_OPT_TIMESTAMPS) != 0 {
        opts[opt_nr as usize].opt_code = TCPOPT_TIMESTAMP as u32;
        opts[opt_nr as usize].opt_val = 0;
        opt_nr += 1;
    }
    opts[opt_nr as usize].opt_code = TCPOPT_MAXSEG as u32;
    opts[opt_nr as usize].opt_val = (*state).mss as u32;
    opt_nr += 1;

    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR_OPTIONS,
        opts.as_ptr() as *const c_void,
        (opt_nr as usize * size_of::<tcp_repair_opt>()) as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR_OPTIONS)".as_ptr());
    }

    if ((*state).info.tcpi_options & TCPI_OPT_TIMESTAMPS) != 0 {
        if setsockopt(
            sk,
            SOL_TCP,
            TCP_TIMESTAMP,
            &(*state).timestamp as *const c_int as *const c_void,
            (opt_nr as usize * size_of::<tcp_repair_opt>()) as socklen_t,
        ) != 0
        {
            test_error(c"setsockopt(TCP_TIMESTAMP)".as_ptr());
        }
    }
    test_sock_restore_queue(sk, TCP_RECV_QUEUE, (*state).in_.buf, (*state).inq_len);
    test_sock_restore_queue(sk, TCP_SEND_QUEUE, (*state).out.buf, (*state).outq_len);
    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR_WINDOW,
        &(*state).trw as *const tcp_repair_window as *const c_void,
        size_of::<tcp_repair_window>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR_WINDOW)".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ao_restore(sk: c_int, state: *mut tcp_ao_repair) {
    if setsockopt(
        sk,
        SOL_TCP,
        TCP_AO_REPAIR,
        state as *const c_void,
        size_of::<tcp_ao_repair>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_AO_REPAIR)".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sock_state_free(state: *mut tcp_sock_state) {
    free((*state).out.buf);
    free((*state).in_.buf);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_enable_repair(sk: c_int) {
    let val: c_int = TCP_REPAIR_ON;

    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR,
        &val as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR)".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_disable_repair(sk: c_int) {
    let val: c_int = TCP_REPAIR_OFF_NO_WP;

    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR,
        &val as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR)".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kill_sk(sk: c_int) {
    test_enable_repair(sk);
    close(sk);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
