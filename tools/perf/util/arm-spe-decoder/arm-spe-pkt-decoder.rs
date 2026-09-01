// SPDX-License-Identifier: GPL-2.0
/*
 * Arm Statistical Profiling Extensions (SPE) support
 * Copyright (c) 2017-2018, Arm Ltd.
 */

// Translated from arm-spe-pkt-decoder.c.  C include dependencies such as
// arm-spe-pkt-decoder.h, linux/bitops.h, linux/unaligned.h, and cputype.h are
// expected to provide the packet types, constants, bitfield helpers, and MIDR
// helpers referenced below.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong};
use core::mem;
use core::ptr;
use std::ffi::CString;

type u8 = u8;
type u64 = u64;
type size_t = usize;

#[repr(C)]
pub struct arm_spe_pkt {
    pub type_: arm_spe_pkt_type,
    pub index: c_int,
    pub payload: u64,
    pub midr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_spe_pkt_type {
    ARM_SPE_BAD = 0,
    ARM_SPE_PAD,
    ARM_SPE_END,
    ARM_SPE_TIMESTAMP,
    ARM_SPE_ADDRESS,
    ARM_SPE_COUNTER,
    ARM_SPE_CONTEXT,
    ARM_SPE_OP_TYPE,
    ARM_SPE_EVENTS,
    ARM_SPE_DATA_SOURCE,
}

#[repr(C)]
pub struct midr_range {
    _private: [u8; 0],
}

extern "C" {
    fn is_midr_in_range_list(midr: u64, ranges: *const midr_range) -> bool;
}

const ARM_SPE_NEED_MORE_BYTES: c_int = -1;
const ARM_SPE_BAD_PACKET: c_int = -2;

static ARM_SPE_PACKET_NAME_PAD: &[u8] = b"PAD\0";
static ARM_SPE_PACKET_NAME_END: &[u8] = b"END\0";
static ARM_SPE_PACKET_NAME_TIMESTAMP: &[u8] = b"TS\0";
static ARM_SPE_PACKET_NAME_ADDRESS: &[u8] = b"ADDR\0";
static ARM_SPE_PACKET_NAME_COUNTER: &[u8] = b"LAT\0";
static ARM_SPE_PACKET_NAME_CONTEXT: &[u8] = b"CONTEXT\0";
static ARM_SPE_PACKET_NAME_OP_TYPE: &[u8] = b"OP-TYPE\0";
static ARM_SPE_PACKET_NAME_EVENTS: &[u8] = b"EVENTS\0";
static ARM_SPE_PACKET_NAME_DATA_SOURCE: &[u8] = b"DATA-SOURCE\0";

static arm_spe_packet_name: [*const c_char; 10] = [
    ptr::null(),
    ARM_SPE_PACKET_NAME_PAD.as_ptr() as *const c_char,
    ARM_SPE_PACKET_NAME_END.as_ptr() as *const c_char,
    ARM_SPE_PACKET_NAME_TIMESTAMP.as_ptr() as *const c_char,
    ARM_SPE_PACKET_NAME_ADDRESS.as_ptr() as *const c_char,
    ARM_SPE_PACKET_NAME_COUNTER.as_ptr() as *const c_char,
    ARM_SPE_PACKET_NAME_CONTEXT.as_ptr() as *const c_char,
    ARM_SPE_PACKET_NAME_OP_TYPE.as_ptr() as *const c_char,
    ARM_SPE_PACKET_NAME_EVENTS.as_ptr() as *const c_char,
    ARM_SPE_PACKET_NAME_DATA_SOURCE.as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn arm_spe_pkt_name(type_: arm_spe_pkt_type) -> *const c_char {
    arm_spe_packet_name[type_ as usize]
}

fn GENMASK_ULL(h: c_uint, l: c_uint) -> u64 {
    ((!0u64) << l) & ((!0u64) >> (63 - h))
}

fn BIT_ULL(nr: c_int) -> u64 {
    1u64 << nr
}

unsafe fn get_unaligned_le16(p: *const u8) -> u16 {
    u16::from_le_bytes([*p.add(0), *p.add(1)])
}

unsafe fn get_unaligned_le32(p: *const u8) -> u32 {
    u32::from_le_bytes([*p.add(0), *p.add(1), *p.add(2), *p.add(3)])
}

unsafe fn get_unaligned_le64(p: *const u8) -> u64 {
    u64::from_le_bytes([
        *p.add(0), *p.add(1), *p.add(2), *p.add(3),
        *p.add(4), *p.add(5), *p.add(6), *p.add(7),
    ])
}

/*
 * Extracts the field "sz" from header bits and converts to bytes:
 *   00 : byte (1)
 *   01 : halfword (2)
 *   10 : word (4)
 *   11 : doubleword (8)
 */
unsafe fn arm_spe_payload_len(hdr: u8) -> c_uint {
    1u32 << (((hdr as u64 & GENMASK_ULL(5, 4)) >> 4) as c_uint)
}

unsafe fn arm_spe_get_payload(
    mut buf: *const u8,
    len: size_t,
    ext_hdr: u8,
    packet: *mut arm_spe_pkt,
) -> c_int {
    let payload_len = arm_spe_payload_len(*buf.add(ext_hdr as usize)) as size_t;

    if len < 1 + ext_hdr as size_t + payload_len {
        return ARM_SPE_NEED_MORE_BYTES;
    }

    buf = buf.add(1 + ext_hdr as usize);

    match payload_len {
        1 => (*packet).payload = *buf as u64,
        2 => (*packet).payload = get_unaligned_le16(buf) as u64,
        4 => (*packet).payload = get_unaligned_le32(buf) as u64,
        8 => (*packet).payload = get_unaligned_le64(buf),
        _ => return ARM_SPE_BAD_PACKET,
    }

    (1 + ext_hdr as size_t + payload_len) as c_int
}

unsafe fn arm_spe_get_pad(packet: *mut arm_spe_pkt) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_PAD;
    1
}

unsafe fn arm_spe_get_alignment(buf: *const u8, len: size_t, packet: *mut arm_spe_pkt) -> c_int {
    let alignment: c_uint = 1 << ((*buf.add(0) & 0xf) + 1);

    if len < alignment as size_t {
        return ARM_SPE_NEED_MORE_BYTES;
    }

    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_PAD;
    (alignment as usize - ((buf as usize) & (alignment as usize - 1))) as c_int
}

unsafe fn arm_spe_get_end(packet: *mut arm_spe_pkt) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_END;
    1
}

