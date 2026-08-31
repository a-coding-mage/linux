// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel_pt_pkt_decoder.c: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_ulong};
use core::mem;
use core::ptr;

pub type size_t = usize;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const BIT63: u64 = 1u64 << 63;

pub const INTEL_PT_NEED_MORE_BYTES: c_int = -1;
pub const INTEL_PT_BAD_PACKET: c_int = -2;
pub const INTEL_PT_VMX_NR_FLAG: u64 = 1u64 << 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum intel_pt_pkt_type {
    INTEL_PT_BAD = 0,
    INTEL_PT_PAD,
    INTEL_PT_TNT,
    INTEL_PT_TIP_PGD,
    INTEL_PT_TIP_PGE,
    INTEL_PT_TSC,
    INTEL_PT_TMA,
    INTEL_PT_MODE_EXEC,
    INTEL_PT_MODE_TSX,
    INTEL_PT_MTC,
    INTEL_PT_TIP,
    INTEL_PT_FUP,
    INTEL_PT_CYC,
    INTEL_PT_VMCS,
    INTEL_PT_PSB,
    INTEL_PT_PSBEND,
    INTEL_PT_CBR,
    INTEL_PT_TRACESTOP,
    INTEL_PT_PIP,
    INTEL_PT_OVF,
    INTEL_PT_MNT,
    INTEL_PT_PTWRITE,
    INTEL_PT_PTWRITE_IP,
    INTEL_PT_EXSTOP,
    INTEL_PT_EXSTOP_IP,
    INTEL_PT_MWAIT,
    INTEL_PT_PWRE,
    INTEL_PT_PWRX,
    INTEL_PT_BBP,
    INTEL_PT_BIP,
    INTEL_PT_BEP,
    INTEL_PT_BEP_IP,
    INTEL_PT_CFE,
    INTEL_PT_CFE_IP,
    INTEL_PT_EVD,
}

use intel_pt_pkt_type::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum intel_pt_pkt_ctx {
    INTEL_PT_NO_CTX = 0,
    INTEL_PT_BLK_4_CTX,
    INTEL_PT_BLK_8_CTX,
}

use intel_pt_pkt_ctx::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_pkt {
    pub type_: intel_pt_pkt_type,
    pub count: c_int,
    pub payload: u64,
}

unsafe extern "C" {
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
}

unsafe fn get_unaligned_le16(p: *const u8) -> u16 {
    u16::from_le(ptr::read_unaligned(p as *const u16))
}

unsafe fn get_unaligned_le32(p: *const u8) -> u32 {
    u32::from_le(ptr::read_unaligned(p as *const u32))
}

unsafe fn get_unaligned_le64(p: *const u8) -> u64 {
    u64::from_le(ptr::read_unaligned(p as *const u64))
}

unsafe fn memcpy_le64(d: *mut u64, s: *const u8, n: size_t) {
    let mut v: u64 = 0;
    ptr::copy_nonoverlapping(s, &mut v as *mut u64 as *mut u8, n);
    *d = u64::from_le(v);
}

