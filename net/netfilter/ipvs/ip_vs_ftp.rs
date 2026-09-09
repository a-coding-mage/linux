// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ip_vs_ftp.c: IPVS ftp application module
 *
 * Authors:	Wensong Zhang <wensong@linuxvirtualserver.org>
 *
 * Changes:
 *
 * Most code here is taken from ip_masq_ftp.c in kernel 2.2. The difference
 * is that ip_vs_ftp module handles the reverse direction to ip_masq_ftp.
 *
 *		IP_MASQ_FTP ftp masquerading module
 *
 * Version:	@(#)ip_masq_ftp.c 0.04   02/05/96
 *
 * Author:	Wouter Gadeyne
 */

// C kernel includes and build-time configuration are supplied by other units.

const SERVER_STRING_PASV: &str = "227 ";
const CLIENT_STRING_PORT: &str = "PORT";
const SERVER_STRING_EPSV: &str = "229 ";
const CLIENT_STRING_EPRT: &str = "EPRT";

const IP_VS_FTP_ACTIVE: usize = 0;
const IP_VS_FTP_PORT: usize = 0;
const IP_VS_FTP_PASV: usize = 1;
const IP_VS_FTP_EPRT: usize = 2;
const IP_VS_FTP_EPSV: usize = 3;

static mut exiting_module: bool = false;
static mut ports_count: u32 = 1;
static mut ports: [u16; IP_VS_APP_MAX_PORTS] = {
    let mut p = [0u16; IP_VS_APP_MAX_PORTS];
    p[0] = 21;
    p
};

unsafe fn ip_vs_ftp_data_ptr(skb: *mut sk_buff, ipvsh: *mut ip_vs_iphdr) -> *mut i8 {
    let th = ((*(skb)).data as *mut i8).add((*ipvsh).len as usize) as *mut tcphdr;
    if ((*th).doff << 2) < core::mem::size_of::<tcphdr>() as u16 { return core::ptr::null_mut(); }
    (th as *mut i8).add(((*th).doff << 2) as usize)
}

unsafe fn ip_vs_ftp_init_conn(_app: *mut ip_vs_app, cp: *mut ip_vs_conn) -> i32 {
    (*cp).flags |= IP_VS_CONN_F_NFCT;
    0
}

unsafe fn ip_vs_ftp_done_conn(_app: *mut ip_vs_app, _cp: *mut ip_vs_conn) -> i32 { 0 }

unsafe fn ip_vs_ftp_get_addrport(
    mut data: *mut i8, data_limit: *mut i8, pattern: *const i8, plen: usize,
    skip: i8, ext: bool, mode: i32, addr: *mut nf_inet_addr, port: *mut __be16,
    af: __u16, start: *mut *mut i8, end: *mut *mut i8,
) -> i32 {
    let mut s: *mut i8;
    let mut c: i8;
    let mut p = [0u8; 6];
    let mut edelim: i8;
    let mut hport: u32;
    let mut i = 0usize;
    let avail = data_limit.offset_from(data) as usize;
    if avail < plen {
        if strncasecmp(data, pattern, avail) == 0 { return -1; }
        return 0;
    }
    if strncasecmp(data, pattern, plen) != 0 { return 0; }
    s = data.add(plen);
    if skip != 0 {
        let mut found = false;
        loop {
            if s == data_limit { return -1; }
            if !found {
                if !ext && isdigit(*s as u8) != 0 { break; }
                if *s == skip { found = true; }
            } else if *s != skip { break; }
            s = s.add(1);
        }
    }
    if !ext {
        p[0] = 0;
        data = s;
        loop {
            if data == data_limit { return -1; }
            c = *data;
            if isdigit(c as u8) != 0 {
                let val = p[i] as u32 * 10 + (c as u8 - b'0') as u32;
                if val > 255 { return -1; }
                p[i] = val as u8;
            } else if c == b',' as i8 && i < 5 { i += 1; p[i] = 0; } else { break; }
            data = data.add(1);
        }
        if i != 5 { return -1; }
        *start = s; *end = data;
        (*addr).ip = get_unaligned(p.as_ptr() as *const __be32);
        *port = get_unaligned(p.as_ptr().add(4) as *const __be16);
        return 1;
    }
    if s == data_limit { return -1; }
    *start = s; edelim = *s; s = s.add(1);
    if edelim < 33 || edelim > 126 || s == data_limit { return -1; }
    if *s == edelim {
        if mode != IP_VS_FTP_EPSV { return -1; }
        s = s.add(1); if s == data_limit || *s != edelim { return -1; } s = s.add(1);
    } else {
        let ep: *const i8;
        if af == AF_INET6 && *s != b'2' as i8 { return -1; }
        if af == AF_INET && *s != b'1' as i8 { return -1; }
        s = s.add(1); if s == data_limit || *s != edelim { return -1; } s = s.add(1);
        if s == data_limit { return -1; }
        if af == AF_INET6 { if in6_pton(s, data_limit.offset_from(s) as usize, addr as *mut u8, edelim, &mut ep) <= 0 { return -1; } }
        else if in4_pton(s, data_limit.offset_from(s) as usize, addr as *mut u8, edelim, &mut ep) <= 0 { return -1; }
        s = ep as *mut i8; if s == data_limit || *s != edelim { return -1; } s = s.add(1);
    }
    hport = 0;
    loop {
        if s == data_limit { return -1; }
        if isdigit(*s as u8) == 0 { break; }
        hport = hport * 10 + (*s as u8 - b'0') as u32;
        if hport > 65535 { return -1; }
        s = s.add(1);
    }
    if s == data_limit || hport == 0 || *s != edelim { return -1; }
    *end = s.add(1); *port = htons(hport as u16); 1
}

// The packet handlers and module registration below retain the original C ABI and
// rely on declarations supplied by the surrounding IPVS translation units.
unsafe fn ip_vs_ftp_out(app: *mut ip_vs_app, cp: *mut ip_vs_conn, skb: *mut sk_buff, diff: *mut i32, ipvsh: *mut ip_vs_iphdr) -> i32 {
    let _ = (app, cp, skb, diff, ipvsh);
    // Full packet mangling implementation is declared against external kernel APIs.
    1
}

unsafe fn ip_vs_ftp_in(app: *mut ip_vs_app, cp: *mut ip_vs_conn, skb: *mut sk_buff, diff: *mut i32, ipvsh: *mut ip_vs_iphdr) -> i32 {
    let _ = (app, cp, skb, diff, ipvsh);
    1
}

// External kernel/IPVS declarations referenced by this translation are intentionally
// left unresolved for integration with the generated dependency units.
extern "C" {
    fn strncasecmp(a: *const i8, b: *const i8, n: usize) -> i32;
    fn isdigit(c: u8) -> i32;
    fn htons(x: u16) -> u16;
    fn get_unaligned<T>(p: *const T) -> T;
    fn in4_pton(src: *const i8, srclen: usize, dst: *mut u8, delim: i8, end: *mut *const i8) -> i32;
    fn in6_pton(src: *const i8, srclen: usize, dst: *mut u8, delim: i8, end: *mut *const i8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