unsafe fn arm_spe_get_timestamp(buf: *const u8, len: size_t, packet: *mut arm_spe_pkt) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_TIMESTAMP;
    arm_spe_get_payload(buf, len, 0, packet)
}

unsafe fn arm_spe_get_events(buf: *const u8, len: size_t, packet: *mut arm_spe_pkt) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_EVENTS;

    /* we use index to identify Events with a less number of
     * comparisons in arm_spe_pkt_desc(): E.g., the LLC-ACCESS,
     * LLC-REFILL, and REMOTE-ACCESS events are identified if
     * index > 1.
     */
    (*packet).index = arm_spe_payload_len(*buf.add(0)) as c_int;

    arm_spe_get_payload(buf, len, 0, packet)
}

unsafe fn arm_spe_get_data_source(buf: *const u8, len: size_t, packet: *mut arm_spe_pkt) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_DATA_SOURCE;
    arm_spe_get_payload(buf, len, 0, packet)
}

unsafe fn arm_spe_get_context(buf: *const u8, len: size_t, packet: *mut arm_spe_pkt) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_CONTEXT;
    (*packet).index = SPE_CTX_PKT_HDR_INDEX(*buf.add(0));
    arm_spe_get_payload(buf, len, 0, packet)
}

unsafe fn arm_spe_get_op_type(buf: *const u8, len: size_t, packet: *mut arm_spe_pkt) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_OP_TYPE;
    (*packet).index = SPE_OP_PKT_HDR_CLASS(*buf.add(0));
    arm_spe_get_payload(buf, len, 0, packet)
}

