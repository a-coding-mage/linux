// SPDX-License-Identifier: GPL-2.0

// C dependencies translated from:
// <stdio.h>, <stdlib.h>, <string.h>, <unistd.h>, <errno.h>,
// <sys/types.h>, <netinet/in.h>, <arpa/inet.h>, and "kselftest.h".

use libc::{
    accept, bind, c_char, c_int, c_void, close, connect, getsockname, getsockopt, listen, printf,
    recv, sa_family_t, send, setsockopt, sockaddr, sockaddr_in, sockaddr_in6, socket, socklen_t,
    ssize_t, AF_INET, AF_INET6, IPPROTO_TCP, IPPROTO_UDP, MSG_PEEK, MSG_TRUNC, SOCK_DGRAM,
    SOCK_STREAM, SOL_SOCKET, SO_PEEK_OFF,
};
use std::mem;
use std::ptr;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

extern "C" {
    fn ksft_perror(msg: *const c_char);
}

#[repr(C)]
union SockaddrUnion {
    sa: sockaddr,
    a4: sockaddr_in,
    a6: sockaddr_in6,
}

unsafe fn afstr(af: c_int, proto: c_int) -> *const c_char {
    if proto == IPPROTO_TCP {
        if af == AF_INET {
            b"TCP/IPv4\0".as_ptr() as *const c_char
        } else {
            b"TCP/IPv6\0".as_ptr() as *const c_char
        }
    } else if af == AF_INET {
        b"UDP/IPv4\0".as_ptr() as *const c_char
    } else {
        b"UDP/IPv6\0".as_ptr() as *const c_char
    }
}

unsafe fn sk_peek_offset_probe(af: sa_family_t, proto: c_int) -> c_int {
    let type_ = if proto == IPPROTO_TCP {
        SOCK_STREAM
    } else {
        SOCK_DGRAM
    };
    let optv: c_int = 0;
    let mut ret: c_int = 0;
    let s: c_int;

    s = socket(af as c_int, type_, proto);
    if s < 0 {
        ksft_perror(b"Temporary TCP socket creation failed\0".as_ptr() as *const c_char);
    } else {
        if setsockopt(
            s,
            SOL_SOCKET,
            SO_PEEK_OFF,
            &optv as *const c_int as *const c_void,
            mem::size_of::<c_int>() as socklen_t,
        ) == 0
        {
            ret = 1;
        } else {
            printf(
                b"%s does not support SO_PEEK_OFF\n\0".as_ptr() as *const c_char,
                afstr(af as c_int, proto),
            );
        }
        close(s);
    }
    ret
}

