// SPDX-License-Identifier: GPL-2.0

// Translated from perf/arch/x86/tests/intel-pt-test.c.
// External types, constants, functions, and enum variants are provided by the
// surrounding perf Rust translation.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8 = core::ffi::c_uchar;

extern "C" {
    static INTEL_PT_PKT_MAX_SZ: usize;
    static INTEL_PT_PKT_DESC_MAX: usize;
    static TEST_FAIL: c_int;
    static TEST_OK: c_int;
    static TEST_SKIP: c_int;

    static INTEL_PT_PAD: c_int;
    static INTEL_PT_TNT: c_int;
    static INTEL_PT_TIP: c_int;
    static INTEL_PT_TIP_PGE: c_int;
    static INTEL_PT_TIP_PGD: c_int;
    static INTEL_PT_FUP: c_int;
    static INTEL_PT_PIP: c_int;
    static INTEL_PT_MODE_EXEC: c_int;
    static INTEL_PT_MODE_TSX: c_int;
    static INTEL_PT_TRACESTOP: c_int;
    static INTEL_PT_CBR: c_int;
    static INTEL_PT_TSC: c_int;
    static INTEL_PT_MTC: c_int;
    static INTEL_PT_TMA: c_int;
    static INTEL_PT_CYC: c_int;
    static INTEL_PT_VMCS: c_int;
    static INTEL_PT_OVF: c_int;
    static INTEL_PT_PSB: c_int;
    static INTEL_PT_PSBEND: c_int;
    static INTEL_PT_MNT: c_int;
    static INTEL_PT_PTWRITE: c_int;
    static INTEL_PT_PTWRITE_IP: c_int;
    static INTEL_PT_EXSTOP: c_int;
    static INTEL_PT_EXSTOP_IP: c_int;
    static INTEL_PT_MWAIT: c_int;
    static INTEL_PT_PWRE: c_int;
    static INTEL_PT_PWRX: c_int;
    static INTEL_PT_BBP: c_int;
    static INTEL_PT_BIP: c_int;
    static INTEL_PT_BEP: c_int;
    static INTEL_PT_BEP_IP: c_int;
    static INTEL_PT_CFE: c_int;
    static INTEL_PT_CFE_IP: c_int;
    static INTEL_PT_EVD: c_int;

    static INTEL_PT_NO_CTX: intel_pt_pkt_ctx;
    static INTEL_PT_BLK_4_CTX: intel_pt_pkt_ctx;
    static INTEL_PT_BLK_8_CTX: intel_pt_pkt_ctx;

    fn intel_pt_pkt_desc(packet: *const intel_pt_pkt, desc: *mut c_char, len: usize) -> c_int;
    fn intel_pt_get_packet(
        buf: *const u8,
        len: c_int,
        packet: *mut intel_pt_pkt,
        ctx: *mut intel_pt_pkt_ctx,
    ) -> c_int;
    fn intel_pt_upd_pkt_ctx(packet: *mut intel_pt_pkt, ctx: *mut intel_pt_pkt_ctx);
    fn pr_debug(fmt: *const c_char, ...);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn cpuid(
        op: c_uint,
        subleaf: c_uint,
        eax: *mut c_uint,
        ebx: *mut c_uint,
        ecx: *mut c_uint,
        edx: *mut c_uint,
    );
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
    fn cpu__max_cpu() -> perf_cpu;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_pkt {
    pub type_: c_int,
    pub count: c_int,
    pub payload: u64,
}

#[allow(non_camel_case_types)]
pub type intel_pt_pkt_ctx = c_int;

#[repr(C)]
pub struct test_suite {
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub skip_reason: *const c_char,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct cpu_set_t {
    __private: [c_ulong; 16],
}

/**
 * struct test_data - Test data.
 * @len: number of bytes to decode
 * @bytes: bytes to decode
 * @ctx: packet context to decode
 * @packet: expected packet
 * @new_ctx: expected new packet context
 * @ctx_unchanged: the packet context must not change
 */
#[repr(C)]
struct test_data {
    len: c_int,
    bytes: [u8; 16],
    ctx: intel_pt_pkt_ctx,
    packet: intel_pt_pkt,
    new_ctx: intel_pt_pkt_ctx,
    ctx_unchanged: c_int,
}

macro_rules! pkt {
    ($type_:expr, $count:expr, $payload:expr) => {
        intel_pt_pkt {
            type_: $type_,
            count: $count,
            payload: $payload,
        }
    };
}

macro_rules! bytes {
    ($($byte:expr),* $(,)?) => {
        {
            let mut out = [0 as u8; 16];
            let input = [$($byte as u8),*];
            let mut i = 0;
            while i < input.len() {
                out[i] = input[i];
                i += 1;
            }
            out
        }
    };
}

macro_rules! td {
    ($len:expr, [$($byte:expr),* $(,)?], $ctx:expr, $packet:expr, $new_ctx:expr, $ctx_unchanged:expr) => {
        test_data {
            len: $len,
            bytes: bytes![$($byte),*],
            ctx: $ctx,
            packet: $packet,
            new_ctx: $new_ctx,
            ctx_unchanged: $ctx_unchanged,
        }
    };
}

static DATA: &[test_data] = unsafe {
    &[
        /* Padding Packet */
        td!(1, [0], 0, pkt!(INTEL_PT_PAD, 0, 0), 0, 1),
        /* Short Taken/Not Taken Packet */
        td!(1, [4], 0, pkt!(INTEL_PT_TNT, 1, 0), 0, 0),
        td!(1, [6], 0, pkt!(INTEL_PT_TNT, 1, 0x20_u64 << 58), 0, 0),
        td!(1, [0x80], 0, pkt!(INTEL_PT_TNT, 6, 0), 0, 0),
        td!(1, [0xfe], 0, pkt!(INTEL_PT_TNT, 6, 0x3f_u64 << 58), 0, 0),
        /* Long Taken/Not Taken Packet */
        td!(8, [0x02, 0xa3, 2], 0, pkt!(INTEL_PT_TNT, 1, 0xa302_u64 << 47), 0, 0),
        td!(8, [0x02, 0xa3, 3], 0, pkt!(INTEL_PT_TNT, 1, 0x1a302_u64 << 47), 0, 0),
        td!(8, [0x02, 0xa3, 0, 0, 0, 0, 0, 0x80], 0, pkt!(INTEL_PT_TNT, 47, 0xa302_u64 << 1), 0, 0),
        td!(8, [0x02, 0xa3, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], 0, pkt!(INTEL_PT_TNT, 47, 0xffffffffffffa302_u64 << 1), 0, 0),
        /* Target IP Packet */
        td!(1, [0x0d], 0, pkt!(INTEL_PT_TIP, 0, 0), 0, 0),
        td!(3, [0x2d, 1, 2], 0, pkt!(INTEL_PT_TIP, 1, 0x201), 0, 0),
        td!(5, [0x4d, 1, 2, 3, 4], 0, pkt!(INTEL_PT_TIP, 2, 0x4030201), 0, 0),
        td!(7, [0x6d, 1, 2, 3, 4, 5, 6], 0, pkt!(INTEL_PT_TIP, 3, 0x60504030201), 0, 0),
        td!(7, [0x8d, 1, 2, 3, 4, 5, 6], 0, pkt!(INTEL_PT_TIP, 4, 0x60504030201), 0, 0),
        td!(9, [0xcd, 1, 2, 3, 4, 5, 6, 7, 8], 0, pkt!(INTEL_PT_TIP, 6, 0x807060504030201), 0, 0),
        /* Packet Generation Enable */
        td!(1, [0x11], 0, pkt!(INTEL_PT_TIP_PGE, 0, 0), 0, 0),
        td!(3, [0x31, 1, 2], 0, pkt!(INTEL_PT_TIP_PGE, 1, 0x201), 0, 0),
        td!(5, [0x51, 1, 2, 3, 4], 0, pkt!(INTEL_PT_TIP_PGE, 2, 0x4030201), 0, 0),
        td!(7, [0x71, 1, 2, 3, 4, 5, 6], 0, pkt!(INTEL_PT_TIP_PGE, 3, 0x60504030201), 0, 0),
        td!(7, [0x91, 1, 2, 3, 4, 5, 6], 0, pkt!(INTEL_PT_TIP_PGE, 4, 0x60504030201), 0, 0),
        td!(9, [0xd1, 1, 2, 3, 4, 5, 6, 7, 8], 0, pkt!(INTEL_PT_TIP_PGE, 6, 0x807060504030201), 0, 0),
        /* Packet Generation Disable */
        td!(1, [0x01], 0, pkt!(INTEL_PT_TIP_PGD, 0, 0), 0, 0),
        td!(3, [0x21, 1, 2], 0, pkt!(INTEL_PT_TIP_PGD, 1, 0x201), 0, 0),
        td!(5, [0x41, 1, 2, 3, 4], 0, pkt!(INTEL_PT_TIP_PGD, 2, 0x4030201), 0, 0),
        td!(7, [0x61, 1, 2, 3, 4, 5, 6], 0, pkt!(INTEL_PT_TIP_PGD, 3, 0x60504030201), 0, 0),
        td!(7, [0x81, 1, 2, 3, 4, 5, 6], 0, pkt!(INTEL_PT_TIP_PGD, 4, 0x60504030201), 0, 0),
        td!(9, [0xc1, 1, 2, 3, 4, 5, 6, 7, 8], 0, pkt!(INTEL_PT_TIP_PGD, 6, 0x807060504030201), 0, 0),
        /* Flow Update Packet */
        td!(1, [0x1d], 0, pkt!(INTEL_PT_FUP, 0, 0), 0, 0),
        td!(3, [0x3d, 1, 2], 0, pkt!(INTEL_PT_FUP, 1, 0x201), 0, 0),
        td!(5, [0x5d, 1, 2, 3, 4], 0, pkt!(INTEL_PT_FUP, 2, 0x4030201), 0, 0),
        td!(7, [0x7d, 1, 2, 3, 4, 5, 6], 0, pkt!(INTEL_PT_FUP, 3, 0x60504030201), 0, 0),
        td!(7, [0x9d, 1, 2, 3, 4, 5, 6], 0, pkt!(INTEL_PT_FUP, 4, 0x60504030201), 0, 0),
        td!(9, [0xdd, 1, 2, 3, 4, 5, 6, 7, 8], 0, pkt!(INTEL_PT_FUP, 6, 0x807060504030201), 0, 0),
        /* Paging Information Packet */
        td!(8, [0x02, 0x43, 2, 4, 6, 8, 10, 12], 0, pkt!(INTEL_PT_PIP, 0, 0xC0A08060402), 0, 0),
        td!(8, [0x02, 0x43, 3, 4, 6, 8, 10, 12], 0, pkt!(INTEL_PT_PIP, 0, 0xC0A08060403), 0, 0),
        /* Mode Exec Packet */
        td!(2, [0x99, 0x00], 0, pkt!(INTEL_PT_MODE_EXEC, 0, 16), 0, 0),
        td!(2, [0x99, 0x01], 0, pkt!(INTEL_PT_MODE_EXEC, 1, 64), 0, 0),
        td!(2, [0x99, 0x02], 0, pkt!(INTEL_PT_MODE_EXEC, 2, 32), 0, 0),
        td!(2, [0x99, 0x04], 0, pkt!(INTEL_PT_MODE_EXEC, 4, 16), 0, 0),
        td!(2, [0x99, 0x05], 0, pkt!(INTEL_PT_MODE_EXEC, 5, 64), 0, 0),
        td!(2, [0x99, 0x06], 0, pkt!(INTEL_PT_MODE_EXEC, 6, 32), 0, 0),
        /* Mode TSX Packet */
        td!(2, [0x99, 0x20], 0, pkt!(INTEL_PT_MODE_TSX, 0, 0), 0, 0),
        td!(2, [0x99, 0x21], 0, pkt!(INTEL_PT_MODE_TSX, 0, 1), 0, 0),
        td!(2, [0x99, 0x22], 0, pkt!(INTEL_PT_MODE_TSX, 0, 2), 0, 0),
        /* Trace Stop Packet */
        td!(2, [0x02, 0x83], 0, pkt!(INTEL_PT_TRACESTOP, 0, 0), 0, 0),
        /* Core:Bus Ratio Packet */
        td!(4, [0x02, 0x03, 0x12, 0], 0, pkt!(INTEL_PT_CBR, 0, 0x12), 0, 1),
        /* Timestamp Counter Packet */
        td!(8, [0x19, 1, 2, 3, 4, 5, 6, 7], 0, pkt!(INTEL_PT_TSC, 0, 0x7060504030201), 0, 1),
        /* Mini Time Counter Packet */
        td!(2, [0x59, 0x12], 0, pkt!(INTEL_PT_MTC, 0, 0x12), 0, 1),
        /* TSC / MTC Alignment Packet */
        td!(7, [0x02, 0x73], 0, pkt!(INTEL_PT_TMA, 0, 0), 0, 1),
        td!(7, [0x02, 0x73, 1, 2], 0, pkt!(INTEL_PT_TMA, 0, 0x201), 0, 1),
        td!(7, [0x02, 0x73, 0, 0, 0, 0xff, 1], 0, pkt!(INTEL_PT_TMA, 0x1ff, 0), 0, 1),
        td!(7, [0x02, 0x73, 0x80, 0xc0, 0, 0xff, 1], 0, pkt!(INTEL_PT_TMA, 0x1ff, 0xc080), 0, 1),
        /* Cycle Count Packet */
        td!(1, [0x03], 0, pkt!(INTEL_PT_CYC, 0, 0), 0, 1),
        td!(1, [0x0b], 0, pkt!(INTEL_PT_CYC, 0, 1), 0, 1),
        td!(1, [0xfb], 0, pkt!(INTEL_PT_CYC, 0, 0x1f), 0, 1),
        td!(2, [0x07, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x20), 0, 1),
        td!(2, [0xff, 0xfe], 0, pkt!(INTEL_PT_CYC, 0, 0xfff), 0, 1),
        td!(3, [0x07, 1, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x1000), 0, 1),
        td!(3, [0xff, 0xff, 0xfe], 0, pkt!(INTEL_PT_CYC, 0, 0x7ffff), 0, 1),
        td!(4, [0x07, 1, 1, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x80000), 0, 1),
        td!(4, [0xff, 0xff, 0xff, 0xfe], 0, pkt!(INTEL_PT_CYC, 0, 0x3ffffff), 0, 1),
        td!(5, [0x07, 1, 1, 1, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x4000000), 0, 1),
        td!(5, [0xff, 0xff, 0xff, 0xff, 0xfe], 0, pkt!(INTEL_PT_CYC, 0, 0x1ffffffff), 0, 1),
        td!(6, [0x07, 1, 1, 1, 1, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x200000000), 0, 1),
        td!(6, [0xff, 0xff, 0xff, 0xff, 0xff, 0xfe], 0, pkt!(INTEL_PT_CYC, 0, 0xffffffffff), 0, 1),
        td!(7, [0x07, 1, 1, 1, 1, 1, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x10000000000), 0, 1),
        td!(7, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe], 0, pkt!(INTEL_PT_CYC, 0, 0x7fffffffffff), 0, 1),
        td!(8, [0x07, 1, 1, 1, 1, 1, 1, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x800000000000), 0, 1),
        td!(8, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe], 0, pkt!(INTEL_PT_CYC, 0, 0x3fffffffffffff), 0, 1),
        td!(9, [0x07, 1, 1, 1, 1, 1, 1, 1, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x40000000000000), 0, 1),
        td!(9, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe], 0, pkt!(INTEL_PT_CYC, 0, 0x1fffffffffffffff), 0, 1),
        td!(10, [0x07, 1, 1, 1, 1, 1, 1, 1, 1, 2], 0, pkt!(INTEL_PT_CYC, 0, 0x2000000000000000), 0, 1),
        td!(10, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe], 0, pkt!(INTEL_PT_CYC, 0, 0xffffffffffffffff), 0, 1),
        /* Virtual-Machine Control Structure Packet */
        td!(7, [0x02, 0xc8, 1, 2, 3, 4, 5], 0, pkt!(INTEL_PT_VMCS, 5, 0x504030201), 0, 0),
        /* Overflow Packet */
        td!(2, [0x02, 0xf3], 0, pkt!(INTEL_PT_OVF, 0, 0), 0, 0),
        td!(2, [0x02, 0xf3], INTEL_PT_BLK_4_CTX, pkt!(INTEL_PT_OVF, 0, 0), 0, 0),
        td!(2, [0x02, 0xf3], INTEL_PT_BLK_8_CTX, pkt!(INTEL_PT_OVF, 0, 0), 0, 0),
        /* Packet Stream Boundary*/
        td!(16, [0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82], 0, pkt!(INTEL_PT_PSB, 0, 0), 0, 0),
        td!(16, [0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82], INTEL_PT_BLK_4_CTX, pkt!(INTEL_PT_PSB, 0, 0), 0, 0),
        td!(16, [0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82, 0x02, 0x82], INTEL_PT_BLK_8_CTX, pkt!(INTEL_PT_PSB, 0, 0), 0, 0),
        /* PSB End Packet */
        td!(2, [0x02, 0x23], 0, pkt!(INTEL_PT_PSBEND, 0, 0), 0, 0),
        /* Maintenance Packet */
        td!(11, [0x02, 0xc3, 0x88, 1, 2, 3, 4, 5, 6, 7], 0, pkt!(INTEL_PT_MNT, 0, 0x7060504030201), 0, 1),
        /* Write Data to PT Packet */
        td!(6, [0x02, 0x12, 1, 2, 3, 4], 0, pkt!(INTEL_PT_PTWRITE, 0, 0x4030201), 0, 0),
        td!(10, [0x02, 0x32, 1, 2, 3, 4, 5, 6, 7, 8], 0, pkt!(INTEL_PT_PTWRITE, 1, 0x807060504030201), 0, 0),
        td!(6, [0x02, 0x92, 1, 2, 3, 4], 0, pkt!(INTEL_PT_PTWRITE_IP, 0, 0x4030201), 0, 0),
        td!(10, [0x02, 0xb2, 1, 2, 3, 4, 5, 6, 7, 8], 0, pkt!(INTEL_PT_PTWRITE_IP, 1, 0x807060504030201), 0, 0),
        /* Execution Stop Packet */
        td!(2, [0x02, 0x62], 0, pkt!(INTEL_PT_EXSTOP, 0, 0), 0, 1),
        td!(2, [0x02, 0xe2], 0, pkt!(INTEL_PT_EXSTOP_IP, 0, 0), 0, 1),
        /* Monitor Wait Packet */
        td!(10, [0x02, 0xc2], 0, pkt!(INTEL_PT_MWAIT, 0, 0), 0, 0),
        td!(10, [0x02, 0xc2, 1, 2, 3, 4, 5, 6, 7, 8], 0, pkt!(INTEL_PT_MWAIT, 0, 0x807060504030201), 0, 0),
        td!(10, [0x02, 0xc2, 0xff, 2, 3, 4, 7, 6, 7, 8], 0, pkt!(INTEL_PT_MWAIT, 0, 0x8070607040302ff), 0, 0),
        /* Power Entry Packet */
        td!(4, [0x02, 0x22], 0, pkt!(INTEL_PT_PWRE, 0, 0), 0, 1),
        td!(4, [0x02, 0x22, 1, 2], 0, pkt!(INTEL_PT_PWRE, 0, 0x0201), 0, 1),
        td!(4, [0x02, 0x22, 0x80, 0x34], 0, pkt!(INTEL_PT_PWRE, 0, 0x3480), 0, 1),
        td!(4, [0x02, 0x22, 0x00, 0x56], 0, pkt!(INTEL_PT_PWRE, 0, 0x5600), 0, 1),
        /* Power Exit Packet */
        td!(7, [0x02, 0xa2], 0, pkt!(INTEL_PT_PWRX, 0, 0), 0, 1),
        td!(7, [0x02, 0xa2, 1, 2, 3, 4, 5], 0, pkt!(INTEL_PT_PWRX, 0, 0x504030201), 0, 1),
        td!(7, [0x02, 0xa2, 0xff, 0xff, 0xff, 0xff, 0xff], 0, pkt!(INTEL_PT_PWRX, 0, 0xffffffffff), 0, 1),
        /* Block Begin Packet */
        td!(3, [0x02, 0x63, 0x00], 0, pkt!(INTEL_PT_BBP, 0, 0), INTEL_PT_BLK_8_CTX, 0),
        td!(3, [0x02, 0x63, 0x80], 0, pkt!(INTEL_PT_BBP, 1, 0), INTEL_PT_BLK_4_CTX, 0),
        td!(3, [0x02, 0x63, 0x1f], 0, pkt!(INTEL_PT_BBP, 0, 0x1f), INTEL_PT_BLK_8_CTX, 0),
        td!(3, [0x02, 0x63, 0x9f], 0, pkt!(INTEL_PT_BBP, 1, 0x1f), INTEL_PT_BLK_4_CTX, 0),
        /* 4-byte Block Item Packet */
        td!(5, [0x04], INTEL_PT_BLK_4_CTX, pkt!(INTEL_PT_BIP, 0, 0), INTEL_PT_BLK_4_CTX, 0),
        td!(5, [0xfc], INTEL_PT_BLK_4_CTX, pkt!(INTEL_PT_BIP, 0x1f, 0), INTEL_PT_BLK_4_CTX, 0),
        td!(5, [0x04, 1, 2, 3, 4], INTEL_PT_BLK_4_CTX, pkt!(INTEL_PT_BIP, 0, 0x04030201), INTEL_PT_BLK_4_CTX, 0),
        td!(5, [0xfc, 1, 2, 3, 4], INTEL_PT_BLK_4_CTX, pkt!(INTEL_PT_BIP, 0x1f, 0x04030201), INTEL_PT_BLK_4_CTX, 0),
        /* 8-byte Block Item Packet */
        td!(9, [0x04], INTEL_PT_BLK_8_CTX, pkt!(INTEL_PT_BIP, 0, 0), INTEL_PT_BLK_8_CTX, 0),
        td!(9, [0xfc], INTEL_PT_BLK_8_CTX, pkt!(INTEL_PT_BIP, 0x1f, 0), INTEL_PT_BLK_8_CTX, 0),
        td!(9, [0x04, 1, 2, 3, 4, 5, 6, 7, 8], INTEL_PT_BLK_8_CTX, pkt!(INTEL_PT_BIP, 0, 0x0807060504030201), INTEL_PT_BLK_8_CTX, 0),
        td!(9, [0xfc, 1, 2, 3, 4, 5, 6, 7, 8], INTEL_PT_BLK_8_CTX, pkt!(INTEL_PT_BIP, 0x1f, 0x0807060504030201), INTEL_PT_BLK_8_CTX, 0),
        /* Block End Packet */
        td!(2, [0x02, 0x33], INTEL_PT_BLK_4_CTX, pkt!(INTEL_PT_BEP, 0, 0), 0, 0),
        td!(2, [0x02, 0xb3], INTEL_PT_BLK_4_CTX, pkt!(INTEL_PT_BEP_IP, 0, 0), 0, 0),
        td!(2, [0x02, 0x33], INTEL_PT_BLK_8_CTX, pkt!(INTEL_PT_BEP, 0, 0), 0, 0),
        td!(2, [0x02, 0xb3], INTEL_PT_BLK_8_CTX, pkt!(INTEL_PT_BEP_IP, 0, 0), 0, 0),
        /* Control Flow Event Packet */
        td!(4, [0x02, 0x13, 0x01, 0x03], 0, pkt!(INTEL_PT_CFE, 1, 3), 0, 0),
        td!(4, [0x02, 0x13, 0x81, 0x03], 0, pkt!(INTEL_PT_CFE_IP, 1, 3), 0, 0),
        td!(4, [0x02, 0x13, 0x1f, 0x00], 0, pkt!(INTEL_PT_CFE, 0x1f, 0), 0, 0),
        td!(4, [0x02, 0x13, 0x9f, 0xff], 0, pkt!(INTEL_PT_CFE_IP, 0x1f, 0xff), 0, 0),
        /*  */
        td!(11, [0x02, 0x53, 0x09, 1, 2, 3, 4, 5, 6, 7], 0, pkt!(INTEL_PT_EVD, 0x09, 0x7060504030201), 0, 0),
        td!(11, [0x02, 0x53, 0x3f, 2, 3, 4, 5, 6, 7, 8], 0, pkt!(INTEL_PT_EVD, 0x3f, 0x8070605040302), 0, 0),
        /* Terminator */
        td!(0, [0], 0, pkt!(0, 0, 0), 0, 0),
    ]
};