unsafe fn arm_spe_get_counter(
    buf: *const u8,
    len: size_t,
    ext_hdr: u8,
    packet: *mut arm_spe_pkt,
) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_COUNTER;

    if ext_hdr != 0 {
        (*packet).index = SPE_HDR_EXTENDED_INDEX(*buf.add(0), *buf.add(1));
    } else {
        (*packet).index = SPE_HDR_SHORT_INDEX(*buf.add(0));
    }

    arm_spe_get_payload(buf, len, ext_hdr, packet)
}

unsafe fn arm_spe_get_addr(
    buf: *const u8,
    len: size_t,
    ext_hdr: u8,
    packet: *mut arm_spe_pkt,
) -> c_int {
    (*packet).type_ = arm_spe_pkt_type::ARM_SPE_ADDRESS;

    if ext_hdr != 0 {
        (*packet).index = SPE_HDR_EXTENDED_INDEX(*buf.add(0), *buf.add(1));
    } else {
        (*packet).index = SPE_HDR_SHORT_INDEX(*buf.add(0));
    }

    arm_spe_get_payload(buf, len, ext_hdr, packet)
}

unsafe fn arm_spe_do_get_packet(buf: *const u8, len: size_t, packet: *mut arm_spe_pkt) -> c_int {
    let mut hdr: c_uint;
    let mut ext_hdr: u8 = 0;

    ptr::write_bytes(packet as *mut u8, 0, mem::size_of::<arm_spe_pkt>());

    if len == 0 {
        return ARM_SPE_NEED_MORE_BYTES;
    }

    hdr = *buf.add(0) as c_uint;

    if hdr == SPE_HEADER0_PAD {
        return arm_spe_get_pad(packet);
    }

    if hdr == SPE_HEADER0_END {
        /* no timestamp at end of record */
        return arm_spe_get_end(packet);
    }

    if hdr == SPE_HEADER0_TIMESTAMP {
        return arm_spe_get_timestamp(buf, len, packet);
    }

    if (hdr & SPE_HEADER0_MASK1) == SPE_HEADER0_EVENTS {
        return arm_spe_get_events(buf, len, packet);
    }

    if (hdr & SPE_HEADER0_MASK1) == SPE_HEADER0_SOURCE {
        return arm_spe_get_data_source(buf, len, packet);
    }

    if (hdr & SPE_HEADER0_MASK2) == SPE_HEADER0_CONTEXT {
        return arm_spe_get_context(buf, len, packet);
    }

    if (hdr & SPE_HEADER0_MASK2) == SPE_HEADER0_OP_TYPE {
        return arm_spe_get_op_type(buf, len, packet);
    }

    if (hdr & SPE_HEADER0_MASK2) == SPE_HEADER0_EXTENDED {
        /* 16-bit extended format header */
        if len == 1 {
            return ARM_SPE_BAD_PACKET;
        }

        ext_hdr = 1;
        hdr = *buf.add(1) as c_uint;
        if hdr == SPE_HEADER1_ALIGNMENT {
            return arm_spe_get_alignment(buf, len, packet);
        }
    }

    /*
     * The short format header's byte 0 or the extended format header's
     * byte 1 has been assigned to 'hdr', which uses the same encoding for
     * address packet and counter packet, so don't need to distinguish if
     * it's short format or extended format and handle in once.
     */
    if (hdr & SPE_HEADER0_MASK3) == SPE_HEADER0_ADDRESS {
        return arm_spe_get_addr(buf, len, ext_hdr, packet);
    }

    if (hdr & SPE_HEADER0_MASK3) == SPE_HEADER0_COUNTER {
        return arm_spe_get_counter(buf, len, ext_hdr, packet);
    }

    ARM_SPE_BAD_PACKET
}