static PACKET_NAME: [&[u8]; 35] = [
    b"Bad Packet!\0",
    b"PAD\0",
    b"TNT\0",
    b"TIP.PGD\0",
    b"TIP.PGE\0",
    b"TSC\0",
    b"TMA\0",
    b"MODE.Exec\0",
    b"MODE.TSX\0",
    b"MTC\0",
    b"TIP\0",
    b"FUP\0",
    b"CYC\0",
    b"VMCS\0",
    b"PSB\0",
    b"PSBEND\0",
    b"CBR\0",
    b"TraceSTOP\0",
    b"PIP\0",
    b"OVF\0",
    b"MNT\0",
    b"PTWRITE\0",
    b"PTWRITE\0",
    b"EXSTOP\0",
    b"EXSTOP\0",
    b"MWAIT\0",
    b"PWRE\0",
    b"PWRX\0",
    b"BBP\0",
    b"BIP\0",
    b"BEP\0",
    b"BEP\0",
    b"CFE\0",
    b"CFE\0",
    b"EVD\0",
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_pkt_name(type_: intel_pt_pkt_type) -> *const c_char {
    PACKET_NAME[type_ as usize].as_ptr() as *const c_char
}

unsafe fn intel_pt_get_long_tnt(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    let mut payload: u64;
    let mut count: c_int;

    if len < 8 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    payload = get_unaligned_le64(buf);

    count = 47;
    while count != 0 {
        if payload & BIT63 != 0 {
            break;
        }
        payload <<= 1;
        count -= 1;
    }

    (*packet).type_ = INTEL_PT_TNT;
    (*packet).count = count;
    (*packet).payload = payload << 1;
    8
}

unsafe fn intel_pt_get_pip(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    let mut payload: u64 = 0;

    if len < 8 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    (*packet).type_ = INTEL_PT_PIP;
    memcpy_le64(&mut payload, buf.add(2), 6);
    (*packet).payload = payload;

    8
}

unsafe fn intel_pt_get_tracestop(packet: *mut intel_pt_pkt) -> c_int {
    (*packet).type_ = INTEL_PT_TRACESTOP;
    2
}

unsafe fn intel_pt_get_cbr(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 4 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_CBR;
    (*packet).payload = get_unaligned_le16(buf.add(2)) as u64;
    4
}

unsafe fn intel_pt_get_vmcs(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 7 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    (*packet).type_ = INTEL_PT_VMCS;
    (*packet).count = 5;
    memcpy_le64(&mut (*packet).payload, buf.add(2), 5);

    7
}

unsafe fn intel_pt_get_ovf(packet: *mut intel_pt_pkt) -> c_int {
    (*packet).type_ = INTEL_PT_OVF;
    2
}

unsafe fn intel_pt_get_psb(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    let mut i: c_int;

    if len < 16 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    i = 2;
    while i < 16 {
        if *buf.add(i as usize) != 2 || *buf.add((i + 1) as usize) != 0x82 {
            return INTEL_PT_BAD_PACKET;
        }
        i += 2;
    }

    (*packet).type_ = INTEL_PT_PSB;
    16
}

unsafe fn intel_pt_get_psbend(packet: *mut intel_pt_pkt) -> c_int {
    (*packet).type_ = INTEL_PT_PSBEND;
    2
}

unsafe fn intel_pt_get_tma(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 7 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    (*packet).type_ = INTEL_PT_TMA;
    (*packet).payload = (*buf.add(2) as u64) | ((*buf.add(3) as u64) << 8);
    (*packet).count = (*buf.add(5) as c_int) | (((*buf.add(6) as u32 & BIT(0)) << 8) as c_int);
    7
}

unsafe fn intel_pt_get_pad(packet: *mut intel_pt_pkt) -> c_int {
    (*packet).type_ = INTEL_PT_PAD;
    1
}

unsafe fn intel_pt_get_mnt(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 11 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_MNT;
    (*packet).payload = get_unaligned_le64(buf.add(3));
    11
}

unsafe fn intel_pt_get_3byte(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 3 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    match *buf.add(2) {
        0x88 => intel_pt_get_mnt(buf, len, packet), /* MNT */
        _ => INTEL_PT_BAD_PACKET,
    }
}

unsafe fn intel_pt_get_ptwrite(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    (*packet).count = ((*buf.add(1) >> 5) & 0x3) as c_int;
    (*packet).type_ = if *buf.add(1) & BIT(7) as u8 != 0 {
        INTEL_PT_PTWRITE_IP
    } else {
        INTEL_PT_PTWRITE
    };

    match (*packet).count {
        0 => {
            if len < 6 {
                return INTEL_PT_NEED_MORE_BYTES;
            }
            (*packet).payload = get_unaligned_le32(buf.add(2)) as u64;
            6
        }
        1 => {
            if len < 10 {
                return INTEL_PT_NEED_MORE_BYTES;
            }
            (*packet).payload = get_unaligned_le64(buf.add(2));
            10
        }
        _ => INTEL_PT_BAD_PACKET,
    }
}

unsafe fn intel_pt_get_exstop(packet: *mut intel_pt_pkt) -> c_int {
    (*packet).type_ = INTEL_PT_EXSTOP;
    2
}

unsafe fn intel_pt_get_exstop_ip(packet: *mut intel_pt_pkt) -> c_int {
    (*packet).type_ = INTEL_PT_EXSTOP_IP;
    2
}

unsafe fn intel_pt_get_mwait(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 10 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_MWAIT;
    (*packet).payload = get_unaligned_le64(buf.add(2));
    10
}

unsafe fn intel_pt_get_pwre(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 4 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_PWRE;
    memcpy_le64(&mut (*packet).payload, buf.add(2), 2);
    4
}

unsafe fn intel_pt_get_pwrx(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 7 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_PWRX;
    memcpy_le64(&mut (*packet).payload, buf.add(2), 5);
    7
}

unsafe fn intel_pt_get_bbp(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 3 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_BBP;
    (*packet).count = (*buf.add(2) >> 7) as c_int;
    (*packet).payload = (*buf.add(2) & 0x1f) as u64;
    3
}

unsafe fn intel_pt_get_bip_4(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 5 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_BIP;
    (*packet).count = (*buf >> 3) as c_int;
    memcpy_le64(&mut (*packet).payload, buf.add(1), 4);
    5
}

unsafe fn intel_pt_get_bip_8(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 9 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_BIP;
    (*packet).count = (*buf >> 3) as c_int;
    (*packet).payload = get_unaligned_le64(buf.add(1));
    9
}

unsafe fn intel_pt_get_bep(len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 2 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_BEP;
    2
}

unsafe fn intel_pt_get_bep_ip(len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 2 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_BEP_IP;
    2
}

unsafe fn intel_pt_get_cfe(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 4 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = if *buf.add(2) & 0x80 != 0 { INTEL_PT_CFE_IP } else { INTEL_PT_CFE };
    (*packet).count = (*buf.add(2) & 0x1f) as c_int;
    (*packet).payload = *buf.add(3) as u64;
    4
}

unsafe fn intel_pt_get_evd(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 11 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_EVD;
    (*packet).count = (*buf.add(2) & 0x3f) as c_int;
    (*packet).payload = *buf.add(3) as u64;
    (*packet).payload = get_unaligned_le64(buf.add(3));
    11
}

unsafe fn intel_pt_get_ext(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 2 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    if (*buf.add(1) & 0x1f) == 0x12 {
        return intel_pt_get_ptwrite(buf, len, packet);
    }

    match *buf.add(1) {
        0xa3 => intel_pt_get_long_tnt(buf, len, packet), /* Long TNT */
        0x43 => intel_pt_get_pip(buf, len, packet),      /* PIP */
        0x83 => intel_pt_get_tracestop(packet),          /* TraceStop */
        0x03 => intel_pt_get_cbr(buf, len, packet),      /* CBR */
        0xc8 => intel_pt_get_vmcs(buf, len, packet),     /* VMCS */
        0xf3 => intel_pt_get_ovf(packet),                /* OVF */
        0x82 => intel_pt_get_psb(buf, len, packet),      /* PSB */
        0x23 => intel_pt_get_psbend(packet),             /* PSBEND */
        0x73 => intel_pt_get_tma(buf, len, packet),      /* TMA */
        0xC3 => intel_pt_get_3byte(buf, len, packet),    /* 3-byte header */
        0x62 => intel_pt_get_exstop(packet),             /* EXSTOP no IP */
        0xE2 => intel_pt_get_exstop_ip(packet),          /* EXSTOP with IP */
        0xC2 => intel_pt_get_mwait(buf, len, packet),    /* MWAIT */
        0x22 => intel_pt_get_pwre(buf, len, packet),     /* PWRE */
        0xA2 => intel_pt_get_pwrx(buf, len, packet),     /* PWRX */
        0x63 => intel_pt_get_bbp(buf, len, packet),      /* BBP */
        0x33 => intel_pt_get_bep(len, packet),           /* BEP no IP */
        0xb3 => intel_pt_get_bep_ip(len, packet),        /* BEP with IP */
        0x13 => intel_pt_get_cfe(buf, len, packet),      /* CFE */
        0x53 => intel_pt_get_evd(buf, len, packet),      /* EVD */
        _ => INTEL_PT_BAD_PACKET,
    }
}

unsafe fn intel_pt_get_short_tnt(mut byte: u32, packet: *mut intel_pt_pkt) -> c_int {
    let mut count: c_int;

    count = 6;
    while count != 0 {
        if byte & BIT(7) != 0 {
            break;
        }
        byte <<= 1;
        count -= 1;
    }

    (*packet).type_ = INTEL_PT_TNT;
    (*packet).count = count;
    (*packet).payload = (byte as u64) << 57;

    1
}

unsafe fn intel_pt_get_cyc(
    mut byte: u32,
    buf: *const u8,
    mut len: size_t,
    packet: *mut intel_pt_pkt,
) -> c_int {
    let mut offs: u32 = 1;
    let mut shift: u32;
    let mut payload: u64 = (byte >> 3) as u64;

    byte >>= 2;
    len -= 1;
    shift = 5;
    while byte & 1 != 0 {
        if offs > 9 {
            return INTEL_PT_BAD_PACKET;
        }
        if len < offs as usize {
            return INTEL_PT_NEED_MORE_BYTES;
        }
        byte = *buf.add(offs as usize) as u32;
        offs += 1;
        payload |= ((byte as u64) >> 1) << shift;
        shift += 7;
    }

    (*packet).type_ = INTEL_PT_CYC;
    (*packet).payload = payload;
    offs as c_int
}

unsafe fn intel_pt_get_ip(
    type_: intel_pt_pkt_type,
    byte: u32,
    buf: *const u8,
    len: size_t,
    packet: *mut intel_pt_pkt,
) -> c_int {
    let ip_len: c_int;

    (*packet).count = (byte >> 5) as c_int;

    match (*packet).count {
        0 => {
            ip_len = 0;
        }
        1 => {
            if len < 3 {
                return INTEL_PT_NEED_MORE_BYTES;
            }
            ip_len = 2;
            (*packet).payload = get_unaligned_le16(buf.add(1)) as u64;
        }
        2 => {
            if len < 5 {
                return INTEL_PT_NEED_MORE_BYTES;
            }
            ip_len = 4;
            (*packet).payload = get_unaligned_le32(buf.add(1)) as u64;
        }
        3 | 4 => {
            if len < 7 {
                return INTEL_PT_NEED_MORE_BYTES;
            }
            ip_len = 6;
            memcpy_le64(&mut (*packet).payload, buf.add(1), 6);
        }
        6 => {
            if len < 9 {
                return INTEL_PT_NEED_MORE_BYTES;
            }
            ip_len = 8;
            (*packet).payload = get_unaligned_le64(buf.add(1));
        }
        _ => return INTEL_PT_BAD_PACKET,
    }

    (*packet).type_ = type_;

    ip_len + 1
}

unsafe fn intel_pt_get_mode(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 2 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    match *buf.add(1) >> 5 {
        0 => {
            (*packet).type_ = INTEL_PT_MODE_EXEC;
            (*packet).count = *buf.add(1) as c_int;
            match *buf.add(1) & 3 {
                0 => (*packet).payload = 16,
                1 => (*packet).payload = 64,
                2 => (*packet).payload = 32,
                _ => return INTEL_PT_BAD_PACKET,
            }
        }
        1 => {
            (*packet).type_ = INTEL_PT_MODE_TSX;
            if (*buf.add(1) & 3) == 3 {
                return INTEL_PT_BAD_PACKET;
            }
            (*packet).payload = (*buf.add(1) & 3) as u64;
        }
        _ => return INTEL_PT_BAD_PACKET,
    }

    2
}

unsafe fn intel_pt_get_tsc(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 8 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_TSC;
    memcpy_le64(&mut (*packet).payload, buf.add(1), 7);
    8
}

unsafe fn intel_pt_get_mtc(buf: *const u8, len: size_t, packet: *mut intel_pt_pkt) -> c_int {
    if len < 2 {
        return INTEL_PT_NEED_MORE_BYTES;
    }
    (*packet).type_ = INTEL_PT_MTC;
    (*packet).payload = *buf.add(1) as u64;
    2
}

unsafe fn intel_pt_do_get_packet(
    buf: *const u8,
    len: size_t,
    packet: *mut intel_pt_pkt,
    ctx: intel_pt_pkt_ctx,
) -> c_int {
    let byte: u32;

    ptr::write_bytes(packet as *mut u8, 0, mem::size_of::<intel_pt_pkt>());

    if len == 0 {
        return INTEL_PT_NEED_MORE_BYTES;
    }

    byte = *buf as u32;

    match ctx {
        INTEL_PT_NO_CTX => {}
        INTEL_PT_BLK_4_CTX => {
            if (byte & 0x7) == 4 {
                return intel_pt_get_bip_4(buf, len, packet);
            }
        }
        INTEL_PT_BLK_8_CTX => {
            if (byte & 0x7) == 4 {
                return intel_pt_get_bip_8(buf, len, packet);
            }
        }
    }

    if !(byte & BIT(0) != 0) {
        if byte == 0 {
            return intel_pt_get_pad(packet);
        }
        if byte == 2 {
            return intel_pt_get_ext(buf, len, packet);
        }
        return intel_pt_get_short_tnt(byte, packet);
    }

    if byte & 2 != 0 {
        return intel_pt_get_cyc(byte, buf, len, packet);
    }

    match byte & 0x1f {
        0x0D => intel_pt_get_ip(INTEL_PT_TIP, byte, buf, len, packet),
        0x11 => intel_pt_get_ip(INTEL_PT_TIP_PGE, byte, buf, len, packet),
        0x01 => intel_pt_get_ip(INTEL_PT_TIP_PGD, byte, buf, len, packet),
        0x1D => intel_pt_get_ip(INTEL_PT_FUP, byte, buf, len, packet),
        0x19 => match byte {
            0x99 => intel_pt_get_mode(buf, len, packet),
            0x19 => intel_pt_get_tsc(buf, len, packet),
            0x59 => intel_pt_get_mtc(buf, len, packet),
            _ => INTEL_PT_BAD_PACKET,
        },
        _ => INTEL_PT_BAD_PACKET,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_upd_pkt_ctx(
    packet: *const intel_pt_pkt,
    ctx: *mut intel_pt_pkt_ctx,
) {
    match (*packet).type_ {
        INTEL_PT_BAD
        | INTEL_PT_PAD
        | INTEL_PT_TSC
        | INTEL_PT_TMA
        | INTEL_PT_MTC
        | INTEL_PT_FUP
        | INTEL_PT_CYC
        | INTEL_PT_CBR
        | INTEL_PT_MNT
        | INTEL_PT_EXSTOP
        | INTEL_PT_EXSTOP_IP
        | INTEL_PT_PWRE
        | INTEL_PT_PWRX
        | INTEL_PT_BIP => {}
        INTEL_PT_TNT
        | INTEL_PT_TIP
        | INTEL_PT_TIP_PGD
        | INTEL_PT_TIP_PGE
        | INTEL_PT_MODE_EXEC
        | INTEL_PT_MODE_TSX
        | INTEL_PT_PIP
        | INTEL_PT_OVF
        | INTEL_PT_VMCS
        | INTEL_PT_TRACESTOP
        | INTEL_PT_PSB
        | INTEL_PT_PSBEND
        | INTEL_PT_PTWRITE
        | INTEL_PT_PTWRITE_IP
        | INTEL_PT_MWAIT
        | INTEL_PT_BEP
        | INTEL_PT_BEP_IP
        | INTEL_PT_CFE
        | INTEL_PT_CFE_IP
        | INTEL_PT_EVD => {
            *ctx = INTEL_PT_NO_CTX;
        }
        INTEL_PT_BBP => {
            if (*packet).count != 0 {
                *ctx = INTEL_PT_BLK_4_CTX;
            } else {
                *ctx = INTEL_PT_BLK_8_CTX;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_get_packet(
    buf: *const u8,
    len: size_t,
    packet: *mut intel_pt_pkt,
    ctx: *mut intel_pt_pkt_ctx,
) -> c_int {
    let mut ret: c_int;

    ret = intel_pt_do_get_packet(buf, len, packet, *ctx);
    if ret > 0 {
        while ret < 8 && len > ret as size_t && *buf.add(ret as usize) == 0 {
            ret += 1;
        }
        intel_pt_upd_pkt_ctx(packet, ctx);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_pkt_desc(
    packet: *const intel_pt_pkt,
    mut buf: *mut c_char,
    buf_len: size_t,
) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let mut nr: c_int;
    let mut payload: c_ulong = (*packet).payload as c_ulong;
    let name: *const c_char = intel_pt_pkt_name((*packet).type_);

    match (*packet).type_ {
        INTEL_PT_BAD
        | INTEL_PT_PAD
        | INTEL_PT_PSB
        | INTEL_PT_PSBEND
        | INTEL_PT_TRACESTOP
        | INTEL_PT_OVF => snprintf(buf, buf_len, b"%s\0".as_ptr() as *const c_char, name),
        INTEL_PT_TNT => {
            let mut blen: size_t = buf_len;

            ret = snprintf(buf, blen, b"%s \0".as_ptr() as *const c_char, name);
            if ret < 0 {
                return ret;
            }
            buf = buf.add(ret as usize);
            blen = blen.wrapping_sub(ret as size_t);
            i = 0;
            while i < (*packet).count {
                if payload & BIT63 as c_ulong != 0 {
                    ret = snprintf(buf, blen, b"T\0".as_ptr() as *const c_char);
                } else {
                    ret = snprintf(buf, blen, b"N\0".as_ptr() as *const c_char);
                }
                if ret < 0 {
                    return ret;
                }
                buf = buf.add(ret as usize);
                blen = blen.wrapping_sub(ret as size_t);
                payload <<= 1;
                i += 1;
            }
            ret = snprintf(
                buf,
                blen,
                b" (%d)\0".as_ptr() as *const c_char,
                (*packet).count,
            );
            if ret < 0 {
                return ret;
            }
            blen = blen.wrapping_sub(ret as size_t);
            (buf_len - blen) as c_int
        }
        INTEL_PT_TIP_PGD | INTEL_PT_TIP_PGE | INTEL_PT_TIP | INTEL_PT_FUP => {
            if (*packet).count == 0 {
                return snprintf(buf, buf_len, b"%s no ip\0".as_ptr() as *const c_char, name);
            }
            snprintf(buf, buf_len, b"%s 0x%llx\0".as_ptr() as *const c_char, name, payload)
        }
        INTEL_PT_CYC
        | INTEL_PT_VMCS
        | INTEL_PT_MTC
        | INTEL_PT_MNT
        | INTEL_PT_CBR
        | INTEL_PT_TSC => {
            snprintf(buf, buf_len, b"%s 0x%llx\0".as_ptr() as *const c_char, name, payload)
        }
        INTEL_PT_TMA => snprintf(
            buf,
            buf_len,
            b"%s CTC 0x%x FC 0x%x\0".as_ptr() as *const c_char,
            name,
            payload as u32,
            (*packet).count,
        ),
        INTEL_PT_MODE_EXEC => snprintf(
            buf,
            buf_len,
            b"%s IF:%d %lld\0".as_ptr() as *const c_char,
            name,
            if (*packet).count & 4 != 0 { 1 } else { 0 },
            payload,
        ),
        INTEL_PT_MODE_TSX => snprintf(
            buf,
            buf_len,
            b"%s TXAbort:%u InTX:%u\0".as_ptr() as *const c_char,
            name,
            ((payload >> 1) & 1) as u32,
            (payload & 1) as u32,
        ),
        INTEL_PT_PIP => {
            nr = if (*packet).payload & INTEL_PT_VMX_NR_FLAG != 0 { 1 } else { 0 };
            payload &= !(INTEL_PT_VMX_NR_FLAG as c_ulong);
            ret = snprintf(
                buf,
                buf_len,
                b"%s 0x%llx (NR=%d)\0".as_ptr() as *const c_char,
                name,
                payload >> 1,
                nr,
            );
            ret
        }
        INTEL_PT_PTWRITE => snprintf(
            buf,
            buf_len,
            b"%s 0x%llx IP:0\0".as_ptr() as *const c_char,
            name,
            payload,
        ),
        INTEL_PT_PTWRITE_IP => snprintf(
            buf,
            buf_len,
            b"%s 0x%llx IP:1\0".as_ptr() as *const c_char,
            name,
            payload,
        ),
        INTEL_PT_BEP | INTEL_PT_EXSTOP => {
            snprintf(buf, buf_len, b"%s IP:0\0".as_ptr() as *const c_char, name)
        }
        INTEL_PT_BEP_IP | INTEL_PT_EXSTOP_IP => {
            snprintf(buf, buf_len, b"%s IP:1\0".as_ptr() as *const c_char, name)
        }
        INTEL_PT_MWAIT => snprintf(
            buf,
            buf_len,
            b"%s 0x%llx Hints 0x%x Extensions 0x%x\0".as_ptr() as *const c_char,
            name,
            payload,
            (payload & 0xff) as u32,
            ((payload >> 32) & 0x3) as u32,
        ),
        INTEL_PT_PWRE => snprintf(
            buf,
            buf_len,
            b"%s 0x%llx HW:%u CState:%u Sub-CState:%u\0".as_ptr() as *const c_char,
            name,
            payload,
            if payload & 0x80 != 0 { 1u32 } else { 0u32 },
            ((payload >> 12) & 0xf) as u32,
            ((payload >> 8) & 0xf) as u32,
        ),
        INTEL_PT_PWRX => snprintf(
            buf,
            buf_len,
            b"%s 0x%llx Last CState:%u Deepest CState:%u Wake Reason 0x%x\0".as_ptr()
                as *const c_char,
            name,
            payload,
            ((payload >> 4) & 0xf) as u32,
            (payload & 0xf) as u32,
            ((payload >> 8) & 0xf) as u32,
        ),
        INTEL_PT_BBP => snprintf(
            buf,
            buf_len,
            b"%s SZ %s-byte Type 0x%llx\0".as_ptr() as *const c_char,
            name,
            if (*packet).count != 0 {
                b"4\0".as_ptr() as *const c_char
            } else {
                b"8\0".as_ptr() as *const c_char
            },
            payload,
        ),
        INTEL_PT_BIP => snprintf(
            buf,
            buf_len,
            b"%s ID 0x%02x Value 0x%llx\0".as_ptr() as *const c_char,
            name,
            (*packet).count,
            payload,
        ),
        INTEL_PT_CFE | INTEL_PT_CFE_IP => snprintf(
            buf,
            buf_len,
            b"%s IP:%d Type 0x%02x Vector 0x%llx\0".as_ptr() as *const c_char,
            name,
            if (*packet).type_ == INTEL_PT_CFE_IP { 1 } else { 0 },
            (*packet).count,
            payload,
        ),
        INTEL_PT_EVD => snprintf(
            buf,
            buf_len,
            b"%s Type 0x%02x Payload 0x%llx\0".as_ptr() as *const c_char,
            name,
            (*packet).count,
            payload,
        ),
    }
}
