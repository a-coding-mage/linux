/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2013 Google Inc.
 * Author: Willem de Bruijn <willemb@google.com>
 *         Daniel Borkmann <dborkman@redhat.com>
 */

/* C header dependencies removed:
 * <sys/types.h>, <sys/socket.h>, <string.h>, <arpa/inet.h>, <unistd.h>,
 * and "kselftest.h".
 */

pub const DATA_LEN: usize = 100;
pub const DATA_CHAR: libc::c_char = b'a' as libc::c_char;
pub const DATA_CHAR_1: libc::c_char = b'b' as libc::c_char;

pub const PORT_BASE: libc::c_int = 8000;

pub unsafe fn pair_udp_setfilter(fd: libc::c_int) {
    /* the filter below checks for all of the following conditions that
     * are based on the contents of create_payload()
     *  ether type 0x800 and
     *  ip proto udp     and
     *  skb->len == DATA_LEN and
     *  udp[38] == 'a' or udp[38] == 'b'
     * It can be generated from the following bpf_asm input:
     *	ldh [12]
     *	jne #0x800, drop	; ETH_P_IP
     *	ldb [23]
     *	jneq #17, drop		; IPPROTO_UDP
     *	ld len			; ld skb->len
     *	jlt #100, drop		; DATA_LEN
     *	ldb [80]
     *	jeq #97, pass		; DATA_CHAR
     *	jne #98, drop		; DATA_CHAR_1
     *	pass:
     *	  ret #-1
     *	drop:
     *	  ret #0
     */
    let mut bpf_filter = [
        libc::sock_filter {
            code: 0x28,
            jt: 0,
            jf: 0,
            k: 0x0000000c,
        },
        libc::sock_filter {
            code: 0x15,
            jt: 0,
            jf: 8,
            k: 0x00000800,
        },
        libc::sock_filter {
            code: 0x30,
            jt: 0,
            jf: 0,
            k: 0x00000017,
        },
        libc::sock_filter {
            code: 0x15,
            jt: 0,
            jf: 6,
            k: 0x00000011,
        },
        libc::sock_filter {
            code: 0x80,
            jt: 0,
            jf: 0,
            k: 0000000000,
        },
        libc::sock_filter {
            code: 0x35,
            jt: 0,
            jf: 4,
            k: 0x00000064,
        },
        libc::sock_filter {
            code: 0x30,
            jt: 0,
            jf: 0,
            k: 0x00000050,
        },
        libc::sock_filter {
            code: 0x15,
            jt: 1,
            jf: 0,
            k: 0x00000061,
        },
        libc::sock_filter {
            code: 0x15,
            jt: 0,
            jf: 1,
            k: 0x00000062,
        },
        libc::sock_filter {
            code: 0x06,
            jt: 0,
            jf: 0,
            k: 0xffffffff,
        },
        libc::sock_filter {
            code: 0x06,
            jt: 0,
            jf: 0,
            k: 0000000000,
        },
    ];
    let mut bpf_prog: libc::sock_fprog = core::mem::zeroed();

    bpf_prog.filter = bpf_filter.as_mut_ptr();
    bpf_prog.len = bpf_filter.len() as libc::c_ushort;

    if libc::setsockopt(
        fd,
        libc::SOL_SOCKET,
        libc::SO_ATTACH_FILTER,
        (&mut bpf_prog as *mut libc::sock_fprog).cast(),
        core::mem::size_of_val(&bpf_prog) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"setsockopt SO_ATTACH_FILTER".as_ptr());
        libc::exit(1);
    }
}