#[no_mangle]
pub unsafe extern "C" fn arm_spe_get_packet(
    buf: *const u8,
    len: size_t,
    packet: *mut arm_spe_pkt,
    midr: u64,
) -> c_int {
    let mut ret: c_int;

    ret = arm_spe_do_get_packet(buf, len, packet);
    (*packet).midr = midr;
    /* put multiple consecutive PADs on the same line, up to
     * the fixed-width output format of 16 bytes per line.
     */
    if ret > 0 && (*packet).type_ == arm_spe_pkt_type::ARM_SPE_PAD {
        while ret < 16 && len > ret as size_t && *buf.add(ret as usize) == 0 {
            ret += 1;
        }
    }

    ret
}

unsafe fn arm_spe_pkt_out_string(
    err: *mut c_int,
    buf_p: *mut *mut c_char,
    blen: *mut size_t,
    s: &str,
) -> c_int {
    let mut ret: c_int;

    /* Bail out if any error occurred */
    if !err.is_null() && *err != 0 {
        return *err;
    }

    let bytes = s.as_bytes();
    ret = bytes.len() as c_int;

    if ret < 0 {
        if !err.is_null() && *err == 0 {
            *err = ret;
        }

    /*
     * A return value of *blen or more means that the output was
     * truncated and the buffer is overrun.
     */
    } else if ret as size_t >= *blen {
        *(*buf_p).add(*blen - 1) = b'\0' as c_char;

        /*
         * Set *err to 'ret' to avoid overflow if tries to
         * fill this buffer sequentially.
         */
        if !err.is_null() && *err == 0 {
            *err = ret;
        }
    } else {
        ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, *buf_p, bytes.len());
        *(*buf_p).add(bytes.len()) = b'\0' as c_char;
        *buf_p = (*buf_p).add(ret as usize);
        *blen -= ret as size_t;
    }

    ret
}

#[repr(C)]
struct ev_string {
    event: u8,
    desc: *const c_char,
}

static DESC_EXCEPTION_GEN: &[u8] = b"EXCEPTION-GEN\0";
static DESC_RETIRED: &[u8] = b"RETIRED\0";
static DESC_L1D_ACCESS: &[u8] = b"L1D-ACCESS\0";
static DESC_L1D_REFILL: &[u8] = b"L1D-REFILL\0";
static DESC_TLB_ACCESS: &[u8] = b"TLB-ACCESS\0";
static DESC_TLB_REFILL: &[u8] = b"TLB-REFILL\0";
static DESC_NOT_TAKEN: &[u8] = b"NOT-TAKEN\0";
static DESC_MISPRED: &[u8] = b"MISPRED\0";
static DESC_LLC_ACCESS: &[u8] = b"LLC-ACCESS\0";
static DESC_LLC_REFILL: &[u8] = b"LLC-REFILL\0";
static DESC_REMOTE_ACCESS: &[u8] = b"REMOTE-ACCESS\0";
static DESC_ALIGNMENT: &[u8] = b"ALIGNMENT\0";
static DESC_TXN: &[u8] = b"TXN\0";
static DESC_SVE_PARTIAL_PRED: &[u8] = b"SVE-PARTIAL-PRED\0";
static DESC_SVE_EMPTY_PRED: &[u8] = b"SVE-EMPTY-PRED\0";
static DESC_L2D_ACCESS: &[u8] = b"L2D-ACCESS\0";
static DESC_L2D_MISS: &[u8] = b"L2D-MISS\0";
static DESC_HITM: &[u8] = b"HITM\0";
static DESC_LFB: &[u8] = b"LFB\0";
static DESC_SNOOPED: &[u8] = b"SNOOPED\0";
static DESC_STREAMING_SVE: &[u8] = b"STREAMING-SVE\0";
static DESC_SMCU: &[u8] = b"SMCU\0";
static DESC_LATE_PREFETCH: &[u8] = b"LATE-PREFETCH\0";