unsafe fn sk_peek_offset_set(s: c_int, offset: c_int) {
    if setsockopt(
        s,
        SOL_SOCKET,
        SO_PEEK_OFF,
        &offset as *const c_int as *const c_void,
        mem::size_of_val(&offset) as socklen_t,
    ) != 0
    {
        ksft_perror(b"Failed to set SO_PEEK_OFF value\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn sk_peek_offset_get(s: c_int) -> c_int {
    let mut offset: c_int = 0;
    let mut len: socklen_t = mem::size_of_val(&offset) as socklen_t;

    if getsockopt(
        s,
        SOL_SOCKET,
        SO_PEEK_OFF,
        &mut offset as *mut c_int as *mut c_void,
        &mut len,
    ) != 0
    {
        ksft_perror(b"Failed to get SO_PEEK_OFF value\n\0".as_ptr() as *const c_char);
    }
    offset
}

unsafe fn sk_peek_offset_test(af: sa_family_t, proto: c_int) -> c_int {
    let type_ = if proto == IPPROTO_TCP {
        SOCK_STREAM
    } else {
        SOCK_DGRAM
    };
    let mut a: SockaddrUnion = mem::zeroed();
    let mut res: c_int = 0;
    let mut s: [c_int; 2] = [0, 0];
    let mut recv_sock: c_int = 0;
    let mut offset: c_int;
    let mut len: ssize_t;
    let mut buf: [u8; 2] = [0, 0];

    a.sa.sa_family = af;

    s[0] = socket(af as c_int, type_, proto);
    recv_sock = s[0];
    s[1] = socket(af as c_int, type_, proto);

    'out: loop {
        if s[0] < 0 || s[1] < 0 {
            ksft_perror(b"Temporary socket creation failed\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        if bind(
            s[0],
            &a.sa as *const sockaddr,
            mem::size_of_val(&a) as socklen_t,
        ) < 0
        {
            ksft_perror(b"Temporary socket bind() failed\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        {
            let mut namelen: socklen_t = mem::size_of_val(&a) as socklen_t;
            if getsockname(s[0], &mut a.sa as *mut sockaddr, &mut namelen) < 0 {
                ksft_perror(
                    b"Temporary socket getsockname() failed\n\0".as_ptr() as *const c_char,
                );
                break 'out;
            }
        }
        if proto == IPPROTO_TCP && listen(s[0], 0) < 0 {
            ksft_perror(b"Temporary socket listen() failed\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        if connect(
            s[1],
            &a.sa as *const sockaddr,
            mem::size_of_val(&a) as socklen_t,
        ) < 0
        {
            ksft_perror(b"Temporary socket connect() failed\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        if proto == IPPROTO_TCP {
            recv_sock = accept(s[0], ptr::null_mut(), ptr::null_mut());
            if recv_sock <= 0 {
                ksft_perror(b"Temporary socket accept() failed\n\0".as_ptr() as *const c_char);
                break 'out;
            }
        }

        /* Some basic tests of getting/setting offset */
        offset = sk_peek_offset_get(recv_sock);
        if offset != -1 {
            ksft_perror(b"Initial value of socket offset not -1\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        sk_peek_offset_set(recv_sock, 0);
        offset = sk_peek_offset_get(recv_sock);
        if offset != 0 {
            ksft_perror(b"Failed to set socket offset to 0\n\0".as_ptr() as *const c_char);
            break 'out;
        }

        /* Transfer a message */
        if send(
            s[1],
            b"ab\0".as_ptr() as *const c_void,
            2,
            0,
        ) != 2
        {
            ksft_perror(b"Temporary probe socket send() failed\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        /* Read first byte */
        len = recv(
            recv_sock,
            buf.as_mut_ptr() as *mut c_void,
            1,
            MSG_PEEK,
        );
        if len != 1 || buf[0] != b'a' {
            ksft_perror(b"Failed to read first byte of message\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        offset = sk_peek_offset_get(recv_sock);
        if offset != 1 {
            ksft_perror(
                b"Offset not forwarded correctly at first byte\n\0".as_ptr() as *const c_char,
            );
            break 'out;
        }
        /* Try to read beyond last byte */
        len = recv(
            recv_sock,
            buf.as_mut_ptr() as *mut c_void,
            2,
            MSG_PEEK,
        );
        if len != 1 || buf[0] != b'b' {
            ksft_perror(b"Failed to read last byte of message\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        offset = sk_peek_offset_get(recv_sock);
        if offset != 2 {
            ksft_perror(
                b"Offset not forwarded correctly at last byte\n\0".as_ptr() as *const c_char,
            );
            break 'out;
        }
        /* Flush message */
        len = recv(
            recv_sock,
            buf.as_mut_ptr() as *mut c_void,
            2,
            MSG_TRUNC,
        );
        if len != 2 {
            ksft_perror(b"Failed to flush message\n\0".as_ptr() as *const c_char);
            break 'out;
        }
        offset = sk_peek_offset_get(recv_sock);
        if offset != 0 {
            ksft_perror(
                b"Offset not reverted correctly after flush\n\0".as_ptr() as *const c_char,
            );
            break 'out;
        }

        printf(
            b"%s with MSG_PEEK_OFF works correctly\n\0".as_ptr() as *const c_char,
            afstr(af as c_int, proto),
        );
        res = 1;
        break 'out;
    }

    if proto == IPPROTO_TCP && recv_sock >= 0 {
        close(recv_sock);
    }
    if s[1] >= 0 {
        close(s[1]);
    }
    if s[0] >= 0 {
        close(s[0]);
    }
    res
}

unsafe fn do_test(proto: c_int) -> c_int {
    let mut res4: c_int;
    let mut res6: c_int;

    res4 = sk_peek_offset_probe(AF_INET as sa_family_t, proto);
    res6 = sk_peek_offset_probe(AF_INET6 as sa_family_t, proto);

    if res4 == 0 && res6 == 0 {
        return KSFT_SKIP;
    }

    if res4 != 0 {
        res4 = sk_peek_offset_test(AF_INET as sa_family_t, proto);
    }

    if res6 != 0 {
        res6 = sk_peek_offset_test(AF_INET6 as sa_family_t, proto);
    }

    if res4 == 0 || res6 == 0 {
        return KSFT_FAIL;
    }

    KSFT_PASS
}

fn main() -> c_int {
    unsafe {
        let restcp: c_int;
        let resudp: c_int;

        restcp = do_test(IPPROTO_TCP);
        resudp = do_test(IPPROTO_UDP);
        if restcp == KSFT_FAIL || resudp == KSFT_FAIL {
            return KSFT_FAIL;
        }

        KSFT_PASS
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
