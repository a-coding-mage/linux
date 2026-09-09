// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 	ucon.c
 *
 * Copyright (c) 2004+ Evgeniy Polyakov <zbr@ioremap.net>
 */

// C headers and kernel-provided types/functions are supplied by the surrounding build.

const NETLINK_CONNECTOR: i32 = 11;
const CN_TEST_IDX: u32 = CN_NETLINK_USERS + 3;
const CN_TEST_VAL: u32 = 0x456;

static mut NEED_EXIT: i32 = 0;
static mut SEQ: u32 = 0;

unsafe fn netlink_send(s: i32, msg: *mut cn_msg) -> i32 {
    let size: usize = NLMSG_SPACE(core::mem::size_of::<cn_msg>() + (*msg).len as usize);
    let mut buf = [0i8; 128];
    let nlh = buf.as_mut_ptr() as *mut nlmsghdr;

    (*nlh).nlmsg_seq = SEQ;
    SEQ = SEQ.wrapping_add(1);
    (*nlh).nlmsg_pid = getpid() as u32;
    (*nlh).nlmsg_type = NLMSG_DONE;
    (*nlh).nlmsg_len = size as u32;
    (*nlh).nlmsg_flags = 0;

    let m = NLMSG_DATA(nlh) as *mut cn_msg;
    // ulog("%s: [%08x.%08x] len=%u, seq=%u, ack=%u.\n", __func__, msg->id.idx, msg->id.val, msg->len, msg->seq, msg->ack);
    memcpy(m as *mut core::ffi::c_void, msg as *const core::ffi::c_void,
           core::mem::size_of::<cn_msg>() + (*msg).len as usize);

    let err = send(s, nlh as *const core::ffi::c_void, size, 0) as i32;
    if err == -1 {
        ulog!("Failed to send: %s [%d].\n", strerror(errno), errno);
    }
    err
}

unsafe fn usage() {
    printf!(
        "Usage: ucon [options] [output file]\n\n\t-h\tthis help screen\n\t-s\tsend buffers to the test module\n\nThe default behavior of ucon is to subscribe to the test module\nand wait for state messages.  Any ones received are dumped to the\nspecified output file (or stdout).  The test module is assumed to\nhave an id of {%u.%u}\n\nIf you get no output, then verify the cn_test module id matches\nthe expected id above.\n",
        CN_TEST_IDX, CN_TEST_VAL
    );
}

pub unsafe fn main(argc: i32, argv: *mut *mut i8) -> i32 {
    let mut s: i32;
    let mut buf = [0i8; 1024];
    let mut len: i32;
    let mut reply: *mut nlmsghdr;
    let mut l_local: sockaddr_nl = core::mem::zeroed();
    let mut data: *mut cn_msg;
    let mut out: *mut FILE;
    let mut tm: time_t = 0;
    let mut pfd: pollfd = core::mem::zeroed();
    let mut send_msgs = false;

    while { s = getopt(argc, argv, c"hs".as_ptr()); s != -1 } {
        match s {
            x if x == 's' as i32 => send_msgs = true,
            x if x == 'h' as i32 => { usage(); return 0; },
            _ => { usage(); return 1; },
        }
    }

    if argc != optind {
        out = fopen(*argv.add(optind as usize), c"a+".as_ptr());
        if out.is_null() {
            ulog!("Unable to open %s for writing: %s\n", *argv.add(1) as *const i8, strerror(errno));
            out = stdout;
        }
    } else { out = stdout; }

    memset(buf.as_mut_ptr() as *mut core::ffi::c_void, 0, buf.len());
    s = socket(PF_NETLINK, SOCK_DGRAM, NETLINK_CONNECTOR);
    if s == -1 { perror(c"socket".as_ptr()); return -1; }

    l_local.nl_family = AF_NETLINK as u16;
    l_local.nl_groups = u32::MAX;
    l_local.nl_pid = 0;
    ulog!("subscribing to %u.%u\n", CN_TEST_IDX, CN_TEST_VAL);

    if bind(s, &mut l_local as *mut sockaddr_nl as *mut sockaddr, core::mem::size_of::<sockaddr_nl>()) == -1 {
        perror(c"bind".as_ptr()); close(s); return -1;
    }

    if send_msgs {
        memset(buf.as_mut_ptr() as *mut core::ffi::c_void, 0, buf.len());
        data = buf.as_mut_ptr() as *mut cn_msg;
        (*data).id.idx = CN_TEST_IDX;
        (*data).id.val = CN_TEST_VAL;
        (*data).seq = SEQ; SEQ = SEQ.wrapping_add(1);
        (*data).ack = 0; (*data).len = 0;
        for _j in 0..10 {
            let mut i = 0;
            while i < 1000 { len = netlink_send(s, data); i += 1; }
            ulog!("%d messages have been sent to %08x.%08x.\n", i, (*data).id.idx, (*data).id.val);
        }
        return 0;
    }

    pfd.fd = s;
    while NEED_EXIT == 0 {
        pfd.events = POLLIN;
        pfd.revents = 0;
        match poll(&mut pfd, 1, -1) {
            0 => NEED_EXIT = 1,
            -1 if errno != EINTR => { NEED_EXIT = 1; break; },
            -1 => continue,
            _ => (),
        }
        if NEED_EXIT != 0 { break; }
        memset(buf.as_mut_ptr() as *mut core::ffi::c_void, 0, buf.len());
        len = recv(s, buf.as_mut_ptr() as *mut core::ffi::c_void, buf.len(), 0);
        if len == -1 { perror(c"recv buf".as_ptr()); close(s); return -1; }
        reply = buf.as_mut_ptr() as *mut nlmsghdr;
        match (*reply).nlmsg_type {
            NLMSG_ERROR => { fprintf(out, c"Error message received.\n".as_ptr()); fflush(out); },
            NLMSG_DONE => {
                data = NLMSG_DATA(reply) as *mut cn_msg;
                time(&mut tm);
                fprintf(out, c"%.24s : [%x.%x] [%08u.%08u].\n".as_ptr(), ctime(&tm), (*data).id.idx, (*data).id.val, (*data).seq, (*data).ack);
                fflush(out);
            },
            _ => (),
        }
    }
    close(s);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