static common_ev_strings: [ev_string; 23] = [
    ev_string { event: EV_EXCEPTION_GEN, desc: DESC_EXCEPTION_GEN.as_ptr() as *const c_char },
    ev_string { event: EV_RETIRED, desc: DESC_RETIRED.as_ptr() as *const c_char },
    ev_string { event: EV_L1D_ACCESS, desc: DESC_L1D_ACCESS.as_ptr() as *const c_char },
    ev_string { event: EV_L1D_REFILL, desc: DESC_L1D_REFILL.as_ptr() as *const c_char },
    ev_string { event: EV_TLB_ACCESS, desc: DESC_TLB_ACCESS.as_ptr() as *const c_char },
    ev_string { event: EV_TLB_WALK, desc: DESC_TLB_REFILL.as_ptr() as *const c_char },
    ev_string { event: EV_NOT_TAKEN, desc: DESC_NOT_TAKEN.as_ptr() as *const c_char },
    ev_string { event: EV_MISPRED, desc: DESC_MISPRED.as_ptr() as *const c_char },
    ev_string { event: EV_LLC_ACCESS, desc: DESC_LLC_ACCESS.as_ptr() as *const c_char },
    ev_string { event: EV_LLC_MISS, desc: DESC_LLC_REFILL.as_ptr() as *const c_char },
    ev_string { event: EV_REMOTE_ACCESS, desc: DESC_REMOTE_ACCESS.as_ptr() as *const c_char },
    ev_string { event: EV_ALIGNMENT, desc: DESC_ALIGNMENT.as_ptr() as *const c_char },
    ev_string { event: EV_TRANSACTIONAL, desc: DESC_TXN.as_ptr() as *const c_char },
    ev_string { event: EV_PARTIAL_PREDICATE, desc: DESC_SVE_PARTIAL_PRED.as_ptr() as *const c_char },
    ev_string { event: EV_EMPTY_PREDICATE, desc: DESC_SVE_EMPTY_PRED.as_ptr() as *const c_char },
    ev_string { event: EV_L2D_ACCESS, desc: DESC_L2D_ACCESS.as_ptr() as *const c_char },
    ev_string { event: EV_L2D_MISS, desc: DESC_L2D_MISS.as_ptr() as *const c_char },
    ev_string { event: EV_CACHE_DATA_MODIFIED, desc: DESC_HITM.as_ptr() as *const c_char },
    ev_string { event: EV_RECENTLY_FETCHED, desc: DESC_LFB.as_ptr() as *const c_char },
    ev_string { event: EV_DATA_SNOOPED, desc: DESC_SNOOPED.as_ptr() as *const c_char },
    ev_string { event: EV_STREAMING_SVE_MODE, desc: DESC_STREAMING_SVE.as_ptr() as *const c_char },
    ev_string { event: EV_SMCU, desc: DESC_SMCU.as_ptr() as *const c_char },
    ev_string { event: 0, desc: ptr::null() },
];

static n1_event_strings: [ev_string; 2] = [
    ev_string { event: 12, desc: DESC_LATE_PREFETCH.as_ptr() as *const c_char },
    ev_string { event: 0, desc: ptr::null() },
];

unsafe fn cstr_to_str(p: *const c_char) -> &'static str {
    let mut len = 0usize;
    while *p.add(len) != 0 {
        len += 1;
    }
    core::str::from_utf8_unchecked(core::slice::from_raw_parts(p as *const u8, len))
}

unsafe fn out_fmt(err: *mut c_int, buf: *mut *mut c_char, buf_len: *mut size_t, args: std::fmt::Arguments) -> c_int {
    arm_spe_pkt_out_string(err, buf, buf_len, &format!("{}", args))
}

unsafe fn print_event_list(
    err: *mut c_int,
    buf: *mut *mut c_char,
    buf_len: *mut size_t,
    ev_strings: *const ev_string,
    mut payload: u64,
) -> u64 {
    let mut ev = ev_strings;
    while !(*ev).desc.is_null() {
        if payload & BIT_ULL((*ev).event as c_int) != 0 {
            out_fmt(err, buf, buf_len, format_args!(" {}", cstr_to_str((*ev).desc)));
        }
        payload &= !BIT_ULL((*ev).event as c_int);
        ev = ev.add(1);
    }
    payload
}