pub unsafe fn pair_udp_open(fds: *mut libc::c_int, port: u16) {
    let mut saddr: libc::sockaddr_in = core::mem::zeroed();
    let mut daddr: libc::sockaddr_in = core::mem::zeroed();

    *fds.add(0) = libc::socket(libc::PF_INET, libc::SOCK_DGRAM, 0);
    *fds.add(1) = libc::socket(libc::PF_INET, libc::SOCK_DGRAM, 0);
    if *fds.add(0) == -1 || *fds.add(1) == -1 {
        libc::fprintf(
            libc::stderr,
            c"ERROR: socket dgram\n".as_ptr(),
        );
        libc::exit(1);
    }

    libc::memset(
        (&mut saddr as *mut libc::sockaddr_in).cast(),
        0,
        core::mem::size_of_val(&saddr),
    );
    saddr.sin_family = libc::AF_INET as libc::sa_family_t;
    saddr.sin_port = libc::htons(port);
    saddr.sin_addr.s_addr = libc::htonl(libc::INADDR_LOOPBACK);

    libc::memset(
        (&mut daddr as *mut libc::sockaddr_in).cast(),
        0,
        core::mem::size_of_val(&daddr),
    );
    daddr.sin_family = libc::AF_INET as libc::sa_family_t;
    daddr.sin_port = libc::htons(port.wrapping_add(1));
    daddr.sin_addr.s_addr = libc::htonl(libc::INADDR_LOOPBACK);

    /* must bind both to get consistent hash result */
    if libc::bind(
        *fds.add(1),
        (&mut daddr as *mut libc::sockaddr_in).cast(),
        core::mem::size_of_val(&daddr) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"bind".as_ptr());
        libc::exit(1);
    }
    if libc::bind(
        *fds.add(0),
        (&mut saddr as *mut libc::sockaddr_in).cast(),
        core::mem::size_of_val(&saddr) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"bind".as_ptr());
        libc::exit(1);
    }
    if libc::connect(
        *fds.add(0),
        (&mut daddr as *mut libc::sockaddr_in).cast(),
        core::mem::size_of_val(&daddr) as libc::socklen_t,
    ) != 0
    {
        libc::perror(c"connect".as_ptr());
        libc::exit(1);
    }
}

pub unsafe fn pair_udp_send_char(fds: *mut libc::c_int, mut num: libc::c_int, payload: libc::c_char) {
    let mut buf = [0 as libc::c_char; DATA_LEN];
    let mut rbuf = [0 as libc::c_char; DATA_LEN];

    libc::memset(
        buf.as_mut_ptr().cast(),
        payload as libc::c_int,
        core::mem::size_of_val(&buf),
    );
    while {
        let old_num = num;
        num -= 1;
        old_num != 0
    } {
        /* Should really handle EINTR and EAGAIN */
        if libc::write(
            *fds.add(0),
            buf.as_ptr().cast(),
            core::mem::size_of_val(&buf),
        ) != core::mem::size_of_val(&buf) as libc::ssize_t
        {
            libc::fprintf(
                libc::stderr,
                c"ERROR: send failed left=%d\n".as_ptr(),
                num,
            );
            libc::exit(1);
        }
        if libc::read(
            *fds.add(1),
            rbuf.as_mut_ptr().cast(),
            core::mem::size_of_val(&rbuf),
        ) != core::mem::size_of_val(&rbuf) as libc::ssize_t
        {
            libc::fprintf(
                libc::stderr,
                c"ERROR: recv failed left=%d\n".as_ptr(),
                num,
            );
            libc::exit(1);
        }
        if libc::memcmp(
            buf.as_ptr().cast(),
            rbuf.as_ptr().cast(),
            core::mem::size_of_val(&buf),
        ) != 0
        {
            libc::fprintf(
                libc::stderr,
                c"ERROR: data failed left=%d\n".as_ptr(),
                num,
            );
            libc::exit(1);
        }
    }
}

pub unsafe fn pair_udp_send(fds: *mut libc::c_int, num: libc::c_int) {
    return pair_udp_send_char(fds, num, DATA_CHAR);
}

pub unsafe fn pair_udp_close(fds: *mut libc::c_int) {
    libc::close(*fds.add(0));
    libc::close(*fds.add(1));
}
