// SPDX-License-Identifier: GPL-2.0-only
/*
 * Authors:
 * (C) 2020 Alexander Aring <alex.aring@gmail.com>
 */

// C dependencies: <net/ipv6.h>, <net/rpl.h>
use crate::{in6_addr, ipv6_rpl_sr_hdr};

const IPV6_RPL_BEST_ADDR_COMPRESSION: u8 = 15;

#[inline]
unsafe fn ipv6_rpl_addr_decompress(
    dst: *mut in6_addr,
    daddr: *const in6_addr,
    post: *const core::ffi::c_void,
    pfx: u8,
) {
    let addr_len = core::mem::size_of::<in6_addr>();
    let pfx_len = pfx as usize;
    core::ptr::copy_nonoverlapping(
        daddr as *const u8,
        dst as *mut u8,
        pfx_len,
    );
    core::ptr::copy_nonoverlapping(
        post as *const u8,
        (*dst).s6_addr.as_mut_ptr().add(pfx_len),
        addr_len - pfx_len,
    );
}

#[inline]
unsafe fn ipv6_rpl_addr_compress(
    dst: *mut core::ffi::c_void,
    addr: *const in6_addr,
    pfx: u8,
) {
    let addr_len = core::mem::size_of::<in6_addr>();
    let pfx_len = pfx as usize;
    core::ptr::copy_nonoverlapping(
        (*addr).s6_addr.as_ptr().add(pfx_len),
        dst as *mut u8,
        addr_len - pfx_len,
    );
}

#[inline]
unsafe fn ipv6_rpl_segdata_pos(
    hdr: *const ipv6_rpl_sr_hdr,
    i: usize,
) -> *mut core::ffi::c_void {
    ((*hdr).rpl_segdata.as_ptr() as *mut u8)
        .add(i * (core::mem::size_of::<in6_addr>() - (*hdr).cmpri as usize))
        as *mut core::ffi::c_void
}

pub unsafe fn ipv6_rpl_srh_decompress(
    outhdr: *mut ipv6_rpl_sr_hdr,
    inhdr: *const ipv6_rpl_sr_hdr,
    daddr: *const in6_addr,
    n: u8,
) {
    (*outhdr).nexthdr = (*inhdr).nexthdr;
    (*outhdr).hdrlen = (((n as usize + 1) * core::mem::size_of::<in6_addr>()) >> 3) as _;
    (*outhdr).pad = 0;
    (*outhdr).type_ = (*inhdr).type_;
    (*outhdr).segments_left = (*inhdr).segments_left;
    (*outhdr).cmpri = 0;
    (*outhdr).cmpre = 0;

    for i in 0..n as usize {
        ipv6_rpl_addr_decompress(
            &mut (*outhdr).rpl_segaddr[i],
            daddr,
            ipv6_rpl_segdata_pos(inhdr, i),
            (*inhdr).cmpri,
        );
    }

    ipv6_rpl_addr_decompress(
        &mut (*outhdr).rpl_segaddr[n as usize],
        daddr,
        ipv6_rpl_segdata_pos(inhdr, n as usize),
        (*inhdr).cmpre,
    );
}

unsafe fn ipv6_rpl_srh_calc_cmpri(
    inhdr: *const ipv6_rpl_sr_hdr,
    daddr: *const in6_addr,
    n: u8,
) -> u8 {
    let addr_len = core::mem::size_of::<in6_addr>();
    for plen in 0..addr_len {
        for i in 0..n as usize {
            if (*daddr).s6_addr[plen] != (*inhdr).rpl_segaddr[i].s6_addr[plen] {
                return plen as u8;
            }
        }
    }
    IPV6_RPL_BEST_ADDR_COMPRESSION
}

unsafe fn ipv6_rpl_srh_calc_cmpre(
    daddr: *const in6_addr,
    last_segment: *const in6_addr,
) -> u8 {
    for plen in 0..core::mem::size_of::<in6_addr>() {
        if (*daddr).s6_addr[plen] != (*last_segment).s6_addr[plen] {
            return plen as u8;
        }
    }
    IPV6_RPL_BEST_ADDR_COMPRESSION
}

pub unsafe fn ipv6_rpl_srh_compress(
    outhdr: *mut ipv6_rpl_sr_hdr,
    inhdr: *const ipv6_rpl_sr_hdr,
    daddr: *const in6_addr,
    n: u8,
) {
    let cmpri = ipv6_rpl_srh_calc_cmpri(inhdr, daddr, n);
    let cmpre = ipv6_rpl_srh_calc_cmpre(daddr, &(*inhdr).rpl_segaddr[n as usize]);
    let seglen = n as usize * (core::mem::size_of::<in6_addr>() - cmpri as usize)
        + (core::mem::size_of::<in6_addr>() - cmpre as usize);

    (*outhdr).nexthdr = (*inhdr).nexthdr;
    (*outhdr).hdrlen = (seglen >> 3) as _;
    if seglen & 0x7 != 0 {
        (*outhdr).hdrlen += 1;
        (*outhdr).pad = (8 - (seglen & 0x7)) as _;
    } else {
        (*outhdr).pad = 0;
    }
    (*outhdr).type_ = (*inhdr).type_;
    (*outhdr).segments_left = (*inhdr).segments_left;
    (*outhdr).cmpri = cmpri;
    (*outhdr).cmpre = cmpre;

    for i in 0..n as usize {
        ipv6_rpl_addr_compress(
            ipv6_rpl_segdata_pos(outhdr, i),
            &(*inhdr).rpl_segaddr[i],
            cmpri,
        );
    }

    ipv6_rpl_addr_compress(
        ipv6_rpl_segdata_pos(outhdr, n as usize),
        &(*inhdr).rpl_segaddr[n as usize],
        cmpre,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