unsafe fn dump_packet(packet: *const intel_pt_pkt, bytes: *const u8, len: c_int) -> c_int {
    let mut desc = [0 as c_char; 256];
    let mut i: c_int;
    let ret: c_int;

    i = 0;
    while i < len {
        pr_debug(c" %02x".as_ptr(), *bytes.add(i as usize) as c_int);
        i += 1;
    }
    while i < 16 {
        pr_debug(c"   ".as_ptr());
        i += 1;
    }
    pr_debug(c"   ".as_ptr());
    ret = intel_pt_pkt_desc(packet, desc.as_mut_ptr(), desc.len());
    if ret < 0 {
        pr_debug(c"intel_pt_pkt_desc failed!\n".as_ptr());
        return TEST_FAIL;
    }
    pr_debug(c"%s\n".as_ptr(), desc.as_ptr());

    TEST_OK
}

unsafe fn decoding_failed(d: *const test_data) {
    pr_debug(c"Decoding failed!\n".as_ptr());
    pr_debug(c"Decoding:  ".as_ptr());
    dump_packet(&(*d).packet, (*d).bytes.as_ptr(), (*d).len);
}

unsafe fn fail(
    d: *const test_data,
    packet: *mut intel_pt_pkt,
    len: c_int,
    new_ctx: intel_pt_pkt_ctx,
) -> c_int {
    decoding_failed(d);

    if len != (*d).len {
        pr_debug(
            c"Expected length: %d   Decoded length %d\n".as_ptr(),
            (*d).len,
            len,
        );
    }

    if (*packet).type_ != (*d).packet.type_ {
        pr_debug(
            c"Expected type: %d   Decoded type %d\n".as_ptr(),
            (*d).packet.type_,
            (*packet).type_,
        );
    }

    if (*packet).count != (*d).packet.count {
        pr_debug(
            c"Expected count: %d   Decoded count %d\n".as_ptr(),
            (*d).packet.count,
            (*packet).count,
        );
    }

    if (*packet).payload != (*d).packet.payload {
        pr_debug(
            c"Expected payload: 0x%llx   Decoded payload 0x%llx\n".as_ptr(),
            (*d).packet.payload as u64,
            (*packet).payload as u64,
        );
    }

    if new_ctx != (*d).new_ctx {
        pr_debug(
            c"Expected packet context: %d   Decoded packet context %d\n".as_ptr(),
            (*d).new_ctx,
            new_ctx,
        );
    }

    TEST_FAIL
}