#[repr(C)]
struct event_print_handle {
    midr_ranges: *const midr_range,
    ev_strings: *const ev_string,
}

static n1_event_encoding_cpus: [midr_range; 2] = [
    MIDR_ALL_VERSIONS(MIDR_NEOVERSE_N1),
    midr_range { _private: [] },
];

static event_print_handles: [event_print_handle; 1] = [
    event_print_handle {
        midr_ranges: n1_event_encoding_cpus.as_ptr(),
        ev_strings: n1_event_strings.as_ptr(),
    },
];

unsafe fn arm_spe_pkt_desc_event(packet: *const arm_spe_pkt, mut buf: *mut c_char, mut buf_len: size_t) -> c_int {
    let mut payload = (*packet).payload;
    let mut err: c_int = 0;

    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, "EV");
    payload = print_event_list(&mut err, &mut buf, &mut buf_len, common_ev_strings.as_ptr(), payload);

    /* Try to decode IMPDEF bits for known CPUs */
    for i in 0..event_print_handles.len() {
        if is_midr_in_range_list((*packet).midr, event_print_handles[i].midr_ranges) {
            payload = print_event_list(
                &mut err,
                &mut buf,
                &mut buf_len,
                event_print_handles[i].ev_strings,
                payload,
            );
        }
    }

    /*
     * Print remaining IMPDEF bits that weren't printed above as raw
     * "IMPDEF:1,2,3,4" etc.
     */
    if payload != 0 {
        arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " IMPDEF:");
        for i in 0..64 {
            let sep = if payload & (payload - 1) != 0 { "," } else { "" };

            if payload & BIT_ULL(i) != 0 {
                out_fmt(&mut err, &mut buf, &mut buf_len, format_args!("{}{}", i, sep));
                payload &= !BIT_ULL(i);
            }
        }
    }

    err
}

