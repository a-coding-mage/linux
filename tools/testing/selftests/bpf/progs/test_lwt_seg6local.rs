#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

// Translated from includes:
// <stddef.h>, <inttypes.h>, <errno.h>, <linux/seg6_local.h>,
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>,
// and "bpf_compiler.h".

use core::mem::size_of;
use core::ptr;

const EINVAL: i32 = 22;
const SR6_FLAG_ALERT: u8 = 1 << 4;

// External constants supplied by Linux/BPF headers.
const SR6_TLV_PADDING: u8 = 4;
const SR6_TLV_HMAC: u8 = 5;
const SR6_TLV_EGRESS: u8 = 2;
const SEG6_LOCAL_ACTION_END_X: i32 = 2;
const SEG6_LOCAL_ACTION_END_T: i32 = 3;
const BPF_DROP: i32 = 2;
const BPF_REDIRECT: i32 = 7;
const BPF_OK: i32 = 0;

#[repr(C)]
pub struct __sk_buff {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C, packed)]
pub struct ip6_t {
    // C source used bitfields:
    // unsigned int ver:4; unsigned int priority:8; unsigned int flow_label:20;
    pub ver_priority_flow_label: u32,
    pub payload_len: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub src_hi: u64,
    pub src_lo: u64,
    pub dst_hi: u64,
    pub dst_lo: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ip6_addr_t {
    pub hi: u64,
    pub lo: u64,
}

#[repr(C, packed)]
pub struct ip6_srh_t {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub type_: u8,
    pub segments_left: u8,
    pub first_segment: u8,
    pub flags: u8,
    pub tag: u16,
    // Flexible array member in C: struct ip6_addr_t segments[0];
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sr6_tlv_t {
    pub type_: u8,
    pub len: u8,
    // Flexible array member in C: unsigned char value[0];
}

unsafe extern "C" {
    fn bpf_lwt_seg6_adjust_srh(skb: *mut __sk_buff, offset: u32, delta: i32) -> i32;
    fn bpf_lwt_seg6_store_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        from: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;
    fn bpf_skb_load_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        to: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;
    fn bpf_lwt_push_encap(
        skb: *mut __sk_buff,
        typ: u32,
        hdr: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;
    fn bpf_lwt_seg6_action(
        skb: *mut __sk_buff,
        action: i32,
        param: *mut core::ffi::c_void,
        param_len: u32,
    ) -> i32;
}

#[inline(always)]
fn bpf_be64_to_cpu(x: u64) -> u64 {
    u64::from_be(x)
}

#[inline(always)]
fn bpf_cpu_to_be64(x: u64) -> u64 {
    x.to_be()
}

#[inline(always)]
fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[inline(always)]
unsafe fn get_srh(skb: *mut __sk_buff) -> *mut ip6_srh_t {
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let mut cursor = (*skb).data as usize as *mut u8;
    let ipver = cursor as *mut u8;

    if (ipver as usize + size_of::<u8>()) > data_end as usize {
        return ptr::null_mut();
    }

    if ((*ipver >> 4) != 6) {
        return ptr::null_mut();
    }

    let ip = cursor as *mut ip6_t;
    cursor = cursor.add(size_of::<ip6_t>());
    if (ip as usize + size_of::<ip6_t>()) > data_end as usize {
        return ptr::null_mut();
    }

    if (*ip).next_header != 43 {
        return ptr::null_mut();
    }

    let srh = cursor as *mut ip6_srh_t;
    cursor = cursor.add(size_of::<ip6_srh_t>());
    if (srh as usize + size_of::<ip6_srh_t>()) > data_end as usize {
        return ptr::null_mut();
    }

    if (*srh).type_ != 4 {
        return ptr::null_mut();
    }

    srh
}

#[inline(always)]
unsafe fn update_tlv_pad(
    skb: *mut __sk_buff,
    new_pad: u32,
    old_pad: u32,
    pad_off: u32,
) -> i32 {
    let mut err: i32;

    if new_pad != old_pad {
        err = bpf_lwt_seg6_adjust_srh(skb, pad_off, new_pad as i32 - old_pad as i32);
        if err != 0 {
            return err;
        }
    }

    if new_pad > 0 {
        let mut pad_tlv_buf: [u8; 16] = [0; 16];
        let pad_tlv = pad_tlv_buf.as_mut_ptr() as *mut sr6_tlv_t;

        (*pad_tlv).type_ = SR6_TLV_PADDING;
        (*pad_tlv).len = (new_pad - 2) as u8;

        err = bpf_lwt_seg6_store_bytes(
            skb,
            pad_off,
            pad_tlv_buf.as_mut_ptr() as *mut core::ffi::c_void,
            new_pad,
        );
        if err != 0 {
            return err;
        }
    }

    0
}

#[inline(always)]
unsafe fn is_valid_tlv_boundary(
    skb: *mut __sk_buff,
    srh: *mut ip6_srh_t,
    tlv_off: *mut u32,
    pad_size: *mut u32,
    pad_off: *mut u32,
) -> i32 {
    let srh_off: u32 = (srh as usize - (*skb).data as usize) as u32;
    // cur_off = end of segments, start of possible TLVs
    let mut cur_off: u32 = srh_off
        + size_of::<ip6_srh_t>() as u32
        + size_of::<ip6_addr_t>() as u32 * ((*srh).first_segment as u32 + 1);
    let mut offset_valid: i32 = 0;
    let mut err: i32;

    *pad_off = 0;

    // we can only go as far as ~10 TLVs due to the BPF max stack size
    // C source requested full loop unrolling.
    for _i in 0..10 {
        let mut tlv: sr6_tlv_t = sr6_tlv_t { type_: 0, len: 0 };

        if cur_off == *tlv_off {
            offset_valid = 1;
        }

        if cur_off >= srh_off + (((*srh).hdrlen as u32 + 1) << 3) {
            break;
        }

        err = bpf_skb_load_bytes(
            skb,
            cur_off,
            &mut tlv as *mut sr6_tlv_t as *mut core::ffi::c_void,
            size_of::<sr6_tlv_t>() as u32,
        );
        if err != 0 {
            return err;
        }

        if tlv.type_ == SR6_TLV_PADDING {
            *pad_size = tlv.len as u32 + size_of::<sr6_tlv_t>() as u32;
            *pad_off = cur_off;

            if *tlv_off == srh_off {
                *tlv_off = cur_off;
                offset_valid = 1;
            }
            break;
        } else if tlv.type_ == SR6_TLV_HMAC {
            break;
        }

        cur_off += size_of::<sr6_tlv_t>() as u32 + tlv.len as u32;
    } // we reached the padding or HMAC TLVs, or the end of the SRH

    if *pad_off == 0 {
        *pad_off = cur_off;
    }

    if *tlv_off == u32::MAX {
        *tlv_off = cur_off;
    } else if offset_valid == 0 {
        return -EINVAL;
    }

    0
}

#[inline(always)]
unsafe fn add_tlv(
    skb: *mut __sk_buff,
    srh: *mut ip6_srh_t,
    mut tlv_off: u32,
    itlv: *mut sr6_tlv_t,
    tlv_size: u8,
) -> i32 {
    let srh_off: u32 = (srh as usize - (*skb).data as usize) as u32;
    let mut len_remaining: u8;
    let mut new_pad: u8;
    let mut pad_off: u32 = 0;
    let mut pad_size: u32 = 0;
    let partial_srh_len: u32;
    let mut err: i32;

    if tlv_off != u32::MAX {
        tlv_off += srh_off;
    }

    if (*itlv).type_ == SR6_TLV_PADDING || (*itlv).type_ == SR6_TLV_HMAC {
        return -EINVAL;
    }

    err = is_valid_tlv_boundary(skb, srh, &mut tlv_off, &mut pad_size, &mut pad_off);
    if err != 0 {
        return err;
    }

    err = bpf_lwt_seg6_adjust_srh(
        skb,
        tlv_off,
        (size_of::<sr6_tlv_t>() as u32 + (*itlv).len as u32) as i32,
    );
    if err != 0 {
        return err;
    }

    err = bpf_lwt_seg6_store_bytes(
        skb,
        tlv_off,
        itlv as *mut core::ffi::c_void,
        tlv_size as u32,
    );
    if err != 0 {
        return err;
    }

    // the following can't be moved inside update_tlv_pad because the
    // bpf verifier has some issues with it
    pad_off += size_of::<sr6_tlv_t>() as u32 + (*itlv).len as u32;
    partial_srh_len = pad_off - srh_off;
    len_remaining = (partial_srh_len % 8) as u8;
    new_pad = 8 - len_remaining;

    if new_pad == 1 {
        // cannot pad for 1 byte only
        new_pad = 9;
    } else if new_pad == 8 {
        new_pad = 0;
    }

    update_tlv_pad(skb, new_pad as u32, pad_size, pad_off)
}

#[inline(always)]
unsafe fn delete_tlv(skb: *mut __sk_buff, srh: *mut ip6_srh_t, mut tlv_off: u32) -> i32 {
    let srh_off: u32 = (srh as usize - (*skb).data as usize) as u32;
    let mut len_remaining: u8;
    let mut new_pad: u8;
    let partial_srh_len: u32;
    let mut pad_off: u32 = 0;
    let mut pad_size: u32 = 0;
    let mut tlv: sr6_tlv_t = sr6_tlv_t { type_: 0, len: 0 };
    let mut err: i32;

    tlv_off += srh_off;

    err = is_valid_tlv_boundary(skb, srh, &mut tlv_off, &mut pad_size, &mut pad_off);
    if err != 0 {
        return err;
    }

    err = bpf_skb_load_bytes(
        skb,
        tlv_off,
        &mut tlv as *mut sr6_tlv_t as *mut core::ffi::c_void,
        size_of::<sr6_tlv_t>() as u32,
    );
    if err != 0 {
        return err;
    }

    err = bpf_lwt_seg6_adjust_srh(
        skb,
        tlv_off,
        -((size_of::<sr6_tlv_t>() as u32 + tlv.len as u32) as i32),
    );
    if err != 0 {
        return err;
    }

    pad_off -= size_of::<sr6_tlv_t>() as u32 + tlv.len as u32;
    partial_srh_len = pad_off - srh_off;
    len_remaining = (partial_srh_len % 8) as u8;
    new_pad = 8 - len_remaining;
    if new_pad == 1 {
        // cannot pad for 1 byte only
        new_pad = 9;
    } else if new_pad == 8 {
        new_pad = 0;
    }

    update_tlv_pad(skb, new_pad as u32, pad_size, pad_off)
}

#[inline(always)]
unsafe fn has_egr_tlv(skb: *mut __sk_buff, srh: *mut ip6_srh_t) -> i32 {
    let tlv_offset: i32 = size_of::<ip6_t>() as i32
        + size_of::<ip6_srh_t>() as i32
        + (((*srh).first_segment as i32 + 1) << 4);
    let mut tlv: sr6_tlv_t = sr6_tlv_t { type_: 0, len: 0 };

    if bpf_skb_load_bytes(
        skb,
        tlv_offset as u32,
        &mut tlv as *mut sr6_tlv_t as *mut core::ffi::c_void,
        size_of::<sr6_tlv_t>() as u32,
    ) != 0
    {
        return 0;
    }

    if tlv.type_ == SR6_TLV_EGRESS && tlv.len == 18 {
        let mut egr_addr: ip6_addr_t = ip6_addr_t { hi: 0, lo: 0 };

        if bpf_skb_load_bytes(
            skb,
            (tlv_offset + 4) as u32,
            &mut egr_addr as *mut ip6_addr_t as *mut core::ffi::c_void,
            16,
        ) != 0
        {
            return 0;
        }

        // check if egress TLV value is correct
        if bpf_be64_to_cpu(egr_addr.hi) == 0xfd00000000000000
            && bpf_be64_to_cpu(egr_addr.lo) == 0x4
        {
            return 1;
        }
    }

    0
}

// This function will push a SRH with segments fd00::1, fd00::2, fd00::3,
// fd00::4
#[unsafe(no_mangle)]
#[unsafe(link_section = "encap_srh")]
pub unsafe extern "C" fn __encap_srh(skb: *mut __sk_buff) -> i32 {
    let hi: u64 = 0xfd00000000000000;
    let mut seg: *mut ip6_addr_t;
    let srh: *mut ip6_srh_t;
    let mut srh_buf: [u8; 72] = [0; 72]; // room for 4 segments
    let mut err: i32;

    srh = srh_buf.as_mut_ptr() as *mut ip6_srh_t;
    (*srh).nexthdr = 0;
    (*srh).hdrlen = 8;
    (*srh).type_ = 4;
    (*srh).segments_left = 3;
    (*srh).first_segment = 3;
    (*srh).flags = 0;
    (*srh).tag = 0;

    seg = (srh as *mut u8).add(size_of::<ip6_srh_t>()) as *mut ip6_addr_t;

    // C source requested full loop unrolling.
    let mut lo: u64 = 0;
    while lo < 4 {
        (*seg).lo = bpf_cpu_to_be64(4 - lo);
        (*seg).hi = bpf_cpu_to_be64(hi);
        seg = (seg as *mut u8).add(size_of::<ip6_addr_t>()) as *mut ip6_addr_t;
        lo += 1;
    }

    err = bpf_lwt_push_encap(
        skb,
        0,
        srh as *mut core::ffi::c_void,
        size_of::<[u8; 72]>() as u32,
    );
    if err != 0 {
        return BPF_DROP;
    }

    BPF_REDIRECT
}

// Add an Egress TLV fc00::4, add the flag A,
// and apply End.X action to fc42::1
#[unsafe(no_mangle)]
#[unsafe(link_section = "add_egr_x")]
pub unsafe extern "C" fn __add_egr_x(skb: *mut __sk_buff) -> i32 {
    let hi: u64 = 0xfc42000000000000;
    let lo: u64 = 0x1;
    let srh: *mut ip6_srh_t = get_srh(skb);
    let new_flags: u8 = SR6_FLAG_ALERT;
    let mut addr: ip6_addr_t = ip6_addr_t { hi: 0, lo: 0 };
    let mut err: i32;
    let mut offset: i32;

    if srh.is_null() {
        return BPF_DROP;
    }

    let mut tlv: [u8; 20] = [
        2, 18, 0, 0, 0xfd, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x4,
    ];

    err = add_tlv(
        skb,
        srh,
        (((*srh).hdrlen as u32 + 1) << 3),
        tlv.as_mut_ptr() as *mut sr6_tlv_t,
        20,
    );
    if err != 0 {
        return BPF_DROP;
    }

    offset = size_of::<ip6_t>() as i32 + 5;
    err = bpf_lwt_seg6_store_bytes(
        skb,
        offset as u32,
        &new_flags as *const u8 as *mut core::ffi::c_void,
        size_of::<u8>() as u32,
    );
    if err != 0 {
        return BPF_DROP;
    }

    addr.lo = bpf_cpu_to_be64(lo);
    addr.hi = bpf_cpu_to_be64(hi);
    err = bpf_lwt_seg6_action(
        skb,
        SEG6_LOCAL_ACTION_END_X,
        &mut addr as *mut ip6_addr_t as *mut core::ffi::c_void,
        size_of::<ip6_addr_t>() as u32,
    );
    if err != 0 {
        return BPF_DROP;
    }
    BPF_REDIRECT
}

// Pop the Egress TLV, reset the flags, change the tag 2442 and finally do a
// simple End action
#[unsafe(no_mangle)]
#[unsafe(link_section = "pop_egr")]
pub unsafe extern "C" fn __pop_egr(skb: *mut __sk_buff) -> i32 {
    let srh: *mut ip6_srh_t = get_srh(skb);
    let new_tag: u16 = bpf_htons(2442);
    let new_flags: u8 = 0;
    let mut err: i32;
    let mut offset: i32;

    if srh.is_null() {
        return BPF_DROP;
    }

    if (*srh).flags != SR6_FLAG_ALERT {
        return BPF_DROP;
    }

    if (*srh).hdrlen != 11 {
        // 4 segments + Egress TLV + Padding TLV
        return BPF_DROP;
    }

    if has_egr_tlv(skb, srh) == 0 {
        return BPF_DROP;
    }

    err = delete_tlv(skb, srh, 8 + ((*srh).first_segment as u32 + 1) * 16);
    if err != 0 {
        return BPF_DROP;
    }

    offset = size_of::<ip6_t>() as i32 + 5;
    if bpf_lwt_seg6_store_bytes(
        skb,
        offset as u32,
        &new_flags as *const u8 as *mut core::ffi::c_void,
        size_of::<u8>() as u32,
    ) != 0
    {
        return BPF_DROP;
    }

    offset = size_of::<ip6_t>() as i32 + 6;
    if bpf_lwt_seg6_store_bytes(
        skb,
        offset as u32,
        &new_tag as *const u16 as *mut core::ffi::c_void,
        size_of::<u16>() as u32,
    ) != 0
    {
        return BPF_DROP;
    }

    BPF_OK
}

// Inspect if the Egress TLV and flag have been removed, if the tag is correct,
// then apply a End.T action to reach the last segment
#[unsafe(no_mangle)]
#[unsafe(link_section = "inspect_t")]
pub unsafe extern "C" fn __inspect_t(skb: *mut __sk_buff) -> i32 {
    let srh: *mut ip6_srh_t = get_srh(skb);
    let mut table: i32 = 117;
    let mut err: i32;

    if srh.is_null() {
        return BPF_DROP;
    }

    if (*srh).flags != 0 {
        return BPF_DROP;
    }

    if (*srh).tag != bpf_htons(2442) {
        return BPF_DROP;
    }

    if (*srh).hdrlen != 8 {
        // 4 segments
        return BPF_DROP;
    }

    err = bpf_lwt_seg6_action(
        skb,
        SEG6_LOCAL_ACTION_END_T,
        &mut table as *mut i32 as *mut core::ffi::c_void,
        size_of::<i32>() as u32,
    );

    if err != 0 {
        return BPF_DROP;
    }

    BPF_REDIRECT
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static __license: [u8; 4] = *b"GPL\0";