unsafe fn test_ctx_unchanged(
    d: *const test_data,
    packet: *mut intel_pt_pkt,
    mut ctx: intel_pt_pkt_ctx,
) -> c_int {
    let old_ctx: intel_pt_pkt_ctx = ctx;

    intel_pt_upd_pkt_ctx(packet, &mut ctx);

    if ctx != old_ctx {
        decoding_failed(d);
        pr_debug(c"Packet context changed!\n".as_ptr());
        return TEST_FAIL;
    }

    TEST_OK
}

unsafe fn test_one(d: *const test_data) -> c_int {
    let mut packet: intel_pt_pkt = core::mem::zeroed();
    let mut ctx: intel_pt_pkt_ctx = (*d).ctx;
    let mut ret: c_int;

    memset(
        &mut packet as *mut intel_pt_pkt as *mut c_void,
        0xff,
        core::mem::size_of::<intel_pt_pkt>(),
    );

    /* Decode a packet */
    ret = intel_pt_get_packet((*d).bytes.as_ptr(), (*d).len, &mut packet, &mut ctx);
    if ret < 0 || ret > 16 {
        decoding_failed(d);
        pr_debug(c"intel_pt_get_packet returned %d\n".as_ptr(), ret);
        return TEST_FAIL;
    }

    /* Some packets must always leave the packet context unchanged */
    if (*d).ctx_unchanged != 0 {
        let mut err: c_int;

        err = test_ctx_unchanged(d, &mut packet, INTEL_PT_NO_CTX);
        if err != 0 {
            return err;
        }
        err = test_ctx_unchanged(d, &mut packet, INTEL_PT_BLK_4_CTX);
        if err != 0 {
            return err;
        }
        err = test_ctx_unchanged(d, &mut packet, INTEL_PT_BLK_8_CTX);
        if err != 0 {
            return err;
        }
    }

    /* Compare to the expected values */
    if ret != (*d).len
        || packet.type_ != (*d).packet.type_
        || packet.count != (*d).packet.count
        || packet.payload != (*d).packet.payload
        || ctx != (*d).new_ctx
    {
        return fail(d, &mut packet, ret, ctx);
    }

    pr_debug(c"Decoded ok:".as_ptr());
    ret = dump_packet(&(*d).packet, (*d).bytes.as_ptr(), (*d).len);

    ret
}