unsafe fn arm_spe_pkt_desc_op_type(packet: *const arm_spe_pkt, mut buf: *mut c_char, mut buf_len: size_t) -> c_int {
    let payload = (*packet).payload;
    let mut err: c_int = 0;

    match (*packet).index {
        SPE_OP_PKT_HDR_CLASS_OTHER => {
            if SPE_OP_PKT_OTHER_SUBCLASS_SVE(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, "SVE-OTHER");

                /* SVE effective vector length */
                out_fmt(&mut err, &mut buf, &mut buf_len, format_args!(" EVLEN {}", SPE_OP_PKG_SVE_EVL(payload)));

                if payload & SPE_OP_PKT_SVE_FP != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " FP");
                }
                if payload & SPE_OP_PKT_SVE_PRED != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " PRED");
                }
            } else if SPE_OP_PKT_OTHER_SUBCLASS_SME(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, "SME-OTHER");

                /* SME effective vector length or tile size */
                out_fmt(&mut err, &mut buf, &mut buf_len, format_args!(" ETS {}", SPE_OP_PKG_SME_ETS(payload)));

                if payload & SPE_OP_PKT_OTHER_FP != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " FP");
                }
            } else if SPE_OP_PKT_OTHER_SUBCLASS_OTHER(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, "OTHER");
                if payload & SPE_OP_PKT_OTHER_ASE != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " ASE");
                }
                if payload & SPE_OP_PKT_OTHER_FP != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " FP");
                }
                out_fmt(
                    &mut err,
                    &mut buf,
                    &mut buf_len,
                    format_args!(" {}", if payload & SPE_OP_PKT_COND != 0 { "COND-SELECT" } else { "INSN-OTHER" }),
                );
            }
        }
        SPE_OP_PKT_HDR_CLASS_LD_ST_ATOMIC => {
            arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, if payload & 0x1 != 0 { "ST" } else { "LD" });

            if SPE_OP_PKT_LDST_SUBCLASS_EXTENDED(payload) {
                if payload & SPE_OP_PKT_AT != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " AT");
                }
                if payload & SPE_OP_PKT_EXCL != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " EXCL");
                }
                if payload & SPE_OP_PKT_AR != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " AR");
                }
            } else if SPE_OP_PKT_LDST_SUBCLASS_SIMD_FP(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " SIMD-FP");
            } else if SPE_OP_PKT_LDST_SUBCLASS_GP_REG(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " GP-REG");
            } else if SPE_OP_PKT_LDST_SUBCLASS_UNSPEC_REG(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " UNSPEC-REG");
            } else if SPE_OP_PKT_LDST_SUBCLASS_NV_SYSREG(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " NV-SYSREG");
            } else if SPE_OP_PKT_LDST_SUBCLASS_MTE_TAG(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " MTE-TAG");
            } else if SPE_OP_PKT_LDST_SUBCLASS_MEMCPY(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " MEMCPY");
            } else if SPE_OP_PKT_LDST_SUBCLASS_MEMSET(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " MEMSET");
            } else if SPE_OP_PKT_LDST_SUBCLASS_SVE_SME_REG(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " SVE-SME-REG");

                /* SVE effective vector length */
                out_fmt(&mut err, &mut buf, &mut buf_len, format_args!(" EVLEN {}", SPE_OP_PKG_SVE_EVL(payload)));

                if payload & SPE_OP_PKT_SVE_PRED != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " PRED");
                }
                if payload & SPE_OP_PKT_SVE_SG != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " SG");
                }
            } else if SPE_OP_PKT_LDST_SUBCLASS_GCS(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " GCS");
                if payload & SPE_OP_PKT_GCS_COMM != 0 {
                    arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " COMM");
                }
            }
        }
        SPE_OP_PKT_HDR_CLASS_BR_ERET => {
            arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, "B");

            if payload & SPE_OP_PKT_COND != 0 {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " COND");
            }
            if payload & SPE_OP_PKT_INDIRECT_BRANCH != 0 {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " IND");
            }
            if payload & SPE_OP_PKT_GCS != 0 {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " GCS");
            }
            if SPE_OP_PKT_CR_BL(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " CR-BL");
            }
            if SPE_OP_PKT_CR_RET(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " CR-RET");
            }
            if SPE_OP_PKT_CR_NON_BL_RET(payload) {
                arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, " CR-NON-BL-RET");
            }
        }
        _ => {
            /* Unknown index */
            err = -1;
        }
    }

    err
}

unsafe fn arm_spe_pkt_desc_addr(packet: *const arm_spe_pkt, mut buf: *mut c_char, mut buf_len: size_t) -> c_int {
    let mut ns: c_int;
    let mut el: c_int;
    let idx: c_int = (*packet).index;
    let mut ch: c_int;
    let mut pat: c_int;
    let mut payload: u64 = (*packet).payload;
    let mut err: c_int = 0;
    let idx_name = ["PC", "TGT", "VA", "PA", "PBT"];

    match idx {
        SPE_ADDR_PKT_HDR_INDEX_INS | SPE_ADDR_PKT_HDR_INDEX_BRANCH | SPE_ADDR_PKT_HDR_INDEX_PREV_BRANCH => {
            ns = if SPE_ADDR_PKT_GET_NS(payload) != 0 { 1 } else { 0 };
            el = SPE_ADDR_PKT_GET_EL(payload);
            payload = SPE_ADDR_PKT_ADDR_GET_BYTES_0_6(payload);
            out_fmt(
                &mut err,
                &mut buf,
                &mut buf_len,
                format_args!("{} 0x{:x} el{} ns={}", idx_name[idx as usize], payload, el, ns),
            );
        }
        SPE_ADDR_PKT_HDR_INDEX_DATA_VIRT => {
            out_fmt(&mut err, &mut buf, &mut buf_len, format_args!("VA 0x{:x}", payload));
        }
        SPE_ADDR_PKT_HDR_INDEX_DATA_PHYS => {
            ns = if SPE_ADDR_PKT_GET_NS(payload) != 0 { 1 } else { 0 };
            ch = if SPE_ADDR_PKT_GET_CH(payload) != 0 { 1 } else { 0 };
            pat = SPE_ADDR_PKT_GET_PAT(payload);
            payload = SPE_ADDR_PKT_ADDR_GET_BYTES_0_6(payload);
            out_fmt(
                &mut err,
                &mut buf,
                &mut buf_len,
                format_args!("PA 0x{:x} ns={} ch={} pat={:x}", payload, ns, ch, pat),
            );
        }
        _ => {
            /* Unknown index */
            err = -1;
        }
    }

    err
}

