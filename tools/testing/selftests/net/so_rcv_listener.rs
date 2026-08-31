// SPDX-License-Identifier: GPL-2.0

// C dependencies translated to libc/std equivalents:
// errno.h, netdb.h, stdbool.h, stdio.h, stdlib.h, string.h, unistd.h,
// linux/types.h, sys/socket.h, netinet/in.h, arpa/inet.h.

use libc::{
    bind, c_char, c_int, c_void, close, cmsghdr, exit, fprintf, getopt, htons, inet_pton, iovec,
    msghdr, perror, printf, recvmsg, sockaddr, sockaddr_in, sockaddr_in6, socket, strchr,
    setsockopt, size_t, stderr, AF_INET, AF_INET6, EINVAL, ENOMSG, EXIT_FAILURE, IPPROTO_UDP,
    SOCK_DGRAM, SOL_SOCKET, SO_MARK, SO_PRIORITY,
};
use std::mem;
use std::ptr;

const SO_RCVPRIORITY: c_int = 82;

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;
}

type __u32 = u32;

#[repr(C)]
struct options {
    val: __u32,
    name: c_int,
    rcvname: c_int,
    host: *const c_char,
    service: *const c_char,
}

static mut opt: options = options {
    val: 0,
    name: 0,
    rcvname: 0,
    host: ptr::null(),
    service: ptr::null(),
};

unsafe fn cmsg_align(len: usize) -> usize {
    (len + mem::size_of::<size_t>() - 1) & !(mem::size_of::<size_t>() - 1)
}

unsafe fn cmsg_space(len: usize) -> usize {
    cmsg_align(mem::size_of::<cmsghdr>()) + cmsg_align(len)
}

unsafe fn cmsg_firsthdr(mhdr: *mut msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen >= mem::size_of::<cmsghdr>() {
        (*mhdr).msg_control as *mut cmsghdr
    } else {
        ptr::null_mut()
    }
}

unsafe fn cmsg_nxthdr(mhdr: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let next = (cmsg as *mut u8).add(cmsg_align((*cmsg).cmsg_len as usize)) as *mut cmsghdr;
    let max = ((*mhdr).msg_control as *mut u8).add((*mhdr).msg_controllen as usize);

    if (next as *mut u8).add(mem::size_of::<cmsghdr>()) > max {
        ptr::null_mut()
    } else {
        next
    }
}

unsafe fn cmsg_data(cmsg: *mut cmsghdr) -> *mut u8 {
    (cmsg as *mut u8).add(cmsg_align(mem::size_of::<cmsghdr>()))
}

unsafe fn usage(bin: *const c_char) -> ! {
    printf(
        b"Usage: %s [opts] <dst host> <dst port / service>\n\0".as_ptr() as *const c_char,
        bin,
    );
    printf(
        b"Options:\n\t\t-M val  Test SO_RCVMARK\n\t\t-P val  Test SO_RCVPRIORITY\n\0"
            .as_ptr() as *const c_char,
    );
    exit(EXIT_FAILURE);
}

unsafe fn parse_args(argc: c_int, argv: *mut *mut c_char) {
    let mut o: c_int;

    loop {
        o = getopt(argc, argv, b"M:P:\0".as_ptr() as *const c_char);
        if o == -1 {
            break;
        }

        match o {
            x if x == b'M' as c_int => {
                opt.val = libc::atoi(optarg) as __u32;
                opt.name = SO_MARK;
                opt.rcvname = libc::SO_RCVMARK;
            }
            x if x == b'P' as c_int => {
                opt.val = libc::atoi(optarg) as __u32;
                opt.name = SO_PRIORITY;
                opt.rcvname = SO_RCVPRIORITY;
            }
            _ => {
                usage(*argv.add(0));
            }
        }
    }

    if optind != argc - 2 {
        usage(*argv.add(0));
    }

    opt.host = *argv.add(optind as usize);
    opt.service = *argv.add((optind + 1) as usize);
}