/*
 * This test feeds byte sequences to the Intel PT packet decoder and checks the
 * results. Changes to the packet context are also checked.
 */
#[no_mangle]
pub unsafe extern "C" fn test__intel_pt_pkt_decoder(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut idx: usize = 0;
    let mut ret: c_int;

    while DATA[idx].len != 0 {
        ret = test_one(&DATA[idx]);
        if ret != 0 {
            return ret;
        }
        idx += 1;
    }

    TEST_OK
}

unsafe fn setaffinity(cpu: c_int) -> c_int {
    let mut cpu_set: cpu_set_t = core::mem::zeroed();

    CPU_ZERO(&mut cpu_set);
    CPU_SET(cpu, &mut cpu_set);
    if sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &cpu_set) != 0 {
        pr_debug(c"sched_setaffinity() failed for CPU %d\n".as_ptr(), cpu);
        return -1;
    }
    0
}

const INTEL_PT_ADDR_FILT_CNT_MASK: c_uint = genmask(2, 0);
const INTEL_PT_SUBLEAF_CNT: usize = 2;
const CPUID_REG_CNT: usize = 4;

const fn bit(nr: c_uint) -> c_uint {
    1_u32 << nr
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    (!0_u32 << l) & (!0_u32 >> (31 - h))
}