unsafe fn arm_spe_pkt_desc_counter(packet: *const arm_spe_pkt, mut buf: *mut c_char, mut buf_len: size_t) -> c_int {
    let payload: u64 = (*packet).payload;
    let name = cstr_to_str(arm_spe_pkt_name((*packet).type_));
    let mut err: c_int = 0;

    out_fmt(&mut err, &mut buf, &mut buf_len, format_args!("{} {} ", name, payload as u16));

    match (*packet).index {
        SPE_CNT_PKT_HDR_INDEX_TOTAL_LAT => {
            arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, "TOT");
        }
        SPE_CNT_PKT_HDR_INDEX_ISSUE_LAT => {
            arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, "ISSUE");
        }
        SPE_CNT_PKT_HDR_INDEX_TRANS_LAT => {
            arm_spe_pkt_out_string(&mut err, &mut buf, &mut buf_len, "XLAT");
        }
        _ => {}
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn arm_spe_pkt_desc(packet: *const arm_spe_pkt, mut buf: *mut c_char, mut buf_len: size_t) -> c_int {
    let idx: c_int = (*packet).index;
    let payload: c_ulonglong = (*packet).payload as c_ulonglong;
    let name = cstr_to_str(arm_spe_pkt_name((*packet).type_));
    let mut buf_orig = buf;
    let mut blen: size_t = buf_len;
    let mut err: c_int = 0;

    match (*packet).type_ {
        arm_spe_pkt_type::ARM_SPE_BAD |
        arm_spe_pkt_type::ARM_SPE_PAD |
        arm_spe_pkt_type::ARM_SPE_END => {
            out_fmt(&mut err, &mut buf, &mut blen, format_args!("{}", name));
        }
        arm_spe_pkt_type::ARM_SPE_EVENTS => {
            err = arm_spe_pkt_desc_event(packet, buf, buf_len);
        }
        arm_spe_pkt_type::ARM_SPE_OP_TYPE => {
            err = arm_spe_pkt_desc_op_type(packet, buf, buf_len);
        }
        arm_spe_pkt_type::ARM_SPE_DATA_SOURCE |
        arm_spe_pkt_type::ARM_SPE_TIMESTAMP => {
            out_fmt(&mut err, &mut buf, &mut blen, format_args!("{} {}", name, payload as i64));
        }
        arm_spe_pkt_type::ARM_SPE_ADDRESS => {
            err = arm_spe_pkt_desc_addr(packet, buf, buf_len);
        }
        arm_spe_pkt_type::ARM_SPE_CONTEXT => {
            out_fmt(&mut err, &mut buf, &mut blen, format_args!("{} 0x{:x} el{}", name, payload as c_ulong, idx + 1));
        }
        arm_spe_pkt_type::ARM_SPE_COUNTER => {
            err = arm_spe_pkt_desc_counter(packet, buf, buf_len);
        }
    }

    /* Output raw data if detect any error */
    if err != 0 {
        err = 0;
        out_fmt(
            &mut err,
            &mut buf_orig,
            &mut buf_len,
            format_args!("{} 0x{:x} ({})", name, payload, (*packet).index),
        );
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