unsafe fn so_rcv_listener_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut err: c_int = 0;
    let mut recv_fd: c_int = -1;
    let mut ret_value: c_int = 0;
    let mut recv_val: __u32;
    let mut cmsg: *mut cmsghdr;
    let mut cbuf = [0u8; cmsg_space(mem::size_of::<__u32>())];
    let mut recv_buf = [0u8; cmsg_space(mem::size_of::<__u32>())];
    let mut iov: [iovec; 1] = [mem::zeroed()];
    let mut msg: msghdr = mem::zeroed();
    let mut recv_addr4: sockaddr_in = mem::zeroed();
    let mut recv_addr6: sockaddr_in6 = mem::zeroed();

    parse_args(argc, argv);

    let family: c_int = if !strchr(opt.host, b':' as c_int).is_null() {
        AF_INET6
    } else {
        AF_INET
    };

    recv_fd = socket(family, SOCK_DGRAM, IPPROTO_UDP);
    if recv_fd < 0 {
        perror(b"Can't open recv socket\0".as_ptr() as *const c_char);
        ret_value = -errno();
        goto_cleanup(recv_fd);
        return ret_value;
    }

    err = setsockopt(
        recv_fd,
        SOL_SOCKET,
        opt.rcvname,
        &opt.val as *const __u32 as *const c_void,
        mem::size_of_val(&opt.val) as libc::socklen_t,
    );
    if err < 0 {
        perror(b"Recv setsockopt error\0".as_ptr() as *const c_char);
        ret_value = -errno();
        goto_cleanup(recv_fd);
        return ret_value;
    }

    if family == AF_INET {
        ptr::write_bytes(
            &mut recv_addr4 as *mut sockaddr_in as *mut u8,
            0,
            mem::size_of_val(&recv_addr4),
        );
        recv_addr4.sin_family = family as libc::sa_family_t;
        recv_addr4.sin_port = htons(libc::atoi(opt.service) as u16);

        if inet_pton(
            family,
            opt.host,
            &mut recv_addr4.sin_addr as *mut _ as *mut c_void,
        ) <= 0
        {
            perror(b"Invalid IPV4 address\0".as_ptr() as *const c_char);
            ret_value = -errno();
            goto_cleanup(recv_fd);
            return ret_value;
        }

        err = bind(
            recv_fd,
            &recv_addr4 as *const sockaddr_in as *const sockaddr,
            mem::size_of_val(&recv_addr4) as libc::socklen_t,
        );
    } else {
        ptr::write_bytes(
            &mut recv_addr6 as *mut sockaddr_in6 as *mut u8,
            0,
            mem::size_of_val(&recv_addr6),
        );
        recv_addr6.sin6_family = family as libc::sa_family_t;
        recv_addr6.sin6_port = htons(libc::atoi(opt.service) as u16);

        if inet_pton(
            family,
            opt.host,
            &mut recv_addr6.sin6_addr as *mut _ as *mut c_void,
        ) <= 0
        {
            perror(b"Invalid IPV6 address\0".as_ptr() as *const c_char);
            ret_value = -errno();
            goto_cleanup(recv_fd);
            return ret_value;
        }

        err = bind(
            recv_fd,
            &recv_addr6 as *const sockaddr_in6 as *const sockaddr,
            mem::size_of_val(&recv_addr6) as libc::socklen_t,
        );
    }

    if err < 0 {
        perror(b"Recv bind error\0".as_ptr() as *const c_char);
        ret_value = -errno();
        goto_cleanup(recv_fd);
        return ret_value;
    }

    iov[0].iov_base = recv_buf.as_mut_ptr() as *mut c_void;
    iov[0].iov_len = recv_buf.len();

    ptr::write_bytes(
        &mut msg as *mut msghdr as *mut u8,
        0,
        mem::size_of_val(&msg),
    );
    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = 1;
    msg.msg_control = cbuf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = cbuf.len();

    err = recvmsg(recv_fd, &mut msg, 0);
    if err < 0 {
        perror(b"Message receive error\0".as_ptr() as *const c_char);
        ret_value = -errno();
        goto_cleanup(recv_fd);
        return ret_value;
    }

    cmsg = cmsg_firsthdr(&mut msg);
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level == SOL_SOCKET && (*cmsg).cmsg_type == opt.name {
            recv_val = *(cmsg_data(cmsg) as *mut __u32);
            printf(
                b"Received value: %u\n\0".as_ptr() as *const c_char,
                recv_val,
            );

            if recv_val != opt.val {
                fprintf(
                    stderr,
                    b"Error: expected value: %u, got: %u\n\0".as_ptr() as *const c_char,
                    opt.val,
                    recv_val,
                );
                ret_value = -EINVAL;
            }
            goto_cleanup(recv_fd);
            return ret_value;
        }
        cmsg = cmsg_nxthdr(&mut msg, cmsg);
    }

    fprintf(
        stderr,
        b"Error: No matching cmsg received\n\0".as_ptr() as *const c_char,
    );
    ret_value = -ENOMSG;

    goto_cleanup(recv_fd);
    ret_value
}

unsafe fn goto_cleanup(recv_fd: c_int) {
    if recv_fd >= 0 {
        close(recv_fd);
    }
}

unsafe fn errno() -> c_int {
    *libc::__errno_location()
}

fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| {
            let mut bytes = arg.into_bytes();
            bytes.push(0);
            bytes
        })
        .collect();
    let mut argv: Vec<*mut c_char> = args
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .collect();

    unsafe {
        std::process::exit(so_rcv_listener_main(
            argv.len() as c_int,
            argv.as_mut_ptr(),
        ));
    }
}
