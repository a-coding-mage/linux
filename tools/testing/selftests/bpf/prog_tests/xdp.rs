// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    static pkt_v4: c_void;
    static pkt_v6: c_void;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn CHECK_FAIL(err: c_int) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char);
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char);
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vip {
    pub protocol: c_uint,
    pub family: c_uint,
}

#[repr(C)]
pub struct iptnl_info {
    pub family: c_uint,
}

#[repr(C)]
pub struct ethhdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ipv6hdr {
    pub nexthdr: u8,
}

#[repr(C)]
pub struct iphdr {
    pub protocol: u8,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: c_uint,
    pub data_out: *mut c_void,
    pub data_size_out: c_uint,
    pub repeat: c_uint,
    pub retval: c_uint,
}

const AF_INET: c_uint = 2;
const AF_INET6: c_uint = 10;
const BPF_PROG_TYPE_XDP: c_int = 6;
const XDP_TX: c_uint = 3;
const IPPROTO_IPIP: c_uint = 4;
const IPPROTO_IPV6: c_uint = 41;

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn test_xdp() {
    let key4 = vip {
        protocol: 6,
        family: AF_INET,
    };
    let key6 = vip {
        protocol: 6,
        family: AF_INET6,
    };
    let value4 = iptnl_info { family: AF_INET };
    let value6 = iptnl_info { family: AF_INET6 };
    let file = cstr(b"./test_xdp.bpf.o\0");
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut buf = [0u8; 128];
    let mut iph6: ipv6hdr = core::mem::zeroed();
    let mut iph: iphdr = core::mem::zeroed();
    let mut prog_fd: c_int = 0;
    let mut err: c_int;
    let map_fd: c_int;
    let mut topts = bpf_test_run_opts {
        data_in: &pkt_v4 as *const c_void,
        data_size_in: size_of_val_raw(&pkt_v4 as *const c_void),
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: size_of::<[u8; 128]>() as c_uint,
        repeat: 1,
        retval: 0,
    };

    err = bpf_prog_test_load(file, BPF_PROG_TYPE_XDP, &mut obj, &mut prog_fd);
    if CHECK_FAIL(err) {
        return;
    }

    map_fd = bpf_find_map(cstr(b"test_xdp\0"), obj, cstr(b"vip2tnl\0"));
    if map_fd < 0 {
        bpf_object__close(obj);
        return;
    }
    bpf_map_update_elem(
        map_fd,
        &key4 as *const vip as *const c_void,
        &value4 as *const iptnl_info as *const c_void,
        0,
    );
    bpf_map_update_elem(
        map_fd,
        &key6 as *const vip as *const c_void,
        &value6 as *const iptnl_info as *const c_void,
        0,
    );

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ptr::copy_nonoverlapping(
        buf.as_ptr().add(size_of::<ethhdr>()),
        &mut iph as *mut iphdr as *mut u8,
        size_of::<iphdr>(),
    );
    ASSERT_OK(err, cstr(b"test_run\0"));
    ASSERT_EQ(topts.retval as u64, XDP_TX as u64, cstr(b"ipv4 test_run retval\0"));
    ASSERT_EQ(
        topts.data_size_out as u64,
        74,
        cstr(b"ipv4 test_run data_size_out\0"),
    );
    ASSERT_EQ(
        iph.protocol as u64,
        IPPROTO_IPIP as u64,
        cstr(b"ipv4 test_run iph.protocol\0"),
    );

    topts.data_in = &pkt_v6 as *const c_void;
    topts.data_size_in = size_of_val_raw(&pkt_v6 as *const c_void);
    topts.data_size_out = size_of::<[u8; 128]>() as c_uint;

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ptr::copy_nonoverlapping(
        buf.as_ptr().add(size_of::<ethhdr>()),
        &mut iph6 as *mut ipv6hdr as *mut u8,
        size_of::<ipv6hdr>(),
    );
    ASSERT_OK(err, cstr(b"test_run\0"));
    ASSERT_EQ(topts.retval as u64, XDP_TX as u64, cstr(b"ipv6 test_run retval\0"));
    ASSERT_EQ(
        topts.data_size_out as u64,
        114,
        cstr(b"ipv6 test_run data_size_out\0"),
    );
    ASSERT_EQ(
        iph6.nexthdr as u64,
        IPPROTO_IPV6 as u64,
        cstr(b"ipv6 test_run iph6.nexthdr\0"),
    );

    bpf_object__close(obj);
}

unsafe fn size_of_val_raw(_ptr: *const c_void) -> c_uint {
    // Translation placeholder for C sizeof(pkt_v4) / sizeof(pkt_v6), supplied by
    // network_helpers.h in the original translation unit.
    0
}
