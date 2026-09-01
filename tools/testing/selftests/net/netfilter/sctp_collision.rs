// SPDX-License-Identifier: GPL-2.0

use libc::{
    atoi, bind, c_char, c_int, c_void, close, connect, htons, inet_addr, listen, recvfrom, sendto,
    setsockopt, sleep, sockaddr, sockaddr_in, socklen_t, socket, strcmp, strlen, timeval, usleep,
    AF_INET, IPPROTO_SCTP, SOCK_SEQPACKET, SOL_SOCKET, SO_RCVTIMEO,
};
use std::env;
use std::ffi::CString;
use std::mem;
use std::ptr;

fn main() {
    let args: Vec<CString> = env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();
    argv.push(ptr::null_mut());

    let ret = unsafe { c_main((argv.len() - 1) as c_int, argv.as_mut_ptr()) };
    std::process::exit(ret);
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut saddr: sockaddr_in = mem::zeroed();
    let mut daddr: sockaddr_in = mem::zeroed();
    let mut len: socklen_t = mem::size_of_val(&daddr) as socklen_t;
    let tv = timeval {
        tv_sec: 25,
        tv_usec: 0,
    };
    let mut buf = *b"hello\0";
    let sd: c_int;
    let mut ret: c_int;

    if argc != 6
        || (strcmp(*argv.add(1), b"server\0".as_ptr() as *const c_char) != 0
            && strcmp(*argv.add(1), b"client\0".as_ptr() as *const c_char) != 0)
    {
        printf(
            b"%s <server|client> <LOCAL_IP> <LOCAL_PORT> <REMOTE_IP> <REMOTE_PORT>\n\0".as_ptr()
                as *const c_char,
            *argv.add(0),
        );
        return -1;
    }

    sd = socket(AF_INET, SOCK_SEQPACKET, IPPROTO_SCTP);
    if sd < 0 {
        printf(b"Failed to create sd\n\0".as_ptr() as *const c_char);
        return -1;
    }

    saddr.sin_family = AF_INET as _;
    saddr.sin_addr.s_addr = inet_addr(*argv.add(2));
    saddr.sin_port = htons(atoi(*argv.add(3)) as _);

    ret = bind(
        sd,
        &saddr as *const sockaddr_in as *const sockaddr,
        mem::size_of_val(&saddr) as socklen_t,
    );
    if ret < 0 {
        printf(b"Failed to bind to address\n\0".as_ptr() as *const c_char);
        goto_out(sd, ret);
        return ret;
    }

    ret = listen(sd, 5);
    if ret < 0 {
        printf(b"Failed to listen on port\n\0".as_ptr() as *const c_char);
        goto_out(sd, ret);
        return ret;
    }

    daddr.sin_family = AF_INET as _;
    daddr.sin_addr.s_addr = inet_addr(*argv.add(4));
    daddr.sin_port = htons(atoi(*argv.add(5)) as _);

    /* make test shorter than 25s */
    ret = setsockopt(
        sd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &tv as *const timeval as *const c_void,
        mem::size_of_val(&tv) as socklen_t,
    );
    if ret < 0 {
        printf(b"Failed to setsockopt SO_RCVTIMEO\n\0".as_ptr() as *const c_char);
        goto_out(sd, ret);
        return ret;
    }

    if strcmp(*argv.add(1), b"server\0".as_ptr() as *const c_char) == 0 {
        sleep(1); /* wait a bit for client's INIT */
        ret = connect(
            sd,
            &daddr as *const sockaddr_in as *const sockaddr,
            len,
        );
        if ret < 0 {
            printf(b"Failed to connect to peer\n\0".as_ptr() as *const c_char);
            goto_out(sd, ret);
            return ret;
        }
        ret = recvfrom(
            sd,
            buf.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&buf),
            0,
            &mut daddr as *mut sockaddr_in as *mut sockaddr,
            &mut len,
        ) as c_int;
        if ret < 0 {
            printf(
                b"Failed to recv msg %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_out(sd, ret);
            return ret;
        }
        ret = sendto(
            sd,
            buf.as_ptr() as *const c_void,
            strlen(buf.as_ptr() as *const c_char) + 1,
            0,
            &daddr as *const sockaddr_in as *const sockaddr,
            len,
        ) as c_int;
        if ret < 0 {
            printf(
                b"Failed to send msg %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_out(sd, ret);
            return ret;
        }
        printf(b"Server: sent! %d\n\0".as_ptr() as *const c_char, ret);
    }

    if strcmp(*argv.add(1), b"client\0".as_ptr() as *const c_char) == 0 {
        usleep(300000); /* wait a bit for server's listening */
        ret = connect(
            sd,
            &daddr as *const sockaddr_in as *const sockaddr,
            len,
        );
        if ret < 0 {
            printf(b"Failed to connect to peer\n\0".as_ptr() as *const c_char);
            goto_out(sd, ret);
            return ret;
        }
        sleep(1); /* wait a bit for server's delayed INIT_ACK to reproduce the issue */
        ret = sendto(
            sd,
            buf.as_ptr() as *const c_void,
            strlen(buf.as_ptr() as *const c_char) + 1,
            0,
            &daddr as *const sockaddr_in as *const sockaddr,
            len,
        ) as c_int;
        if ret < 0 {
            printf(
                b"Failed to send msg %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_out(sd, ret);
            return ret;
        }
        ret = recvfrom(
            sd,
            buf.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&buf),
            0,
            &mut daddr as *mut sockaddr_in as *mut sockaddr,
            &mut len,
        ) as c_int;
        if ret < 0 {
            printf(
                b"Failed to recv msg %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_out(sd, ret);
            return ret;
        }
        printf(b"Client: rcvd! %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret = 0;

    close(sd);
    ret
}

unsafe fn goto_out(sd: c_int, ret: c_int) {
    close(sd);
    let _ = ret;
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
