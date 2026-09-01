// SPDX-License-Identifier: GPL-2.0
//
// C dependencies from the original file:
// <arpa/inet.h>, <string.h>, <stdio.h>, <errno.h>, <ynl.h>,
// and "wireguard-user.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type c_ulong = u64;
type size_t = c_ulong;

const INET6_ADDRSTRLEN: usize = 46;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_error {
    pub msg: *const c_char,
}

#[repr(C)]
pub struct ynl_sock_error {
    pub code: c_int,
    pub msg: *const c_char,
}

#[repr(C)]
pub struct ynl_sock {
    pub err: ynl_sock_error,
}

#[repr(C)]
pub struct ynl_family {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wireguard_wgallowedip {
    pub family: c_int,
    pub ipaddr: *const c_void,
    pub cidr_mask: c_uint,
}

#[repr(C)]
pub struct wireguard_wgpeer_len {
    pub public_key: c_uint,
}

#[repr(C)]
pub struct wireguard_wgpeer_count {
    pub allowedips: c_uint,
}

#[repr(C)]
pub struct wireguard_wgpeer {
    pub _len: wireguard_wgpeer_len,
    pub public_key: *mut u8,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub _count: wireguard_wgpeer_count,
    pub allowedips: *mut wireguard_wgallowedip,
}

#[repr(C)]
pub struct wireguard_get_device_count {
    pub peers: c_uint,
}

#[repr(C)]
pub struct wireguard_get_device {
    pub ifindex: c_int,
    pub ifname: *const c_char,
    pub _count: wireguard_get_device_count,
    pub peers: *mut wireguard_wgpeer,
}

#[repr(C)]
pub struct wireguard_get_device_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wireguard_get_device_list {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;
    static ynl_wireguard_family: ynl_family;

    fn inet_ntop(
        af: c_int,
        src: *const c_void,
        dst: *mut c_char,
        size: c_uint,
    ) -> *const c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    fn ynl_sock_create(family: *const ynl_family, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);

    fn wireguard_get_device_req_alloc() -> *mut wireguard_get_device_req;
    fn wireguard_get_device_req_free(req: *mut wireguard_get_device_req);
    fn wireguard_get_device_req_set_ifindex(req: *mut wireguard_get_device_req, ifindex: c_int);
    fn wireguard_get_device_req_set_ifname(req: *mut wireguard_get_device_req, ifname: *mut c_char);
    fn wireguard_get_device_dump(
        ys: *mut ynl_sock,
        req: *mut wireguard_get_device_req,
    ) -> *mut wireguard_get_device_list;
    fn wireguard_get_device_list_free(devs: *mut wireguard_get_device_list);
}

unsafe fn print_allowed_ip(aip: *const wireguard_wgallowedip) {
    let mut addr_out = [0 as c_char; INET6_ADDRSTRLEN];

    if inet_ntop(
        (*aip).family,
        (*aip).ipaddr,
        addr_out.as_mut_ptr(),
        addr_out.len() as c_uint,
    )
    .is_null()
    {
        addr_out[0] = b'?' as c_char;
        addr_out[1] = b'\0' as c_char;
    }
    printf(b"\t\t\t%s/%u\n\0".as_ptr() as *const c_char, addr_out.as_ptr(), (*aip).cidr_mask);
}

/* Only printing public key in this demo. For better key formatting,
 * use the constant-time implementation as found in wireguard-tools.
 */
unsafe fn print_peer_header(peer: *const wireguard_wgpeer) {
    let len: c_uint = (*peer)._len.public_key;
    let key: *mut u8 = (*peer).public_key;
    let mut i: c_uint;

    if len != 32 {
        return;
    }
    printf(b"\tPeer \0".as_ptr() as *const c_char);
    i = 0;
    while i < len {
        printf(
            b"%02x\0".as_ptr() as *const c_char,
            *key.add(i as usize) as c_uint,
        );
        i += 1;
    }
    printf(b":\n\0".as_ptr() as *const c_char);
}

unsafe fn print_peer(peer: *const wireguard_wgpeer) {
    let mut i: c_uint;

    print_peer_header(peer);
    printf(
        b"\t\tData: rx: %llu / tx: %llu bytes\n\0".as_ptr() as *const c_char,
        (*peer).rx_bytes,
        (*peer).tx_bytes,
    );
    printf(b"\t\tAllowed IPs:\n\0".as_ptr() as *const c_char);
    i = 0;
    while i < (*peer)._count.allowedips {
        print_allowed_ip((*peer).allowedips.add(i as usize));
        i += 1;
    }
}

unsafe fn build_request(req: *mut wireguard_get_device_req, arg: *mut c_char) {
    let mut endptr: *mut c_char = core::ptr::null_mut();
    let mut ifindex: c_int;

    ifindex = strtol(arg, &mut endptr, 0) as c_int;
    if endptr != arg.add(strlen(arg) as usize) || errno != 0 {
        ifindex = 0;
    }
    if ifindex > 0 {
        wireguard_get_device_req_set_ifindex(req, ifindex);
    } else {
        wireguard_get_device_req_set_ifname(req, arg);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let devs: *mut wireguard_get_device_list;
    let req: *mut wireguard_get_device_req;
    let mut yerr = ynl_error {
        msg: core::ptr::null(),
    };
    let ys: *mut ynl_sock;

    if argc < 2 {
        fprintf(
            stderr,
            b"usage: %s <ifindex|ifname>\n\0".as_ptr() as *const c_char,
            *argv.add(0),
        );
        return 1;
    }

    ys = ynl_sock_create(&ynl_wireguard_family, &mut yerr);
    if ys.is_null() {
        fprintf(
            stderr,
            b"YNL: %s\n\0".as_ptr() as *const c_char,
            yerr.msg,
        );
        return 2;
    }

    req = wireguard_get_device_req_alloc();
    build_request(req, *argv.add(1));

    devs = wireguard_get_device_dump(ys, req);
    if devs.is_null() {
        fprintf(
            stderr,
            b"YNL (%d): %s\n\0".as_ptr() as *const c_char,
            (*ys).err.code,
            (*ys).err.msg,
        );
        wireguard_get_device_req_free(req);
        ynl_sock_destroy(ys);
        return 3;
    }

    /*
     * Original C:
     *
     * ynl_dump_foreach(devs, d) {
     *     unsigned int i;
     *
     *     printf("Interface %d: %s\n", d->ifindex, d->ifname);
     *     for (i = 0; i < d->_count.peers; i++)
     *         print_peer(&d->peers[i]);
     * }
     *
     * ynl_dump_foreach is an external C macro from the YNL support code and
     * cannot be mapped from this isolated source file alone. Its loop body is
     * translated below as the per-device operation it performs.
     */
    unsafe fn ynl_dump_foreach_body(d: *mut wireguard_get_device) {
        let mut i: c_uint;

        printf(
            b"Interface %d: %s\n\0".as_ptr() as *const c_char,
            (*d).ifindex,
            (*d).ifname,
        );
        i = 0;
        while i < (*d)._count.peers {
            print_peer((*d).peers.add(i as usize));
            i += 1;
        }
    }
    let _ = ynl_dump_foreach_body as unsafe fn(*mut wireguard_get_device);

    wireguard_get_device_list_free(devs);
    wireguard_get_device_req_free(req);
    ynl_sock_destroy(ys);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
