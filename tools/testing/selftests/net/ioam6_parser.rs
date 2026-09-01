// SPDX-License-Identifier: GPL-2.0+
/*
 * Author: Justin Iurman (justin.iurman@uliege.be)
 *
 * IOAM tester for IPv6, see ioam6.sh for details on each test case.
 */
// C dependencies removed from executable Rust: arpa/inet.h, errno.h, limits.h,
// linux/const.h, linux/if_ether.h, linux/ioam6.h, linux/ipv6.h, stdbool.h,
// stdlib.h, string.h, unistd.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

const AF_INET6: c_int = 10;
const AF_PACKET: c_int = 17;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_BINDTODEVICE: c_int = 25;
const SO_RCVTIMEO: c_int = 20;
const IPPROTO_IPV6: __u8 = 41;
const IPPROTO_ICMPV6: __u8 = 58;
const IPPROTO_HOPOPTS: __u8 = 0;
const ETH_P_IPV6: __u16 = 0x86DD;
const IPV6_TLV_PADN: __u8 = 1;
const IPV6_TLV_IOAM: __u8 = 49;
const IOAM6_TYPE_PREALLOC: __u8 = 0;
const ULONG_MAX: c_ulong = c_ulong::MAX;
const ERANGE: c_int = 34;

unsafe extern "C" {
    static mut errno: c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn recv(socket: c_int, buffer: *mut c_void, length: usize, flags: c_int) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr32: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
struct ipv6hdr {
    priority_version: __u8,
    flow_lbl: [__u8; 3],
    payload_len: __u16,
    nexthdr: __u8,
    hop_limit: __u8,
    saddr: in6_addr,
    daddr: in6_addr,
}

#[repr(C)]
struct ipv6_hopopt_hdr {
    nexthdr: __u8,
    hdrlen: __u8,
}

#[repr(C)]
struct ioam6_hdr {
    opt_type: __u8,
    opt_len: __u8,
    type_: __u8,
    reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ioam6_trace_type {
    bits: __u32,
}

impl ioam6_trace_type {
    unsafe fn bit(&self, bit: u32) -> bool {
        (__be32_to_cpu(self.bits) & (1u32 << (31 - bit))) != 0
    }
}

#[repr(C)]
struct ioam6_trace_hdr {
    namespace_id: __u16,
    nodelen: __u8,
    overflow_remlen: __u8,
    type_be32: __u32,
    type_: ioam6_trace_type,
    data: [__u8; 0],
}

impl ioam6_trace_hdr {
    unsafe fn overflow(&self) -> __u8 {
        self.overflow_remlen >> 7
    }

    unsafe fn remlen(&self) -> __u8 {
        self.overflow_remlen & 0x7f
    }
}

#[repr(C)]
struct ioam_config {
    id: __u32,
    wide: __u64,
    ingr_id: __u16,
    egr_id: __u16,
    ingr_wide: __u32,
    egr_wide: __u32,
    ns_data: __u32,
    ns_wide: __u64,
    sc_id: __u32,
    hlim: __u8,
    sc_data: *const c_char,
}

/*
 * Be careful if you modify structs below - everything MUST be kept synchronized
 * with configurations inside ioam6.sh and always reflect the same.
 */

static mut node1: ioam_config = ioam_config {
    id: 1,
    wide: 11111111,
    ingr_id: 0xffff, /* default value */
    egr_id: 101,
    ingr_wide: 0xffffffff, /* default value */
    egr_wide: 101101,
    ns_data: 0xdeadbeef,
    ns_wide: 0xcafec0caf00dc0de,
    sc_id: 777,
    sc_data: b"something that will be 4n-aligned\0".as_ptr() as *const c_char,
    hlim: 64,
};

static mut node2: ioam_config = ioam_config {
    id: 2,
    wide: 22222222,
    ingr_id: 201,
    egr_id: 202,
    ingr_wide: 201201,
    egr_wide: 202202,
    ns_data: 0xffffffff, /* default value */
    ns_wide: 0xffffffffffffffff, /* default value */
    sc_id: 0xffffff, /* default value */
    sc_data: ptr::null(),
    hlim: 63,
};

#[repr(i32)]
enum test_id {
    __TEST_OUT_MIN,
    TEST_OUT_UNDEF_NS,
    TEST_OUT_NO_ROOM,
    TEST_OUT_NO_ROOM_OSS,
    TEST_OUT_BIT0,
    TEST_OUT_BIT1,
    TEST_OUT_BIT2,
    TEST_OUT_BIT3,
    TEST_OUT_BIT4,
    TEST_OUT_BIT5,
    TEST_OUT_BIT6,
    TEST_OUT_BIT7,
    TEST_OUT_BIT8,
    TEST_OUT_BIT9,
    TEST_OUT_BIT10,
    TEST_OUT_BIT11,
    TEST_OUT_BIT22,
    TEST_OUT_SIZE4,
    TEST_OUT_SIZE8,
    TEST_OUT_SIZE12,
    TEST_OUT_SIZE16,
    TEST_OUT_SIZE20,
    TEST_OUT_SIZE24,
    TEST_OUT_SIZE28,
    TEST_OUT_SIZE32,
    TEST_OUT_SIZE36,
    TEST_OUT_SIZE40,
    TEST_OUT_SIZE44,
    TEST_OUT_SIZE48,
    TEST_OUT_SIZE52,
    TEST_OUT_SIZE56,
    TEST_OUT_SIZE60,
    TEST_OUT_SIZE64,
    TEST_OUT_SIZE68,
    TEST_OUT_SIZE72,
    TEST_OUT_SIZE76,
    TEST_OUT_SIZE80,
    TEST_OUT_SIZE84,
    TEST_OUT_SIZE88,
    TEST_OUT_SIZE92,
    TEST_OUT_SIZE96,
    TEST_OUT_SIZE100,
    TEST_OUT_SIZE104,
    TEST_OUT_SIZE108,
    TEST_OUT_SIZE112,
    TEST_OUT_SIZE116,
    TEST_OUT_SIZE120,
    TEST_OUT_SIZE124,
    TEST_OUT_SIZE128,
    TEST_OUT_SIZE132,
    TEST_OUT_SIZE136,
    TEST_OUT_SIZE140,
    TEST_OUT_SIZE144,
    TEST_OUT_SIZE148,
    TEST_OUT_SIZE152,
    TEST_OUT_SIZE156,
    TEST_OUT_SIZE160,
    TEST_OUT_SIZE164,
    TEST_OUT_SIZE168,
    TEST_OUT_SIZE172,
    TEST_OUT_SIZE176,
    TEST_OUT_SIZE180,
    TEST_OUT_SIZE184,
    TEST_OUT_SIZE188,
    TEST_OUT_SIZE192,
    TEST_OUT_SIZE196,
    TEST_OUT_SIZE200,
    TEST_OUT_SIZE204,
    TEST_OUT_SIZE208,
    TEST_OUT_SIZE212,
    TEST_OUT_SIZE216,
    TEST_OUT_SIZE220,
    TEST_OUT_SIZE224,
    TEST_OUT_SIZE228,
    TEST_OUT_SIZE232,
    TEST_OUT_SIZE236,
    TEST_OUT_SIZE240,
    TEST_OUT_SIZE244,
    TEST_OUT_FULL_SUPP_TRACE,
    __TEST_OUT_MAX,
    __TEST_IN_MIN,
    TEST_IN_UNDEF_NS,
    TEST_IN_NO_ROOM,
    TEST_IN_NO_ROOM_OSS,
    TEST_IN_DISABLED,
    TEST_IN_OFLAG,
    TEST_IN_BIT0,
    TEST_IN_BIT1,
    TEST_IN_BIT2,
    TEST_IN_BIT3,
    TEST_IN_BIT4,
    TEST_IN_BIT5,
    TEST_IN_BIT6,
    TEST_IN_BIT7,
    TEST_IN_BIT8,
    TEST_IN_BIT9,
    TEST_IN_BIT10,
    TEST_IN_BIT11,
    TEST_IN_BIT22,
    TEST_IN_SIZE4,
    TEST_IN_SIZE8,
    TEST_IN_SIZE12,
    TEST_IN_SIZE16,
    TEST_IN_SIZE20,
    TEST_IN_SIZE24,
    TEST_IN_SIZE28,
    TEST_IN_SIZE32,
    TEST_IN_SIZE36,
    TEST_IN_SIZE40,
    TEST_IN_SIZE44,
    TEST_IN_SIZE48,
    TEST_IN_SIZE52,
    TEST_IN_SIZE56,
    TEST_IN_SIZE60,
    TEST_IN_SIZE64,
    TEST_IN_SIZE68,
    TEST_IN_SIZE72,
    TEST_IN_SIZE76,
    TEST_IN_SIZE80,
    TEST_IN_SIZE84,
    TEST_IN_SIZE88,
    TEST_IN_SIZE92,
    TEST_IN_SIZE96,
    TEST_IN_SIZE100,
    TEST_IN_SIZE104,
    TEST_IN_SIZE108,
    TEST_IN_SIZE112,
    TEST_IN_SIZE116,
    TEST_IN_SIZE120,
    TEST_IN_SIZE124,
    TEST_IN_SIZE128,
    TEST_IN_SIZE132,
    TEST_IN_SIZE136,
    TEST_IN_SIZE140,
    TEST_IN_SIZE144,
    TEST_IN_SIZE148,
    TEST_IN_SIZE152,
    TEST_IN_SIZE156,
    TEST_IN_SIZE160,
    TEST_IN_SIZE164,
    TEST_IN_SIZE168,
    TEST_IN_SIZE172,
    TEST_IN_SIZE176,
    TEST_IN_SIZE180,
    TEST_IN_SIZE184,
    TEST_IN_SIZE188,
    TEST_IN_SIZE192,
    TEST_IN_SIZE196,
    TEST_IN_SIZE200,
    TEST_IN_SIZE204,
    TEST_IN_SIZE208,
    TEST_IN_SIZE212,
    TEST_IN_SIZE216,
    TEST_IN_SIZE220,
    TEST_IN_SIZE224,
    TEST_IN_SIZE228,
    TEST_IN_SIZE232,
    TEST_IN_SIZE236,
    TEST_IN_SIZE240,
    TEST_IN_SIZE244,
    TEST_IN_FULL_SUPP_TRACE,
    __TEST_IN_MAX,
    __TEST_MAX,
}

use test_id::*;

unsafe fn __be16_to_cpu(x: __u16) -> __u16 {
    __u16::from_be(x)
}
unsafe fn __be32_to_cpu(x: __u32) -> __u32 {
    __u32::from_be(x)
}
unsafe fn __be64_to_cpu(x: __u64) -> __u64 {
    __u64::from_be(x)
}
unsafe fn __cpu_to_be16(x: __u16) -> __u16 {
    x.to_be()
}
fn __ALIGN_KERNEL(x: usize, a: usize) -> usize {
    (x + (a - 1)) & !(a - 1)
}

unsafe fn is_out_size(tid: c_int) -> bool {
    tid >= TEST_OUT_SIZE4 as c_int && tid <= TEST_OUT_SIZE244 as c_int
}

unsafe fn is_in_size(tid: c_int) -> bool {
    tid >= TEST_IN_SIZE4 as c_int && tid <= TEST_IN_SIZE244 as c_int
}

unsafe fn check_header(
    tid: c_int,
    trace: *mut ioam6_trace_hdr,
    trace_type: __u32,
    trace_size: __u8,
    ioam_ns: __u16,
) -> c_int {
    if __be16_to_cpu((*trace).namespace_id) != ioam_ns
        || __be32_to_cpu((*trace).type_be32) != (trace_type << 8)
    {
        return 1;
    }

    if tid == TEST_OUT_UNDEF_NS as c_int
        || tid == TEST_IN_UNDEF_NS as c_int
        || tid == TEST_IN_DISABLED as c_int
    {
        return ((*trace).overflow() == 1 || (*trace).nodelen != 1 || (*trace).remlen() != 1) as c_int;
    }

    if tid == TEST_OUT_NO_ROOM as c_int
        || tid == TEST_IN_NO_ROOM as c_int
        || tid == TEST_IN_OFLAG as c_int
    {
        return ((*trace).overflow() == 0 || (*trace).nodelen != 2 || (*trace).remlen() != 1) as c_int;
    }

    if tid == TEST_OUT_NO_ROOM_OSS as c_int {
        return ((*trace).overflow() == 0 || (*trace).nodelen != 0 || (*trace).remlen() != 1) as c_int;
    }

    if tid == TEST_IN_NO_ROOM_OSS as c_int
        || tid == TEST_OUT_BIT22 as c_int
        || tid == TEST_IN_BIT22 as c_int
    {
        return ((*trace).overflow() == 1 || (*trace).nodelen != 0 || (*trace).remlen() != 0) as c_int;
    }

    if (tid >= TEST_OUT_BIT0 as c_int && tid <= TEST_OUT_BIT7 as c_int)
        || tid == TEST_OUT_BIT11 as c_int
        || (tid >= TEST_IN_BIT0 as c_int && tid <= TEST_IN_BIT7 as c_int)
        || tid == TEST_IN_BIT11 as c_int
    {
        return ((*trace).overflow() == 1 || (*trace).nodelen != 1 || (*trace).remlen() != 0) as c_int;
    }

    if (tid >= TEST_OUT_BIT8 as c_int && tid <= TEST_OUT_BIT10 as c_int)
        || (tid >= TEST_IN_BIT8 as c_int && tid <= TEST_IN_BIT10 as c_int)
    {
        return ((*trace).overflow() == 1 || (*trace).nodelen != 2 || (*trace).remlen() != 0) as c_int;
    }

    if is_out_size(tid) {
        return ((*trace).overflow() == 1
            || (*trace).nodelen != 1
            || (*trace).remlen() != trace_size / 4) as c_int;
    }

    if is_in_size(tid) {
        return ((*trace).overflow() == 1
            || (*trace).nodelen != 1
            || (*trace).remlen() != (trace_size / 4).wrapping_sub((*trace).nodelen)) as c_int;
    }

    if tid == TEST_OUT_FULL_SUPP_TRACE as c_int || tid == TEST_IN_FULL_SUPP_TRACE as c_int {
        return ((*trace).overflow() == 1 || (*trace).nodelen != 15 || (*trace).remlen() != 0) as c_int;
    }

    1
}

unsafe fn read_be32(p: *const __u8) -> __u32 {
    __be32_to_cpu(ptr::read_unaligned(p as *const __u32))
}

unsafe fn read_be64(p: *const __u8) -> __u64 {
    __be64_to_cpu(ptr::read_unaligned(p as *const __u64))
}

unsafe fn check_data(
    trace: *mut ioam6_trace_hdr,
    trace_size: __u8,
    cnf: ioam_config,
    is_output: bool,
) -> c_int {
    let mut len: usize;
    let mut i: u32;
    let mut aligned: __u8;
    let mut raw64: __u64;
    let mut raw32: __u32;
    let mut p: *mut __u8;

    if (*trace).type_.bit(12)
        || (*trace).type_.bit(13)
        || (*trace).type_.bit(14)
        || (*trace).type_.bit(15)
        || (*trace).type_.bit(16)
        || (*trace).type_.bit(17)
        || (*trace).type_.bit(18)
        || (*trace).type_.bit(19)
        || (*trace).type_.bit(20)
        || (*trace).type_.bit(21)
        || (*trace).type_.bit(23)
    {
        return 1;
    }

    i = 0;
    while i < ((*trace).remlen() as u32) * 4 {
        if *(*trace).data.as_ptr().add(i as usize) != 0 {
            return 1;
        }
        i += 1;
    }

    if (*trace).remlen() * 4 == trace_size {
        return 0;
    }

    p = (*trace).data.as_mut_ptr().add(((*trace).remlen() as usize) * 4);

    if (*trace).type_.bit(0) {
        raw32 = read_be32(p);
        if cnf.hlim as u32 != raw32 >> 24 || cnf.id != (raw32 & 0xffffff) {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(1) {
        raw32 = read_be32(p);
        if cnf.ingr_id as u32 != raw32 >> 16 || cnf.egr_id as u32 != (raw32 & 0xffff) {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(2) {
        raw32 = read_be32(p);
        if (is_output && raw32 != 0xffffffff) || (!is_output && (raw32 == 0 || raw32 == 0xffffffff)) {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(3) {
        raw32 = read_be32(p);
        if (is_output && raw32 != 0xffffffff) || (!is_output && (raw32 == 0 || raw32 == 0xffffffff)) {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(4) {
        if read_be32(p) != 0xffffffff {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(5) {
        if read_be32(p) != cnf.ns_data {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(6) {
        if read_be32(p) == 0xffffffff {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(7) {
        if read_be32(p) != 0xffffffff {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(8) {
        raw64 = read_be64(p);
        if cnf.hlim as u64 != raw64 >> 56 || cnf.wide != (raw64 & 0xffffffffffffff) {
            return 1;
        }
        p = p.add(size_of::<__u64>());
    }

    if (*trace).type_.bit(9) {
        if read_be32(p) != cnf.ingr_wide {
            return 1;
        }
        p = p.add(size_of::<__u32>());

        if read_be32(p) != cnf.egr_wide {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(10) {
        if read_be64(p) != cnf.ns_wide {
            return 1;
        }
        p = p.add(size_of::<__u64>());
    }

    if (*trace).type_.bit(11) {
        if read_be32(p) != 0xffffffff {
            return 1;
        }
        p = p.add(size_of::<__u32>());
    }

    if (*trace).type_.bit(22) {
        len = if !cnf.sc_data.is_null() { strlen(cnf.sc_data) } else { 0 };
        aligned = if !cnf.sc_data.is_null() {
            __ALIGN_KERNEL(len, 4) as __u8
        } else {
            0
        };

        raw32 = read_be32(p);
        if aligned as u32 != (raw32 >> 24) * 4 || cnf.sc_id != (raw32 & 0xffffff) {
            return 1;
        }
        p = p.add(size_of::<__u32>());

        if !cnf.sc_data.is_null() {
            if strncmp(p as *const c_char, cnf.sc_data, len) != 0 {
                return 1;
            }

            p = p.add(len);
            aligned = aligned.wrapping_sub(len as __u8);

            while aligned != 0 {
                aligned = aligned.wrapping_sub(1);
                if *p != b'\0' {
                    return 1;
                }
                p = p.add(size_of::<__u8>());
            }
        }
    }

    0
}

unsafe fn check_ioam_trace(
    tid: c_int,
    trace: *mut ioam6_trace_hdr,
    trace_type: __u32,
    trace_size: __u8,
    ioam_ns: __u16,
) -> c_int {
    if check_header(tid, trace, trace_type, trace_size, ioam_ns) != 0 {
        return 1;
    }

    if tid > __TEST_OUT_MIN as c_int && tid < __TEST_OUT_MAX as c_int {
        return check_data(trace, trace_size, node1, true);
    }

    if tid > __TEST_IN_MIN as c_int && tid < __TEST_IN_MAX as c_int {
        return check_data(trace, trace_size, node2, false);
    }

    1
}

unsafe fn cstr_eq(s: *const c_char, lit: &'static [u8]) -> bool {
    strcmp(lit.as_ptr() as *const c_char, s) == 0
}

unsafe fn str2id(tname: *const c_char) -> c_int {
    macro_rules! map {
        ($name:literal, $id:ident) => {
            if cstr_eq(tname, concat!($name, "\0").as_bytes()) {
                return $id as c_int;
            }
        };
    }

    map!("output_undef_ns", TEST_OUT_UNDEF_NS);
    map!("output_no_room", TEST_OUT_NO_ROOM);
    map!("output_no_room_oss", TEST_OUT_NO_ROOM_OSS);
    map!("output_bit0", TEST_OUT_BIT0);
    map!("output_bit1", TEST_OUT_BIT1);
    map!("output_bit2", TEST_OUT_BIT2);
    map!("output_bit3", TEST_OUT_BIT3);
    map!("output_bit4", TEST_OUT_BIT4);
    map!("output_bit5", TEST_OUT_BIT5);
    map!("output_bit6", TEST_OUT_BIT6);
    map!("output_bit7", TEST_OUT_BIT7);
    map!("output_bit8", TEST_OUT_BIT8);
    map!("output_bit9", TEST_OUT_BIT9);
    map!("output_bit10", TEST_OUT_BIT10);
    map!("output_bit11", TEST_OUT_BIT11);
    map!("output_bit22", TEST_OUT_BIT22);
    map!("output_size4", TEST_OUT_SIZE4);
    map!("output_size8", TEST_OUT_SIZE8);
    map!("output_size12", TEST_OUT_SIZE12);
    map!("output_size16", TEST_OUT_SIZE16);
    map!("output_size20", TEST_OUT_SIZE20);
    map!("output_size24", TEST_OUT_SIZE24);
    map!("output_size28", TEST_OUT_SIZE28);
    map!("output_size32", TEST_OUT_SIZE32);
    map!("output_size36", TEST_OUT_SIZE36);
    map!("output_size40", TEST_OUT_SIZE40);
    map!("output_size44", TEST_OUT_SIZE44);
    map!("output_size48", TEST_OUT_SIZE48);
    map!("output_size52", TEST_OUT_SIZE52);
    map!("output_size56", TEST_OUT_SIZE56);
    map!("output_size60", TEST_OUT_SIZE60);
    map!("output_size64", TEST_OUT_SIZE64);
    map!("output_size68", TEST_OUT_SIZE68);
    map!("output_size72", TEST_OUT_SIZE72);
    map!("output_size76", TEST_OUT_SIZE76);
    map!("output_size80", TEST_OUT_SIZE80);
    map!("output_size84", TEST_OUT_SIZE84);
    map!("output_size88", TEST_OUT_SIZE88);
    map!("output_size92", TEST_OUT_SIZE92);
    map!("output_size96", TEST_OUT_SIZE96);
    map!("output_size100", TEST_OUT_SIZE100);
    map!("output_size104", TEST_OUT_SIZE104);
    map!("output_size108", TEST_OUT_SIZE108);
    map!("output_size112", TEST_OUT_SIZE112);
    map!("output_size116", TEST_OUT_SIZE116);
    map!("output_size120", TEST_OUT_SIZE120);
    map!("output_size124", TEST_OUT_SIZE124);
    map!("output_size128", TEST_OUT_SIZE128);
    map!("output_size132", TEST_OUT_SIZE132);
    map!("output_size136", TEST_OUT_SIZE136);
    map!("output_size140", TEST_OUT_SIZE140);
    map!("output_size144", TEST_OUT_SIZE144);
    map!("output_size148", TEST_OUT_SIZE148);
    map!("output_size152", TEST_OUT_SIZE152);
    map!("output_size156", TEST_OUT_SIZE156);
    map!("output_size160", TEST_OUT_SIZE160);
    map!("output_size164", TEST_OUT_SIZE164);
    map!("output_size168", TEST_OUT_SIZE168);
    map!("output_size172", TEST_OUT_SIZE172);
    map!("output_size176", TEST_OUT_SIZE176);
    map!("output_size180", TEST_OUT_SIZE180);
    map!("output_size184", TEST_OUT_SIZE184);
    map!("output_size188", TEST_OUT_SIZE188);
    map!("output_size192", TEST_OUT_SIZE192);
    map!("output_size196", TEST_OUT_SIZE196);
    map!("output_size200", TEST_OUT_SIZE200);
    map!("output_size204", TEST_OUT_SIZE204);
    map!("output_size208", TEST_OUT_SIZE208);
    map!("output_size212", TEST_OUT_SIZE212);
    map!("output_size216", TEST_OUT_SIZE216);
    map!("output_size220", TEST_OUT_SIZE220);
    map!("output_size224", TEST_OUT_SIZE224);
    map!("output_size228", TEST_OUT_SIZE228);
    map!("output_size232", TEST_OUT_SIZE232);
    map!("output_size236", TEST_OUT_SIZE236);
    map!("output_size240", TEST_OUT_SIZE240);
    map!("output_size244", TEST_OUT_SIZE244);
    map!("output_full_supp_trace", TEST_OUT_FULL_SUPP_TRACE);
    map!("input_undef_ns", TEST_IN_UNDEF_NS);
    map!("input_no_room", TEST_IN_NO_ROOM);
    map!("input_no_room_oss", TEST_IN_NO_ROOM_OSS);
    map!("input_disabled", TEST_IN_DISABLED);
    map!("input_oflag", TEST_IN_OFLAG);
    map!("input_bit0", TEST_IN_BIT0);
    map!("input_bit1", TEST_IN_BIT1);
    map!("input_bit2", TEST_IN_BIT2);
    map!("input_bit3", TEST_IN_BIT3);
    map!("input_bit4", TEST_IN_BIT4);
    map!("input_bit5", TEST_IN_BIT5);
    map!("input_bit6", TEST_IN_BIT6);
    map!("input_bit7", TEST_IN_BIT7);
    map!("input_bit8", TEST_IN_BIT8);
    map!("input_bit9", TEST_IN_BIT9);
    map!("input_bit10", TEST_IN_BIT10);
    map!("input_bit11", TEST_IN_BIT11);
    map!("input_bit22", TEST_IN_BIT22);
    map!("input_size4", TEST_IN_SIZE4);
    map!("input_size8", TEST_IN_SIZE8);
    map!("input_size12", TEST_IN_SIZE12);
    map!("input_size16", TEST_IN_SIZE16);
    map!("input_size20", TEST_IN_SIZE20);
    map!("input_size24", TEST_IN_SIZE24);
    map!("input_size28", TEST_IN_SIZE28);
    map!("input_size32", TEST_IN_SIZE32);
    map!("input_size36", TEST_IN_SIZE36);
    map!("input_size40", TEST_IN_SIZE40);
    map!("input_size44", TEST_IN_SIZE44);
    map!("input_size48", TEST_IN_SIZE48);
    map!("input_size52", TEST_IN_SIZE52);
    map!("input_size56", TEST_IN_SIZE56);
    map!("input_size60", TEST_IN_SIZE60);
    map!("input_size64", TEST_IN_SIZE64);
    map!("input_size68", TEST_IN_SIZE68);
    map!("input_size72", TEST_IN_SIZE72);
    map!("input_size76", TEST_IN_SIZE76);
    map!("input_size80", TEST_IN_SIZE80);
    map!("input_size84", TEST_IN_SIZE84);
    map!("input_size88", TEST_IN_SIZE88);
    map!("input_size92", TEST_IN_SIZE92);
    map!("input_size96", TEST_IN_SIZE96);
    map!("input_size100", TEST_IN_SIZE100);
    map!("input_size104", TEST_IN_SIZE104);
    map!("input_size108", TEST_IN_SIZE108);
    map!("input_size112", TEST_IN_SIZE112);
    map!("input_size116", TEST_IN_SIZE116);
    map!("input_size120", TEST_IN_SIZE120);
    map!("input_size124", TEST_IN_SIZE124);
    map!("input_size128", TEST_IN_SIZE128);
    map!("input_size132", TEST_IN_SIZE132);
    map!("input_size136", TEST_IN_SIZE136);
    map!("input_size140", TEST_IN_SIZE140);
    map!("input_size144", TEST_IN_SIZE144);
    map!("input_size148", TEST_IN_SIZE148);
    map!("input_size152", TEST_IN_SIZE152);
    map!("input_size156", TEST_IN_SIZE156);
    map!("input_size160", TEST_IN_SIZE160);
    map!("input_size164", TEST_IN_SIZE164);
    map!("input_size168", TEST_IN_SIZE168);
    map!("input_size172", TEST_IN_SIZE172);
    map!("input_size176", TEST_IN_SIZE176);
    map!("input_size180", TEST_IN_SIZE180);
    map!("input_size184", TEST_IN_SIZE184);
    map!("input_size188", TEST_IN_SIZE188);
    map!("input_size192", TEST_IN_SIZE192);
    map!("input_size196", TEST_IN_SIZE196);
    map!("input_size200", TEST_IN_SIZE200);
    map!("input_size204", TEST_IN_SIZE204);
    map!("input_size208", TEST_IN_SIZE208);
    map!("input_size212", TEST_IN_SIZE212);
    map!("input_size216", TEST_IN_SIZE216);
    map!("input_size220", TEST_IN_SIZE220);
    map!("input_size224", TEST_IN_SIZE224);
    map!("input_size228", TEST_IN_SIZE228);
    map!("input_size232", TEST_IN_SIZE232);
    map!("input_size236", TEST_IN_SIZE236);
    map!("input_size240", TEST_IN_SIZE240);
    map!("input_size244", TEST_IN_SIZE244);
    map!("input_full_supp_trace", TEST_IN_FULL_SUPP_TRACE);

    -1
}

unsafe fn ipv6_addr_equal(a1: *const in6_addr, a2: *const in6_addr) -> c_int {
    (((*a1).s6_addr32[0] ^ (*a2).s6_addr32[0])
        | ((*a1).s6_addr32[1] ^ (*a2).s6_addr32[1])
        | ((*a1).s6_addr32[2] ^ (*a2).s6_addr32[2])
        | ((*a1).s6_addr32[3] ^ (*a2).s6_addr32[3])
        == 0) as c_int
}

unsafe fn get_u32(val: *mut __u32, arg: *const c_char, base: c_int) -> c_int {
    let res: c_ulong;
    let mut ptr_: *mut c_char = ptr::null_mut();

    if arg.is_null() || *arg == 0 {
        return -1;
    }
    res = strtoul(arg, &mut ptr_, base);

    if ptr_.is_null() || ptr_ == arg as *mut c_char || *ptr_ != 0 {
        return -1;
    }

    if res == ULONG_MAX && errno == ERANGE {
        return -1;
    }

    if res > 0xFFFFFFFFu64 as c_ulong {
        return -1;
    }

    *val = res as __u32;
    0
}

unsafe fn get_u16(val: *mut __u16, arg: *const c_char, base: c_int) -> c_int {
    let res: c_ulong;
    let mut ptr_: *mut c_char = ptr::null_mut();

    if arg.is_null() || *arg == 0 {
        return -1;
    }
    res = strtoul(arg, &mut ptr_, base);

    if ptr_.is_null() || ptr_ == arg as *mut c_char || *ptr_ != 0 {
        return -1;
    }

    if res == ULONG_MAX && errno == ERANGE {
        return -1;
    }

    if res > 0xFFFFu64 as c_ulong {
        return -1;
    }

    *val = res as __u16;
    0
}

unsafe fn get_u8(val: *mut __u8, arg: *const c_char, base: c_int) -> c_int {
    let res: c_ulong;
    let mut ptr_: *mut c_char = ptr::null_mut();

    if arg.is_null() || *arg == 0 {
        return -1;
    }
    res = strtoul(arg, &mut ptr_, base);

    if ptr_.is_null() || ptr_ == arg as *mut c_char || *ptr_ != 0 {
        return -1;
    }

    if res == ULONG_MAX && errno == ERANGE {
        return -1;
    }

    if res > 0xFFu64 as c_ulong {
        return -1;
    }

    *val = res as __u8;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut buffer: [__u8; 512] = [0; 512];
    let mut ptr_: *mut __u8;
    let mut nexthdr: __u8;
    let mut tr_size: __u8 = 0;
    let mut trace: *mut ioam6_trace_hdr;
    let mut hoplen: u32;
    let mut ret: u32 = 1;
    let mut hbh: *mut ipv6_hopopt_hdr;
    let mut fd: c_int;
    let mut size: isize;
    let mut testname_id: c_int;
    let mut src: in6_addr = in6_addr { s6_addr32: [0; 4] };
    let mut dst: in6_addr = in6_addr { s6_addr32: [0; 4] };
    let mut ioam6: *mut ioam6_hdr;
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ipv6: *mut ipv6hdr;
    let mut tr_type: __u32 = 0;
    let mut ioam_ns: __u16 = 0;

    if argc != 9 {
        return ret as c_int;
    }

    testname_id = str2id(*argv.add(2));

    if testname_id < 0
        || inet_pton(AF_INET6, *argv.add(3), &mut src as *mut _ as *mut c_void) != 1
        || inet_pton(AF_INET6, *argv.add(4), &mut dst as *mut _ as *mut c_void) != 1
        || get_u32(&mut tr_type, *argv.add(5), 16) != 0
        || get_u8(&mut tr_size, *argv.add(6), 0) != 0
        || get_u16(&mut ioam_ns, *argv.add(7), 0) != 0
    {
        return ret as c_int;
    }

    nexthdr = if strcmp(*argv.add(8), b"encap\0".as_ptr() as *const c_char) == 0 {
        IPPROTO_IPV6
    } else {
        IPPROTO_ICMPV6
    };

    hoplen = size_of::<ipv6_hopopt_hdr>() as u32;
    hoplen += 2; // 2-byte padding for alignment
    hoplen += size_of::<ioam6_hdr>() as u32; // IOAM option header
    hoplen += size_of::<ioam6_trace_hdr>() as u32; // IOAM trace header
    hoplen += tr_size as u32; // IOAM trace size
    hoplen += (tr_size % 8) as u32; // optional padding

    fd = socket(AF_PACKET, SOCK_DGRAM, __cpu_to_be16(ETH_P_IPV6) as c_int);
    if fd < 0 {
        return ret as c_int;
    }

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        *argv.add(1) as *const c_void,
        strlen(*argv.add(1)) as u32,
    ) != 0
    {
        close(fd);
        return ret as c_int;
    }

    timeout.tv_sec = 1;
    timeout.tv_usec = 0;
    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &timeout as *const _ as *const c_void,
        size_of::<timeval>() as u32,
    ) != 0
    {
        close(fd);
        return ret as c_int;
    }

    loop {
        size = recv(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len(), 0);
        if size <= 0 {
            close(fd);
            return ret as c_int;
        }

        ipv6 = buffer.as_mut_ptr() as *mut ipv6hdr;

        /* Skip packets that do not have the expected src/dst address or that
         * do not have a Hop-by-hop.
         */
        if ipv6_addr_equal(&(*ipv6).saddr, &src) == 0
            || ipv6_addr_equal(&(*ipv6).daddr, &dst) == 0
            || (*ipv6).nexthdr != IPPROTO_HOPOPTS
        {
            continue;
        }

        break;
    }

    /* Check Hbh's Next Header and Size. */
    hbh = buffer.as_mut_ptr().add(size_of::<ipv6hdr>()) as *mut ipv6_hopopt_hdr;
    if (*hbh).nexthdr != nexthdr || (*hbh).hdrlen != ((hoplen >> 3) - 1) as __u8 {
        close(fd);
        return ret as c_int;
    }

    /* Check we have a 2-byte padding for alignment. */
    ptr_ = (hbh as *mut __u8).add(size_of::<ipv6_hopopt_hdr>());
    if *ptr_.add(0) != IPV6_TLV_PADN && *ptr_.add(1) != 0 {
        close(fd);
        return ret as c_int;
    }

    /* Check we now have the IOAM option. */
    ptr_ = ptr_.add(2);
    if *ptr_.add(0) != IPV6_TLV_IOAM {
        close(fd);
        return ret as c_int;
    }

    /* Check its size and the IOAM option type. */
    ioam6 = ptr_ as *mut ioam6_hdr;
    if (*ioam6).opt_len
        != (size_of::<ioam6_hdr>() - 2 + size_of::<ioam6_trace_hdr>() + tr_size as usize) as __u8
        || (*ioam6).type_ != IOAM6_TYPE_PREALLOC
    {
        close(fd);
        return ret as c_int;
    }

    trace = ptr_.add(size_of::<ioam6_hdr>()) as *mut ioam6_trace_hdr;

    /* Check the trailing 4-byte padding (potentially). */
    ptr_ = (trace as *mut __u8)
        .add(size_of::<ioam6_trace_hdr>())
        .add(tr_size as usize);
    if tr_size % 8 != 0
        && *ptr_.add(0) != IPV6_TLV_PADN
        && *ptr_.add(1) != 2
        && *ptr_.add(2) != 0
        && *ptr_.add(3) != 0
    {
        close(fd);
        return ret as c_int;
    }

    /* Check the IOAM header and data. */
    ret = check_ioam_trace(testname_id, trace, tr_type, tr_size, ioam_ns) as u32;
    close(fd);
    ret as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