#[repr(C)]
#[derive(Copy, Clone)]
union cpuid_result_regs {
    named: cpuid_result_named,
    reg: [c_uint; CPUID_REG_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cpuid_result_named {
    eax: c_uint,
    ebx: c_uint,
    ecx: c_uint,
    edx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cpuid_result {
    u: cpuid_result_regs,
}

impl cpuid_result {
    unsafe fn eax(&self) -> c_uint {
        self.u.named.eax
    }
    unsafe fn ebx(&self) -> c_uint {
        self.u.named.ebx
    }
    unsafe fn ecx(&self) -> c_uint {
        self.u.named.ecx
    }
    unsafe fn edx(&self) -> c_uint {
        self.u.named.edx
    }
    unsafe fn reg(&self, i: usize) -> c_uint {
        self.u.reg[i]
    }
    unsafe fn eax_mut(&mut self) -> *mut c_uint {
        &mut self.u.named.eax
    }
    unsafe fn ebx_mut(&mut self) -> *mut c_uint {
        &mut self.u.named.ebx
    }
    unsafe fn ecx_mut(&mut self) -> *mut c_uint {
        &mut self.u.named.ecx
    }
    unsafe fn edx_mut(&mut self) -> *mut c_uint {
        &mut self.u.named.edx
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct pt_caps {
    subleaf: [cpuid_result; INTEL_PT_SUBLEAF_CNT],
}

unsafe fn get_pt_caps(cpu: c_int, caps: *mut pt_caps) -> c_int {
    let mut r: cpuid_result = core::mem::zeroed();
    let mut i: usize;

    if setaffinity(cpu) != 0 {
        return -1;
    }

    memset(
        caps as *mut c_void,
        0,
        core::mem::size_of::<pt_caps>(),
    );

    i = 0;
    while i < INTEL_PT_SUBLEAF_CNT {
        cpuid(20, i as c_uint, r.eax_mut(), r.ebx_mut(), r.ecx_mut(), r.edx_mut());
        pr_debug(c"CPU %d CPUID leaf 20 subleaf %d\n".as_ptr(), cpu, i as c_int);
        pr_debug(c"eax = 0x%08x\n".as_ptr(), r.eax());
        pr_debug(c"ebx = 0x%08x\n".as_ptr(), r.ebx());
        pr_debug(c"ecx = 0x%08x\n".as_ptr(), r.ecx());
        pr_debug(c"edx = 0x%08x\n".as_ptr(), r.edx());
        (*caps).subleaf[i] = r;
        i += 1;
    }

    0
}

unsafe fn is_hybrid() -> bool {
    let mut eax: c_uint = 0;
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;
    let result: bool;

    cpuid(7, 0, &mut eax, &mut ebx, &mut ecx, &mut edx);
    result = (edx & bit(15)) != 0;
    pr_debug(
        c"Is %shybrid : CPUID leaf 7 subleaf 0 edx %#x (bit-15 indicates hybrid)\n".as_ptr(),
        if result { c"".as_ptr() } else { c"not ".as_ptr() },
        edx,
    );
    result
}

unsafe fn compare_caps(cpu: c_int, caps: *mut pt_caps, caps0: *mut pt_caps) -> c_int {
    let mask = pt_caps {
        /* Mask of bits to check*/
        subleaf: [
            cpuid_result {
                u: cpuid_result_regs {
                    named: cpuid_result_named {
                        eax: 0,
                        ebx: genmask(8, 0),
                        ecx: genmask(3, 0),
                        edx: 0,
                    },
                },
            },
            cpuid_result {
                u: cpuid_result_regs {
                    named: cpuid_result_named {
                        eax: genmask(31, 16),
                        ebx: genmask(31, 0),
                        ecx: 0,
                        edx: 0,
                    },
                },
            },
        ],
    };
    let mut m: c_uint;
    let mut reg: c_uint;
    let mut reg0: c_uint;
    let mut ret: c_int = 0;
    let mut i: usize;
    let mut j: usize;

    i = 0;
    while i < INTEL_PT_SUBLEAF_CNT {
        j = 0;
        while j < CPUID_REG_CNT {
            m = mask.subleaf[i].reg(j);
            reg = m & (*caps).subleaf[i].reg(j);
            reg0 = m & (*caps0).subleaf[i].reg(j);
            if (reg & reg0) != reg0 {
                pr_debug(
                    c"CPU %d subleaf %d reg %d FAIL %#x vs %#x\n".as_ptr(),
                    cpu,
                    i as c_int,
                    j as c_int,
                    reg,
                    reg0,
                );
                ret = -1;
            }
            j += 1;
        }
        i += 1;
    }

    m = INTEL_PT_ADDR_FILT_CNT_MASK;
    reg = m & (*caps).subleaf[1].eax();
    reg0 = m & (*caps0).subleaf[1].eax();
    if reg < reg0 {
        pr_debug(
            c"CPU %d subleaf 1 reg 0 FAIL address filter count %#x vs %#x\n".as_ptr(),
            cpu,
            reg,
            reg0,
        );
        ret = -1;
    }

    if ret == 0 {
        pr_debug(c"CPU %d OK\n".as_ptr(), cpu);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn test__intel_pt_hybrid_compat(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let max_cpu: c_int = cpu__max_cpu().cpu;
    let mut last_caps: pt_caps;
    let mut caps0: pt_caps = core::mem::zeroed();
    let mut ret: c_int = TEST_OK;
    let mut cpu: c_int;

    if !is_hybrid() {
        (*(*test).test_cases.add(subtest as usize)).skip_reason = c"not hybrid".as_ptr();
        return TEST_SKIP;
    }

    if get_pt_caps(0, &mut caps0) != 0 {
        return TEST_FAIL;
    }

    cpu = 1;
    last_caps = caps0;
    while cpu < max_cpu {
        let mut caps: pt_caps = core::mem::zeroed();

        if get_pt_caps(cpu, &mut caps) != 0 {
            pr_debug(c"CPU %d not found\n".as_ptr(), cpu);
            cpu += 1;
            continue;
        }
        if memcmp(
            &caps as *const pt_caps as *const c_void,
            &last_caps as *const pt_caps as *const c_void,
            core::mem::size_of::<pt_caps>(),
        ) == 0
        {
            pr_debug(c"CPU %d same caps as previous CPU\n".as_ptr(), cpu);
            cpu += 1;
            continue;
        }
        if compare_caps(cpu, &mut caps, &mut caps0) != 0 {
            ret = TEST_FAIL;
        }
        last_caps = caps;
        cpu += 1;
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
