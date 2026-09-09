/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <uapi/linux/dccp.h> dependent declarations.

/// Return the extended header immediately following the basic DCCP header.
#[inline]
pub unsafe fn dccp_hdrx(dh: *const dccp_hdr) -> *mut dccp_hdr_ext {
    (dh as *const u8).add(core::mem::size_of::<dccp_hdr>()) as *mut dccp_hdr_ext
}

#[inline]
pub unsafe fn __dccp_basic_hdr_len(dh: *const dccp_hdr) -> u32 {
    (core::mem::size_of::<dccp_hdr>()
        + if (*dh).dccph_x != 0 {
            core::mem::size_of::<dccp_hdr_ext>()
        } else {
            0
        }) as u32
}

#[inline]
pub unsafe fn dccp_hdr_seq(dh: *const dccp_hdr) -> __u64 {
    let mut seq_nr: __u64 = ntohs((*dh).dccph_seq) as __u64;

    if (*dh).dccph_x != 0 {
        seq_nr = (seq_nr << 32)
            .wrapping_add(ntohl((*dccp_hdrx(dh)).dccph_seq_low) as __u64);
    } else {
        seq_nr = seq_nr.wrapping_add(((*dh).dccph_seq2 as u32 as __u64) << 16);
    }

    seq_nr
}

#[inline]
pub unsafe fn __dccp_hdr_len(dh: *const dccp_hdr) -> u32 {
    __dccp_basic_hdr_len(dh) + dccp_packet_hdr_len((*dh).dccph_type)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
